use std::time::Duration;

use crate::{
    getters::{
        get_account,
        get_all_operators_in_ncn,
        get_all_vaults_in_ncn,
        get_ballot_box,
    //     get_base_reward_receiver_rewards, get_base_reward_router,
        get_epoch_snapshot,
    //     get_ncn_reward_receiver_rewards, get_ncn_reward_router, get_operator,
        get_operator_snapshot,
        // get_stake_pool_accounts, get_tip_router_config,
        get_vault,
        get_vault_registry, get_weight_table,
    },
    handler::CliHandler,
    log::boring_progress_bar,
};
use anyhow::{anyhow, Ok, Result};
use jito_restaking_client::instructions::{
    InitializeNcnBuilder, InitializeNcnOperatorStateBuilder, InitializeNcnVaultTicketBuilder,
    InitializeOperatorBuilder, InitializeOperatorVaultTicketBuilder, NcnWarmupOperatorBuilder,
    OperatorWarmupNcnBuilder, WarmupNcnVaultTicketBuilder, WarmupOperatorVaultTicketBuilder,
};
use jito_restaking_core::{
    config::Config as RestakingConfig, ncn::Ncn, ncn_operator_state::NcnOperatorState,
    ncn_vault_ticket::NcnVaultTicket, operator::Operator,
    operator_vault_ticket::OperatorVaultTicket,
};
use jito_restaking_core::config::Config;
use relayer_ncn_client::instructions::{AdminRegisterStMintBuilder, AdminSetWeightBuilder, InitializeBallotBoxBuilder, InitializeConfigBuilder, InitializeEpochSnapshotBuilder, InitializeOperatorSnapshotBuilder, InitializeVaultRegistryBuilder, InitializeWeightTableBuilder, ReallocBallotBoxBuilder, ReallocOperatorSnapshotBuilder, ReallocVaultRegistryBuilder, ReallocWeightTableBuilder, RegisterVaultBuilder, SnapshotVaultOperatorDelegationBuilder};
use relayer_ncn_core::{
    config::Config as NcnConfig,
    // account_payer::AccountPayer,
    ballot_box::BallotBox,
    // base_fee_group::BaseFeeGroup,
    // base_reward_router::{BaseRewardReceiver, BaseRewardRouter},
    config::Config as RelayerNcnConfig,
    constants::MAX_REALLOC_BYTES,
    // epoch_marker::EpochMarker,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    // epoch_state::EpochState,
    // ncn_fee_group::NcnFeeGroup,
    // ncn_reward_router::{NcnRewardReceiver, NcnRewardRouter},
    vault_registry::VaultRegistry,
    weight_table::WeightTable,
};
use jito_vault_client::instructions::{
    AddDelegationBuilder, InitializeVaultBuilder, InitializeVaultNcnTicketBuilder,
    InitializeVaultOperatorDelegationBuilder, MintToBuilder, WarmupVaultNcnTicketBuilder,
};
use jito_vault_core::{
    config::Config as VaultConfig, vault::Vault, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
};
use log::info;
use solana_client::rpc_config::RpcSendTransactionConfig;

use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    native_token::sol_to_lamports,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction::{create_account, transfer},
    system_program,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use tokio::time::sleep;
use crate::getters::get_operator;

// --------------------- ADMIN ------------------------------
#[allow(clippy::too_many_arguments)]
pub async fn admin_create_config(
    handler: &CliHandler,
    epochs_before_stall: u64,
    valid_slots_after_consensus: u64,
    // epochs_after_consensus_before_close: u64,
    // dao_fee_bps: u16,
    // block_engine_fee: u16,
    // default_ncn_fee_bps: u16,
    // fee_wallet: Option<Pubkey>,
    // tie_breaker_admin: Option<Pubkey>,
) -> Result<()> {
    let keypair = handler.keypair()?;
    let client = handler.rpc_client();

    let ncn = *handler.ncn()?;

    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
    //
    // let fee_wallet = fee_wallet.unwrap_or_else(|| keypair.pubkey());
    // let tie_breaker_admin = tie_breaker_admin.unwrap_or_else(|| keypair.pubkey());

    let initialize_config_ix = InitializeConfigBuilder::new()
        .config(config)
        .ncn_admin(keypair.pubkey())
        .ncn(ncn)
        .epochs_before_stall(epochs_before_stall)
        .valid_slots_after_consensus(valid_slots_after_consensus)
        // .epochs_after_consensus_before_close(epochs_after_consensus_before_close)
        // .dao_fee_bps(dao_fee_bps)
        // .block_engine_fee_bps(block_engine_fee)
        // .default_ncn_fee_bps(default_ncn_fee_bps)
        // .tie_breaker_admin(keypair.pubkey())
        // .fee_wallet(fee_wallet)
        .restaking_program(handler.restaking_program_id)
        .instruction();

    let program = client.get_account(&handler.relayer_ncn_program_id).await?;

    info!(
        "\n\n----------------------\nProgram: {:?}\n\nProgram Account:\n{:?}\n\nIX:\n{:?}\n----------------------\n",
        &handler.relayer_ncn_program_id, program, &initialize_config_ix
    );

    send_and_log_transaction(
        handler,
        &[initialize_config_ix],
        &[],
        "Created Relayer Ncn Config",
        &[
            format!("NCN: {:?}", ncn),
            format!("Ncn Admin: {:?}", keypair.pubkey()),
            // format!("Fee Wallet: {:?}", fee_wallet),
            // format!("Tie Breaker Admin: {:?}", tie_breaker_admin),
            format!(
                "Valid Slots After Consensus: {:?}",
                valid_slots_after_consensus
            ),
            // format!("DAO Fee BPS: {:?}", dao_fee_bps),
            // format!("Block Engine Fee BPS: {:?}", block_engine_fee),
            // format!("Default NCN Fee BPS: {:?}", default_ncn_fee_bps),
        ],
    )
        .await?;

    Ok(())
}

pub async fn admin_register_st_mint(
    handler: &CliHandler,
    vault: &Pubkey,
    // ncn_fee_group: NcnFeeGroup,
    reward_multiplier_bps: u64,
    // switchboard_feed: Option<Pubkey>,
    no_feed_weight: Option<u128>,
) -> Result<()> {
    let keypair = handler.keypair()?;

    let ncn = *handler.ncn()?;

    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let (vault_registry, _, _) =
        VaultRegistry::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let vault_account = get_vault(handler, vault).await?;

    let mut register_st_mint_builder = AdminRegisterStMintBuilder::new();

    register_st_mint_builder
        .config(config)
        .admin(keypair.pubkey())
        .vault_registry(vault_registry)
        .ncn(ncn)
        .st_mint(vault_account.supported_mint)
        // .ncn_fee_group(ncn_fee_group.group)
        .restaking_program(handler.restaking_program_id)
        .reward_multiplier_bps(reward_multiplier_bps);

    // if let Some(switchboard_feed) = switchboard_feed {
    //     register_st_mint_builder.switchboard_feed(switchboard_feed);
    // }

    if let Some(no_feed_weight) = no_feed_weight {
        register_st_mint_builder.no_feed_weight(no_feed_weight);
    }

    let register_st_mint_ix = register_st_mint_builder.instruction();

    send_and_log_transaction(
        handler,
        &[register_st_mint_ix],
        &[],
        "Registered ST Mint",
        &[
            format!("NCN: {:?}", ncn),
            format!("ST Mint: {:?}", vault_account.supported_mint),
            // format!("NCN Fee Group: {:?}", ncn_fee_group.group),
            format!("Reward Multiplier BPS: {:?}", reward_multiplier_bps),
            // format!(
            //     "Switchboard Feed: {:?}",
            //     switchboard_feed.unwrap_or_default()
            // ),
            format!("No Feed Weight: {:?}", no_feed_weight.unwrap_or_default()),
        ],
    )
        .await?;

    Ok(())
}

pub async fn admin_set_weight(
    handler: &CliHandler,
    vault: &Pubkey,
    epoch: u64,
    weight: u128,
) -> Result<()> {
    let vault_account = get_vault(handler, vault).await?;

    admin_set_weight_with_st_mint(handler, &vault_account.supported_mint, epoch, weight).await
}

pub async fn admin_set_weight_with_st_mint(
    handler: &CliHandler,
    st_mint: &Pubkey,
    epoch: u64,
    weight: u128,
) -> Result<()> {
    let keypair = handler.keypair()?;

    let ncn = *handler.ncn()?;

    let (weight_table, _, _) =
        WeightTable::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let admin_set_weight_ix = AdminSetWeightBuilder::new()
        .ncn(ncn)
        .weight_table(weight_table)
        // .epoch_state(epoch_state)
        .weight_table_admin(keypair.pubkey())
        .restaking_program(jito_restaking_program::id())
        .st_mint(*st_mint)
        .weight(weight)
        .epoch(epoch)
        .instruction();

    send_and_log_transaction(
        handler,
        &[admin_set_weight_ix],
        &[],
        "Set Weight",
        &[
            format!("NCN: {:?}", ncn),
            format!("Epoch: {:?}", epoch),
            format!("ST Mint: {:?}", st_mint),
            format!("Weight: {:?}", weight),
        ],
    )
        .await?;

    Ok(())
}

