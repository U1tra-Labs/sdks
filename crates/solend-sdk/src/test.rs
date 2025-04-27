use std::str::FromStr;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use crate::*;

// Transaction.rs
#[test]
fn transaction_size_computation() {
    let result = transaction::get_size_of_transaction(
        vec![Instruction {
            program_id: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            accounts: vec![],
            data: vec![]
        }],
        false,
        None
    ).unwrap();
    assert_eq!(result, 70);
}

#[test]
fn compress_u16() {
    let a = 0u16;
    let b = 128u16;
    let c = 16384u16;
    
    assert_eq!(transaction::get_size_of_compressed_u16(&a), 1);
    assert_eq!(transaction::get_size_of_compressed_u16(&b), 2);
    assert_eq!(transaction::get_size_of_compressed_u16(&c), 3);
}

// LendingMarket.rs
#[test]
fn should_parse_market() {
    let client = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com");
    let pubkey = Pubkey::from_str("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY").unwrap();
    let acct = client.get_account(&pubkey).unwrap();
    state::lending_market::LendingMarket::from_bytes(&acct.data).unwrap();
}

// LendingMarketMetadata.rs
#[test]
fn should_parse_market_metadata() {
    let client = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com");
    let pubkey = Pubkey::from_str("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY").unwrap();
    let acct = client.get_account(&pubkey).unwrap();
    state::lending_market_metadata::LendingMarketMetadata::from_bytes(&acct.data).unwrap();
}