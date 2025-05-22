use solana_sdk::pubkey::Pubkey;

pub struct ReserveCollateral {
  /** Reserve collateral mint address */
  pub mint_pubkey: Pubkey,
  /** Reserve collateral mint supply, used for exchange rate */
  pub mint_total_supply: u64,
  /** Reserve collateral supply address */
  pub supply_vault: Pubkey,
  pub padding1: Vec<u64>,
  pub padding2: Vec<u64>
}