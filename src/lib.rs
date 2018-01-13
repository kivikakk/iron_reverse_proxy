//! Some simple `BeforeMiddleware` to make using Iron behind a reverse proxy easier.
//!
//! ```
//! # extern crate iron;
//! # extern crate iron_reverse_proxy;
//! # fn main() {
//! use iron::prelude::*;
//! # let handler = |req: &mut Request| {
//! #   Ok(Response::new())
//! # };
//! let mut ch = Chain::new(handler);
//! ch.link_before(iron_reverse_proxy::ReverseProxyMiddleware);
//! # }
//! ```
//!
//! And you're done. Works particularly well with [`router`](https://crates.io/crates/router)'s [`url_for!` macro](https://docs.rs/router/0.6.0/router/macro.url_for.html), as it depends on the `Request.url` property, which this middleware modifies.

extern crate iron;
#[cfg(test)]
extern crate iron_test;
extern crate url;

use iron::prelude::*;
use iron::Url;
use iron::BeforeMiddleware;
use std::str;

/// A `BeforeMiddleware` which checks common `X-Forwarded-*` headers and applies them to the request URL.
pub struct ReverseProxyMiddleware;

impl BeforeMiddleware for ReverseProxyMiddleware {
    /// Check the request for the following request headers:
    ///
    /// * `X-Forwarded-Host`
    /// * `X-Forwarded-Proto`
    /// * `X-Forwarded-Port`
    ///
    /// If (and only if) `X-Forwarded-Host` is present, all present headers will be applied to the request URL.
    fn before(&self, req: &mut Request) -> IronResult<()> {
        if let Some(xfh) = req.headers.get_raw("x-forwarded-host") {
            let mut url: url::Url = req.url.clone().into();
            url.set_host(Some(str::from_utf8(&xfh[0]).unwrap()))
                .unwrap();
            if let Some(xfp) = req.headers.get_raw("x-forwarded-proto") {
                url.set_scheme(str::from_utf8(&xfp[0]).unwrap()).unwrap();
            }
            if let Some(xfp) = req.headers.get_raw("x-forwarded-port") {
                let port = str::from_utf8(&xfp[0]).unwrap().parse().unwrap();
                match (url.scheme(), port) {
                    ("http", 80) | ("https", 443) => url.set_port(None).unwrap(),
                    _ => url.set_port(Some(port)).unwrap(),
                }
            }
            req.url = Url::from_generic_url(url).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ReverseProxyMiddleware;
    use iron::BeforeMiddleware;
    use iron::Url;
    use iron::headers::Headers;
    use iron::request::Request;
    use iron::response::Response;
    use iron_test::request;

    fn test_middleware(url: &str, headers: &[(&str, &[u8])], result: &str) {
        let mut h = Headers::new();
        for &(k, v) in headers {
            h.set_raw(k.to_string(), vec![v.to_vec()]);
        }
        let result = Url::parse(result).unwrap();

        request::get(url, h, &move |req: &mut Request| {
            ReverseProxyMiddleware.before(req).unwrap();
            assert_eq!(req.url, result);
            Ok(Response::new())
        }).unwrap();
    }

    #[test]
    fn it_works() {
        test_middleware("http://localhost:3000/", &[], "http://localhost:3000/");
        test_middleware(
            "http://localhost:3000/",
            &[("x-forwarded-host", b"thing")],
            "http://thing:3000/",
        );
        test_middleware(
            "http://localhost:3000/",
            &[("x-forwarded-host", b"thing"), ("x-forwarded-port", b"80")],
            "http://thing/",
        );
        test_middleware(
            "http://localhost:3000/",
            &[
                ("x-forwarded-host", b"thing"),
                ("x-forwarded-port", b"80"),
                ("x-forwarded-proto", b"https"),
            ],
            "https://thing:80/",
        );
    }
}