// pub async fn admin_set_tie_breaker(
//     handler: &CliHandler,
//     epoch: u64,
//     meta_merkle_root: [u8; 32],
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (ballot_box, _, _) =
//         BallotBox::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let set_tie_breaker_ix = AdminSetTieBreakerBuilder::new()
//         .epoch_state(epoch_state)
//         .config(ncn_config)
//         .ballot_box(ballot_box)
//         .ncn(ncn)
//         .tie_breaker_admin(keypair.pubkey())
//         .meta_merkle_root(meta_merkle_root)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[set_tie_breaker_ix],
//         &[],
//         "Set Tie Breaker",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Meta Merkle Root: {:?}", meta_merkle_root),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn admin_set_parameters(
//     handler: &CliHandler,
//     epochs_before_stall: Option<u64>,
//     epochs_after_consensus_before_close: Option<u64>,
//     valid_slots_after_consensus: Option<u64>,
//     starting_valid_epoch: Option<u64>,
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//     let ncn = *handler.ncn()?;
//
//     let config_pda = TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn).0;
//
//     let mut ix = AdminSetParametersBuilder::new();
//     ix.config(config_pda).ncn(ncn).ncn_admin(keypair.pubkey());
//
//     if let Some(epochs) = epochs_before_stall {
//         ix.epochs_before_stall(epochs);
//     }
//
//     if let Some(epochs) = epochs_after_consensus_before_close {
//         ix.epochs_after_consensus_before_close(epochs);
//     }
//
//     if let Some(slots) = valid_slots_after_consensus {
//         ix.valid_slots_after_consensus(slots);
//     }
//
//     if let Some(epoch) = starting_valid_epoch {
//         ix.starting_valid_epoch(epoch);
//     }
//
//     send_and_log_transaction(
//         handler,
//         &[ix.instruction()],
//         &[],
//         "Set Parameters",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Epochs Before Stall: {:?}", epochs_before_stall),
//             format!(
//                 "Epochs After Consensus Before Close: {:?}",
//                 epochs_after_consensus_before_close
//             ),
//             format!(
//                 "Valid Slots After Consensus: {:?}",
//                 valid_slots_after_consensus
//             ),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn admin_fund_account_payer(handler: &CliHandler, amount: f64) -> Result<()> {
//     let keypair = handler.keypair()?;
//     let ncn = *handler.ncn()?;
//
//     let (account_payer, _, _) =
//         AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let transfer_ix = transfer(&keypair.pubkey(), &account_payer, sol_to_lamports(amount));
//
//     send_and_log_transaction(
//         handler,
//         &[transfer_ix],
//         &[],
//         "Fund Account Payer",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Amount: {:?} SOL", amount),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// --------------------- Relayer NCN ------------------------------

pub async fn create_vault_registry(handler: &CliHandler) -> Result<()> {
    let keypair = handler.keypair()?;
    let ncn = *handler.ncn()?;

    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let (vault_registry, _, _) =
        VaultRegistry::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let vault_registry_account = get_account(handler, &vault_registry).await?;

    // Skip if vault registry already exists
    if vault_registry_account.is_none() {
        let initialize_vault_registry_ix = InitializeVaultRegistryBuilder::new()
            .config(config)
            // .account_payer(account_payer)
            .ncn(ncn)
            .vault_registry(vault_registry)
            .payer(keypair.pubkey())
            .instruction();

        send_and_log_transaction(
            handler,
            &[initialize_vault_registry_ix],
            &[],
            "Created Vault Registry",
            &[format!("NCN: {:?}", ncn)],
        )
            .await?;
    }

    // Number of reallocations needed based on VaultRegistry::SIZE
    let num_reallocs = (VaultRegistry::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;

    let realloc_vault_registry_ix = ReallocVaultRegistryBuilder::new()
        .config(config)
        .vault_registry(vault_registry)
        .ncn(ncn)
        // .account_payer(account_payer)
        .system_program(system_program::id())
        .payer(keypair.pubkey())
        .instruction();

    let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
    realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
    for _ in 0..num_reallocs {
        realloc_ixs.push(realloc_vault_registry_ix.clone());
    }

    send_and_log_transaction(
        handler,
        &realloc_ixs,
        &[],
        "Reallocated Vault Registry",
        &[
            format!("NCN: {:?}", ncn),
            format!("Number of reallocations: {:?}", num_reallocs),
        ],
    )
        .await?;

    Ok(())
}

pub async fn register_vault(handler: &CliHandler, vault: &Pubkey) -> Result<()> {
    let ncn = *handler.ncn()?;
    let vault = *vault;

    let restaking_config_address =
        Config::find_program_address(&jito_restaking_program::id()).0;
    // let (relayer_ncn_config_config, _, _) =
    //     RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let (vault_registry, _, _) =
        VaultRegistry::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let vault_ncn_ticket =
        VaultNcnTicket::find_program_address(&handler.vault_program_id, &vault, &ncn).0;

    let (ncn_vault_ticket, _, _) =
        NcnVaultTicket::find_program_address(&handler.restaking_program_id, &ncn, &vault);

    let register_vault_ix = RegisterVaultBuilder::new()
        .restaking_config(restaking_config_address)
        // .config(tip_router_config)
        .vault_registry(vault_registry)
        .vault(vault)
        .ncn(ncn)
        .ncn_vault_ticket(ncn_vault_ticket)
        .vault_ncn_ticket(vault_ncn_ticket)
        .vault_registry(vault_registry)
        .restaking_program_id(handler.restaking_program_id)
        .vault_program_id(handler.vault_program_id)
        .instruction();

    send_and_log_transaction(
        handler,
        &[register_vault_ix],
        &[],
        "Registered Vault",
        &[format!("NCN: {:?}", ncn), format!("Vault: {:?}", vault)],
    )
        .await?;

    Ok(())
}

// pub async fn create_epoch_state(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let (config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (account_payer, _, _) =
//         AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
//     let (epoch_marker, _, _) =
//         EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);
//
//     let epoch_state_account = get_account(handler, &epoch_state).await?;
//
//     // Skip if ballot box already exists
//     if epoch_state_account.is_none() {
//         // Initialize ballot box
//         let initialize_ballot_box_ix = InitializeEpochStateBuilder::new()
//             .epoch_marker(epoch_marker)
//             .config(config)
//             .epoch_state(epoch_state)
//             .ncn(ncn)
//             .epoch(epoch)
//             .account_payer(account_payer)
//             .system_program(system_program::id())
//             .instruction();
//
//         send_and_log_transaction(
//             handler,
//             &[initialize_ballot_box_ix],
//             &[],
//             "Initialized Epoch State",
//             &[format!("NCN: {:?}", ncn), format!("Epoch: {:?}", epoch)],
//         )
//             .await?;
//     }
//
//     // Number of reallocations needed based on BallotBox::SIZE
//     let num_reallocs = (EpochState::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;
//
//     // Realloc ballot box
//     let realloc_ballot_box_ix = ReallocEpochStateBuilder::new()
//         .config(config)
//         .epoch_state(epoch_state)
//         .ncn(ncn)
//         .epoch(epoch)
//         .account_payer(account_payer)
//         .system_program(system_program::id())
//         .instruction();
//
//     let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
//     realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
//     for _ in 0..num_reallocs {
//         realloc_ixs.push(realloc_ballot_box_ix.clone());
//     }
//
//     send_and_log_transaction(
//         handler,
//         &realloc_ixs,
//         &[],
//         "Reallocated Epoch State",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Epoch: {:?}", epoch),
//             format!("Number of reallocations: {:?}", num_reallocs),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }

pub async fn create_weight_table(handler: &CliHandler, epoch: u64) -> Result<()> {
    let ncn = *handler.ncn()?;
    let keypair = handler.keypair()?;
    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let (vault_registry, _, _) =
        VaultRegistry::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    let (weight_table, _, _) =
        WeightTable::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);
    //
    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
    // let (epoch_marker, _, _) =
    //     EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);

    let weight_table_account = get_account(handler, &weight_table).await?;

    // Skip if weight table already exists
    if weight_table_account.is_none() {
        // Initialize weight table
        let initialize_weight_table_ix = InitializeWeightTableBuilder::new()
            // .epoch_marker(epoch_marker)
            .vault_registry(vault_registry)
            .ncn(ncn)
            // .epoch_state(epoch_state)
            .weight_table(weight_table)
            .payer(keypair.pubkey())
            .restaking_program(jito_restaking_program::id())
            // .account_payer(account_payer)
            .system_program(system_program::id())
            .epoch(epoch)
            .instruction();

        send_and_log_transaction(
            handler,
            &[initialize_weight_table_ix],
            &[],
            "Initialized Weight Table",
            &[format!("NCN: {:?}", ncn), format!("Epoch: {:?}", epoch)],
        )
            .await?;
    }

    // Number of reallocations needed based on WeightTable::SIZE
    let num_reallocs = (WeightTable::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;

    // Realloc weight table
    let realloc_weight_table_ix = ReallocWeightTableBuilder::new()
        .config(config)
        .weight_table(weight_table)
        .ncn(ncn)
        // .epoch_state(epoch_state)
        .vault_registry(vault_registry)
        .epoch(epoch)
        .payer(keypair.pubkey())
        // .account_payer(account_payer)
        .system_program(system_program::id())
        .instruction();

    let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
    realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
    for _ in 0..num_reallocs {
        realloc_ixs.push(realloc_weight_table_ix.clone());
    }

    send_and_log_transaction(
        handler,
        &realloc_ixs,
        &[],
        "Reallocated Weight Table",
        &[
            format!("NCN: {:?}", ncn),
            format!("Epoch: {:?}", epoch),
            format!("Number of reallocations: {:?}", num_reallocs),
        ],
    )
        .await?;

    Ok(())
}

// pub async fn set_weight(handler: &CliHandler, vault: &Pubkey, epoch: u64) -> Result<()> {
//     let vault_account = get_vault(handler, vault).await?;
//
//     set_weight_with_st_mint(handler, &vault_account.supported_mint, epoch).await
// }
//
// pub async fn set_weight_with_st_mint(
//     handler: &CliHandler,
//     st_mint: &Pubkey,
//     epoch: u64,
// ) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let vault_registry = get_vault_registry(handler).await?;
//
//     let mint_entry = vault_registry.get_mint_entry(st_mint)?;
//     let switchboard_feed = mint_entry.switchboard_feed();
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (weight_table, _, _) =
//         WeightTable::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let set_weight_ix = SwitchboardSetWeightBuilder::new()
//         .ncn(ncn)
//         .weight_table(weight_table)
//         .epoch_state(epoch_state)
//         .st_mint(*st_mint)
//         .switchboard_feed(*switchboard_feed)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[set_weight_ix],
//         &[],
//         "Set Weight Using Switchboard Feed",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Epoch: {:?}", epoch),
//             format!("ST Mint: {:?}", st_mint),
//             format!("Switchboard Feed: {:?}", switchboard_feed),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }

pub async fn create_epoch_snapshot(handler: &CliHandler, epoch: u64) -> Result<()> {
    let ncn = *handler.ncn()?;
    let keypair = handler.keypair()?;
    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);

    let (weight_table, _, _) =
        WeightTable::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (epoch_snapshot, _, _) =
        EpochSnapshot::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
    // let (epoch_marker, _, _) =
    //     EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);

    let initialize_epoch_snapshot_ix = InitializeEpochSnapshotBuilder::new()
        // .epoch_marker(epoch_marker)
        .config(config)
        .ncn(ncn)
        // .epoch_state(epoch_state)
        .weight_table(weight_table)
        .epoch_snapshot(epoch_snapshot)
        .restaking_program(jito_restaking_program::id())
        .payer(keypair.pubkey())
        // .account_payer(account_payer)
        .system_program(system_program::id())
        .epoch(epoch)
        .instruction();

    send_and_log_transaction(
        handler,
        &[initialize_epoch_snapshot_ix],
        &[],
        "Initialized Epoch Snapshot",
        &[format!("NCN: {:?}", ncn), format!("Epoch: {:?}", epoch)],
    )
        .await?;

    Ok(())
}

