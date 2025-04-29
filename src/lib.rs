//! Middleware to rate-limit requests built on
//! [`reqwest_middleware`](https://docs.rs/reqwest-middleware/latest/reqwest_middleware),
//! [`reqwest_ratelimit`] and [`leaky_bucket`].
//!
//! Provides a [`reqwest_ratelimit::RateLimiter`] adapter for [`leaky_bucket::RateLimiter`].
//!
//! ## Example
//!
//! ```
#![doc = include_str!("example.rs")]
//! ```
pub use leaky_bucket;

/// [`reqwest_ratelimit::RateLimiter`] adapter for [`leaky_bucket::RateLimiter`].
pub struct RateLimiterAdapter(leaky_bucket::RateLimiter);

/// Request rate-limiting middleware.
pub type Middleware = reqwest_ratelimit::Middleware<RateLimiterAdapter>;

impl reqwest_ratelimit::RateLimiter for RateLimiterAdapter {
    async fn acquire_permit(&self) {
        self.0.acquire_one().await
    }
}

/// Creates a new [`Middleware`] rate-limiting all requests using the provided [`leaky_bucket::RateLimiter`].
pub fn rate_limit_all(rate_limiter: leaky_bucket::RateLimiter) -> Middleware {
    reqwest_ratelimit::all(RateLimiterAdapter(rate_limiter))
}
