use solana_sdk::{account_info::AccountInfo, pubkey::Pubkey};

use crate::{idl_codegen::Reserve};

pub struct KaminoReserve {
    pub state: Reserve,
    pub address: Pubkey,
    pub symbol: String,
    pub token_oracle_price: _,
    pub stats: _,
    farm_data: _,
    data: AccountInfo<'static>,
    
}