pub async fn create_operator_snapshot(
    handler: &CliHandler,
    operator: &Pubkey,
    epoch: u64,
) -> Result<()> {
    let ncn = *handler.ncn()?;
    let keypair = handler.keypair()?;
    let operator = *operator;

    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (ncn_operator_state, _, _) =
        NcnOperatorState::find_program_address(&handler.restaking_program_id, &ncn, &operator);

    let (epoch_snapshot, _, _) =
        EpochSnapshot::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
        &handler.relayer_ncn_program_id,
        &operator,
        &ncn,
        epoch,
    );

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
    // let (epoch_marker, _, _) =
    //     EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);

    let operator_snapshot_account = get_account(handler, &operator_snapshot).await?;

    // Skip if operator snapshot already exists
    if operator_snapshot_account.is_none() {
        // Initialize operator snapshot
        let initialize_operator_snapshot_ix = InitializeOperatorSnapshotBuilder::new()
            // .epoch_marker(epoch_marker)
            .config(config)
            .ncn(ncn)
            .operator(operator)
            // .epoch_state(epoch_state)
            .ncn_operator_state(ncn_operator_state)
            .epoch_snapshot(epoch_snapshot)
            .operator_snapshot(operator_snapshot)
            .payer(keypair.pubkey())
            .restaking_program(jito_restaking_program::id())
            // .account_payer(account_payer)
            .system_program(system_program::id())
            .epoch(epoch)
            .instruction();

        send_and_log_transaction(
            handler,
            &[initialize_operator_snapshot_ix],
            &[],
            "Initialized Operator Snapshot",
            &[
                format!("NCN: {:?}", ncn),
                format!("Operator: {:?}", operator),
                format!("Epoch: {:?}", epoch),
            ],
        )
            .await?;
    }

    // Number of reallocations needed based on OperatorSnapshot::SIZE
    let num_reallocs = (OperatorSnapshot::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;
    let ncn_config = NcnConfig::find_program_address(&relayer_ncn_program::id(), &ncn).0;
    // Realloc operator snapshot
    let realloc_operator_snapshot_ix = ReallocOperatorSnapshotBuilder::new()
        // .config(config)
        .ncn_config(ncn_config)
        .restaking_config(RestakingConfig::find_program_address(&handler.restaking_program_id).0)
        .ncn(ncn)
        .operator(operator)
        // .epoch_state(epoch_state)
        .ncn_operator_state(ncn_operator_state)
        .epoch_snapshot(epoch_snapshot)
        .operator_snapshot(operator_snapshot)
        .payer(keypair.pubkey())
        .restaking_program(jito_restaking_program::id())
        // .account_payer(account_payer)
        .system_program(system_program::id())
        .epoch(epoch)
        .instruction();

    let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
    realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
    for _ in 0..num_reallocs {
        realloc_ixs.push(realloc_operator_snapshot_ix.clone());
    }

    send_and_log_transaction(
        handler,
        &realloc_ixs,
        &[],
        "Reallocated Operator Snapshot",
        &[
            format!("NCN: {:?}", ncn),
            format!("Operator: {:?}", operator),
            format!("Epoch: {:?}", epoch),
            format!("Number of reallocations: {:?}", num_reallocs),
        ],
    )
        .await?;

    Ok(())
}
pub async fn snapshot_vault_operator_delegation(
    handler: &CliHandler,
    vault: &Pubkey,
    operator: &Pubkey,
    epoch: u64,
) -> Result<()> {
    let ncn = *handler.ncn()?;

    let vault = *vault;
    let operator = *operator;

    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (restaking_config, _, _) =
        RestakingConfig::find_program_address(&handler.restaking_program_id);

    let (vault_ncn_ticket, _, _) =
        VaultNcnTicket::find_program_address(&handler.vault_program_id, &vault, &ncn);

    let (ncn_vault_ticket, _, _) =
        NcnVaultTicket::find_program_address(&handler.restaking_program_id, &ncn, &vault);

    let (vault_operator_delegation, _, _) =
        VaultOperatorDelegation::find_program_address(&handler.vault_program_id, &vault, &operator);

    let (weight_table, _, _) =
        WeightTable::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (epoch_snapshot, _, _) =
        EpochSnapshot::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
        &handler.relayer_ncn_program_id,
        &operator,
        &ncn,
        epoch,
    );

    let snapshot_vault_operator_delegation_ix = SnapshotVaultOperatorDelegationBuilder::new()
        .config(config)
        // .epoch_state(epoch_state)
        .restaking_config(restaking_config)
        .ncn(ncn)
        .operator(operator)
        .vault(vault)
        .vault_ncn_ticket(vault_ncn_ticket)
        .ncn_vault_ticket(ncn_vault_ticket)
        .vault_operator_delegation(vault_operator_delegation)
        .weight_table(weight_table)
        .epoch_snapshot(epoch_snapshot)
        .operator_snapshot(operator_snapshot)
        .vault_program(jito_vault_program::id())
        .restaking_program(jito_restaking_program::id())
        .epoch(epoch)
        .instruction();

    send_and_log_transaction(
        handler,
        &[snapshot_vault_operator_delegation_ix],
        &[],
        "Snapshotted Vault Operator Delegation",
        &[
            format!("NCN: {:?}", ncn),
            format!("Vault: {:?}", vault),
            format!("Operator: {:?}", operator),
            format!("Epoch: {:?}", epoch),
        ],
    )
        .await?;

    Ok(())
}

