use std::str::FromStr;

use crate::{
    args::{Args, ProgramCommand},
    getters::{
    //     get_account_payer,
    get_all_operators_in_ncn,
    get_all_tickets,
    get_all_vaults_in_ncn,
        get_ballot_box,
    // get_base_reward_receiver, get_base_reward_router, get_epoch_snapshot,
    //     get_epoch_state,
    get_ncn,
    get_ncn_operator_state,
    // get_ncn_reward_receiver,
    //     get_ncn_reward_router,
    get_ncn_vault_ticket,
    // get_stake_pool, get_tip_router_config,
    //     get_total_epoch_rent_cost, get_total_rewards_to_be_distributed,
    get_vault_ncn_ticket,
        get_vault_operator_delegation,
    get_vault_registry,
    },
    keeper::keeper_loop::startup_keeper,
};
use anyhow::{anyhow, Result};
// use jito_tip_router_core::{base_fee_group::BaseFeeGroup, ncn_fee_group::NcnFeeGroup};
use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::lamports_to_sol,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};
use crate::getters::get_restaking_config;
use crate::instructions::{admin_create_config, admin_register_st_mint, admin_set_weight, create_and_add_test_operator, create_and_add_test_vault, create_ballot_box, create_epoch_snapshot, create_operator_snapshot, create_test_ncn, create_vault_registry, create_weight_table, register_vault, snapshot_vault_operator_delegation};

pub struct CliHandler {
    pub rpc_url: String,
    pub commitment: CommitmentConfig,
    keypair: Option<Keypair>,
    pub restaking_program_id: Pubkey,
    pub vault_program_id: Pubkey,
    pub relayer_ncn_program_id: Pubkey,
    pub relayer_hub_program_id: Pubkey,
    pub token_program_id: Pubkey,
    ncn: Option<Pubkey>,
    pub epoch: u64,
    rpc_client: RpcClient,
    pub retries: u64,
    pub priority_fee_micro_lamports: u64,
}

impl CliHandler {
    pub async fn from_args(args: &Args) -> Result<Self> {
        let rpc_url = args.rpc_url.clone();
        CommitmentConfig::confirmed();
        let commitment = CommitmentConfig::from_str(&args.commitment)?;

        let keypair = args
            .keypair_path
            .as_ref()
            .map(|k| read_keypair_file(k).unwrap());

        let restaking_program_id = Pubkey::from_str(&args.restaking_program_id)?;

        let vault_program_id = Pubkey::from_str(&args.vault_program_id)?;

        let relayer_ncn_program_id = Pubkey::from_str(&args.relayer_ncn_program_id)?;

        let relayer_hub_program_id = Pubkey::from_str(&args.relayer_hub_program_id)?;

        let token_program_id = Pubkey::from_str(&args.token_program_id)?;

        let ncn = args
            .ncn
            .clone()
            .map(|id| Pubkey::from_str(&id))
            .transpose()?;

        let rpc_client = RpcClient::new_with_commitment(rpc_url.clone(), commitment);

        let mut handler = Self {
            rpc_url,
            commitment,
            keypair,
            restaking_program_id,
            vault_program_id,
            relayer_ncn_program_id,
            relayer_hub_program_id,
            token_program_id,
            ncn,
            epoch: u64::MAX,
            rpc_client,
            retries: args.transaction_retries,
            priority_fee_micro_lamports: args.priority_fee_micro_lamports,
        };

        handler.epoch = {
            if args.epoch.is_some() {
                args.epoch.unwrap()
            } else {
                let client = handler.rpc_client();
                let epoch_info = client.get_epoch_info().await?;
                epoch_info.epoch
            }
        };

        Ok(handler)
    }

    pub const fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    pub fn keypair(&self) -> Result<&Keypair> {
        self.keypair.as_ref().ok_or_else(|| anyhow!("No keypair"))
    }

    pub fn ncn(&self) -> Result<&Pubkey> {
        self.ncn.as_ref().ok_or_else(|| anyhow!("No NCN address"))
    }

