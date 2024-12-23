use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_check_whitelisted(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [whitelist_info, whitelist_entry_info, whitelisted_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, false)?;

    load_signer(whitelisted_info, false)?;

    WhitelistEntry::load(
        program_id,
        whitelist_info.key,
        whitelisted_info.key,
        whitelist_entry_info,
        false,
    )?;
    let whitelist_entry_data = whitelist_entry_info.data.borrow();
    let whitelist_entry = WhitelistEntry::try_from_slice_unchecked(&whitelist_entry_data)?;

    whitelist_entry.check_parent(whitelist_info.key)?;
    whitelist_entry.check_whitelisted(whitelisted_info.key)?;

    Ok(())
}
