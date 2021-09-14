//! Instruction types

#![allow(clippy::too_many_arguments)]

use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar;
use std::mem::size_of;

/// Fee rate as a ratio
/// Fee is minted on deposit
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fee {
    /// denominator of the fee ratio
    pub denominator: u64,
    /// numerator of the fee ratio
    pub numerator: u64,
}

/// Inital values for the Stake Pool
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitArgs {
    /// Fee paid to the owner in pool tokens
    pub fee: Fee,
}

/// Instructions supported by the StakePool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum StakePoolInstruction {
    Initialize(InitArgs),
    CreateValidatorStakeAccount,
    AddValidatorStakeAccount,
    RemoveValidatorStakeAccount,
    UpdateListBalance,
    UpdatePoolBalance,
    Deposit,
    Withdraw(u64),
    SetStakingAuthority,
    SetOwner,
    AddLiquidity(u64),
    SellstSOL(u64),
}

impl StakePoolInstruction {
    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(false);
    }

    /// Serializes an [StakePoolInstruction](enum.StakePoolInstruction.html) into a byte buffer.
    /// TODO efficient packing here
    pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
        Ok(false)
    }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
    Ok(false)
}

/// Creates an 'initialize' instruction.
pub fn initialize(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    owner: &Pubkey,
    validator_stake_list: &Pubkey,
    pool_mint: &Pubkey,
    owner_pool_account: &Pubkey,
    token_program_id: &Pubkey,
    init_args: InitArgs,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates `CreateValidatorStakeAccount` instruction (create new stake account for the validator)
pub fn create_validator_stake_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    funder: &Pubkey,
    stake_account: &Pubkey,
    validator: &Pubkey,
    stake_authority: &Pubkey,
    withdraw_authority: &Pubkey,
    system_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates `AddValidatorStakeAccount` instruction (add new validator stake account to the pool)
pub fn add_validator_stake_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    owner: &Pubkey,
    stake_pool_deposit: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_account: &Pubkey,
    pool_tokens_to: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates `RemoveValidatorStakeAccount` instruction (remove validator stake account from the pool)
pub fn remove_validator_stake_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    owner: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    new_stake_authority: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_account: &Pubkey,
    burn_from: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates `UpdateListBalance` instruction (update validator stake account balances)

pub fn update_list_balance(

    program_id: &Pubkey,
    validator_stake_list_storage: &Pubkey,
    validator_stake_list: &[&Pubkey],

) -> Result<Instruction, ProgramError> {

    Ok(false)

}

/// Creates `UpdatePoolBalance` instruction (pool balance from the stake account list balances)
pub fn update_pool_balance(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list_storage: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates a 'Deposit' instruction.
pub fn deposit(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list_storage: &Pubkey,
    stake_pool_deposit: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_to_join: &Pubkey,
    validator_stake_accont: &Pubkey,
    pool_tokens_to: &Pubkey,
    pool_fee_to: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

///create instruction add_liquidity
pub fn instruction_add_liquidity(
    amount:u64,
    program_id: &Pubkey,
    liq_pool_state_account: &Pubkey,
    spl_token_program_id: &Pubkey,
    meta_lp_mint_account: &Pubkey,
    meta_lp_mint_authority: &Pubkey,
    user_wsol_source_account: &Pubkey,
    user_wsol_withdraw_auth: &Pubkey,
    liq_pool_wsol_dest_account: &Pubkey,
    user_dest_meta_lp_account: &Pubkey,

) -> Result<Instruction, ProgramError> {

    Ok(false)
}

///create instruction sell_st_sol
pub fn instruction_sell_stsol(
    amount:u64,
    program_id: &Pubkey,
    stake_pool_state_account: &Pubkey,
    liq_pool_state_account: &Pubkey,
    spl_token_program_id: &Pubkey,
    liq_pool_wsol_account: &Pubkey,
    liq_pool_st_sol_account: &Pubkey,
    liq_pool_authority: &Pubkey,
    user_wsol_account: &Pubkey,
    user_st_sol_account: &Pubkey,
    user_withdraw_auth: &Pubkey,

) -> Result<Instruction, ProgramError> {

    Ok(false)
}

/// Creates a 'withdraw' instruction.
pub fn withdraw(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list_storage: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_to_split: &Pubkey,
    stake_to_receive: &Pubkey,
    user_withdrawer: &Pubkey,
    burn_from: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates a 'set staking authority' instruction.
pub fn set_staking_authority(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    stake_pool_owner: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_account_to_update: &Pubkey,
    stake_account_new_authority: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}

/// Creates a 'set owner' instruction.
pub fn set_owner(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    stake_pool_owner: &Pubkey,
    stake_pool_new_owner: &Pubkey,
    stake_pool_new_fee_receiver: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(false)
}
