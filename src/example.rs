use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

async fn run() {
    let limiter = RateLimiter::builder().max(10).initial(0).refill(5).build();
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(reqwest_leaky_bucket::rate_limit_all(limiter))
        .build();
    client.get("https://crates.io").send().await.unwrap();
}
