pub mod lending_market;
pub mod lending_market_metadata;
pub mod ratelimiter;

pub struct LastUpdateLayout {
    pub slot: u64,
    pub stale: bool
}