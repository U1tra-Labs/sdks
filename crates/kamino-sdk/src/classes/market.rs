use std::collections::HashMap;

use solana_client::{rpc_client::RpcClient, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig}, rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}};
use solana_sdk::{
    account_info::AccountInfo,
    pubkey::Pubkey
};

use crate::{error::KaminoError, idl_types::accounts::lending_market::LendingMarket, PROGRAM_ID};

use super::reserve::KaminoReserve;

pub struct ReserveRewardInfo {
    pub rewards_per_second: f64,
    pub rewards_remaining: f64,
    pub reward_apr: f32,
    pub reward_mint: Pubkey,
    pub total_investment_usd: f32,
    pub reward_price: f32
}

pub struct KaminoMarket {
    connection: RpcClient,
    pub address: Pubkey,
    pub state: LendingMarket,
    recent_slot_duration_ms: u32,
    pub program_id: Pubkey,
    pub reserves_active: HashMap<Pubkey, KaminoReserve>
}

impl KaminoMarket {
    fn constructor(
        connection: RpcClient, 
        market_address: &Pubkey, 
        recent_slot_duration_ms: u32,
        program_id: Option<&Pubkey>,
        state: LendingMarket,
        reserves: HashMap<Pubkey, KaminoReserve>
    ) -> Result<Self, KaminoError> {
        Ok(Self {
            connection,
            address: *market_address,
            recent_slot_duration_ms,
            reserves_active: get_reserves_active(reserves),
            state,
            program_id: *program_id.unwrap_or(&PROGRAM_ID)
        })
    }
    
    pub fn new(
        connection: RpcClient, 
        market_address: &Pubkey, 
        recent_slot_duration_ms: u32,
        program_id: Option<&Pubkey>,
        with_reserves: Option<bool>
    ) -> Result<Self, KaminoError> {
        let market = LendingMarket::fetch(connection, market_address, program_id)?;
        if recent_slot_duration_ms <= 0 {
            return Err(KaminoError::Invalid);
        }
        let with_reserves = match with_reserves {
            Some(v) => v,
            None => true
        };
        
        let reserves: HashMap<Pubkey, _> = if with_reserves {
            
        } else {
            HashMap::new()
        };
        
        return Self::constructor(
            connection, 
            market_address, 
            recent_slot_duration_ms, 
            program_id, 
            market,
            reserves
        )
    }
}

pub fn get_reserves_active(reserves: HashMap<Pubkey, KaminoReserve>) -> HashMap<Pubkey, KaminoReserve> {
    let mut new: HashMap<Pubkey, KaminoReserve> = HashMap::new();
    for (key, value) in reserves.iter() {
        if value.state.config.status == 0 {
            new.insert(*key, *value);
        }
    }
    new
}

pub fn get_reserves_for_market(
    market: &Pubkey, 
    connection: RpcClient,
    program_id: &Pubkey,
    recent_slot_duration_ms: u32
) -> Result<HashMap<Pubkey, KaminoReserve>, KaminoError> {
    let reserves = connection.get_program_accounts_with_config(market, RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(0),
            RpcFilterType::Memcmp(Memcmp::new(
                32, 
                MemcmpEncodedBytes::Bytes(market.to_bytes().to_vec())
            ))
        ]),
        account_config: RpcAccountInfoConfig::default(),
        with_context: None,
        sort_results: None
    }).map_err(|_| KaminoError::FailedToFetch)?;
    let deserialized_reserves = reserves
        .iter()
        .map(|r| {
            if r.1.data.is_empty() {
                
            }
        })
        .collect();
}