# reqwest-leaky-bucket

[`leaky-bucket`](https://crates.io/crates/reqwest-middleware) rate-limit middleware implementation for
[`reqwest-middleware`](https://crates.io/crates/reqwest-middleware) based on 
[`reqwest-ratelimit`](https://crates.io/crates/reqwest-ratelimit).

[![Crates.io](https://img.shields.io/crates/v/reqwest-leaky-bucket.svg)](https://crates.io/crates/reqwest-leaky-bucket)
[![Docs.rs](https://docs.rs/reqwest-leaky-bucket/badge.svg)](https://docs.rs/reqwest-leaky-bucket)

## Usage

```rust
use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

async fn run() {
    let limiter = RateLimiter::builder().max(10).initial(0).refill(5).build();
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(reqwest_leaky_bucket::rate_limit_all(limiter))
        .build();
    client.get("https://crates.io").send().await.unwrap();
}
```
