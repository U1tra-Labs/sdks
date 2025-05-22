use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize, Serialize)]
pub struct ElevationGroup {
    pub max_liquidation_bonus_bps: u16,
    pub id: u16,
    pub ltv_pct: u32,
    pub liquidation_threshold_pct: u32,
    pub allow_new_loans: u32,
    pub max_reserves_as_collateral: u32,
    pub padding0: u32,
    /** Mandatory debt reserve for this elevation group */
    pub debt_reserve: Pubkey,
    pub padding1: Vec<u64>
}