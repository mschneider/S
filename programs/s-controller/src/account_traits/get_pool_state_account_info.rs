use s_controller_interface::{EndRebalanceAccounts, StartRebalanceAccounts, SyncSolValueAccounts};
use solana_program::account_info::AccountInfo;

use super::{DstLstMintOf, DstLstPoolReservesOf, SrcLstMintOf, SrcLstPoolReservesOf};

pub trait GetPoolStateAccountInfo<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for EndRebalanceAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

// impls for src_dst wrapper newtypes

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for SrcLstMintOf<'me, A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for DstLstMintOf<'me, A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for SrcLstPoolReservesOf<'me, A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for DstLstPoolReservesOf<'me, A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}