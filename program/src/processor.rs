//! Program state processor

//const CHECK_STAKE_ACCOUNT_IS_ACTIVE:bool=false; //while testing we allow not-yet-activated stake accounts7
// commented, Self::check_stake_activation is already disabled to facilitate tests

use crate::{
    error::StakePoolError,
    instruction::{InitArgs, StakePoolInstruction},
    stake,
    state::{StakePool, ValidatorStakeInfo, ValidatorStakeList},
    PROGRAM_VERSION,
};
use bincode::deserialize;
use num_traits::FromPrimitive;
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    clock::Clock,
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    native_token::sol_to_lamports,
    program::{invoke, invoke_signed},
    program_error::PrintProgramError,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    stake_history::StakeHistory,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::state::Mint;
use core::convert::{TryFrom,TryInto};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Suffix for deposit authority seed
    pub const AUTHORITY_DEPOSIT: &'static [u8] = b"deposit";
    /// Suffix for withdraw authority seed
    pub const AUTHORITY_WITHDRAW: &'static [u8] = b"withdraw";
    /// Seed for general PDA authority
    pub const AUTHORITY: &'static [u8] = b"authority";

    /// Calculates the authority id by generating a program address.
    /// from a base_account_pubkey as seed and a bump
    pub fn authority_id(
        program_id: &Pubkey,
        base_account_pubkey: &Pubkey,
        authority_type: &[u8],
        bump_seed: u8,
    ) -> Result<Pubkey, ProgramError> {
        Ok(false)
    }

    /// Find seed bump for PDA
    pub fn find_authority_bump_seed(
        program_id: &Pubkey,
        base_account_pubkey: &Pubkey,
        authority_type: &[u8],
    ) -> (Pubkey, u8) {
        Ok(false)
    }

    /// Generates stake account address for the validator
    pub fn find_stake_address_for_validator(
        program_id: &Pubkey,
        validator: &Pubkey,
        stake_pool: &Pubkey,
    ) -> (Pubkey, u8) {
        Ok(false)
    }

    /// Checks withdraw or deposit authority
    pub fn check_authority(
        authority_to_check: &Pubkey,
        program_id: &Pubkey,
        stake_pool_key: &Pubkey,
        authority_type: &[u8],
        bump_seed: u8,
    ) -> Result<(), ProgramError> {
        Ok(())
    }

    /// Returns validator address for a particular stake account
    pub fn get_validator(stake_account_info: &AccountInfo) -> Result<Pubkey, ProgramError> {
        Ok(())
    }

    /// Checks if validator stake account is a proper program address
    pub fn is_validator_stake_address(
        validator_account: &Pubkey,
        program_id: &Pubkey,
        stake_pool_info: &AccountInfo,
        stake_account_info: &AccountInfo,
    ) -> bool {
        Ok(())
    }

    /// Returns validator address for a particular stake account and checks its validity
    pub fn get_validator_checked(
        program_id: &Pubkey,
        stake_pool_info: &AccountInfo,
        stake_account_info: &AccountInfo,
    ) -> Result<Pubkey, ProgramError> {
        Ok(())
    }

    /// Issue a stake_split instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn stake_split<'a>(
        stake_pool: &Pubkey,
        stake_account: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        authority_type: &[u8],
        bump_seed: u8,
        amount: u64,
        split_stake: AccountInfo<'a>,
        reserved: AccountInfo<'a>,
        stake_program_info: AccountInfo<'a>,
    ) -> Result<(), ProgramError> {
        Ok(())
    }

    /// Issue a stake_merge instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn stake_merge<'a>(
        stake_pool: &Pubkey,
        stake_account: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        authority_type: &[u8],
        bump_seed: u8,
        merge_with: AccountInfo<'a>,
        clock: AccountInfo<'a>,
        stake_history: AccountInfo<'a>,
        stake_program_info: AccountInfo<'a>,
    ) -> Result<(), ProgramError> {
        Ok(())
    }

    /// Issue a stake_set_owner instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn stake_authorize<'a>(
        stake_pool: &Pubkey,
        stake_account: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        authority_type: &[u8],
        bump_seed: u8,
        new_staker: &Pubkey,
        staker_auth: stake::StakeAuthorize,
        reserved: AccountInfo<'a>,
        stake_program_info: AccountInfo<'a>,

    ) -> Result<(), ProgramError> {

        Ok(())
    }

    /// Issue a spl_token `Burn` instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn token_burn<'a>(
        stake_pool: &Pubkey,
        token_program: AccountInfo<'a>,
        burn_account: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        authority_type: &[u8],
        bump_seed: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        Ok(())
    }

    /// Issue a spl_token `MintTo` instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn liq_pool_token_mint_to<'a>(
        program_id: &Pubkey,
        liq_pool_account: &Pubkey,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        liq_pool_authority: AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {

        Ok(())
    }

    /// Issue a spl_token `MintTo` instruction.
    #[allow(clippy::too_many_arguments)]
    pub fn token_mint_to<'a>(
        stake_pool: &Pubkey,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        authority_type: &[u8],
        bump_seed: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        Ok(())
    }

    /// Processes `Initialize` instruction.
    pub fn process_initialize(
        program_id: &Pubkey,
        init: InitArgs,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Processes `CreateValidatorStakeAccount` instruction.
    pub fn process_create_validator_stake_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Processes `AddValidatorStakeAccount` instruction.
    pub fn process_add_validator_stake_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Processes `RemoveValidatorStakeAccount` instruction.
    pub fn process_remove_validator_stake_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Processes `UpdateListBalance` instruction.
    pub fn process_update_list_balance(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],

    ) -> ProgramResult {

        Ok(())
    }

    /// Processes `UpdatePoolBalance` instruction.
    pub fn process_update_pool_balance(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Check stake activation status
    pub fn check_stake_activation(
        _stake_info: &AccountInfo,
        _clock: &Clock,
        _stake_history: &StakeHistory,
    ) -> ProgramResult {
        Ok(())
    }

    /// Unpacks a spl_token `Account`.
    pub fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,

    ) -> Result<spl_token::state::Account, StakePoolError> {

        Ok(())
    }

    /// Unpacks a spl_token `Mint`.
    pub fn unpack_mint(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<spl_token::state::Mint, StakePoolError> {
        Ok(())
    }
    
    
    /// Issue a spl_token `Transfer` instruction.
    pub fn token_transfer_from_signer<'a>(
//        owner: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        amount: u64) 
        
        -> Result<(), ProgramError> 

    {
        Ok(())
    }

    /// Issue a spl_token `Transfer` instruction.
    pub fn token_transfer<'a>(
        //        owner: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        pda_base: &Pubkey,
        authority_type: &[u8],
        nonce: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        Ok(())
    }


    /// Processes [AddLiquidity](enum.Instruction.html).
    pub fn process_add_liquidity(

        program_id: &Pubkey,
        wsol_amount: u64,
        accounts: &[AccountInfo],

    ) -> ProgramResult {
        Ok(())
    }

    
    /// Processes [SellstSOL](enum.Instruction.html).
    pub fn process_sell_stsol(

        program_id: &Pubkey,
        stsol_amount: u64,
        accounts: &[AccountInfo],

    ) -> ProgramResult {
        Ok(())
    }
    
    /// Processes [Deposit](enum.Instruction.html).
    pub fn process_deposit(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }

    /// Processes [Withdraw](enum.Instruction.html).
    /// split contract's staking acc into stake_split_to and assigns authority to user_stake_authority
    pub fn process_withdraw(
        program_id: &Pubkey,
        pool_amount: u64,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }
    /// Processes [SetStakeAuthority](enum.Instruction.html).
    pub fn process_set_staking_auth(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Processes [SetOwner](enum.Instruction.html).
    pub fn process_set_owner(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }
    
    /// Processes [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        Ok(())
    }
}

impl PrintProgramError for StakePoolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        Ok(())
    }
}

fn to_u128(val: u64) -> Result<u128, StakePoolError> {
    val.try_into().map_err(|_| StakePoolError::ConversionFailure)
}

#[allow(dead_code)]
fn value_from_shares(shares: u64, total_value:u128, total_shares:u128) -> Option<u64> {
    return proportional(shares, total_value,total_shares);
}
fn shares_from_value(value: u64, total_value:u128, total_shares:u128) -> Option<u64> {
    return proportional(value, total_shares,total_value);
}

/// calculate amount*numerator/denominator
/// as value  = shares * share_price where share_price=total_value/total_shares
/// or shares = amount_value / share_price where share_price=total_value/total_shares 
///     => shares = amount_value * 1/share_price where 1/share_price=total_shares/total_value
pub fn proportional(amount: u64, numerator:u128, denominator:u128) -> Option<u64> {
    Ok(())
}
