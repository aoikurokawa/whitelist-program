//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct RemoveFromWhitelist {
      
              
          pub whitelist: solana_program::pubkey::Pubkey,
          
              
          pub whitelist_entry: solana_program::pubkey::Pubkey,
          
              
          pub whitelisted_info: solana_program::pubkey::Pubkey,
          
              
          pub admin_info: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl RemoveFromWhitelist {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whitelist,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.whitelist_entry,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whitelisted_info,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.admin_info,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = RemoveFromWhitelistInstructionData::new().try_to_vec().unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::NCN_PORTAL_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RemoveFromWhitelistInstructionData {
            discriminator: u8,
      }

impl RemoveFromWhitelistInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 3,
                  }
  }
}

impl Default for RemoveFromWhitelistInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `RemoveFromWhitelist`.
///
/// ### Accounts:
///
          ///   0. `[]` whitelist
                ///   1. `[writable]` whitelist_entry
          ///   2. `[]` whitelisted_info
                ///   3. `[signer]` admin_info
                ///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct RemoveFromWhitelistBuilder {
            whitelist: Option<solana_program::pubkey::Pubkey>,
                whitelist_entry: Option<solana_program::pubkey::Pubkey>,
                whitelisted_info: Option<solana_program::pubkey::Pubkey>,
                admin_info: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RemoveFromWhitelistBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn whitelist(&mut self, whitelist: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whitelist = Some(whitelist);
                    self
    }
            #[inline(always)]
    pub fn whitelist_entry(&mut self, whitelist_entry: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whitelist_entry = Some(whitelist_entry);
                    self
    }
            #[inline(always)]
    pub fn whitelisted_info(&mut self, whitelisted_info: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whitelisted_info = Some(whitelisted_info);
                    self
    }
            #[inline(always)]
    pub fn admin_info(&mut self, admin_info: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.admin_info = Some(admin_info);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = RemoveFromWhitelist {
                              whitelist: self.whitelist.expect("whitelist is not set"),
                                        whitelist_entry: self.whitelist_entry.expect("whitelist_entry is not set"),
                                        whitelisted_info: self.whitelisted_info.expect("whitelisted_info is not set"),
                                        admin_info: self.admin_info.expect("admin_info is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `remove_from_whitelist` CPI accounts.
  pub struct RemoveFromWhitelistCpiAccounts<'a, 'b> {
          
                    
              pub whitelist: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub whitelist_entry: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub whitelisted_info: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub admin_info: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `remove_from_whitelist` CPI instruction.
pub struct RemoveFromWhitelistCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub whitelist: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub whitelist_entry: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub whitelisted_info: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub admin_info: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> RemoveFromWhitelistCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: RemoveFromWhitelistCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              whitelist: accounts.whitelist,
              whitelist_entry: accounts.whitelist_entry,
              whitelisted_info: accounts.whitelisted_info,
              admin_info: accounts.admin_info,
              system_program: accounts.system_program,
                }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whitelist.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whitelist_entry.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whitelisted_info.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.admin_info.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = RemoveFromWhitelistInstructionData::new().try_to_vec().unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::NCN_PORTAL_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.whitelist.clone());
                        account_infos.push(self.whitelist_entry.clone());
                        account_infos.push(self.whitelisted_info.clone());
                        account_infos.push(self.admin_info.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `RemoveFromWhitelist` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` whitelist
                ///   1. `[writable]` whitelist_entry
          ///   2. `[]` whitelisted_info
                ///   3. `[signer]` admin_info
          ///   4. `[]` system_program
#[derive(Clone, Debug)]
pub struct RemoveFromWhitelistCpiBuilder<'a, 'b> {
  instruction: Box<RemoveFromWhitelistCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RemoveFromWhitelistCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(RemoveFromWhitelistCpiBuilderInstruction {
      __program: program,
              whitelist: None,
              whitelist_entry: None,
              whitelisted_info: None,
              admin_info: None,
              system_program: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn whitelist(&mut self, whitelist: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whitelist = Some(whitelist);
                    self
    }
      #[inline(always)]
    pub fn whitelist_entry(&mut self, whitelist_entry: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whitelist_entry = Some(whitelist_entry);
                    self
    }
      #[inline(always)]
    pub fn whitelisted_info(&mut self, whitelisted_info: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whitelisted_info = Some(whitelisted_info);
                    self
    }
      #[inline(always)]
    pub fn admin_info(&mut self, admin_info: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.admin_info = Some(admin_info);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
        let instruction = RemoveFromWhitelistCpi {
        __program: self.instruction.__program,
                  
          whitelist: self.instruction.whitelist.expect("whitelist is not set"),
                  
          whitelist_entry: self.instruction.whitelist_entry.expect("whitelist_entry is not set"),
                  
          whitelisted_info: self.instruction.whitelisted_info.expect("whitelisted_info is not set"),
                  
          admin_info: self.instruction.admin_info.expect("admin_info is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct RemoveFromWhitelistCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                whitelist_entry: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                whitelisted_info: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                admin_info: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
