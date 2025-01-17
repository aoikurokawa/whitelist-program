//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CheckWhitelisted {
    pub whitelist: solana_program::pubkey::Pubkey,

    pub whitelisted: solana_program::pubkey::Pubkey,
}

impl CheckWhitelisted {
    pub fn instruction(
        &self,
        args: CheckWhitelistedInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CheckWhitelistedInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whitelist,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whitelisted,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CheckWhitelistedInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::NCN_PORTAL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CheckWhitelistedInstructionData {
    discriminator: u8,
}

impl CheckWhitelistedInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 3 }
    }
}

impl Default for CheckWhitelistedInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CheckWhitelistedInstructionArgs {
    pub proof: Vec<[u8; 32]>,
}

/// Instruction builder for `CheckWhitelisted`.
///
/// ### Accounts:
///
///   0. `[]` whitelist
///   1. `[signer]` whitelisted
#[derive(Clone, Debug, Default)]
pub struct CheckWhitelistedBuilder {
    whitelist: Option<solana_program::pubkey::Pubkey>,
    whitelisted: Option<solana_program::pubkey::Pubkey>,
    proof: Option<Vec<[u8; 32]>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CheckWhitelistedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn whitelist(&mut self, whitelist: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whitelist = Some(whitelist);
        self
    }
    #[inline(always)]
    pub fn whitelisted(&mut self, whitelisted: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whitelisted = Some(whitelisted);
        self
    }
    #[inline(always)]
    pub fn proof(&mut self, proof: Vec<[u8; 32]>) -> &mut Self {
        self.proof = Some(proof);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CheckWhitelisted {
            whitelist: self.whitelist.expect("whitelist is not set"),
            whitelisted: self.whitelisted.expect("whitelisted is not set"),
        };
        let args = CheckWhitelistedInstructionArgs {
            proof: self.proof.clone().expect("proof is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `check_whitelisted` CPI accounts.
pub struct CheckWhitelistedCpiAccounts<'a, 'b> {
    pub whitelist: &'b solana_program::account_info::AccountInfo<'a>,

    pub whitelisted: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `check_whitelisted` CPI instruction.
pub struct CheckWhitelistedCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whitelist: &'b solana_program::account_info::AccountInfo<'a>,

    pub whitelisted: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CheckWhitelistedInstructionArgs,
}

impl<'a, 'b> CheckWhitelistedCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CheckWhitelistedCpiAccounts<'a, 'b>,
        args: CheckWhitelistedInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            whitelist: accounts.whitelist,
            whitelisted: accounts.whitelisted,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whitelist.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whitelisted.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CheckWhitelistedInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::NCN_PORTAL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whitelist.clone());
        account_infos.push(self.whitelisted.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CheckWhitelisted` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whitelist
///   1. `[signer]` whitelisted
#[derive(Clone, Debug)]
pub struct CheckWhitelistedCpiBuilder<'a, 'b> {
    instruction: Box<CheckWhitelistedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CheckWhitelistedCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CheckWhitelistedCpiBuilderInstruction {
            __program: program,
            whitelist: None,
            whitelisted: None,
            proof: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn whitelist(
        &mut self,
        whitelist: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whitelist = Some(whitelist);
        self
    }
    #[inline(always)]
    pub fn whitelisted(
        &mut self,
        whitelisted: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whitelisted = Some(whitelisted);
        self
    }
    #[inline(always)]
    pub fn proof(&mut self, proof: Vec<[u8; 32]>) -> &mut Self {
        self.instruction.proof = Some(proof);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CheckWhitelistedInstructionArgs {
            proof: self.instruction.proof.clone().expect("proof is not set"),
        };
        let instruction = CheckWhitelistedCpi {
            __program: self.instruction.__program,

            whitelist: self.instruction.whitelist.expect("whitelist is not set"),

            whitelisted: self
                .instruction
                .whitelisted
                .expect("whitelisted is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CheckWhitelistedCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whitelisted: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    proof: Option<Vec<[u8; 32]>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
