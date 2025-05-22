use solana_sdk::pubkey::Pubkey;

use crate::idl_types::types;

pub const DISCRIMINATOR: [u8; 8] = [43, 242, 204, 202, 26, 247, 59, 127];

pub struct Reserve {
    pub version: u32,
    /** Last slot when supply and rates updated */
    pub last_update: types::last_update::LastUpdate,
    /** Lending market address */
    pub lending_market: Pubkey,
    pub farm_collateral: Pubkey,
    pub farm_debt: Pubkey,
    /** Reserve liquidity */
    pub liquidity: types::reserve_liquidity::ReserveLiquidity,
    pub reserve_liquidity_padding: Vec<u32>,
    /** Reserve collateral */
    pub collateral: types::reserve_collateral::ReserveCollateral,
    pub reserve_collateral_padding: Vec<u32>,
    /** Reserve configuration values */
    pub config: types::reserve_config::ReserveConfig,
    pub config_padding: Vec<u32>,
    pub borrowed_amount_outside_elevation_group: u64,
    /**
    * Amount of token borrowed in lamport of debt asset in the given
    * elevation group when this reserve is part of the collaterals.
    */
    pub borrowed_amounts_against_this_reserve_in_elevation_groups: Vec<u64>,
    pub padding: Vec<u32>,

    pub discriminator: [u8; 8],
}