pub async fn create_ballot_box(handler: &CliHandler, epoch: u64) -> Result<()> {
    let ncn = *handler.ncn()?;
    let keypair = handler.keypair()?;
    let (config, _, _) =
        RelayerNcnConfig::find_program_address(&handler.relayer_ncn_program_id, &ncn);

    // let (epoch_state, _, _) =
    //     EpochState::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    let (ballot_box, _, _) =
        BallotBox::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    // let (account_payer, _, _) =
    //     AccountPayer::find_program_address(&handler.relayer_ncn_program_id, &ncn);
    // let (epoch_marker, _, _) =
    //     EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);

    let ballot_box_account = get_account(handler, &ballot_box).await?;

    // Skip if ballot box already exists
    if ballot_box_account.is_none() {
        // Initialize ballot box
        let initialize_ballot_box_ix = InitializeBallotBoxBuilder::new()
            // .epoch_marker(epoch_marker)
            .config(config)
            // .epoch_state(epoch_state)
            .ballot_box(ballot_box)
            .ncn(ncn)
            .epoch(epoch)
            .payer(keypair.pubkey())
            // .account_payer(account_payer)
            .system_program(system_program::id())
            .instruction();

        send_and_log_transaction(
            handler,
            &[initialize_ballot_box_ix],
            &[],
            "Initialized Ballot Box",
            &[format!("NCN: {:?}", ncn), format!("Epoch: {:?}", epoch)],
        )
            .await?;
    }

    // Number of reallocations needed based on BallotBox::SIZE
    let num_reallocs = (BallotBox::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;

    // Realloc ballot box
    let realloc_ballot_box_ix = ReallocBallotBoxBuilder::new()
        .config(config)
        // .epoch_state(epoch_state)
        .ballot_box(ballot_box)
        .ncn(ncn)
        .epoch(epoch)
        .payer(keypair.pubkey())
        // .account_payer(account_payer)
        .system_program(system_program::id())
        .instruction();

    let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
    realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
    for _ in 0..num_reallocs {
        realloc_ixs.push(realloc_ballot_box_ix.clone());
    }

    send_and_log_transaction(
        handler,
        &realloc_ixs,
        &[],
        "Reallocated Ballot Box",
        &[
            format!("NCN: {:?}", ncn),
            format!("Epoch: {:?}", epoch),
            format!("Number of reallocations: {:?}", num_reallocs),
        ],
    )
        .await?;

    Ok(())
}

// pub async fn operator_cast_vote(
//     handler: &CliHandler,
//     operator: &Pubkey,
//     epoch: u64,
//     meta_merkle_root: [u8; 32],
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//
//     let ncn = *handler.ncn()?;
//
//     let operator = *operator;
//
//     let (config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ballot_box, _, _) =
//         BallotBox::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (epoch_snapshot, _, _) =
//         EpochSnapshot::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//         &handler.tip_router_program_id,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let cast_vote_ix = CastVoteBuilder::new()
//         .config(config)
//         .epoch_state(epoch_state)
//         .ballot_box(ballot_box)
//         .ncn(ncn)
//         .epoch_snapshot(epoch_snapshot)
//         .operator_snapshot(operator_snapshot)
//         .operator(operator)
//         .operator_voter(keypair.pubkey())
//         .meta_merkle_root(meta_merkle_root)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[cast_vote_ix],
//         &[],
//         "Cast Vote",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Operator: {:?}", operator),
//             format!("Meta Merkle Root: {:?}", meta_merkle_root),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn create_base_reward_router(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_receiver, _, _) =
//         BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (account_payer, _, _) =
//         AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
//     let (epoch_marker, _, _) =
//         EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);
//
//     let base_reward_router_account = get_account(handler, &base_reward_router).await?;
//
//     // Skip if base reward router already exists
//     if base_reward_router_account.is_none() {
//         let initialize_base_reward_router_ix = InitializeBaseRewardRouterBuilder::new()
//             .epoch_marker(epoch_marker)
//             .ncn(ncn)
//             .epoch_state(epoch_state)
//             .base_reward_router(base_reward_router)
//             .base_reward_receiver(base_reward_receiver)
//             .account_payer(account_payer)
//             .system_program(system_program::id())
//             .epoch(epoch)
//             .instruction();
//
//         send_and_log_transaction(
//             handler,
//             &[initialize_base_reward_router_ix],
//             &[],
//             "Initialized Base Reward Router",
//             &[format!("NCN: {:?}", ncn), format!("Epoch: {:?}", epoch)],
//         )
//             .await?;
//     }
//
//     // Number of reallocations needed based on BaseRewardRouter::SIZE
//     let num_reallocs = (BaseRewardRouter::SIZE as f64 / MAX_REALLOC_BYTES as f64).ceil() as u64 - 1;
//
//     let realloc_base_reward_router_ix = ReallocBaseRewardRouterBuilder::new()
//         .config(TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn).0)
//         .epoch_state(epoch_state)
//         .base_reward_router(base_reward_router)
//         .ncn(ncn)
//         .epoch(epoch)
//         .account_payer(account_payer)
//         .system_program(system_program::id())
//         .instruction();
//
//     let mut realloc_ixs = Vec::with_capacity(num_reallocs as usize);
//     realloc_ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(1_400_000));
//     for _ in 0..num_reallocs {
//         realloc_ixs.push(realloc_base_reward_router_ix.clone());
//     }
//
//     send_and_log_transaction(
//         handler,
//         &realloc_ixs,
//         &[],
//         "Reallocated Base Reward Router",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Epoch: {:?}", epoch),
//             format!("Number of reallocations: {:?}", num_reallocs),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn create_ncn_reward_router(
//     handler: &CliHandler,
//     ncn_fee_group: NcnFeeGroup,
//     operator: &Pubkey,
//     epoch: u64,
// ) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let operator = *operator;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//         &handler.tip_router_program_id,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (account_payer, _, _) =
//         AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
//     let (epoch_marker, _, _) =
//         EpochMarker::find_program_address(&jito_tip_router_program::id(), &ncn, epoch);
//
//     let initialize_ncn_reward_router_ix = InitializeNcnRewardRouterBuilder::new()
//         .epoch_marker(epoch_marker)
//         .epoch_state(epoch_state)
//         .ncn(ncn)
//         .operator(operator)
//         .operator_snapshot(operator_snapshot)
//         .ncn_reward_router(ncn_reward_router)
//         .ncn_reward_receiver(ncn_reward_receiver)
//         .account_payer(account_payer)
//         .system_program(system_program::id())
//         .ncn_fee_group(ncn_fee_group.group)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[initialize_ncn_reward_router_ix],
//         &[],
//         "Initialized NCN Reward Router",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Operator: {:?}", operator),
//             format!("NCN Fee Group: {:?}", ncn_fee_group.group),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn route_base_rewards(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let config = TipRouterConfig::find_program_address(&jito_tip_router_program::id(), &ncn).0;
//
//     let (epoch_snapshot, _, _) =
//         EpochSnapshot::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ballot_box, _, _) =
//         BallotBox::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_receiver, _, _) =
//         BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     // Using max iterations defined in BaseRewardRouter
//     let max_iterations: u16 = BaseRewardRouter::MAX_ROUTE_BASE_ITERATIONS;
//
//     let mut still_routing = true;
//     while still_routing {
//         let route_base_rewards_ix = RouteBaseRewardsBuilder::new()
//             .epoch_state(epoch_state)
//             .config(config)
//             .ncn(ncn)
//             .epoch_snapshot(epoch_snapshot)
//             .ballot_box(ballot_box)
//             .base_reward_router(base_reward_router)
//             .base_reward_receiver(base_reward_receiver)
//             .max_iterations(max_iterations)
//             .epoch(epoch)
//             .instruction();
//
//         let instructions = vec![
//             ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
//             route_base_rewards_ix,
//         ];
//
//         send_and_log_transaction(
//             handler,
//             &instructions,
//             &[],
//             "Routed Base Rewards",
//             &[
//                 format!("NCN: {:?}", ncn),
//                 format!("Epoch: {:?}", epoch),
//                 format!("Max iterations: {:?}", max_iterations),
//             ],
//         )
//             .await?;
//
//         // Check if we need to continue routing
//         let base_reward_router_account = get_base_reward_router(handler, epoch).await?;
//         still_routing = base_reward_router_account.still_routing();
//     }
//
//     Ok(())
// }
//
// pub async fn route_ncn_rewards(
//     handler: &CliHandler,
//     operator: &Pubkey,
//     ncn_fee_group: NcnFeeGroup,
//     epoch: u64,
// ) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let operator = *operator;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//         &handler.tip_router_program_id,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     // Using max iterations defined in NcnRewardRouter
//     let max_iterations: u16 = NcnRewardRouter::MAX_ROUTE_NCN_ITERATIONS;
//
//     let mut still_routing = true;
//     while still_routing {
//         let route_ncn_rewards_ix = RouteNcnRewardsBuilder::new()
//             .epoch_state(epoch_state)
//             .ncn(ncn)
//             .operator(operator)
//             .operator_snapshot(operator_snapshot)
//             .ncn_reward_router(ncn_reward_router)
//             .ncn_reward_receiver(ncn_reward_receiver)
//             .ncn_fee_group(ncn_fee_group.group)
//             .max_iterations(max_iterations)
//             .epoch(epoch)
//             .instruction();
//
//         let instructions = vec![
//             ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
//             route_ncn_rewards_ix,
//         ];
//
//         send_and_log_transaction(
//             handler,
//             &instructions,
//             &[],
//             "Routed NCN Rewards",
//             &[
//                 format!("NCN: {:?}", ncn),
//                 format!("Operator: {:?}", operator),
//                 format!("NCN Fee Group: {:?}", ncn_fee_group.group),
//                 format!("Epoch: {:?}", epoch),
//                 format!("Max iterations: {:?}", max_iterations),
//             ],
//         )
//             .await?;
//
//         // Check if we need to continue routing
//         let ncn_reward_router_account =
//             get_ncn_reward_router(handler, ncn_fee_group, &operator, epoch).await?;
//         still_routing = ncn_reward_router_account.still_routing();
//     }
//
//     Ok(())
// }
//
// pub async fn distribute_base_ncn_rewards(
//     handler: &CliHandler,
//     operator: &Pubkey,
//     ncn_fee_group: NcnFeeGroup,
//     epoch: u64,
// ) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     let operator = *operator;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_receiver, _, _) =
//         BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         &operator,
//         &ncn,
//         epoch,
//     );
//
//     let distribute_base_ncn_rewards_ix = DistributeBaseNcnRewardRouteBuilder::new()
//         .epoch_state(epoch_state)
//         .config(ncn_config)
//         .ncn(ncn)
//         .operator(operator)
//         .base_reward_router(base_reward_router)
//         .base_reward_receiver(base_reward_receiver)
//         .ncn_reward_router(ncn_reward_router)
//         .ncn_reward_receiver(ncn_reward_receiver)
//         .system_program(system_program::id())
//         .ncn_fee_group(ncn_fee_group.group)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[distribute_base_ncn_rewards_ix],
//         &[],
//         "Distributed Base NCN Rewards",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Operator: {:?}", operator),
//             format!("NCN Fee Group: {:?}", ncn_fee_group.group),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn distribute_base_rewards(
//     handler: &CliHandler,
//     base_fee_group: BaseFeeGroup,
//     epoch: u64,
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_receiver, _, _) =
//         BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let tip_router_config = get_tip_router_config(handler).await?;
//     let base_fee_wallet = tip_router_config
//         .fee_config
//         .base_fee_wallet(base_fee_group)?;
//
//     let stake_pool_accounts = get_stake_pool_accounts(handler).await?;
//
//     let base_fee_wallet_ata =
//         get_associated_token_address(base_fee_wallet, &stake_pool_accounts.stake_pool.pool_mint);
//
//     let create_base_fee_wallet_ata_ix =
//         spl_associated_token_account::instruction::create_associated_token_account_idempotent(
//             &keypair.pubkey(),
//             base_fee_wallet,
//             &stake_pool_accounts.stake_pool.pool_mint,
//             &handler.token_program_id,
//         );
//
//     let distribute_base_ncn_rewards_ix = DistributeBaseRewardsBuilder::new()
//         .epoch_state(epoch_state)
//         .config(ncn_config)
//         .ncn(ncn)
//         .base_reward_router(base_reward_router)
//         .base_reward_receiver(base_reward_receiver)
//         .system_program(system_program::id())
//         .epoch(epoch)
//         .base_fee_wallet(*base_fee_wallet)
//         .base_fee_wallet_ata(base_fee_wallet_ata)
//         .base_fee_group(base_fee_group.group)
//         .pool_mint(stake_pool_accounts.stake_pool.pool_mint)
//         .manager_fee_account(stake_pool_accounts.stake_pool.manager_fee_account)
//         .referrer_pool_tokens_account(stake_pool_accounts.referrer_pool_tokens_account)
//         .reserve_stake(stake_pool_accounts.stake_pool.reserve_stake)
//         .stake_pool(stake_pool_accounts.stake_pool_address)
//         .stake_pool_withdraw_authority(stake_pool_accounts.stake_pool_withdraw_authority)
//         .stake_pool_program(stake_pool_accounts.stake_pool_program_id)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[
//             create_base_fee_wallet_ata_ix,
//             distribute_base_ncn_rewards_ix,
//         ],
//         &[],
//         "Distributed Base Rewards",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Base Fee Group: {:?}", base_fee_group.group),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn distribute_ncn_vault_rewards(
//     handler: &CliHandler,
//     vault: &Pubkey,
//     operator: &Pubkey,
//     ncn_fee_group: NcnFeeGroup,
//     epoch: u64,
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//         &handler.tip_router_program_id,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let stake_pool_accounts = get_stake_pool_accounts(handler).await?;
//
//     let vault = *vault;
//     let vault_ata = get_associated_token_address(&vault, &stake_pool_accounts.stake_pool.pool_mint);
//
//     let create_vault_ata_ix =
//         spl_associated_token_account::instruction::create_associated_token_account_idempotent(
//             &keypair.pubkey(),
//             &vault,
//             &stake_pool_accounts.stake_pool.pool_mint,
//             &handler.token_program_id,
//         );
//
//     let distribute_ncn_vault_rewards_ix = DistributeNcnVaultRewardsBuilder::new()
//         .epoch_state(epoch_state)
//         .config(ncn_config)
//         .ncn(ncn)
//         .operator(*operator)
//         .vault(vault)
//         .vault_ata(vault_ata)
//         .operator_snapshot(operator_snapshot)
//         .ncn_reward_router(ncn_reward_router)
//         .ncn_reward_receiver(ncn_reward_receiver)
//         .pool_mint(stake_pool_accounts.stake_pool.pool_mint)
//         .manager_fee_account(stake_pool_accounts.stake_pool.manager_fee_account)
//         .referrer_pool_tokens_account(stake_pool_accounts.referrer_pool_tokens_account)
//         .reserve_stake(stake_pool_accounts.stake_pool.reserve_stake)
//         .stake_pool(stake_pool_accounts.stake_pool_address)
//         .stake_pool_withdraw_authority(stake_pool_accounts.stake_pool_withdraw_authority)
//         .stake_pool_program(stake_pool_accounts.stake_pool_program_id)
//         .token_program(handler.token_program_id)
//         .system_program(system_program::id())
//         .ncn_fee_group(ncn_fee_group.group)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[create_vault_ata_ix, distribute_ncn_vault_rewards_ix],
//         &[],
//         "Distributed NCN Vault Rewards",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Vault: {:?}", vault),
//             format!("Operator: {:?}", operator),
//             format!("NCN Fee Group: {:?}", ncn_fee_group.group),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn distribute_ncn_operator_rewards(
//     handler: &CliHandler,
//     operator: &Pubkey,
//     ncn_fee_group: NcnFeeGroup,
//     epoch: u64,
// ) -> Result<()> {
//     let keypair = handler.keypair()?;
//     let ncn = *handler.ncn()?;
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (ncn_config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//         &handler.tip_router_program_id,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     let stake_pool_accounts = get_stake_pool_accounts(handler).await?;
//
//     let operator_ata =
//         get_associated_token_address(operator, &stake_pool_accounts.stake_pool.pool_mint);
//
//     let create_operator_ata_ix =
//         spl_associated_token_account::instruction::create_associated_token_account_idempotent(
//             &keypair.pubkey(),
//             operator,
//             &stake_pool_accounts.stake_pool.pool_mint,
//             &handler.token_program_id,
//         );
//
//     let distribute_ncn_operator_rewards_ix = DistributeNcnOperatorRewardsBuilder::new()
//         .epoch_state(epoch_state)
//         .config(ncn_config)
//         .ncn(ncn)
//         .operator(*operator)
//         .operator_ata(operator_ata)
//         .operator_snapshot(operator_snapshot)
//         .ncn_reward_router(ncn_reward_router)
//         .ncn_reward_receiver(ncn_reward_receiver)
//         .pool_mint(stake_pool_accounts.stake_pool.pool_mint)
//         .manager_fee_account(stake_pool_accounts.stake_pool.manager_fee_account)
//         .referrer_pool_tokens_account(stake_pool_accounts.referrer_pool_tokens_account)
//         .reserve_stake(stake_pool_accounts.stake_pool.reserve_stake)
//         .stake_pool(stake_pool_accounts.stake_pool_address)
//         .stake_pool_withdraw_authority(stake_pool_accounts.stake_pool_withdraw_authority)
//         .stake_pool_program(stake_pool_accounts.stake_pool_program_id)
//         .token_program(handler.token_program_id)
//         .system_program(system_program::id())
//         .ncn_fee_group(ncn_fee_group.group)
//         .epoch(epoch)
//         .instruction();
//
//     send_and_log_transaction(
//         handler,
//         &[create_operator_ata_ix, distribute_ncn_operator_rewards_ix],
//         &[],
//         "Distributed NCN Operator Rewards",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Operator: {:?}", operator),
//             format!("NCN Fee Group: {:?}", ncn_fee_group.group),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// pub async fn close_epoch_account(
//     handler: &CliHandler,
//     ncn: Pubkey,
//     epoch: u64,
//     account_to_close: Pubkey,
//     receiver_to_close: Option<Pubkey>,
// ) -> Result<()> {
//     let (epoch_marker, _, _) =
//         EpochMarker::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (account_payer, _, _) =
//         AccountPayer::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let (config, _, _) =
//         TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
//
//     let account_already_closed = get_account(handler, &account_to_close)
//         .await?
//         .map_or(true, |account| {
//             account.data.is_empty() || account.lamports == 0
//         });
//     if account_already_closed {
//         info!("Account already closed: {:?}", account_to_close);
//         return Ok(());
//     }
//
//     let config_account = get_tip_router_config(handler).await?;
//     let dao_wallet = *config_account
//         .fee_config
//         .base_fee_wallet(BaseFeeGroup::dao())
//         .expect("No DAO wallet ( close_epoch_account )");
//
//     let mut ix = CloseEpochAccountBuilder::new();
//
//     ix.account_payer(account_payer)
//         .epoch_marker(epoch_marker)
//         .config(config)
//         .account_to_close(account_to_close)
//         .epoch_state(epoch_state)
//         .ncn(ncn)
//         .dao_wallet(dao_wallet)
//         .system_program(system_program::id())
//         .epoch(epoch);
//
//     if let Some(receiver_to_close) = receiver_to_close {
//         ix.receiver_to_close(Some(receiver_to_close));
//     }
//
//     send_and_log_transaction(
//         handler,
//         &[ix.instruction()],
//         &[],
//         "Close Epoch Account",
//         &[
//             format!("NCN: {:?}", ncn),
//             format!("Account to Close: {:?}", account_to_close),
//             format!("Receiver to Close: {:?}", receiver_to_close),
//             format!("Epoch: {:?}", epoch),
//         ],
//     )
//         .await?;
//
//     Ok(())
// }
//
// --------------------- MIDDLEWARE ------------------------------

pub async fn get_or_create_weight_table(handler: &CliHandler, epoch: u64) -> Result<WeightTable> {
    let ncn = *handler.ncn()?;

    let (weight_table, _, _) =
        WeightTable::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    if get_account(handler, &weight_table)
        .await?
        .map_or(true, |table| table.data.len() < WeightTable::SIZE)
    {
        create_weight_table(handler, epoch).await?;
        // TODO:
        let all_ncn_vaults = get_all_vaults_in_ncn(handler).await?;
        let vault = all_ncn_vaults[0];
        println!("==========vault:{}, epoch:{}", vault, epoch);
        admin_set_weight(handler, &vault, epoch, 100).await.expect("TODO: panic message");
    }
    get_weight_table(handler, epoch).await
}

pub async fn get_or_create_epoch_snapshot(
    handler: &CliHandler,
    epoch: u64,
) -> Result<EpochSnapshot> {
    let ncn = *handler.ncn()?;
    let (epoch_snapshot, _, _) =
        EpochSnapshot::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    if get_account(handler, &epoch_snapshot)
        .await?
        .map_or(true, |snapshot| snapshot.data.len() < EpochSnapshot::SIZE)
    {
        create_epoch_snapshot(handler, epoch).await?;
    }

    get_epoch_snapshot(handler, epoch).await
}

pub async fn get_or_create_operator_snapshot(
    handler: &CliHandler,
    operator: &Pubkey,
    epoch: u64,
) -> Result<OperatorSnapshot> {
    let ncn = *handler.ncn()?;
    let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
        &handler.relayer_ncn_program_id,
        operator,
        &ncn,
        epoch,
    );

    if get_account(handler, &operator_snapshot)
        .await?
        .map_or(true, |snapshot| {
            snapshot.data.len() < OperatorSnapshot::SIZE
        })
    {
        create_operator_snapshot(handler, operator, epoch).await?;
    }
    get_operator_snapshot(handler, operator, epoch).await
}

