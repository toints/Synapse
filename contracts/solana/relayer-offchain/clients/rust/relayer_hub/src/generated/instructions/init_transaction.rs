//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
pub struct InitTransaction {
            /// Relayer account.

    
              
          pub relayer: solana_program::pubkey::Pubkey,
                /// Program configuration account.

    
              
          pub config: solana_program::pubkey::Pubkey,
                /// Relayer configuration account.

    
              
          pub relayer_info: solana_program::pubkey::Pubkey,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub pool: solana_program::pubkey::Pubkey,
                /// Transaction account.

    
              
          pub transaction: solana_program::pubkey::Pubkey,
                /// Transaction account.

    
              
          pub epoch_sequence: solana_program::pubkey::Pubkey,
                /// Transaction account.

    
              
          pub final_transaction: solana_program::pubkey::Pubkey,
                /// System program.

    
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl InitTransaction {
  pub fn instruction(&self, args: InitTransactionInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: InitTransactionInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(8+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.relayer,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.relayer_info,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.transaction,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.epoch_sequence,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.final_transaction,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = InitTransactionInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::RELAYER_HUB_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct InitTransactionInstructionData {
            discriminator: [u8; 8],
                        }

impl InitTransactionInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [144, 48, 27, 226, 98, 225, 195, 163],
                                                            }
  }
}

impl Default for InitTransactionInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct InitTransactionInstructionArgs {
                  pub sequence: u64,
                pub epoch: u64,
                pub data: Vec<u8>,
      }


/// Instruction builder for `InitTransaction`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` relayer
          ///   1. `[]` config
          ///   2. `[]` relayer_info
                ///   3. `[writable]` pool
                ///   4. `[writable]` transaction
                ///   5. `[writable]` epoch_sequence
                ///   6. `[writable]` final_transaction
                ///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitTransactionBuilder {
            relayer: Option<solana_program::pubkey::Pubkey>,
                config: Option<solana_program::pubkey::Pubkey>,
                relayer_info: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                transaction: Option<solana_program::pubkey::Pubkey>,
                epoch_sequence: Option<solana_program::pubkey::Pubkey>,
                final_transaction: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                        sequence: Option<u64>,
                epoch: Option<u64>,
                data: Option<Vec<u8>>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitTransactionBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            /// Relayer account.
#[inline(always)]
    pub fn relayer(&mut self, relayer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.relayer = Some(relayer);
                    self
    }
            /// Program configuration account.
#[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.config = Some(config);
                    self
    }
            /// Relayer configuration account.
#[inline(always)]
    pub fn relayer_info(&mut self, relayer_info: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.relayer_info = Some(relayer_info);
                    self
    }
            /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pool = Some(pool);
                    self
    }
            /// Transaction account.
#[inline(always)]
    pub fn transaction(&mut self, transaction: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.transaction = Some(transaction);
                    self
    }
            /// Transaction account.
#[inline(always)]
    pub fn epoch_sequence(&mut self, epoch_sequence: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.epoch_sequence = Some(epoch_sequence);
                    self
    }
            /// Transaction account.
