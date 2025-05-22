pub struct LastUpdate {
  /** Last slot when updated */
  pub slot: u64,
  /** True when marked stale, false when slot updated */
  pub stale: u32,
  /** Status of the prices used to calculate the last update */
  pub price_status: u32,
  pub placeholder: Vec<u8>
}