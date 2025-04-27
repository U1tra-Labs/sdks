use serde::{Serialize, Deserialize};

pub const RATE_LIMITER_LEN: usize = 56;

#[derive(Serialize, Deserialize, Debug)]
pub struct RateLimiter {
  pub config: RateLimiterConfig,
  pub previous_quantity: u64,
  pub window_start: u64,
  pub current_quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedRateLimiter {
    pub config: RateLimiterConfig,
    pub window_start: u64,
    pub previous_quantity: u64,
    pub current_quantity: u64,
    pub remaining_outflow: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RateLimiterConfig {
    pub window_duration: u64,
    pub max_outflow: u64,
}