#[allow(clippy::large_stack_frames)]
pub async fn get_or_create_ballot_box(handler: &CliHandler, epoch: u64) -> Result<BallotBox> {
    let ncn = *handler.ncn()?;
    let (ballot_box, _, _) =
        BallotBox::find_program_address(&handler.relayer_ncn_program_id, &ncn, epoch);

    if get_account(handler, &ballot_box)
        .await?
        .map_or(true, |ballot_box| ballot_box.data.len() < BallotBox::SIZE)
    {
        create_ballot_box(handler, epoch).await?;
    }
    get_ballot_box(handler, epoch).await
}

// pub async fn get_or_create_base_reward_router(
//     handler: &CliHandler,
//     epoch: u64,
// ) -> Result<BaseRewardRouter> {
//     let ncn = *handler.ncn()?;
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     if get_account(handler, &base_reward_router)
//         .await?
//         .map_or(true, |router| router.data.len() < BaseRewardRouter::SIZE)
//     {
//         create_base_reward_router(handler, epoch).await?;
//     }
//     get_base_reward_router(handler, epoch).await
// }
//
// pub async fn get_or_create_ncn_reward_router(
//     handler: &CliHandler,
//     ncn_fee_group: NcnFeeGroup,
//     operator: &Pubkey,
//     epoch: u64,
// ) -> Result<NcnRewardRouter> {
//     let ncn = *handler.ncn()?;
//     let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//         &handler.tip_router_program_id,
//         ncn_fee_group,
//         operator,
//         &ncn,
//         epoch,
//     );
//
//     if get_account(handler, &ncn_reward_router)
//         .await?
//         .map_or(true, |router| router.data.len() < NcnRewardRouter::SIZE)
//     {
//         create_ncn_reward_router(handler, ncn_fee_group, operator, epoch).await?;
//     }
//     get_ncn_reward_router(handler, ncn_fee_group, operator, epoch).await
// }
//
// --------------------- CRANKERS ------------------------------

