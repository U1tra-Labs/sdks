use solana_sdk::pubkey::Pubkey;
use super::big_fraction_bytes;

pub struct ReserveLiquidity {
  /** Reserve liquidity mint address */
  pub mint_pubkey: Pubkey,
  /** Reserve liquidity supply address */
  pub supply_vault: Pubkey,
  /** Reserve liquidity fee collection address */
  pub fee_vault: Pubkey,
  /** Reserve liquidity available */
  pub available_amount: u128,
  /** Reserve liquidity borrowed (scaled fraction) */
  pub borrowed_amount_sf: u64,
  /** Reserve liquidity market price in quote currency (scaled fraction) */
  pub market_price_sf: u64,
  /** Unix timestamp of the market price (from the oracle) */
  pub market_price_last_updated_ts: u32,
  /** Reserve liquidity mint decimals */
  pub mint_decimals: u8,
  /**
   * Timestamp when the last refresh reserve detected that the liquidity amount is above the deposit cap. When this threshold is crossed, then redemptions (auto-deleverage) are enabled.
   * If the threshold is not crossed, then the timestamp is set to 0
   */
  pub deposit_limit_crossed_timestamp: u32,
  /**
   * Timestamp when the last refresh reserve detected that the borrowed amount is above the borrow cap. When this threshold is crossed, then redemptions (auto-deleverage) are enabled.
   * If the threshold is not crossed, then the timestamp is set to 0
   */
  pub borrow_limit_crossed_timestamp: u32,
  /** Reserve liquidity cumulative borrow rate (scaled fraction) */
  pub cumulative_borrow_rate_bsf: big_fraction_bytes::BigFractionBytesFields,
  /** Reserve cumulative protocol fees (scaled fraction) */
  pub accumulated_protocol_fees_sf: u64,
  /** Reserve cumulative referrer fees (scaled fraction) */
  pub accumulated_referrer_fees_sf: u64,
  /** Reserve pending referrer fees, to be claimed in refresh_obligation by referrer or protocol (scaled fraction) */
  pub pending_referrer_fees_sf: u64,
  /** Reserve referrer fee absolute rate calculated at each refresh_reserve operation (scaled fraction) */
  pub absolute_referral_rate_sf: u64,
  /** Token program of the liquidity mint */
  pub token_program: Pubkey,
  pub padding2: Vec<u64>,
  pub padding3: Vec<u64>,
}