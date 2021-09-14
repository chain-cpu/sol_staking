//! FIXME copied from the solana stake program

use serde_derive::{Deserialize, Serialize};
use solana_program::{
    clock::{Epoch, UnixTimestamp},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    stake_history::StakeHistory,
    system_instruction, sysvar,
};
use std::str::FromStr;

solana_program::declare_id!("Stake11111111111111111111111111111111111111");

const STAKE_CONFIG: &str = "StakeConfig11111111111111111111111111111111";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum StakeInstruction {
    Initialize(Authorized, Lockup),
    Authorize(Pubkey, StakeAuthorize),
    DelegateStake,
    Split(u64),
    Withdraw(u64),
    Deactivate,
    SetLockupNOTUSED,
    Merge,
    AuthorizeWithSeedNOTUSED,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub enum StakeState {
    Uninitialized,
    Initialized(Meta),
    Stake(Meta, Stake),
    RewardsPool,
}


#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Meta {
    pub rent_exempt_reserve: u64,
    pub authorized: Authorized,
    pub lockup: Lockup,
}


#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Stake {
    pub delegation: Delegation,
    /// credits observed is credits from vote account state when delegated or redeemed
    pub credits_observed: u64,
}


#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Delegation {
    /// to whom the stake is delegated
    pub voter_pubkey: Pubkey,
    /// activated stake amount, set at delegate() time
    pub stake: u64,
    /// epoch at which this stake was activated, std::Epoch::MAX if is a bootstrap stake
    pub activation_epoch: Epoch,
    /// epoch the stake was deactivated, std::Epoch::MAX if not deactivated
    pub deactivation_epoch: Epoch,
    /// how much stake we can activate per-epoch as a fraction of currently effective stake
    pub warmup_cooldown_rate: f64,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum StakeAuthorize {
    Staker,
    Withdrawer,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Authorized {
    pub staker: Pubkey,
    pub withdrawer: Pubkey,
}


#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Lockup {
    /// UnixTimestamp at which this stake will allow withdrawal, unless the
    ///   transaction is signed by the custodian
    pub unix_timestamp: UnixTimestamp,
    /// epoch height at which this stake will allow withdrawal, unless the
    ///   transaction is signed by the custodian
    pub epoch: Epoch,
    /// custodian signature on a transaction exempts the operation from
    ///  lockup constraints
    pub custodian: Pubkey,
}


impl StakeState {
    /// Get Delegation
    pub fn delegation(&self) -> Option<Delegation> {
        match self {
            StakeState::Stake(_meta, stake) => Some(stake.delegation),
            _ => None,
        }
    }
}


impl Delegation {
    /// Create new Delegation
    pub fn new(
        voter_pubkey: &Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        warmup_cooldown_rate: f64,
    ) -> Self {
        Self {
            voter_pubkey: *voter_pubkey,
            stake,
            activation_epoch,
            warmup_cooldown_rate,
            ..Delegation::default()
        }
    }
    /// Check if it bootstrap
    pub fn is_bootstrap(&self) -> bool {
        self.activation_epoch == std::u64::MAX
    }

    /// Return tuple (effective, activating, deactivating) stake
    #[allow(clippy::comparison_chain)]
    pub fn stake_activating_and_deactivating(
        &self,
        target_epoch: Epoch,
        history: Option<&StakeHistory>,
        fix_stake_deactivate: bool,
    ) -> (u64, u64, u64) {
        Ok(())
    }

    // returned tuple is (effective, activating) stake
    fn stake_and_activating(
        &self,
        target_epoch: Epoch,
        history: Option<&StakeHistory>,
        fix_stake_deactivate: bool,
    ) -> (u64, u64) {
        Ok(())
    }
}


pub fn split_only(
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    lamports: u64,
    split_stake_pubkey: &Pubkey,
) -> Instruction {
    Ok(())
}


pub fn authorize(
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    new_authorized_pubkey: &Pubkey,
    stake_authorize: StakeAuthorize,
) -> Instruction {
    Ok(())
}


pub fn merge(
    destination_stake_pubkey: &Pubkey,
    source_stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
) -> Instruction {
    Ok(())
}


pub fn create_account(
    from_pubkey: &Pubkey,
    stake_pubkey: &Pubkey,
    authorized: &Authorized,
    lockup: &Lockup,
    lamports: u64,
) -> Vec<Instruction> {
    Ok(())
}


pub fn initialize(stake_pubkey: &Pubkey, authorized: &Authorized, lockup: &Lockup) -> Instruction {
    Ok(())
}


pub fn delegate_stake(
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
) -> Instruction {
    Ok(())
}