#[inline(always)]
    pub fn final_transaction(&mut self, final_transaction: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.final_transaction = Some(final_transaction);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
/// System program.
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn sequence(&mut self, sequence: u64) -> &mut Self {
        self.sequence = Some(sequence);
        self
      }
                #[inline(always)]
      pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
        self
      }
                #[inline(always)]
      pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data = Some(data);
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
    let accounts = InitTransaction {
                              relayer: self.relayer.expect("relayer is not set"),
                                        config: self.config.expect("config is not set"),
                                        relayer_info: self.relayer_info.expect("relayer_info is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        transaction: self.transaction.expect("transaction is not set"),
                                        epoch_sequence: self.epoch_sequence.expect("epoch_sequence is not set"),
                                        final_transaction: self.final_transaction.expect("final_transaction is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
          let args = InitTransactionInstructionArgs {
                                                              sequence: self.sequence.clone().expect("sequence is not set"),
                                                                  epoch: self.epoch.clone().expect("epoch is not set"),
                                                                  data: self.data.clone().expect("data is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `init_transaction` CPI accounts.
  pub struct InitTransactionCpiAccounts<'a, 'b> {
                  /// Relayer account.

      
                    
              pub relayer: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Program configuration account.

      
                    
              pub config: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Relayer configuration account.

      
                    
              pub relayer_info: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction pool account.One transaction pool per chain.

      
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction account.

      
                    
              pub transaction: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction account.

      
                    
              pub epoch_sequence: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Transaction account.

      
                    
              pub final_transaction: &'b solana_program::account_info::AccountInfo<'a>,
                        /// System program.

      
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `init_transaction` CPI instruction.
pub struct InitTransactionCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Relayer account.

    
              
          pub relayer: &'b solana_program::account_info::AccountInfo<'a>,
                /// Program configuration account.

    
              
          pub config: &'b solana_program::account_info::AccountInfo<'a>,
                /// Relayer configuration account.

    
              
          pub relayer_info: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction pool account.One transaction pool per chain.

    
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction account.

    
              
          pub transaction: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction account.

    
              
          pub epoch_sequence: &'b solana_program::account_info::AccountInfo<'a>,
                /// Transaction account.

    
              
          pub final_transaction: &'b solana_program::account_info::AccountInfo<'a>,
                /// System program.

    
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: InitTransactionInstructionArgs,
  }

impl<'a, 'b> InitTransactionCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: InitTransactionCpiAccounts<'a, 'b>,
              args: InitTransactionInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              relayer: accounts.relayer,
              config: accounts.config,
              relayer_info: accounts.relayer_info,
              pool: accounts.pool,
              transaction: accounts.transaction,
              epoch_sequence: accounts.epoch_sequence,
              final_transaction: accounts.final_transaction,
              system_program: accounts.system_program,
                    __args: args,
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
    let mut accounts = Vec::with_capacity(8+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.relayer.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.relayer_info.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.transaction.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.epoch_sequence.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.final_transaction.key,
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
    let mut data = InitTransactionInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::RELAYER_HUB_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.relayer.clone());
                        account_infos.push(self.config.clone());
                        account_infos.push(self.relayer_info.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.transaction.clone());
                        account_infos.push(self.epoch_sequence.clone());
                        account_infos.push(self.final_transaction.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `InitTransaction` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` relayer
          ///   1. `[]` config
          ///   2. `[]` relayer_info
                ///   3. `[writable]` pool
                ///   4. `[writable]` transaction
                ///   5. `[writable]` epoch_sequence
                ///   6. `[writable]` final_transaction
          ///   7. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitTransactionCpiBuilder<'a, 'b> {
  instruction: Box<InitTransactionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitTransactionCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(InitTransactionCpiBuilderInstruction {
      __program: program,
              relayer: None,
              config: None,
              relayer_info: None,
              pool: None,
              transaction: None,
              epoch_sequence: None,
              final_transaction: None,
              system_program: None,
                                            sequence: None,
                                epoch: None,
                                data: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      /// Relayer account.
#[inline(always)]
    pub fn relayer(&mut self, relayer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.relayer = Some(relayer);
                    self
    }
      /// Program configuration account.
#[inline(always)]
    pub fn config(&mut self, config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.config = Some(config);
                    self
    }
      /// Relayer configuration account.
#[inline(always)]
    pub fn relayer_info(&mut self, relayer_info: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.relayer_info = Some(relayer_info);
                    self
    }
      /// Transaction pool account.One transaction pool per chain.
#[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pool = Some(pool);
                    self
    }
      /// Transaction account.
#[inline(always)]
    pub fn transaction(&mut self, transaction: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.transaction = Some(transaction);
                    self
    }
      /// Transaction account.
#[inline(always)]
    pub fn epoch_sequence(&mut self, epoch_sequence: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.epoch_sequence = Some(epoch_sequence);
                    self
    }
      /// Transaction account.
#[inline(always)]
    pub fn final_transaction(&mut self, final_transaction: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.final_transaction = Some(final_transaction);
                    self
    }
      /// System program.
#[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn sequence(&mut self, sequence: u64) -> &mut Self {
        self.instruction.sequence = Some(sequence);
        self
      }
                #[inline(always)]
      pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
        self
      }
                #[inline(always)]
      pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.instruction.data = Some(data);
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
          let args = InitTransactionInstructionArgs {
                                                              sequence: self.instruction.sequence.clone().expect("sequence is not set"),
                                                                  epoch: self.instruction.epoch.clone().expect("epoch is not set"),
                                                                  data: self.instruction.data.clone().expect("data is not set"),
                                    };
        let instruction = InitTransactionCpi {
        __program: self.instruction.__program,
                  
          relayer: self.instruction.relayer.expect("relayer is not set"),
                  
          config: self.instruction.config.expect("config is not set"),
                  
          relayer_info: self.instruction.relayer_info.expect("relayer_info is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          transaction: self.instruction.transaction.expect("transaction is not set"),
                  
          epoch_sequence: self.instruction.epoch_sequence.expect("epoch_sequence is not set"),
                  
          final_transaction: self.instruction.final_transaction.expect("final_transaction is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct InitTransactionCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            relayer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                relayer_info: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                transaction: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                epoch_sequence: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                final_transaction: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        sequence: Option<u64>,
                epoch: Option<u64>,
                data: Option<Vec<u8>>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

