use bincode::Options;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use crate::{error::KaminoError, idl_types::types::elevation_groups::ElevationGroup, PROGRAM_ID};

pub const LENDING_MARKET_SIZE: usize = 272;

#[derive(Deserialize, Serialize)]
pub struct LendingMarket {
    /// Version of lending market
    pub version: u8,
    /// Bump seed for derived authority address
    pub bump_seed: u32,
    /// Owner authority which can add new reserves
    pub lending_market_owner: Pubkey,
    /// Temporary cache of the lending market owner, used in update_lending_market_owner
    pub lending_market_owner_cache: Pubkey,
    /**
     Currency market prices are quoted in
     
     e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    */
    pub quote_currency: Vec<u8>,
    /// Referral fee for the lending market, as bps out of the total protocol fee
    pub referral_fee_bps: u16,
    pub emergency_mode: u8,
    /**
     Whether the obligations on this market should be subject to auto-deleveraging after deposit
     or borrow limit is crossed.
     Besides this flag, the particular reserve's flag also needs to be enabled (logical `AND`).
     **NOTE:** this also affects the individual "target LTV" deleveraging.
    */
    pub auto_deleverage_enabled: u8,
    pub borrow_disabled: u8,
    /**
     Refresh price from oracle only if it's older than this percentage of the price max age.
     e.g. if the max age is set to 100s and this is set to 80%, the price will be refreshed if it's older than 80s.
     Price is always refreshed if this set to 0.
    */
    pub price_refresh_trigger_to_max_age_pct: f32,
    /// Percentage of the total borrowed value in an obligation available for liquidation
    pub liquidation_max_debt_close_factor_pct: u8,
    /// Minimum acceptable unhealthy LTV before max_debt_close_factor_pct becomes 100%
    pub insolvency_risk_unhealthy_ltv_pct: f32,
    /// Minimum liquidation value threshold triggering full liquidation for an obligation
    pub min_full_liquidation_value_threshold: u64,
    /// Max allowed liquidation value in one ix call
    pub max_liquidatable_debt_market_value_at_once: u64,
    /// [DEPRECATED] Global maximum unhealthy borrow value allowed for any obligation
    pub reserved0: Vec<u64>,
    /// Global maximum allowed borrow value allowed for any obligation
    pub global_allowed_borrow_value: u64,
    /// The address of the risk council, in charge of making parameter and risk decisions on behalf of the protocol
    pub risk_council: Pubkey,
    /// [DEPRECATED] Reward points multiplier per obligation type
    pub reserved1: Vec<u64>,
    /// Elevation groups are used to group together reserves that have the same risk parameters and can bump the ltv and liquidation threshold
    pub elevation_groups: Vec<ElevationGroup>,
    pub elevation_group_padding: Vec<u64>,
    /// Min net value accepted to be found in a position after any lending action in an obligation (scaled by quote currency decimals)
    pub min_net_value_in_obligation_sf: u64,
    /// Minimum value to enforce smallest ltv priority checks on the collateral reserves on liquidation
    pub min_value_skip_liquidation_ltv_checks: u64,
    /// Market name, zero-padded.
    pub name: Vec<u8>,
    /// Minimum value to enforce highest borrow factor priority checks on the debt reserves on liquidation
    pub min_value_skip_liquidation_bf_checks: u64,
    /**
     Minimum amount of deposit at creation of a reserve to prevent artificial inflation
     Note: this amount cannot be recovered, the ctoken associated are never minted
    */
    pub min_initial_deposit_amount: u64,
    /// Whether the obligation orders should be evaluated during liquidations.
    pub obligation_orders_enabled: u8,
    pub padding2: Vec<u8>,
    pub padding1: Vec<u64>
}

impl LendingMarket {
    /// Get bincode configuration optimized for Solana
    fn get_bincode_config() -> impl bincode::config::Options {
        let config = bincode::config::DefaultOptions::new();
        config.with_little_endian()
              .with_fixint_encoding()
              .with_limit(LENDING_MARKET_SIZE as u64)
    }
    
    pub fn fetch(
        c: RpcClient, 
        address: &Pubkey, 
        program_id: Option<&Pubkey>
    ) -> Result<Self, KaminoError> {
        let program_id = match program_id {
            Some(pid) => pid,
            None => &PROGRAM_ID
        };
        let info = c.get_account(address).map_err(|_| KaminoError::FailedToFetch)?;
        if &info.owner != program_id {
            return Err(KaminoError::InvalidProgramData);
        };
        
        Self::from_bytes(&info.data)
    }
    
    pub fn fetch_multiple(
        c: RpcClient,
        addresses: &[Pubkey],
        program_id: Option<Pubkey>
    ) -> Vec<Result<Self, KaminoError>> {
        let program_id = match program_id {
            Some(pid) => pid,
            None => PROGRAM_ID
        };
        let i = c.get_multiple_accounts(addresses).map_err(|_| KaminoError::FailedToFetch)?;
        i.iter().map(|acct| {
            if let Some(acct) = acct {
                
            } else {
                
            }
        })
    }
    
    pub fn from_bytes(data: &[u8]) -> Result<Self, KaminoError> {
        if data.len() < LENDING_MARKET_SIZE {
            return Err(KaminoError::InvalidProgramData);
        }
        // standard deserialize
        if let Ok(meta) = bincode::deserialize(data) {
            return Ok(meta);
        }
        // solana-optimized deserialize
        let config = Self::get_bincode_config();
        if let Ok(meta) = config.deserialize(data) {
            return Ok(meta);
        }
        // try to deserialize with a slice that matches the expected size
        if data.len() > LENDING_MARKET_SIZE {
            match bincode::deserialize(&data[..LENDING_MARKET_SIZE]) {
                Ok(metadata) => {
                    eprintln!("Warning: Successfully deserialized metadata using truncated data");
                    return Ok(metadata);
                }
                Err(_) => { }
            }
        }
        
        Err(KaminoError::FailedToParse)
    }
}