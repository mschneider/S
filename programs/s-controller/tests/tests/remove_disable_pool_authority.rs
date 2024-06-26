use s_controller_interface::{
    remove_disable_pool_authority_ix, RemoveDisablePoolAuthorityIxArgs, SControllerError,
};
use s_controller_lib::{
    index_to_u32,
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_disable_pool_authority_list, RemoveDisablePoolAuthorityByPubkeyFreeArgs,
    RemoveDisablePoolAuthorityFreeArgs,
};
use s_controller_test_utils::{
    assert_disable_authority_removed, DisablePoolAuthorityListBanksClient,
    DisablePoolAuthorityListProgramTest, MockPoolState, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::{assert_custom_err, test_fixtures_dir, IntoAccount};
use solana_program_test::ProgramTest;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use crate::common::SControllerProgramTest;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    // NOTE: assumes keypairs are unique
    let disable_pool_authority_kps = [Keypair::new(), Keypair::new(), Keypair::new()];
    let disable_pool_authority_pks: Vec<_> = disable_pool_authority_kps
        .iter()
        .map(|k| k.pubkey())
        .collect();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_disable_pool_authority_list(&disable_pool_authority_pks);
    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
    let disable_pool_authority_list =
        try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();

    // Remove an authority by the admin
    {
        let before_len = disable_pool_authority_list.len();
        let target_index = 0;
        let target_authority = disable_pool_authority_pks[target_index];
        let keys = RemoveDisablePoolAuthorityFreeArgs {
            index: target_index,
            refund_rent_to: payer.pubkey(),
            signer: mock_auth_kp.pubkey(),
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
            disable_pool_authority_list: KeyedAccount {
                pubkey: DISABLE_POOL_AUTHORITY_LIST_ID,
                account: disable_pool_authority_list_acc.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = remove_disable_pool_authority_ix(
            keys,
            RemoveDisablePoolAuthorityIxArgs {
                index: index_to_u32(target_index).unwrap(),
            },
        )
        .unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        assert_disable_authority_removed(&mut banks_client, target_authority, before_len).await;
    }

    let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
    let disable_pool_authority_list =
        try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
    let before_len = disable_pool_authority_list.len();

    // Try to remove another authority by authority, should fail
    {
        let target_authority_kp = &disable_pool_authority_kps[2];
        let (keys, args) = RemoveDisablePoolAuthorityByPubkeyFreeArgs {
            refund_rent_to: payer.pubkey(),
            signer: target_authority_kp.pubkey(),
            authority: disable_pool_authority_kps[1].pubkey(),
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
            disable_pool_authority_list: KeyedAccount {
                pubkey: DISABLE_POOL_AUTHORITY_LIST_ID,
                account: disable_pool_authority_list_acc.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = remove_disable_pool_authority_ix(keys, args).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, target_authority_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();
        assert_custom_err(
            err,
            SControllerError::UnauthorizedDisablePoolAuthoritySigner,
        );
    }

    // Remove an authority by the authority
    {
        let target_index = 1;
        let target_authority_kp = &disable_pool_authority_kps[target_index + 1]; // since 0 was removed
        let keys = RemoveDisablePoolAuthorityFreeArgs {
            index: target_index,
            refund_rent_to: payer.pubkey(),
            signer: target_authority_kp.pubkey(),
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
            disable_pool_authority_list: KeyedAccount {
                pubkey: DISABLE_POOL_AUTHORITY_LIST_ID,
                account: disable_pool_authority_list_acc.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = remove_disable_pool_authority_ix(
            keys,
            RemoveDisablePoolAuthorityIxArgs {
                index: index_to_u32(target_index).unwrap(),
            },
        )
        .unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, target_authority_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        assert_disable_authority_removed(
            &mut banks_client,
            target_authority_kp.pubkey(),
            before_len,
        )
        .await;
    }
}