pub async fn crank_register_vaults(handler: &CliHandler) -> Result<()> {
    let all_ncn_vaults = get_all_vaults_in_ncn(handler).await?;
    let vault_registry = get_vault_registry(handler).await?;
    let all_registered_vaults: Vec<Pubkey> = vault_registry
        .get_valid_vault_entries()
        .iter()
        .map(|entry| *entry.vault())
        .collect();

    let vaults_to_register: Vec<Pubkey> = all_ncn_vaults
        .iter()
        .filter(|vault| !all_registered_vaults.contains(vault))
        .copied()
        .collect();

    //TODO check if ST mint is registered first

    for vault in vaults_to_register.iter() {
        let result = register_vault(handler, vault).await;

        if let Err(err) = result {
            log::error!(
                "Failed to register vault: {:?} with error: {:?}",
                vault,
                err
            );
        }
    }

    Ok(())
}

pub async fn crank_set_weight(handler: &CliHandler, epoch: u64) -> Result<()> {
    let weight_table = get_or_create_weight_table(handler, epoch).await?;
    // let st_mints = weight_table
    //     .table()
    //     .iter()
    //     .filter(|entry| !entry.is_empty() && !entry.is_set())
    //     .map(|entry| *entry.st_mint())
    //     .collect::<Vec<Pubkey>>();
    //
    // for st_mint in st_mints {
    //     let result = set_weight_with_st_mint(handler, &st_mint, epoch).await;
    //
    //     if let Err(err) = result {
    //         log::error!(
    //             "Failed to set weight for st_mint: {:?} in epoch: {:?} with error: {:?}",
    //             st_mint,
    //             epoch,
    //             err
    //         );
    //     }
    // }

    Ok(())
}

pub async fn crank_snapshot(handler: &CliHandler, epoch: u64) -> Result<()> {
    let vault_registry = get_vault_registry(handler).await?;

    let operators = get_all_operators_in_ncn(handler).await?;
    let all_vaults: Vec<Pubkey> = vault_registry
        .get_valid_vault_entries()
        .iter()
        .map(|entry| *entry.vault())
        .collect();

    let epoch_snapshot = get_or_create_epoch_snapshot(handler, epoch).await?;
    if epoch_snapshot.finalized() {
        log::info!(
            "Epoch snapshot already finalized for epoch: {:?}. Skipping snapshotting.",
            epoch
        );
        return Ok(());
    }

    for operator in operators.iter() {
        // Create Vault Operator Delegation
        let result = get_or_create_operator_snapshot(handler, operator, epoch).await;

        if result.is_err() {
            log::error!(
                "Failed to get or create operator snapshot for operator: {:?} in epoch: {:?} with error: {:?}",
                operator,
                epoch,
                result.err().unwrap()
            );
            continue;
        };

        let operator_snapshot = result?;

        let vaults_to_run: Vec<Pubkey> = all_vaults
            .iter()
            .filter(|vault| !operator_snapshot.contains_vault(vault))
            .cloned()
            .collect();

        for vault in vaults_to_run.iter() {
            let result = snapshot_vault_operator_delegation(handler, &vault, operator, epoch).await;

            if let Err(err) = result {
                log::error!(
                    "Failed to snapshot vault operator delegation for vault: {:?} and operator: {:?} in epoch: {:?} with error: {:?}",
                    vault,
                    operator,
                    epoch,
                    err
                );
            }
        }
    }

    Ok(())
}

#[allow(clippy::large_stack_frames)]
pub async fn crank_vote(handler: &CliHandler, epoch: u64) -> Result<()> {
    // VOTE

    let ballot_box = get_or_create_ballot_box(handler, epoch).await?;
    if ballot_box.is_consensus_reached() {
        log::info!(
            "Consensus already reached for epoch: {:?}. Skipping voting.",
            epoch
        );
        return Ok(());
    }

    Ok(())
}

// #[allow(clippy::large_stack_frames)]
// pub async fn crank_test_vote(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let voter = handler.keypair()?.pubkey();
//     let meta_merkle_root = [8; 32];
//     let operators = get_all_operators_in_ncn(handler).await?;
//
//     for operator in operators.iter() {
//         let operator_account = get_operator(handler, operator).await?;
//
//         if operator_account.voter.ne(&voter) {
//             continue;
//         }
//
//         let result = operator_cast_vote(handler, operator, epoch, meta_merkle_root).await;
//
//         if let Err(err) = result {
//             log::error!(
//                 "Failed to cast vote for operator: {:?} in epoch: {:?} with error: {:?}",
//                 operator,
//                 epoch,
//                 err
//             );
//         }
//     }
//
//     let ballot_box = get_or_create_ballot_box(handler, epoch).await?;
//
//     // Send 'Test' Rewards
//     if ballot_box.is_consensus_reached() {
//         let (base_reward_receiver_address, _, _) = BaseRewardReceiver::find_program_address(
//             &handler.tip_router_program_id,
//             handler.ncn().unwrap(),
//             epoch,
//         );
//
//         let base_reward_receiver = get_account(handler, &base_reward_receiver_address).await?;
//
//         if base_reward_receiver.is_none() {
//             let keypair = handler.keypair()?;
//
//             let lamports = sol_to_lamports(0.1);
//             let transfer_ix = transfer(&keypair.pubkey(), &base_reward_receiver_address, lamports);
//
//             send_and_log_transaction(
//                 handler,
//                 &[transfer_ix],
//                 &[],
//                 "Sent Test Rewards",
//                 &[format!("Epoch: {:?}", epoch)],
//             )
//                 .await?;
//         }
//     }
//
//     Ok(())
// }
//
// //TODO Multi-thread sending the TXs
// pub async fn crank_distribute(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let operators = get_all_operators_in_ncn(handler).await?;
//
//     let epoch_snapshot = get_or_create_epoch_snapshot(handler, epoch).await?;
//     let fees = epoch_snapshot.fees();
//
//     let base_reward_router = get_or_create_base_reward_router(handler, epoch).await?;
//
//     let base_reward_receiver_rewards = get_base_reward_receiver_rewards(handler, epoch).await?;
//     if base_reward_receiver_rewards > 0 {
//         route_base_rewards(handler, epoch).await?;
//     }
//
//     for group in BaseFeeGroup::all_groups() {
//         if fees.base_fee_bps(group)? == 0 {
//             continue;
//         }
//
//         if base_reward_router.base_fee_group_reward(group)? != 0 {
//             let result = distribute_base_rewards(handler, group, epoch).await;
//
//             if let Err(err) = result {
//                 log::error!(
//                 "Failed to distribute base rewards for group: {:?} in epoch: {:?} with error: {:?}",
//                 group,
//                 epoch,
//                 err
//             );
//             }
//         }
//     }
//
//     for operator in operators.iter() {
//         for group in NcnFeeGroup::all_groups() {
//             if fees.ncn_fee_bps(group)? == 0 {
//                 continue;
//             }
//
//             let result = get_or_create_ncn_reward_router(handler, group, operator, epoch).await;
//
//             if let Err(err) = result {
//                 log::error!(
//                     "Failed to get or create ncn reward router: {:?} in epoch: {:?} with error: {:?}",
//                     operator,
//                     epoch,
//                     err
//                 );
//                 continue;
//             }
//
//             if base_reward_router
//                 .ncn_fee_group_reward_route(operator)?
//                 .rewards(group)?
//                 != 0
//             {
//                 let result = distribute_base_ncn_rewards(handler, operator, group, epoch).await;
//
//                 if let Err(err) = result {
//                     log::error!(
//                     "Failed to distribute base ncn rewards for operator: {:?} in epoch: {:?} with error: {:?}",
//                     operator,
//                     epoch,
//                     err
//                 );
//                     continue;
//                 }
//             }
//
//             let ncn_reward_receiver_rewards =
//                 get_ncn_reward_receiver_rewards(handler, group, operator, epoch).await?;
//
//             if ncn_reward_receiver_rewards > 0 {
//                 let result = route_ncn_rewards(handler, operator, group, epoch).await;
//
//                 if let Err(err) = result {
//                     log::error!(
//                     "Failed to route ncn rewards for operator: {:?} in epoch: {:?} with error: {:?}",
//                     operator,
//                     epoch,
//                     err
//                 );
//                     continue;
//                 }
//             }
//
//             let result = get_or_create_ncn_reward_router(handler, group, operator, epoch).await;
//
//             if let Err(err) = result {
//                 log::error!(
//                     "Failed to get or create ncn reward router: {:?} in epoch: {:?} with error: {:?}",
//                     operator,
//                     epoch,
//                     err
//                 );
//                 continue;
//             }
//             let ncn_reward_router = result?;
//
//             if ncn_reward_router.operator_rewards() != 0 {
//                 let result = distribute_ncn_operator_rewards(handler, operator, group, epoch).await;
//
//                 if let Err(err) = result {
//                     log::error!(
//                     "Failed to distribute ncn operator rewards for operator: {:?} in epoch: {:?} with error: {:?}",
//                     operator,
//                     epoch,
//                     err
//                 );
//                     continue;
//                 }
//             }
//
//             let vaults_to_route = ncn_reward_router
//                 .vault_reward_routes()
//                 .iter()
//                 .filter(|route| !route.is_empty() && route.has_rewards())
//                 .map(|route| route.vault())
//                 .collect::<Vec<Pubkey>>();
//
//             for vault in vaults_to_route {
//                 let result: std::result::Result<(), anyhow::Error> =
//                     distribute_ncn_vault_rewards(handler, &vault, operator, group, epoch).await;
//
//                 if let Err(err) = result {
//                     log::error!(
//                         "Failed to distribute ncn vault rewards for vault: {:?} and operator: {:?} in epoch: {:?} with error: {:?}",
//                         vault,
//                         operator,
//                         epoch,
//                         err
//                     );
//                 }
//             }
//         }
//     }
//
//     Ok(())
// }
//
// pub async fn crank_close_epoch_accounts(handler: &CliHandler, epoch: u64) -> Result<()> {
//     let ncn = *handler.ncn()?;
//
//     // One last distribution crank
//     let result = crank_distribute(handler, epoch).await;
//     if result.is_err() {
//         log::error!(
//             "Failed to distribute rewards before closing for epoch: {:?} with error: {:?}",
//             epoch,
//             result.err().unwrap()
//         );
//     }
//
//     // Close NCN Reward Routers
//     let all_operators = get_all_operators_in_ncn(handler).await?;
//     for operator in all_operators.iter() {
//         for group in NcnFeeGroup::all_groups() {
//             let (ncn_reward_router, _, _) = NcnRewardRouter::find_program_address(
//                 &handler.tip_router_program_id,
//                 group,
//                 operator,
//                 &ncn,
//                 epoch,
//             );
//
//             let (ncn_reward_receiver, _, _) = NcnRewardReceiver::find_program_address(
//                 &handler.tip_router_program_id,
//                 group,
//                 operator,
//                 &ncn,
//                 epoch,
//             );
//
//             let result = close_epoch_account(
//                 handler,
//                 ncn,
//                 epoch,
//                 ncn_reward_router,
//                 Some(ncn_reward_receiver),
//             )
//                 .await;
//
//             if let Err(err) = result {
//                 log::error!(
//                     "Failed to close ncn reward router: {:?} in epoch: {:?} with error: {:?}",
//                     ncn_reward_router,
//                     epoch,
//                     err
//                 );
//             }
//         }
//     }
//
//     // Close Base Reward Router
//     let (base_reward_router, _, _) =
//         BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let (base_reward_receiver, _, _) =
//         BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let result = close_epoch_account(
//         handler,
//         ncn,
//         epoch,
//         base_reward_router,
//         Some(base_reward_receiver),
//     )
//         .await;
//
//     if let Err(err) = result {
//         log::error!(
//             "Failed to close base reward router: {:?} in epoch: {:?} with error: {:?}",
//             base_reward_router,
//             epoch,
//             err
//         );
//     }
//
//     // Close Ballot Box
//     let (ballot_box, _, _) =
//         BallotBox::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let result = close_epoch_account(handler, ncn, epoch, ballot_box, None).await;
//
//     if let Err(err) = result {
//         log::error!(
//             "Failed to close ballot box: {:?} in epoch: {:?} with error: {:?}",
//             ballot_box,
//             epoch,
//             err
//         );
//     }
//
//     // Close Operator Snapshots
//     for operator in all_operators.iter() {
//         let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
//             &handler.tip_router_program_id,
//             operator,
//             &ncn,
//             epoch,
//         );
//
//         let result = close_epoch_account(handler, ncn, epoch, operator_snapshot, None).await;
//
//         if let Err(err) = result {
//             log::error!(
//                 "Failed to close operator snapshot: {:?} in epoch: {:?} with error: {:?}",
//                 operator_snapshot,
//                 epoch,
//                 err
//             );
//         }
//     }
//
//     // Close Epoch Snapshot
//     let (epoch_snapshot, _, _) =
//         EpochSnapshot::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let result = close_epoch_account(handler, ncn, epoch, epoch_snapshot, None).await;
//
//     if let Err(err) = result {
//         log::error!(
//             "Failed to close epoch snapshot: {:?} in epoch: {:?} with error: {:?}",
//             epoch_snapshot,
//             epoch,
//             err
//         );
//     }
//
//     // Close Weight Table
//     let (weight_table, _, _) =
//         WeightTable::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let result = close_epoch_account(handler, ncn, epoch, weight_table, None).await;
//
//     if let Err(err) = result {
//         log::error!(
//             "Failed to close weight table: {:?} in epoch: {:?} with error: {:?}",
//             weight_table,
//             epoch,
//             err
//         );
//     }
//
//     // Close Epoch State
//     let (epoch_state, _, _) =
//         EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
//
//     let result = close_epoch_account(handler, ncn, epoch, epoch_state, None).await;
//
//     if let Err(err) = result {
//         log::error!(
//             "Failed to close epoch state: {:?} in epoch: {:?} with error: {:?}",
//             epoch_state,
//             epoch,
//             err
//         );
//     }
//
//     Ok(())
// }

