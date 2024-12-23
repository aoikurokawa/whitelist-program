use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::instruction::NcnPortalInstruction;

pub fn initialize_whitelist(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    admin: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*whitelist, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::InitializeWhitelist
            .try_to_vec()
            .unwrap(),
    }
}

pub fn add_to_whitelist(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    whitelist_entry: &Pubkey,
    whitelisted: &Pubkey,
    admin: &Pubkey,
    rate_limiting: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*whitelist, false),
        AccountMeta::new(*whitelist_entry, false),
        AccountMeta::new_readonly(*whitelisted, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::AddToWhitelist { rate_limiting }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn check_whitelisted(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    whitelist_entry: &Pubkey,
    whitelisted: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*whitelist, false),
        AccountMeta::new_readonly(*whitelist_entry, false),
        AccountMeta::new_readonly(*whitelisted, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::CheckWhitelisted.try_to_vec().unwrap(),
    }
}

pub fn remove_from_whitelist(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    whitelist_entry: &Pubkey,
    whitelisted: &Pubkey,
    admin: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*whitelist, false),
        AccountMeta::new(*whitelist_entry, false),
        AccountMeta::new_readonly(*whitelisted, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::RemoveFromWhitelist
            .try_to_vec()
            .unwrap(),
    }
}
