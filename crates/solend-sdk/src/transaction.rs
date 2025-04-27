use solana_sdk::{
    instruction::Instruction,
    packet::PACKET_DATA_SIZE,
    pubkey::Pubkey,
    signer::keypair::Keypair,
};

use crate::error::SolendError;

/**
    If the transaction doesn't contain a `setComputeUnitLimit` instruction, the default compute budget is 200,000 units per instruction.
 */
pub const DEFAULT_COMPUTE_BUDGET_UNITS: usize = 200000;
/**
    The maximum size of a Solana transaction, leaving some room for the compute budget instructions.
 */
pub const PACKET_DATA_SIZE_WITH_ROOM_FOR_COMPUTE_BUDGET: usize = PACKET_DATA_SIZE - 52;

/**
    An instruction with some extra information that will be used to build transactions.
 */
pub struct InstructionWithEphemeralSigners {
    /** The instruction */
    pub instruction: Instruction,
    /** The ephemeral signers that need to sign the transaction where this instruction will be */
    pub signers: Vec<Keypair>,
    /** The compute units that this instruction requires, useful if greater than `DEFAULT_COMPUTE_BUDGET_UNITS`  */
    pub compute_units: Option<usize>
}

/**
    The priority fee configuration for transactions
 */
pub struct PriorityFeeConfig {
  /** This is the priority fee in micro lamports, it gets passed down to `setComputeUnitPrice`  */
  pub compute_unit_price_micro_lamports: Option<usize>,
  pub tight_compute_budget: Option<bool>,
  pub jito_tip_lamports: Option<bool>,
  pub jito_bundle_size: Option<bool>,
}

/**
    A default priority fee configuration. Using a priority fee is helpful even when you're not writing to hot accounts.
 */
pub const DEFAULT_PRIORITY_FEE_CONFIG: PriorityFeeConfig = PriorityFeeConfig {
  compute_unit_price_micro_lamports: Some(50000),
  tight_compute_budget: None,
  jito_tip_lamports: None,
  jito_bundle_size: None
};

/**
 * Get the size of a transaction that would contain the provided array of instructions
 * This is based on {@link https://solana.com/docs/core/transactions}.
 *
 * Each transaction has the following layout :
 *
 * - A compact array of all signatures
 * - A 3-bytes message header
 * - A compact array with all the account addresses
 * - A recent blockhash
 * - A compact array of instructions
 *
 * If the transaction is a `versioned_transaction`, it also contains an extra byte at the beginning, indicating the version and an array of `MessageAddressTableLookup` at the end.
 * After this field there is an array of indexes into the address lookup table that represents the accounts from the address lookup table used in the transaction.
 *
 * Each instruction has the following layout :
 * - One byte indicating the index of the program in the account addresses array
 * - A compact array of indices into the account addresses array, indicating which accounts are used by the instruction
 * - A compact array of serialized instruction data
 */
pub fn get_size_of_transaction(
  instructions: Vec<Instruction>,
  versioned_transaction: bool,
  address_lookup_table_addresses: Option<Vec<Pubkey>>
) -> Result<u16, SolendError> {
    let mut programs: Vec<String> = vec![];
    let mut signers: Vec<String> = vec![];
    let mut accounts: Vec<String> = vec![];
    
    for ix in &instructions {
        programs.push(ix.program_id.to_string());
        accounts.push(ix.program_id.to_string());
        for key in &ix.accounts {
            if key.is_signer {
                signers.push(key.pubkey.to_string());
            }
            accounts.push(key.pubkey.to_string());   
        }
    }

    let mut ix_map = instructions
        .iter()
        .map(
            |ix| -> Result<u16, SolendError> {
                let ix_len: u16 = ix.data.len()
                    .try_into()
                    .map_err(|_| SolendError::ConversionWouldOverflow)?;
                let ix_account_len: u16 = ix.accounts.len()
                    .try_into()
                    .map_err(|_| SolendError::ConversionWouldOverflow)?;
                let ix_account_size_compressed: u16 = 
                    get_size_of_compressed_u16(&ix_account_len)
                    .into();
                let ix_size_compressed: u16 = get_size_of_compressed_u16(&ix_len).into();
                Ok(ix_account_len + ix_account_size_compressed + ix_size_compressed + ix_len + 1)
            }
        );
    
    if ix_map.any(|v| v.is_err()) {
        return Err(SolendError::TransactionTooLarge);
    }
  
    let instruction_sizes: u16 = match ix_map.map(|v| v.unwrap_or(0)).reduce(|a, b| a + b) {
        Some(size) => size,
        None => 0
    };

  let mut number_of_address_lookups: u16 = 0;
  let signers_len: &u16 = &signers.len().try_into()
      .map_err(|_| SolendError::ConversionWouldOverflow)?;
  
  if let Some(address_lookup_table_addresses) = address_lookup_table_addresses {
    let lookup_table_addresses: Vec<String> = address_lookup_table_addresses.iter().map(| address |
      address.to_string()
    ).collect();
    let total_number_of_accounts = accounts.len();
    accounts = accounts
        .iter_mut()
        .filter(| account | !lookup_table_addresses.contains(account))
        .map(| account_key | account_key.to_owned())
        .collect();
    accounts = [accounts, programs, signers]
        .iter()
        .flatten()
        .map(| account_key | account_key.to_owned())
        .collect();
    number_of_address_lookups = (total_number_of_accounts - accounts.len())
        .try_into().map_err(|_| SolendError::ConversionWouldOverflow)?; // This number is equal to the number of accounts that are in the lookup table and are neither signers nor programs
  }

  let accounts_len: u16 = accounts.len().try_into()
      .map_err(|_| SolendError::ConversionWouldOverflow)?;
  let instructions_len: u16 = instructions.len().try_into()
      .map_err(|_| SolendError::ConversionWouldOverflow)?;    
  
  let compressed_signers: u16 = get_size_of_compressed_u16(signers_len).into();
  let compressed_accounts: u16 = get_size_of_compressed_u16(&accounts_len).into();
  let compressed_instructions: u16 = get_size_of_compressed_u16(&instructions_len).into();
  
  return
    Ok(compressed_signers +
    signers_len * 64 + // array of signatures
    3 + compressed_accounts +
    32 * accounts_len + // array of account addresses
    32 + // recent blockhash
    compressed_instructions +
    instruction_sizes + // array of instructions
    (if versioned_transaction { 2u16 } else { 0u16 }) + // transaction version and number of address lookup tables
    (if versioned_transaction && number_of_address_lookups != 0 { 32u16 } else { 0u16 }) + // address lookup table address (we only support 1 address lookup table)
    (if versioned_transaction && number_of_address_lookups != 0 { 2u16 } else { 0u16 }) + // number of address lookup indexes
    number_of_address_lookups)
}

fn boolean_to_int(b: bool) -> u8 {
    match b {
        true => 1,
        false => 0
    }
}

/**
 * Get the size of n in bytes when serialized as a CompressedU16. Compact arrays use a CompactU16 to store the length of the array.
 */
pub fn get_size_of_compressed_u16(n: &u16) -> u8 {
  return 1 + boolean_to_int(n >= &128) + boolean_to_int(n >= &16384);
}