// --------------------- NCN SETUP ------------------------------

//TODO create NCN
//TODO create Operator
//TODO add vault to NCN
//TODO add operator to NCN
//TODO remove vault from NCN
//TODO remove operator from NCN

// --------------------- TEST NCN --------------------------------

pub async fn create_test_ncn(handler: &CliHandler) -> Result<()> {
    let keypair = handler.keypair()?;
    let base = Keypair::new();
    let (ncn, _, _) = Ncn::find_program_address(&handler.restaking_program_id, &base.pubkey());
    let ncn_config_address =
        NcnConfig::find_program_address(&relayer_ncn_program::id(), &ncn).0;
    let (config, _, _) = RestakingConfig::find_program_address(&handler.restaking_program_id);

    let mut ix_builder = InitializeNcnBuilder::new();
    ix_builder
        .config(config)
        .admin(keypair.pubkey())
        .base(base.pubkey())
        .ncn(ncn)
        .instruction();

    send_and_log_transaction(
        handler,
        &[ix_builder.instruction()],
        &[&base],
        "Created Test Ncn",
        &[format!("NCN: {:?},ncn-config:{:?}", ncn, ncn_config_address)],
    )
        .await?;

    Ok(())
}

pub async fn create_and_add_test_operator(
    handler: &CliHandler,
    operator_fee_bps: u16,
) -> Result<()> {
    let ncn_admin = handler.keypair()?;
    let keypair = Keypair::new();
    // let keypair = Keypair::from_base58_string("");

    info!("operator admin keypair:{},{}", keypair.to_base58_string(),&keypair.pubkey());

    let ncn = *handler.ncn()?;

    let base = Keypair::new();
    info!("operator base keypair:{},{}", base.to_base58_string(),&base.pubkey());

    let (operator, _, _) =
        Operator::find_program_address(&handler.restaking_program_id, &base.pubkey());

    let (ncn_operator_state, _, _) =
        NcnOperatorState::find_program_address(&handler.restaking_program_id, &ncn, &operator);

    let (config, _, _) = RestakingConfig::find_program_address(&handler.restaking_program_id);

    // -------------- Initialize Operator --------------
    let initalize_operator_ix = InitializeOperatorBuilder::new()
        .config(config)
        .admin(keypair.pubkey())
        .base(base.pubkey())
        .operator(operator)
        .operator_fee_bps(operator_fee_bps)
        .instruction();

    let initialize_ncn_operator_state_ix = InitializeNcnOperatorStateBuilder::new()
        .config(config)
        .payer(ncn_admin.pubkey())
        .admin(ncn_admin.pubkey())
        .operator(operator)
        .ncn(ncn)
        .ncn_operator_state(ncn_operator_state)
        .instruction();

    let ncn_warmup_operator_ix = NcnWarmupOperatorBuilder::new()
        .config(config)
        .admin(ncn_admin.pubkey())
        .ncn(ncn)
        .operator(operator)
        .ncn_operator_state(ncn_operator_state)
        .instruction();

    let operator_warmup_ncn_ix = OperatorWarmupNcnBuilder::new()
        .config(config)
        .admin(keypair.pubkey())
        .ncn(ncn)
        .operator(operator)
        .ncn_operator_state(ncn_operator_state)
        .instruction();

    send_and_log_transaction(
        handler,
        &[initalize_operator_ix, initialize_ncn_operator_state_ix],
        &[&keypair, &base],
        "Created Test Operator",
        &[
            format!("NCN: {:?}", ncn),
            format!("Operator: {:?}", operator),
        ],
    )
        .await?;

    sleep(Duration::from_millis(1000)).await;

    send_and_log_transaction(
        handler,
        &[ncn_warmup_operator_ix, operator_warmup_ncn_ix],
        &[&keypair],
        "Warmed up Operator",
        &[
            format!("NCN: {:?}", ncn),
            format!("Operator: {:?}", operator),
        ],
    )
        .await?;

    Ok(())
}

