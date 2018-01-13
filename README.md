# iron_reverse_proxy [![Build Status](https://travis-ci.org/kivikakk/iron_reverse_proxy.svg?branch=master)](https://travis-ci.org/kivikakk/iron_reverse_proxy)

- [Documentation](https://docs.rs/iron_reverse_proxy)
- [Repository](https://github.com/kivikakk/iron_reverse_proxy)
- [Crates.io](https://crates.io/crates/iron_reverse_proxy)

Some simple `BeforeMiddleware` to make using Iron behind a reverse proxy easier.

Usage:

```rust
extern crate iron_reverse_proxy;

use iron::prelude::*;

let ch = Chain::new();
ch.link_before(iron_reverse_proxy::ReverseProxyMiddleware);
```

And you're done. Works particularly well with [`router`](https://crates.io/crates/router)'s [`url_for!` macro](https://docs.rs/router/0.6.0/router/macro.url_for.html), as it depends on the `Request.url` property, which this middleware modifies.

## License

Licensed under the MIT, see `LICENSE`.
