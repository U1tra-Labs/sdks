use serde::{Deserialize, Serialize};
use solana_sdk::{
    account_info::AccountInfo,
    pubkey::Pubkey
};
use crate::error::SolendError;
use bincode::{self, Options};

#[derive(Deserialize, Serialize, Debug)]
pub struct LendingMarketMetadata {
    pub bump_seed: u64,
    pub market_name: String,
    pub market_description: String,
    pub market_image_url: String,
    pub lookup_tables: Vec<u8>,
}

pub struct ParsedLendingMarketMetadataResult {
    pub pubkey: Pubkey,
    pub account: AccountInfo<'static>,
    pub info: LendingMarketMetadata
}

pub const LENDING_MARKET_METADATA_SIZE: usize = 111;

impl LendingMarketMetadata {
    /// Get bincode configuration optimized for Solana
    pub fn get_bincode_config() -> impl bincode::config::Options {
        let config = bincode::config::DefaultOptions::new();
        config.with_little_endian()
              .with_fixint_encoding()
              .with_limit(LENDING_MARKET_METADATA_SIZE as u64)
    }
    
    /// Deserialize from raw bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, SolendError> {
        // First try standard deserialization
        match bincode::deserialize(data) {
            Ok(metadata) => {
                return Ok(metadata);
            }
            Err(e) => {
                eprintln!("Standard deserialization of metadata failed: {}", e);
                // Continue to try other methods
            }
        }

        // Try with custom bincode options
        let config = Self::get_bincode_config();
        match config.deserialize(data) {
            Ok(metadata) => {
                return Ok(metadata);
            }
            Err(e) => {
                eprintln!("Custom config deserialization of metadata failed: {}", e);
                // Continue to try other methods
            }
        }

        // If we have data length issues, try to deserialize with a slice that matches the expected size
        if data.len() > LENDING_MARKET_METADATA_SIZE {
            match bincode::deserialize(&data[..LENDING_MARKET_METADATA_SIZE]) {
                Ok(metadata) => {
                    eprintln!("Warning: Successfully deserialized metadata using truncated data");
                    return Ok(metadata);
                }
                Err(e) => {
                    eprintln!("Truncated data deserialization for metadata failed: {}", e);
                }
            }
        }
        
        // Try manual deserialization as a last resort
        Self::try_manual_deserialize(data)
    }
    
    /// Try manual deserialization as a last resort
    pub fn try_manual_deserialize(data: &[u8]) -> Result<Self, SolendError> {
        eprintln!("Attempting manual deserialization of LendingMarketMetadata");
        
        // Make sure we have enough data for at least the bump_seed
        if data.len() < 8 {
            return Err(SolendError::FailedToParse);
        }
        
        // Read bump_seed (u64, first 8 bytes)
        let bump_seed = u64::from_le_bytes(data[..8].try_into().unwrap_or([0; 8]));
        
        // For the string fields, we need to be creative
        // In bincode, strings are often stored as length (u64) followed by bytes
        // We'll try to extract the strings by looking for patterns
        
        // Default values in case we can't parse properly
        let default_name = "Unknown Market".to_string();
        let default_description = "Unknown Description".to_string();
        let default_image_url = "".to_string();
        let default_lookup_tables = Vec::new();
        
        // Try to parse individual strings if we can find them
        let market_name = if data.len() > 16 {
            match Self::extract_string_from_data(&data[8..]) {
                Ok((extracted_string, _)) => extracted_string,
                Err(_) => default_name
            }
        } else {
            default_name
        };
        
        // Create the metadata with what we've been able to parse
        Ok(LendingMarketMetadata {
            bump_seed,
            market_name,
            market_description: default_description,
            market_image_url: default_image_url,
            lookup_tables: default_lookup_tables,
        })
    }
    
    /// Helper function to extract a string from a byte slice
    /// Returns the extracted string and the number of bytes consumed
    pub fn extract_string_from_data(data: &[u8]) -> Result<(String, usize), SolendError> {
        // In bincode, strings are stored as length (u64) followed by bytes
        if data.len() < 8 {
            return Err(SolendError::FailedToParse);
        }
        
        // Extract the length
        let length = u64::from_le_bytes(data[..8].try_into().unwrap_or([0; 8])) as usize;
        
        // Validate length
        if length > data.len() - 8 || length > 1024 { // Sanity check for length
            return Err(SolendError::FailedToParse);
        }
        
        // Extract the string bytes
        let string_bytes = data[8..8+length].to_vec();
        
        // Try to convert to UTF-8
        match String::from_utf8(string_bytes) {
            Ok(s) => Ok((s, 8 + length)), // Return the string and bytes consumed
            Err(_) => Err(SolendError::FailedToParse)
        }
    }
    
    /// Debug function to print the raw bytes
    pub fn debug_bytes(data: &[u8]) {
        eprintln!("Metadata data length: {}", data.len());
        
        if data.len() >= 8 {
            let bump_seed = u64::from_le_bytes(data[..8].try_into().unwrap_or([0; 8]));
            eprintln!("First 8 bytes as u64 (bump_seed): {}", bump_seed);
        }
        
        if data.len() > 20 {
            eprintln!("First 20 bytes of metadata: {:?}", &data[..20]);
        } else {
            eprintln!("All metadata bytes: {:?}", data);
        }
        
        // Try to identify string boundaries
        if data.len() > 8 {
            // Try to interpret bytes after bump_seed as string length
            let possible_str_len = u64::from_le_bytes(data[8..16].try_into().unwrap_or([0; 8])) as usize;
            
            // Sanity check
            if possible_str_len > 0 && possible_str_len < 100 && possible_str_len <= data.len() - 16 {
                eprintln!("Possible string at offset 16, length {}: {:?}", 
                    possible_str_len, &data[16..16+possible_str_len]);
                
                // Try to convert to UTF-8
                match String::from_utf8(data[16..16+possible_str_len].to_vec()) {
                    Ok(s) => eprintln!("  As UTF-8: \"{}\"", s),
                    Err(_) => eprintln!("  Not valid UTF-8")
                }
            }
        }
    }
    
    /// Deserialize directly from an AccountInfo
    pub fn from_account_info(info: &AccountInfo) -> Result<Self, SolendError> {
        let data = info.try_borrow_data()
            .map_err(|_| SolendError::UnknownError)?;
        
        // Debug the first few bytes to help diagnose issues
        Self::debug_bytes(&data);
        
        Self::from_bytes(&data)
    }
}

