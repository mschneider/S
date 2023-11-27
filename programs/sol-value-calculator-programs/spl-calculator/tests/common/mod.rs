use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use spl_calculator_lib::SplSolValCalc;
use test_utils::{AddAccount, KeyedUiAccount, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT};

pub struct JitoNormalProgramTest {
    pub program_test: ProgramTest,
    pub jito_stake_pool: KeyedReadonlyAccount,
    pub spl_stake_pool_prog: KeyedReadonlyAccount,
}

pub fn jito_normal_program_test() -> JitoNormalProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "spl_sol_value_calculator",
        spl_calculator_lib::program::ID,
        processor!(spl_calculator::entrypoint::process_instruction),
    );

    let spl_stake_pool_prog_ui_acc =
        KeyedUiAccount::from_test_fixtures_file("spl-stake-pool-prog.json");
    let jito_stake_pool_ui_acc = KeyedUiAccount::from_test_fixtures_file("jito-stake-pool.json");

    let spl_stake_pool_prog = spl_stake_pool_prog_ui_acc.to_keyed_readonly_account();
    let jito_stake_pool = jito_stake_pool_ui_acc.to_keyed_readonly_account();

    program_test_add_mock_calculator_state::<SplSolValCalc>(
        ProgramTestAddMockCalculatorStateArgs {
            program_test: &mut program_test,
            manager: Pubkey::default(),
            last_upgrade_slot: SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
        },
    );
    program_test = program_test
        .add_keyed_ui_account(spl_stake_pool_prog_ui_acc)
        .add_keyed_ui_account(jito_stake_pool_ui_acc)
        .add_test_fixtures_account("spl-stake-pool-prog-data.json");

    JitoNormalProgramTest {
        program_test,
        spl_stake_pool_prog,
        jito_stake_pool,
    }
}