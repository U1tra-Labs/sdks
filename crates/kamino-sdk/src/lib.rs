use solana_sdk::pubkey::Pubkey;

pub mod utils;
pub mod error;
pub mod classes;
pub mod idl_types;
pub mod idl_codegen;

pub const PROGRAM_ID: Pubkey = 
    Pubkey::from_str_const("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");