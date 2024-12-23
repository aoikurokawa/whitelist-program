use clap::{command, Subcommand};
use solana_sdk::pubkey::Pubkey;

/// The CLI handler for the restaking program
#[derive(Subcommand)]
pub enum NcnPortalCommands {
    /// Initialize, get, and set the config struct
    Whitelist {
        #[command(subcommand)]
        action: WhitelistActions,
    },
}

/// The actions that can be performed on the restaking config
#[derive(Subcommand)]
pub enum WhitelistActions {
    /// Initialize the whitelist
    Initialize,
    /// Get the whitelist
    Get,
    /// Add to whitelist
    AddToWhitelist {
        whitelisted: Pubkey,
        rate_limiting: u64,
    },
    /// Remove from whitelist
    RemoveFromWhitelist { whitelisted: Pubkey },
}
