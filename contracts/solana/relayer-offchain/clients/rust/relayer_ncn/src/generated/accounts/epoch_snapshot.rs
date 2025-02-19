//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::generated::types::StakeWeights;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EpochSnapshot {
    pub discriminator: u64,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub ncn: Pubkey,
    pub epoch: u64,
    pub bump: u8,
    pub slot_created: u64,
    pub slot_finalized: u64,
    pub operator_count: u64,
    pub vault_count: u64,
    pub operators_registered: u64,
    pub valid_operator_vault_delegations: u64,
    pub stake_weights: StakeWeights,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub reserved: [u8; 128],
}

impl EpochSnapshot {
    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for EpochSnapshot {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for EpochSnapshot {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for EpochSnapshot {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for EpochSnapshot {
    fn owner() -> Pubkey {
        crate::RELAYER_NCN_PROGRAM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for EpochSnapshot {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for EpochSnapshot {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
