//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
pub struct RegisterTxPool {
            /// Only owner.

    
              
          pub owner: solana_program::pubkey::Pubkey,
                /// Program configuration account.

    
              
          pub config: solana_program::pubkey::Pubkey,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub pool: solana_program::pubkey::Pubkey,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub final_pool: solana_program::pubkey::Pubkey,
                /// System program.

    
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl RegisterTxPool {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(5+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.final_pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = RegisterTxPoolInstructionData::new().try_to_vec().unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::RELAYER_HUB_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct RegisterTxPoolInstructionData {
            discriminator: [u8; 8],
      }

impl RegisterTxPoolInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [177, 142, 71, 227, 109, 58, 75, 57],
                  }
  }
}

impl Default for RegisterTxPoolInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `RegisterTxPool`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` owner
          ///   1. `[]` config
                ///   2. `[writable]` pool
                ///   3. `[writable]` final_pool
                ///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct RegisterTxPoolBuilder {
            owner: Option<solana_program::pubkey::Pubkey>,
                config: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                final_pool: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RegisterTxPoolBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            /// Only owner.
#[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.owner = Some(owner);
                    self
    }
            /// Program configuration account.
#[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.config = Some(config);
                    self
    }
            /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pool = Some(pool);
                    self
    }
            /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn final_pool(&mut self, final_pool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.final_pool = Some(final_pool);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
/// System program.
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
    let accounts = RegisterTxPool {
                              owner: self.owner.expect("owner is not set"),
                                        config: self.config.expect("config is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        final_pool: self.final_pool.expect("final_pool is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `register_tx_pool` CPI accounts.
  pub struct RegisterTxPoolCpiAccounts<'a, 'b> {
                  /// Only owner.

      
                    
              pub owner: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Program configuration account.

      
                    
              pub config: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction pool account.One transaction pool per chain.

      
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction pool account.One transaction pool per chain.

      
                    
              pub final_pool: &'b solana_program::account_info::AccountInfo<'a>,
                        /// System program.

      
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `register_tx_pool` CPI instruction.
pub struct RegisterTxPoolCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Only owner.

    
              
          pub owner: &'b solana_program::account_info::AccountInfo<'a>,
                /// Program configuration account.

    
              
          pub config: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub final_pool: &'b solana_program::account_info::AccountInfo<'a>,
                /// System program.

    
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> RegisterTxPoolCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: RegisterTxPoolCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              owner: accounts.owner,
              config: accounts.config,
              pool: accounts.pool,
              final_pool: accounts.final_pool,
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
    let mut accounts = Vec::with_capacity(5+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.final_pool.key,
            false
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
    let data = RegisterTxPoolInstructionData::new().try_to_vec().unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::RELAYER_HUB_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.owner.clone());
                        account_infos.push(self.config.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.final_pool.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `RegisterTxPool` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` owner
          ///   1. `[]` config
                ///   2. `[writable]` pool
                ///   3. `[writable]` final_pool
          ///   4. `[]` system_program
#[derive(Clone, Debug)]
pub struct RegisterTxPoolCpiBuilder<'a, 'b> {
  instruction: Box<RegisterTxPoolCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RegisterTxPoolCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(RegisterTxPoolCpiBuilderInstruction {
      __program: program,
              owner: None,
              config: None,
              pool: None,
              final_pool: None,
              system_program: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      /// Only owner.
#[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.owner = Some(owner);
                    self
    }
      /// Program configuration account.
#[inline(always)]
    pub fn config(&mut self, config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.config = Some(config);
                    self
    }
      /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pool = Some(pool);
                    self
    }
      /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn final_pool(&mut self, final_pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.final_pool = Some(final_pool);
                    self
    }
      /// System program.
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
        let instruction = RegisterTxPoolCpi {
        __program: self.instruction.__program,
                  
          owner: self.instruction.owner.expect("owner is not set"),
                  
          config: self.instruction.config.expect("config is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          final_pool: self.instruction.final_pool.expect("final_pool is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct RegisterTxPoolCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                final_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