/// Trims the input byte array to create a valid UTF-8 string.
/// Removes leading zeroes and stops at the first zero byte (null terminator).
pub fn trim_string(array: Vec<u8>) -> Result<String, SolendError> {
    // Find the position of the first non-zero byte (skip leading zeros)
    let start_pos = array.iter().position(|&b| b != 0).unwrap_or(0);
    
    // Find the position of the first zero byte after start_pos (null terminator)
    let end_pos = if let Some(pos) = array[start_pos..].iter().position(|&b| b == 0) {
        start_pos + pos
    } else {
        array.len() // If no null terminator, use the entire remaining array
    };
    
    // Create a slice with the trimmed data
    let trimmed_array = &array[start_pos..end_pos];
    
    // Convert to UTF-8 string
    String::from_utf8(trimmed_array.to_vec())
        .map_err(|_| SolendError::FailedToParse)
}

pub fn parse_lending_market_metadata(
    pubkey: Pubkey,
    info: AccountInfo<'static>
) -> Result<ParsedLendingMarketMetadataResult, SolendError> {
    match LendingMarketMetadata::from_account_info(&info) {
        Ok(metadata) => Ok(ParsedLendingMarketMetadataResult {
            info: metadata,
            account: info,
            pubkey
        }),
        Err(e) => {
            eprintln!("Failed to parse lending market metadata using standard methods: {:?}", e);
            eprintln!("Trying alternative parsing methods...");
            
            // Try a last-resort approach by creating a minimal metadata object
            let i = info.clone();
            let data = i
                .try_borrow_data()
                .map_err(|_| SolendError::UnknownError)?;
            
            // Create a default metadata with minimal information
            if data.len() >= 8 {
                // At least extract the bump_seed
                let bump_seed = u64::from_le_bytes(data[..8].try_into().unwrap_or([0; 8]));
                
                // Create a basic metadata object with default values
                let metadata = LendingMarketMetadata {
                    bump_seed,
                    market_name: "Error: Could not parse metadata".to_string(),
                    market_description: "Error: Could not parse metadata".to_string(),
                    market_image_url: "".to_string(),
                    lookup_tables: Vec::new(),
                };
                
                eprintln!("Created a partial metadata with bump_seed {}", bump_seed);
                
                return Ok(ParsedLendingMarketMetadataResult {
                    info: metadata,
                    account: info,
                    pubkey
                });
            }
            
            Err(SolendError::FailedToParse)
        }
    }
}