use borsh::BorshSerialize;
use pricing_programs_interface::{
    PriceLpTokensToMintIxArgs, PriceLpTokensToMintIxData, PriceLpTokensToRedeemIxArgs,
    PriceLpTokensToRedeemIxData,
};
use s_controller_interface::SControllerError;
use s_controller_lib::try_pool_state;
use sanctum_onchain_utils::utils::account_info_to_account_meta;
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use crate::account_traits::{GetLstMintAccountInfo, GetPoolStateAccountInfo};

use super::get_le_u64_return_data;

#[derive(Clone, Copy, Debug)]
pub struct PricingProgramIxArgs {
    /// Amount of LST
    pub amount: u64,
    /// SOL value of `amount` LST
    pub sol_value: u64,
}

/// CPI call to either `PriceLpTokensToRedeem` or `PriceLpTokensToMint`
#[derive(Clone, Copy, Debug)]
pub struct PricingProgramPriceLpCpi<'me, 'info> {
    /// The pricing program to invoke
    pub program: &'me AccountInfo<'info>,

    /// The mint of the LST that the pricing program is being called for
    pub lst_mint: &'me AccountInfo<'info>,

    /// Remaining accounts required by the pricing program
    pub remaining_accounts: &'me [AccountInfo<'info>],
}

impl<'me, 'info> PricingProgramPriceLpCpi<'me, 'info> {
    /// Args:
    /// - `ix_accounts`: the calling instruction's accounts, excluding accounts_suffix_slice.
    ///     Should be a `*Accounts` struct generated by solores
    /// - `accounts_suffix_slice`: subslice of instruction accounts where first account is the pricing program
    ///     and remaining slice is remaining_accounts (excludes `lst_mint`)
    pub fn from_ix_accounts<G: GetLstMintAccountInfo<'me, 'info>>(
        ix_accounts: &G,
        accounts_suffix_slice: &'me [AccountInfo<'info>],
    ) -> Result<Self, ProgramError> {
        let program = accounts_suffix_slice
            .get(0)
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
        Ok(Self {
            program,
            lst_mint: ix_accounts.get_lst_mint_account_info(),
            remaining_accounts: accounts_suffix_slice
                .get(1..)
                .ok_or(ProgramError::NotEnoughAccountKeys)?,
        })
    }

    pub fn verify_correct_pricing_program<G: GetPoolStateAccountInfo<'me, 'info>>(
        &self,
        ix_accounts: &G,
    ) -> Result<(), ProgramError> {
        let pool_state_bytes = ix_accounts
            .get_pool_state_account_info()
            .try_borrow_data()?;
        let pool_state = try_pool_state(&pool_state_bytes)?;
        if *self.program.key != pool_state.pricing_program {
            return Err(SControllerError::IncorrectPricingProgram.into());
        }
        Ok(())
    }

    pub fn invoke_price_lp_tokens_to_mint(
        self,
        args: PricingProgramIxArgs,
    ) -> Result<u64, ProgramError> {
        let ix = self.create_price_lp_tokens_to_mint_ix(args)?;
        self.invoke_interface_ix(ix)
    }

    pub fn invoke_price_lp_tokens_to_redeem(
        self,
        args: PricingProgramIxArgs,
    ) -> Result<u64, ProgramError> {
        let ix = self.create_price_lp_tokens_to_redeem_ix(args)?;
        self.invoke_interface_ix(ix)
    }

    fn create_price_lp_tokens_to_mint_ix(
        &self,
        PricingProgramIxArgs { amount, sol_value }: PricingProgramIxArgs,
    ) -> Result<Instruction, ProgramError> {
        Ok(Instruction {
            program_id: *self.program.key,
            accounts: self.create_account_metas(),
            data: PriceLpTokensToMintIxData(PriceLpTokensToMintIxArgs { amount, sol_value })
                .try_to_vec()?,
        })
    }

    fn create_price_lp_tokens_to_redeem_ix(
        &self,
        PricingProgramIxArgs { amount, sol_value }: PricingProgramIxArgs,
    ) -> Result<Instruction, ProgramError> {
        Ok(Instruction {
            program_id: *self.program.key,
            accounts: self.create_account_metas(),
            data: PriceLpTokensToRedeemIxData(PriceLpTokensToRedeemIxArgs { amount, sol_value })
                .try_to_vec()?,
        })
    }

    fn invoke_interface_ix(self, interface_ix: Instruction) -> Result<u64, ProgramError> {
        let accounts = self.create_account_info_slice();
        invoke(&interface_ix, &accounts)?;
        let res = get_le_u64_return_data().ok_or(SControllerError::FaultySolValueCalculator)?;
        Ok(res)
    }

    fn create_account_info_slice(self) -> Vec<AccountInfo<'info>> {
        let Self {
            lst_mint,
            remaining_accounts,
            ..
        } = self;
        [&[lst_mint.clone()], remaining_accounts].concat()
    }

    fn create_account_metas(&self) -> Vec<AccountMeta> {
        let mut res = vec![AccountMeta::new_readonly(*self.lst_mint.key, false)];
        for r in self.remaining_accounts.iter() {
            res.push(account_info_to_account_meta(r));
        }
        res
    }
}