    #[allow(clippy::large_stack_frames)]
    pub async fn handle(&self, action: ProgramCommand) -> Result<()> {
        match action {
            // Keeper
            ProgramCommand::Keeper {
                loop_timeout_ms,
                error_timeout_ms,
            } => startup_keeper(self, loop_timeout_ms, error_timeout_ms).await,

            // Admin
            ProgramCommand::AdminCreateConfig {
                epochs_before_stall,
                valid_slots_after_consensus,
                // epochs_after_consensus_before_close,
                // dao_fee_bps,
                // block_engine_fee_bps,
                // default_ncn_fee_bps,
                // fee_wallet,
                // tie_breaker_admin,
            } => {
                // let fee_wallet =
                //     fee_wallet.map(|s| Pubkey::from_str(&s).expect("error parsing fee wallet"));
                // let tie_breaker = tie_breaker_admin
                //     .map(|s| Pubkey::from_str(&s).expect("error parsing tie breaker admin"));
                admin_create_config(
                    self,
                    epochs_before_stall,
                    valid_slots_after_consensus,
                    // epochs_after_consensus_before_close,
                    // dao_fee_bps,
                    // block_engine_fee_bps,
                    // default_ncn_fee_bps,
                    // fee_wallet,
                    // tie_breaker,
                )
                    .await
            }
            ProgramCommand::AdminRegisterStMint {
                vault,
                // ncn_fee_group,
                reward_multiplier_bps,
                // switchboard_feed,
                no_feed_weight,
            } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                // let switchboard = switchboard_feed
                //     .map(|s| Pubkey::from_str(&s).expect("error parsing switchboard feed"));
                // let ncn_fee_group =
                //     NcnFeeGroup::try_from(ncn_fee_group).expect("error parsing fee group");
                admin_register_st_mint(
                    self,
                    &vault,
                    // ncn_fee_group,
                    reward_multiplier_bps,
                    // switchboard,
                    no_feed_weight,
                )
                    .await
            }
            ProgramCommand::AdminSetWeight { vault, weight } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                admin_set_weight(self, &vault, self.epoch, weight).await
            }
            // ProgramCommand::AdminSetTieBreaker { meta_merkle_root } => {
            //     todo!(
            //         "Create and implement admin set tie breaker: {}",
            //         meta_merkle_root
            //     );
            //     // let merkle_root = hex::decode(meta_merkle_root).expect("error parsing merkle root");
            //     // let mut root = [0u8; 32];
            //     // root.copy_from_slice(&merkle_root);
            //     // admin_set_tie_breaker(self, root).await
            // }
            // ProgramCommand::AdminSetParameters {
            //     epochs_before_stall,
            //     epochs_after_consensus_before_close,
            //     valid_slots_after_consensus,
            //     starting_valid_epoch,
            // } => {
            //     admin_set_parameters(
            //         self,
            //         epochs_before_stall,
            //         epochs_after_consensus_before_close,
            //         valid_slots_after_consensus,
            //         starting_valid_epoch,
            //     )
            //         .await?;
            //     let config = get_tip_router_config(self).await?;
            //     info!("\n\n--- Parameters Set ---\nepochs_before_stall: {}\nepochs_after_consensus_before_close: {}\nvalid_slots_after_consensus: {}\nstarting_valid_epoch: {}\n",
            //         config.epochs_before_stall(),
            //         config.epochs_after_consensus_before_close(),
            //         config.valid_slots_after_consensus(),
            //         config.starting_valid_epoch()
            //     );
            //
            //     Ok(())
            // }
            // ProgramCommand::AdminFundAccountPayer { amount_in_sol } => {
            //     admin_fund_account_payer(self, amount_in_sol).await
            // }

            // Instructions
            ProgramCommand::CreateVaultRegistry {} => create_vault_registry(self).await,

