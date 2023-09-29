//! Middleware to rate-limit requests built on [`reqwest_middleware`],
//! [`reqwest_ratelimit`] and [`leaky_bucket`].
//!
//! Provides a [`reqwest_ratelimit::RateLimiter`] adapter for [`leaky_bucket::RateLimiter`].
//!
//! ## Example
//!
//! ```
//! use async_trait::async_trait;
//! use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
//! use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
//!
//! async fn run() {
//!     let limiter = RateLimiter::builder().max(10).initial(0).refill(5).build();
//!
//!     let client = ClientBuilder::new(reqwest::Client::new())
//!         .with(reqwest_leaky_bucket::rate_limit_all(limiter))
//!         .build();
//!
//!     client.get("https://crates.io").send().await.unwrap();
//! }
//! ```
pub use leaky_bucket;

use async_trait::async_trait;

/// [`reqwest_ratelimit::RateLimiter`] adapter for [`leaky_bucket::RateLimiter`].
pub struct RateLimiterAdapter(leaky_bucket::RateLimiter);

/// Request rate-limiting middleware.
pub type Middleware = reqwest_ratelimit::Middleware<RateLimiterAdapter>;

#[async_trait]
impl reqwest_ratelimit::RateLimiter for RateLimiterAdapter {
    async fn acquire_permit(&self) { self.0.acquire_one().await }
}

/// Creates a new [`Middleware`] rate-limiting all requests using the provided [`leaky_bucket::RateLimiter`].
pub fn rate_limit_all(rate_limiter: leaky_bucket::RateLimiter) -> Middleware {
    reqwest_ratelimit::all(RateLimiterAdapter(rate_limiter))
}
