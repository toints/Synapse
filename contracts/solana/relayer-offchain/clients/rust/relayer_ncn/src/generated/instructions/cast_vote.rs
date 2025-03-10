//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct CastVote {
    pub config: solana_program::pubkey::Pubkey,

    pub ballot_box: solana_program::pubkey::Pubkey,

    pub ncn: solana_program::pubkey::Pubkey,

    pub epoch_snapshot: solana_program::pubkey::Pubkey,

    pub operator_snapshot: solana_program::pubkey::Pubkey,

    pub operator: solana_program::pubkey::Pubkey,

    pub operator_admin: solana_program::pubkey::Pubkey,

    pub restaking_program: solana_program::pubkey::Pubkey,
}

impl CastVote {
    pub fn instruction(
        &self,
        args: CastVoteInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CastVoteInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.ballot_box,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.epoch_snapshot,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.operator_snapshot,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.operator,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.operator_admin,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.restaking_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CastVoteInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::RELAYER_NCN_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CastVoteInstructionData {
    discriminator: u8,
}

impl CastVoteInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 14 }
    }
}

impl Default for CastVoteInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CastVoteInstructionArgs {
    pub meta_merkle_root: [u8; 32],
    pub epoch: u64,
}

/// Instruction builder for `CastVote`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` ballot_box
///   2. `[]` ncn
///   3. `[]` epoch_snapshot
///   4. `[]` operator_snapshot
///   5. `[]` operator
///   6. `[signer]` operator_admin
///   7. `[]` restaking_program
#[derive(Clone, Debug, Default)]
pub struct CastVoteBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    ballot_box: Option<solana_program::pubkey::Pubkey>,
    ncn: Option<solana_program::pubkey::Pubkey>,
    epoch_snapshot: Option<solana_program::pubkey::Pubkey>,
    operator_snapshot: Option<solana_program::pubkey::Pubkey>,
    operator: Option<solana_program::pubkey::Pubkey>,
    operator_admin: Option<solana_program::pubkey::Pubkey>,
    restaking_program: Option<solana_program::pubkey::Pubkey>,
    meta_merkle_root: Option<[u8; 32]>,
    epoch: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CastVoteBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn ballot_box(&mut self, ballot_box: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ballot_box = Some(ballot_box);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn epoch_snapshot(&mut self, epoch_snapshot: solana_program::pubkey::Pubkey) -> &mut Self {
        self.epoch_snapshot = Some(epoch_snapshot);
        self
    }
    #[inline(always)]
    pub fn operator_snapshot(
        &mut self,
        operator_snapshot: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.operator_snapshot = Some(operator_snapshot);
        self
    }
    #[inline(always)]
    pub fn operator(&mut self, operator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.operator = Some(operator);
        self
    }
    #[inline(always)]
    pub fn operator_admin(&mut self, operator_admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.operator_admin = Some(operator_admin);
        self
    }
    #[inline(always)]
    pub fn restaking_program(
        &mut self,
        restaking_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.restaking_program = Some(restaking_program);
        self
    }
    #[inline(always)]
    pub fn meta_merkle_root(&mut self, meta_merkle_root: [u8; 32]) -> &mut Self {
        self.meta_merkle_root = Some(meta_merkle_root);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
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
        let accounts = CastVote {
            config: self.config.expect("config is not set"),
            ballot_box: self.ballot_box.expect("ballot_box is not set"),
            ncn: self.ncn.expect("ncn is not set"),
            epoch_snapshot: self.epoch_snapshot.expect("epoch_snapshot is not set"),
            operator_snapshot: self
                .operator_snapshot
                .expect("operator_snapshot is not set"),
            operator: self.operator.expect("operator is not set"),
            operator_admin: self.operator_admin.expect("operator_admin is not set"),
            restaking_program: self
                .restaking_program
                .expect("restaking_program is not set"),
        };
        let args = CastVoteInstructionArgs {
            meta_merkle_root: self
                .meta_merkle_root
                .clone()
                .expect("meta_merkle_root is not set"),
            epoch: self.epoch.clone().expect("epoch is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `cast_vote` CPI accounts.
pub struct CastVoteCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_snapshot: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator_snapshot: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub restaking_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `cast_vote` CPI instruction.
pub struct CastVoteCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_snapshot: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator_snapshot: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub restaking_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CastVoteInstructionArgs,
}

impl<'a, 'b> CastVoteCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CastVoteCpiAccounts<'a, 'b>,
        args: CastVoteInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            ballot_box: accounts.ballot_box,
            ncn: accounts.ncn,
            epoch_snapshot: accounts.epoch_snapshot,
            operator_snapshot: accounts.operator_snapshot,
            operator: accounts.operator,
            operator_admin: accounts.operator_admin,
            restaking_program: accounts.restaking_program,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ballot_box.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.epoch_snapshot.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.operator_snapshot.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.operator.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.operator_admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.restaking_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CastVoteInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::RELAYER_NCN_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.ballot_box.clone());
        account_infos.push(self.ncn.clone());
        account_infos.push(self.epoch_snapshot.clone());
        account_infos.push(self.operator_snapshot.clone());
        account_infos.push(self.operator.clone());
        account_infos.push(self.operator_admin.clone());
        account_infos.push(self.restaking_program.clone());
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

/// Instruction builder for `CastVote` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` ballot_box
///   2. `[]` ncn
///   3. `[]` epoch_snapshot
///   4. `[]` operator_snapshot
///   5. `[]` operator
///   6. `[signer]` operator_admin
///   7. `[]` restaking_program
#[derive(Clone, Debug)]
pub struct CastVoteCpiBuilder<'a, 'b> {
    instruction: Box<CastVoteCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CastVoteCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CastVoteCpiBuilderInstruction {
            __program: program,
            config: None,
            ballot_box: None,
            ncn: None,
            epoch_snapshot: None,
            operator_snapshot: None,
            operator: None,
            operator_admin: None,
            restaking_program: None,
            meta_merkle_root: None,
            epoch: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn ballot_box(
        &mut self,
        ballot_box: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ballot_box = Some(ballot_box);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn epoch_snapshot(
        &mut self,
        epoch_snapshot: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_snapshot = Some(epoch_snapshot);
        self
    }
    #[inline(always)]
    pub fn operator_snapshot(
        &mut self,
        operator_snapshot: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operator_snapshot = Some(operator_snapshot);
        self
    }
    #[inline(always)]
    pub fn operator(
        &mut self,
        operator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operator = Some(operator);
        self
    }
    #[inline(always)]
    pub fn operator_admin(
        &mut self,
        operator_admin: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operator_admin = Some(operator_admin);
        self
    }
    #[inline(always)]
    pub fn restaking_program(
        &mut self,
        restaking_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.restaking_program = Some(restaking_program);
        self
    }
    #[inline(always)]
    pub fn meta_merkle_root(&mut self, meta_merkle_root: [u8; 32]) -> &mut Self {
        self.instruction.meta_merkle_root = Some(meta_merkle_root);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
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
        let args = CastVoteInstructionArgs {
            meta_merkle_root: self
                .instruction
                .meta_merkle_root
                .clone()
                .expect("meta_merkle_root is not set"),
            epoch: self.instruction.epoch.clone().expect("epoch is not set"),
        };
        let instruction = CastVoteCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            ballot_box: self.instruction.ballot_box.expect("ballot_box is not set"),

            ncn: self.instruction.ncn.expect("ncn is not set"),

            epoch_snapshot: self
                .instruction
                .epoch_snapshot
                .expect("epoch_snapshot is not set"),

            operator_snapshot: self
                .instruction
                .operator_snapshot
                .expect("operator_snapshot is not set"),

            operator: self.instruction.operator.expect("operator is not set"),

            operator_admin: self
                .instruction
                .operator_admin
                .expect("operator_admin is not set"),

            restaking_program: self
                .instruction
                .restaking_program
                .expect("restaking_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CastVoteCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ballot_box: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch_snapshot: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    operator_snapshot: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    operator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    operator_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    restaking_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    meta_merkle_root: Option<[u8; 32]>,
    epoch: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