pub async fn create_and_add_test_vault(
    handler: &CliHandler,
    deposit_fee_bps: u16,
    withdrawal_fee_bps: u16,
    reward_fee_bps: u16,
) -> Result<()> {
    let keypair = handler.keypair()?;

    let ncn = *handler.ncn()?;

    let vrt_mint = Keypair::new();
    let token_mint = Keypair::new();
    let base = Keypair::new();
    let (vault, _, _) = Vault::find_program_address(&handler.vault_program_id, &base.pubkey());

    let (vault_config, _, _) = VaultConfig::find_program_address(&handler.vault_program_id);
    let (restaking_config, _, _) =
        RestakingConfig::find_program_address(&handler.restaking_program_id);

    let all_operators = get_all_operators_in_ncn(handler).await?;

    // -------------- Create Mint -----------------
    let admin_ata = spl_associated_token_account::get_associated_token_address(
        &keypair.pubkey(),
        &token_mint.pubkey(),
    );

    let create_mint_account_ix = create_account(
        &keypair.pubkey(),
        &token_mint.pubkey(),
        Rent::default().minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN as u64,
        &handler.token_program_id,
    );
    let initialize_mint_ix = spl_token::instruction::initialize_mint2(
        &handler.token_program_id,
        &token_mint.pubkey(),
        &keypair.pubkey(),
        None,
        9,
    )
        .unwrap();
    let create_admin_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &keypair.pubkey(),
            &keypair.pubkey(),
            &token_mint.pubkey(),
            &handler.token_program_id,
        );
    let mint_to_ix = spl_token::instruction::mint_to(
        &handler.token_program_id,
        &token_mint.pubkey(),
        &admin_ata,
        &keypair.pubkey(),
        &[],
        1_000_000,
    )
        .unwrap();

    send_and_log_transaction(
        handler,
        &[
            create_mint_account_ix,
            initialize_mint_ix,
            create_admin_ata_ix,
            mint_to_ix,
        ],
        &[&token_mint],
        "Created Test Mint",
        &[format!("Token Mint: {:?}", token_mint.pubkey())],
    )
        .await?;

    // -------------- Initialize Vault --------------
    let initialize_vault_ix = InitializeVaultBuilder::new()
        .config(vault_config)
        .admin(keypair.pubkey())
        .base(base.pubkey())
        .vault(vault)
        .vrt_mint(vrt_mint.pubkey())
        .token_mint(token_mint.pubkey())
        .reward_fee_bps(reward_fee_bps)
        .withdrawal_fee_bps(withdrawal_fee_bps)
        .decimals(9)
        .deposit_fee_bps(deposit_fee_bps)
        .system_program(system_program::id())
        .instruction();

    let create_vault_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &keypair.pubkey(),
            &vault,
            &token_mint.pubkey(),
            &handler.token_program_id,
        );
    let create_admin_vrt_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &keypair.pubkey(),
            &keypair.pubkey(),
            &vrt_mint.pubkey(),
            &handler.token_program_id,
        );
    let create_vault_vrt_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &keypair.pubkey(),
            &vault,
            &vrt_mint.pubkey(),
            &handler.token_program_id,
        );

    let vault_token_ata = get_associated_token_address(&vault, &token_mint.pubkey());
    let admin_token_ata = get_associated_token_address(&keypair.pubkey(), &token_mint.pubkey());
    let admin_vrt_ata = get_associated_token_address(&keypair.pubkey(), &vrt_mint.pubkey());

    let mint_to_ix = MintToBuilder::new()
        .config(vault_config)
        .vault(vault)
        .vrt_mint(vrt_mint.pubkey())
        .depositor(keypair.pubkey())
        .depositor_token_account(admin_token_ata)
        .depositor_vrt_token_account(admin_vrt_ata)
        .vault_fee_token_account(admin_vrt_ata)
        .vault_token_account(vault_token_ata)
        .amount_in(10_000)
        .min_amount_out(0)
        .instruction();

    send_and_log_transaction(
        handler,
        &[
            initialize_vault_ix,
            create_vault_ata_ix,
            create_admin_vrt_ata_ix,
            create_vault_vrt_ata_ix,
            mint_to_ix,
        ],
        &[&base, &vrt_mint],
        "Created Test Vault",
        &[
            format!("NCN: {:?}", ncn),
            format!("Vault: {:?}", vault),
            format!("Token Mint: {:?}", token_mint.pubkey()),
            format!("VRT Mint: {:?}", vrt_mint.pubkey()),
        ],
    )
        .await?;

    // -------------- Initialize Vault <> NCN Ticket --------------

    let (ncn_vault_ticket, _, _) =
        NcnVaultTicket::find_program_address(&handler.restaking_program_id, &ncn, &vault);

    let (vault_ncn_ticket, _, _) =
        VaultNcnTicket::find_program_address(&handler.vault_program_id, &vault, &ncn);

    let initialize_ncn_vault_ticket_ix = InitializeNcnVaultTicketBuilder::new()
        .config(restaking_config)
        .admin(keypair.pubkey())
        .ncn(ncn)
        .vault(vault)
        .payer(keypair.pubkey())
        .ncn_vault_ticket(ncn_vault_ticket)
        .instruction();

    let initialize_vault_ncn_ticket_ix = InitializeVaultNcnTicketBuilder::new()
        .config(vault_config)
        .admin(keypair.pubkey())
        .vault(vault)
        .ncn(ncn)
        .payer(keypair.pubkey())
        .vault_ncn_ticket(vault_ncn_ticket)
        .ncn_vault_ticket(ncn_vault_ticket)
        .instruction();

    send_and_log_transaction(
        handler,
        &[
            initialize_ncn_vault_ticket_ix,
            initialize_vault_ncn_ticket_ix,
        ],
        &[],
        "Initialized Vault and NCN Tickets",
        &[format!("NCN: {:?}", ncn), format!("Vault: {:?}", vault)],
    )
        .await?;

    sleep(Duration::from_millis(1000)).await;

    let warmup_ncn_vault_ticket_ix = WarmupNcnVaultTicketBuilder::new()
        .config(restaking_config)
        .admin(keypair.pubkey())
        .ncn(ncn)
        .vault(vault)
        .ncn_vault_ticket(ncn_vault_ticket)
        .instruction();

    let warmup_vault_ncn_ticket_ix = WarmupVaultNcnTicketBuilder::new()
        .config(vault_config)
        .admin(keypair.pubkey())
        .vault(vault)
        .ncn(ncn)
        .vault_ncn_ticket(vault_ncn_ticket)
        .instruction();

    send_and_log_transaction(
        handler,
        &[warmup_ncn_vault_ticket_ix, warmup_vault_ncn_ticket_ix],
        &[],
        "Warmed up NCN Vault Tickets",
        &[format!("NCN: {:?}", ncn), format!("Vault: {:?}", vault)],
    )
        .await?;

    for operator in all_operators {
        let op = get_operator(&handler, &operator).await?;
        let op_keypair =Keypair::from_base58_string("");
        let (operator_vault_ticket, _, _) = OperatorVaultTicket::find_program_address(
            &handler.restaking_program_id,
            &operator,
            &vault,
        );

        let (vault_operator_delegation, _, _) = VaultOperatorDelegation::find_program_address(
            &handler.vault_program_id,
            &vault,
            &operator,
        );

        let initialize_operator_vault_ticket_ix = InitializeOperatorVaultTicketBuilder::new()
            .config(restaking_config)
            .admin(op.admin)
            .operator(operator)
            .vault(vault)
            .operator_vault_ticket(operator_vault_ticket)
            .payer(keypair.pubkey())
            .instruction();
        // do_initialize_operator_vault_ticket

        send_and_log_transaction(
            handler,
            &[initialize_operator_vault_ticket_ix],
            &[&op_keypair],
            "Connected Vault and Operator",
            &[
                format!("NCN: {:?}", ncn),
                format!("Operator: {:?}", operator),
                format!("Vault: {:?}", vault),
            ],
        )
            .await?;

        sleep(Duration::from_millis(1000)).await;

        // do_initialize_vault_operator_delegation
        let warmup_operator_vault_ticket_ix = WarmupOperatorVaultTicketBuilder::new()
            .config(restaking_config)
            .admin(op.admin)
            .operator(operator)
            .vault(vault)
            .operator_vault_ticket(operator_vault_ticket)
            .instruction();

        let initialize_vault_operator_delegation_ix =
            InitializeVaultOperatorDelegationBuilder::new()
                .config(vault_config)
                .admin(keypair.pubkey())
                .vault(vault)
                .payer(keypair.pubkey())
                .operator(operator)
                .operator_vault_ticket(operator_vault_ticket)
                .vault_operator_delegation(vault_operator_delegation)
                .instruction();

        let delegate_to_operator_ix = AddDelegationBuilder::new()
            .config(vault_config)
            .vault(vault)
            .operator(operator)
            .vault_operator_delegation(vault_operator_delegation)
            .admin(keypair.pubkey())
            .amount(1000)
            .instruction();

        send_and_log_transaction(
            handler,
            &[
                warmup_operator_vault_ticket_ix,
                initialize_vault_operator_delegation_ix,
                delegate_to_operator_ix,
            ],
            &[&op_keypair],
            "Delegated to Operator",
            &[
                format!("NCN: {:?}", ncn),
                format!("Operator: {:?}", operator),
                format!("Vault: {:?}", vault),
                format!("Amount: {:?}", 1000),
            ],
        )
            .await?;
    }

    Ok(())
}

// --------------------- HELPERS -------------------------

pub async fn send_and_log_transaction(
    handler: &CliHandler,
    instructions: &[Instruction],
    signing_keypairs: &[&Keypair],
    title: &str,
    log_items: &[String],
) -> Result<()> {
    sleep(Duration::from_secs(1)).await;

    let signature = send_transactions(handler, instructions, signing_keypairs).await?;

    log_transaction(title, signature, log_items);

    Ok(())
}

pub async fn send_transactions(
    handler: &CliHandler,
    instructions: &[Instruction],
    signing_keypairs: &[&Keypair],
) -> Result<Signature> {
    let client = handler.rpc_client();
    let keypair = handler.keypair()?;
    let retries = handler.retries;
    let priority_fee_micro_lamports = handler.priority_fee_micro_lamports;

    let mut all_instructions = vec![];

    all_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(
        priority_fee_micro_lamports,
    ));

    all_instructions.extend_from_slice(instructions);

    for iteration in 0..retries {
        let blockhash = client.get_latest_blockhash().await?;

        // Create a vector that combines all signing keypairs
        let mut all_signers = vec![keypair];
        all_signers.extend(signing_keypairs.iter());

        let tx = Transaction::new_signed_with_payer(
            &all_instructions,
            Some(&keypair.pubkey()),
            &all_signers, // Pass the reference to the vector of keypair references
            blockhash,
        );

        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        };
        let result = client
            .send_and_confirm_transaction_with_spinner_and_config(&tx, client.commitment(), config)
            .await;

        if result.is_err() {
            info!(
                "Retrying transaction after {}s {}/{}",
                (1 + iteration),
                iteration,
                retries
            );

            boring_progress_bar((1 + iteration) * 1000).await;
            continue;
        }

        return Ok(result.unwrap());
    }

    // last retry
    let blockhash = client.get_latest_blockhash().await?;

    // Create a vector that combines all signing keypairs
    let mut all_signers = vec![keypair];
    all_signers.extend(signing_keypairs.iter());

    let tx = Transaction::new_signed_with_payer(
        instructions,
        Some(&keypair.pubkey()),
        &all_signers, // Pass the reference to the vector of keypair references
        blockhash,
    );

    let result = client.send_and_confirm_transaction(&tx).await;

    if let Err(e) = result {
        return Err(anyhow!("\nError: \n\n{:?}\n\n", e));
    }

    Ok(result.unwrap())
}

pub fn log_transaction(title: &str, signature: Signature, log_items: &[String]) {
    let mut log_message = format!(
        "\n\n---------- {} ----------\nSignature: {:?}",
        title, signature
    );

    for item in log_items {
        log_message.push_str(&format!("\n{}", item));
    }

    log_message.push('\n');
    info!("{}", log_message);
}