            ProgramCommand::RegisterVault { vault } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                register_vault(self, &vault).await
            }

            // ProgramCommand::CreateEpochState {} => create_epoch_state(self, self.epoch).await,

            ProgramCommand::CreateWeightTable {} => create_weight_table(self, self.epoch).await,

            // ProgramCommand::SetWeight { vault } => {
            //     let vault = Pubkey::from_str(&vault).expect("error parsing vault");
            //     set_weight(self, &vault, self.epoch).await
            // }

            ProgramCommand::CreateEpochSnapshot {} => create_epoch_snapshot(self, self.epoch).await,

            ProgramCommand::CreateOperatorSnapshot { operator } => {
                let operator = Pubkey::from_str(&operator).expect("error parsing operator");
                create_operator_snapshot(self, &operator, self.epoch).await
            }

            ProgramCommand::SnapshotVaultOperatorDelegation { vault, operator } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                let operator = Pubkey::from_str(&operator).expect("error parsing operator");
                snapshot_vault_operator_delegation(self, &vault, &operator, self.epoch).await
            }

            ProgramCommand::CreateBallotBox {} => create_ballot_box(self, self.epoch).await,

            // ProgramCommand::OperatorCastVote {
            //     operator,
            //     meta_merkle_root,
            // } => {
            //     todo!(
            //         "Create and implement admin cast vote: {} {}",
            //         operator,
            //         meta_merkle_root
            //     );
            //     // let operator = Pubkey::from_str(&operator).expect("error parsing operator");
            //     // let merkle_root = hex::decode(meta_merkle_root).expect("error parsing merkle root");
            //     // let mut root = [0u8; 32];
            //     // root.copy_from_slice(&merkle_root);
            //     // admin_cast_vote(self, &operator, root).await
            // }
            //
            // ProgramCommand::CreateBaseRewardRouter {} => {
            //     create_base_reward_router(self, self.epoch).await
            // }
            //
            // ProgramCommand::CreateNcnRewardRouter {
            //     operator,
            //     ncn_fee_group,
            // } => {
            //     let operator = Pubkey::from_str(&operator).expect("error parsing operator");
            //     let ncn_fee_group =
            //         NcnFeeGroup::try_from(ncn_fee_group).expect("error parsing fee group");
            //     create_ncn_reward_router(self, ncn_fee_group, &operator, self.epoch).await
            // }
            //
            // ProgramCommand::RouteBaseRewards {} => route_base_rewards(self, self.epoch).await,
            //
            // ProgramCommand::RouteNcnRewards {
            //     operator,
            //     ncn_fee_group,
            // } => {
            //     let operator = Pubkey::from_str(&operator).expect("error parsing operator");
            //     let ncn_fee_group =
            //         NcnFeeGroup::try_from(ncn_fee_group).expect("error parsing fee group");
            //     route_ncn_rewards(self, &operator, ncn_fee_group, self.epoch).await
            // }
            //
            // ProgramCommand::DistributeBaseNcnRewards {
            //     operator,
            //     ncn_fee_group,
            // } => {
            //     let operator = Pubkey::from_str(&operator).expect("error parsing operator");
            //     let ncn_fee_group =
            //         NcnFeeGroup::try_from(ncn_fee_group).expect("error parsing fee group");
            //     distribute_base_ncn_rewards(self, &operator, ncn_fee_group, self.epoch).await
            // }

            // Getters
            ProgramCommand::GetNcn {} => {
                let ncn = get_ncn(self).await?;
                info!("NCN: {:?}", ncn);
                Ok(())
            }
            ProgramCommand::GetNcnOperatorState { operator } => {
                let operator = Pubkey::from_str(&operator).expect("error parsing operator");
                let ncn_operator_state = get_ncn_operator_state(self, &operator).await?;
                info!("NCN Operator State: {:?}", ncn_operator_state);
                let restaking_config = get_restaking_config(self).await?;
                let client = self.rpc_client();
                let slot = client.get_epoch_info().await?.absolute_slot;
                let epoch_length = restaking_config.epoch_length();
                let xxx = ncn_operator_state
                    .ncn_opt_in_state
                    .is_active(slot, epoch_length);
                let yyy = ncn_operator_state.ncn_opt_in_state.state(slot, epoch_length);
                info!("xxx: {:?},{:?},{:?},{:?},{:?}", xxx,slot,epoch_length, slot as f64/epoch_length as f64, yyy);
                Ok(())
            }
            ProgramCommand::GetVaultNcnTicket { vault } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                let ncn_ticket = get_vault_ncn_ticket(self, &vault).await?;
                info!("Vault NCN Ticket: {:?}", ncn_ticket);
                Ok(())
            }
            ProgramCommand::GetNcnVaultTicket { vault } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                let ncn_ticket = get_ncn_vault_ticket(self, &vault).await?;
                info!("NCN Vault Ticket: {:?}", ncn_ticket);
                Ok(())
            }
            ProgramCommand::GetVaultOperatorDelegation { vault, operator } => {
                let vault = Pubkey::from_str(&vault).expect("error parsing vault");
                let operator = Pubkey::from_str(&operator).expect("error parsing operator");

                let vault_operator_delegation =
                    get_vault_operator_delegation(self, &vault, &operator).await?;

                info!("Vault Operator Delegation: {:?}", vault_operator_delegation);
                Ok(())
            }
            ProgramCommand::GetAllOperatorsInNcn {} => {
                let operators = get_all_operators_in_ncn(self).await?;

                info!("Operators: {:?}", operators);
                Ok(())
            }
            ProgramCommand::GetAllVaultsInNcn {} => {
                let vaults = get_all_vaults_in_ncn(self).await?;
                info!("Vaults: {:?}", vaults);
                Ok(())
            }
            ProgramCommand::GetAllTickets {} => {
                let all_tickets = get_all_tickets(self).await?;

                for tickets in all_tickets.iter() {
                    info!("Tickets: {}", tickets);
                }

                Ok(())
            }
            // ProgramCommand::GetTipRouterConfig {} => {
            //     let config = get_tip_router_config(self).await?;
            //     info!("Tip Router Config: {:?}", config);
            //     Ok(())
            // }
            ProgramCommand::GetVaultRegistry {} => {
                let vault_registry = get_vault_registry(self).await?;
                info!("Vault Registry: {:?}", vault_registry);
                Ok(())
            }
            // ProgramCommand::GetEpochState {} => {
            //     let epoch_state = get_epoch_state(self, self.epoch).await?;
            //     let current_slot = self.rpc_client.get_slot().await?;
            //     info!(
            //         "\n\n--- Epoch State ---\nEpoch: {}\nSlot consensus {} {}\nDistribute Progress: {:?}",
            //         epoch_state.epoch(),
            //         epoch_state.slot_consensus_reached(),
            //         current_slot,
            //         epoch_state.total_distribution_progress()
            //     );
            //     // info!("Epoch State: {:?}", epoch_state);
            //     Ok(())
            // }
            // ProgramCommand::GetStakePool {} => {
            //     let stake_pool = get_stake_pool(self).await?;
            //     info!("Stake Pool: {:?}", stake_pool);
            //     Ok(())
            // }
            ProgramCommand::GetBallotBox {} => {
                let ballot_box = get_ballot_box(self, self.epoch).await?;
                info!("Ballot Box: {:?}", ballot_box);
                Ok(())
            }
            // ProgramCommand::GetBaseRewardRouter {} => {
            //     let total_rewards_to_be_distributed =
            //         get_total_rewards_to_be_distributed(self, self.epoch).await?;
            //     let base_reward_router = get_base_reward_router(self, self.epoch).await?;
            //     let (base_reward_receiver_address, base_reward_receiver_account) =
            //         get_base_reward_receiver(self, self.epoch).await?;
            //     let rent = self
            //         .rpc_client
            //         .get_minimum_balance_for_rent_exemption(0)
            //         .await?;
            //     // info!("Base Reward Router: {:?}", base_reward_router);
            //     info!(
            //         "\n\n --- Base Reward Router ---\ntotal to distribute: {}\nreceiver: {}: {}\nrewards in receiver: {}\nrewards processed: {}\nbase fee group reward: {}\nncn fee group reward: {}\n",
            //         total_rewards_to_be_distributed,
            //         base_reward_receiver_address,
            //         base_reward_receiver_account.lamports,
            //         base_reward_receiver_account.lamports - rent,
            //         base_reward_router.rewards_processed(),
            //         base_reward_router.base_fee_group_reward(BaseFeeGroup::default()).unwrap(),
            //         base_reward_router.ncn_fee_group_rewards(NcnFeeGroup::default()).unwrap()
            //     );
            //     Ok(())
            // }
            // ProgramCommand::GetNcnRewardRouters {} => {
            //     let all_operators = get_all_operators_in_ncn(self).await?;
            //     let rent = self
            //         .rpc_client
            //         .get_minimum_balance_for_rent_exemption(0)
            //         .await?;
            //     let epoch_snapshot = get_epoch_snapshot(self, self.epoch).await?;
            //     let fees = epoch_snapshot.fees();
            //
            //     let mut valid_ncn_groups: Vec<NcnFeeGroup> = Vec::new();
            //     for group in NcnFeeGroup::all_groups() {
            //         if fees.ncn_fee_bps(group)? > 0 {
            //             valid_ncn_groups.push(group);
            //         }
            //     }
            //
            //     for operator in all_operators.iter() {
            //         for group in valid_ncn_groups.iter() {
            //             let ncn_reward_router =
            //                 get_ncn_reward_router(self, *group, operator, self.epoch).await?;
            //             let (ncn_reward_receiver_address, ncn_reward_receiver_account) =
            //                 get_ncn_reward_receiver(self, *group, operator, self.epoch).await?;
            //
            //             let mut routes = String::new();
            //             for vault_route in ncn_reward_router.vault_reward_routes() {
            //                 if !vault_route.is_empty() {
            //                     routes.push_str(&format!(
            //                         "{:?}: {}\n",
            //                         vault_route.vault(),
            //                         vault_route.rewards()
            //                     ));
            //                 }
            //             }
            //
            //             info!(
            //                 "\n\n --- NCN Reward Router ---\noperator: {}\ngroup: {:?}\nreceiver: {} {}\nrewards in receiver: {}\nrewards processed: {}\noperator rewards: {}\n{}\n",
            //                 operator,
            //                 group,
            //                 ncn_reward_receiver_address,
            //                 ncn_reward_receiver_account.lamports,
            //                 ncn_reward_receiver_account.lamports - rent,
            //                 ncn_reward_router.rewards_processed(),
            //                 ncn_reward_router.operator_rewards(),
            //                 routes
            //             );
            //         }
            //     }
            //
            //     Ok(())
            // }
            // ProgramCommand::GetAccountPayer {} => {
            //     let account_payer = get_account_payer(self).await?;
            //     info!(
            //         "\n\n--- Account Payer ---\nBalance: {}\n",
            //         lamports_to_sol(account_payer.lamports)
            //     );
            //     Ok(())
            // }
            // ProgramCommand::GetTotalEpochRentCost {} => {
            //     let total_epoch_rent_cost = get_total_epoch_rent_cost(self).await?;
            //     info!(
            //         "\n\n--- Total Epoch Rent Cost ---\nCost: {}\n",
            //         lamports_to_sol(total_epoch_rent_cost)
            //     );
            //     Ok(())
            // }

            // Testers
            ProgramCommand::Test {} => {
                info!("Test!");
                Ok(())
            }
            ProgramCommand::CreateTestNcn {} => create_test_ncn(self).await,
            ProgramCommand::CreateAndAddTestOperator { operator_fee_bps } => {
                create_and_add_test_operator(self, operator_fee_bps).await
            }
            ProgramCommand::CreateAndAddTestVault {
                deposit_fee_bps,
                withdrawal_fee_bps,
                reward_fee_bps,
            } => {
                create_and_add_test_vault(self, deposit_fee_bps, withdrawal_fee_bps, reward_fee_bps)
                    .await
            }
            _ => {Ok(())}
        }
    }
}
