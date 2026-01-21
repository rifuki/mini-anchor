use mini_anchor::{Account, Accounts, Signer};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[mini_anchor::account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

fn create_account_info<'a>(
    key: &'a Pubkey,
    is_signer: bool,
    is_writeable: bool,
    lamports: &'a mut u64,
    data: &'a mut [u8],
    owner: &'a Pubkey,
) -> AccountInfo<'a> {
    AccountInfo::new(
        key,
        is_signer,
        is_writeable,
        lamports,
        data,
        owner,
        false,
        0,
    )
}

#[test]
fn test_try_accounts_success() {
    let program_id = Pubkey::new_unique();
    let counter_key = Pubkey::new_unique();
    let authority_key = Pubkey::new_unique();

    let mut counter_lamports = 1000u64;
    let mut counter_data = vec![0u8; Counter::SPACE];
    let mut authority_lamports = 1000u64;
    let mut authority_data = vec![0u8; 0];

    let counter_info = create_account_info(
        &counter_key,
        false, // not signer
        true,  // writeable
        &mut counter_lamports,
        &mut counter_data,
        &program_id,
    );

    let authority_info = create_account_info(
        &authority_key,
        true,  // is signer
        false, // not writeable
        &mut authority_lamports,
        &mut authority_data,
        &program_id,
    );

    let accounts = vec![counter_info, authority_info];

    // Test try_accounts
    let result = Initialize::try_accounts(&accounts);
    assert!(result.is_ok());

    let ctx = result.unwrap();
    assert_eq!(ctx.counter.key(), &counter_key);
    assert_eq!(ctx.authority.key(), &authority_key);
}

#[test]
fn test_try_accounts_not_enough_accounts() {
    let program_id = Pubkey::new_unique();
    let counter_key = Pubkey::new_unique();

    let mut counter_lamports = 1000u64;
    let mut counter_data = vec![0u8; Counter::SPACE];

    let counter_info = create_account_info(
        &counter_key,
        false,
        true,
        &mut counter_lamports,
        &mut counter_data,
        &program_id,
    );

    // Only one account, but Initialize needs 2
    let accounts = vec![counter_info];

    let result = Initialize::try_accounts(&accounts);
    assert!(result.is_err());
}

#[test]
fn test_try_accounts_missing_signer() {
    let program_id = Pubkey::new_unique();
    let counter_key = Pubkey::new_unique();
    let authority_key = Pubkey::new_unique();

    let mut counter_lamports = 1000u64;
    let mut authority_lamports = 1000u64;
    let mut counter_data = vec![0u8; Counter::SPACE];
    let mut authority_data = vec![0u8; 0];

    let counter_info = create_account_info(
        &counter_key,
        false,
        true,
        &mut counter_lamports,
        &mut counter_data,
        &program_id,
    );

    let authority_info = create_account_info(
        &authority_key,
        false,
        false,
        &mut authority_lamports,
        &mut authority_data,
        &program_id,
    );

    let accounts = vec![counter_info, authority_info];

    let result = Initialize::try_accounts(&accounts);
    assert!(result.is_err()); // Should fail due to missing signer
}

#[test]
fn test_try_accounts_not_writeable() {
    let program_id = Pubkey::new_unique();
    let counter_key = Pubkey::new_unique();
    let authority_key = Pubkey::new_unique();

    let mut counter_lamports = 1000u64;
    let mut counter_data = vec![0u8; Counter::SPACE];
    let mut authority_lamports = 1000u64;
    let mut authority_data = vec![0u8; 0];

    let counter_info = create_account_info(
        &counter_key,
        false,
        false, // NOT writeable! but #[account(mut)] requires it
        &mut counter_lamports,
        &mut counter_data,
        &program_id,
    );

    let authority_info = create_account_info(
        &authority_key,
        true,
        false,
        &mut authority_lamports,
        &mut authority_data,
        &program_id,
    );

    let accounts = vec![counter_info, authority_info];

    let result = Initialize::try_accounts(&accounts);
    assert!(result.is_err()); // Should fail - counter not writeable
}
