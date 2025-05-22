use anchor_client::anchor_lang::declare_program;
use anchor_client::anchor_lang::prelude::*;

declare_program!(klend);
declare_program!(vault);

pub use klend::{accounts::{LendingMarket, self}, types::{self, *}};
pub use vault::accounts::{Reserve, VaultState};