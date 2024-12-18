//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NcnPortalError {
    /// 0 - NcnPortalWhitelistAdminInvalid
    #[error("NcnPortalWhitelistAdminInvalid")]
    NcnPortalWhitelistAdminInvalid = 0x0,
    /// 1 - NcnPortalParentInvalid
    #[error("NcnPortalParentInvalid")]
    NcnPortalParentInvalid = 0x1,
    /// 2 - NcnPortalWhitelistedInvalid
    #[error("NcnPortalWhitelistedInvalid")]
    NcnPortalWhitelistedInvalid = 0x2,
    /// 3000 - ArithmeticOverflow
    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 0xBB8,
    /// 3001 - ArithmeticUnderflow
    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow = 0xBB9,
    /// 3002 - DivisionByZero
    #[error("DivisionByZero")]
    DivisionByZero = 0xBBA,
}

impl solana_program::program_error::PrintProgramError for NcnPortalError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
