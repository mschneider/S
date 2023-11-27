use generic_pool_calculator_interface::{lst_to_sol_verify_account_keys, LstToSolAccounts};
use generic_pool_calculator_lib::utils::{
    verify_no_stake_pool_prog_upgrade, VerifyNoStakePoolProgUpgradeArgs,
};
use sanctum_onchain_utils::utils::{load_accounts, log_and_return_wrong_acc_err};
use solana_program::{
    account_info::AccountInfo, clock::Clock, program_error::ProgramError, sysvar::Sysvar,
};
use spl_calculator_interface::SplStakePool;
use spl_calculator_lib::{
    account_resolvers::SplLstSolCommonRootAccounts, verify_pool_updated_for_this_epoch,
    SplSolValCalc,
};

/// Assumes:
/// - LstToSolAccounts/Keys and SolToLstAccounts/Keys are identical
pub fn verify_lst_sol_common(accounts: &[AccountInfo<'_>]) -> Result<SplStakePool, ProgramError> {
    let actual: LstToSolAccounts = load_accounts(accounts)?;

    let root_keys = SplLstSolCommonRootAccounts {
        spl_stake_pool: actual.pool_state,
        spl_stake_pool_prog: actual.pool_program,
    };
    let (intermediate, stake_pool) = root_keys.resolve()?;
    let expected = intermediate.resolve::<SplSolValCalc>()?.into();

    lst_to_sol_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    // accounts should all be read-only, no need to verify_account_privileges

    verify_no_stake_pool_prog_upgrade(VerifyNoStakePoolProgUpgradeArgs {
        stake_pool_prog_data: actual.pool_program_data,
        calculator_state: actual.state,
    })?;

    verify_pool_updated_for_this_epoch(&stake_pool, &Clock::get()?)?;

    Ok(stake_pool)
}