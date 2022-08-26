#![cfg(feature = "test-sbf")]

mod program_test;
use {
    program_test::{TestContext, TokenContext},
    solana_program_test::tokio,
    solana_sdk::{
        instruction::InstructionError, pubkey::Pubkey, signature::Signer, signer::keypair::Keypair,
        transaction::TransactionError, transport::TransportError,
    },
    spl_token_2022::error::TokenError,
    spl_token_client::token::{ExtensionInitializationParams, TokenError as TokenClientError},
};

#[derive(PartialEq)]
enum TestMode {
    All,
    CheckedOnly,
}

async fn run_basic_transfers(context: TestContext, test_mode: TestMode) {
    let TokenContext {
        decimals,
        mint_authority,
        token,
        alice,
        bob,
        ..
    } = context.token_context.unwrap();

    let alice_account = Keypair::new();
    let alice_account = token
        .create_auxiliary_token_account(&alice_account, &alice.pubkey())
        .await
        .unwrap();
    let bob_account = Keypair::new();
    let bob_account = token
        .create_auxiliary_token_account(&bob_account, &bob.pubkey())
        .await
        .unwrap();

    // mint a token
    let amount = 10;
    token
        .mint_to(
            &alice_account,
            &mint_authority.pubkey(),
            amount,
            Some(decimals),
            &vec![&mint_authority],
        )
        .await
        .unwrap();

    if test_mode == TestMode::All {
        // unchecked is ok
        token
            .transfer(
                &alice_account,
                &bob_account,
                &alice.pubkey(),
                1,
                None,
                &vec![&alice],
            )
            .await
            .unwrap();
    }

    // checked is ok
    token
        .transfer(
            &alice_account,
            &bob_account,
            &alice.pubkey(),
            1,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap();

    // transfer too much is not ok
    let error = token
        .transfer(
            &alice_account,
            &bob_account,
            &alice.pubkey(),
            amount,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap_err();
    assert_eq!(
        error,
        TokenClientError::Client(Box::new(TransportError::TransactionError(
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(TokenError::InsufficientFunds as u32)
            )
        )))
    );

    // wrong signer
    let error = token
        .transfer(
            &alice_account,
            &bob_account,
            &bob.pubkey(),
            1,
            Some(decimals),
            &vec![&bob],
        )
        .await
        .unwrap_err();
    assert_eq!(
        error,
        TokenClientError::Client(Box::new(TransportError::TransactionError(
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(TokenError::OwnerMismatch as u32)
            )
        )))
    );
}

#[tokio::test]
async fn basic() {
    let mut context = TestContext::new().await;
    context.init_token_with_mint(vec![]).await.unwrap();
    run_basic_transfers(context, TestMode::All).await;
}

#[tokio::test]
async fn basic_with_extension() {
    let mut context = TestContext::new().await;
    context
        .init_token_with_mint(vec![ExtensionInitializationParams::TransferFeeConfig {
            transfer_fee_config_authority: Some(Pubkey::new_unique()),
            withdraw_withheld_authority: Some(Pubkey::new_unique()),
            transfer_fee_basis_points: 100u16,
            maximum_fee: 1_000_000u64,
        }])
        .await
        .unwrap();
    run_basic_transfers(context, TestMode::CheckedOnly).await;
}

async fn run_self_transfers(context: TestContext, test_mode: TestMode) {
    let TokenContext {
        decimals,
        mint_authority,
        token,
        alice,
        ..
    } = context.token_context.unwrap();

    let alice_account = Keypair::new();
    let alice_account = token
        .create_auxiliary_token_account(&alice_account, &alice.pubkey())
        .await
        .unwrap();

    // mint a token
    let amount = 10;
    token
        .mint_to(
            &alice_account,
            &mint_authority.pubkey(),
            amount,
            Some(decimals),
            &vec![&mint_authority],
        )
        .await
        .unwrap();

    // self transfer is ok
    token
        .transfer(
            &alice_account,
            &alice_account,
            &alice.pubkey(),
            1,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap();
    if test_mode == TestMode::All {
        token
            .transfer(
                &alice_account,
                &alice_account,
                &alice.pubkey(),
                1,
                None,
                &vec![&alice],
            )
            .await
            .unwrap();
    }

    // too much self transfer is not ok
    let error = token
        .transfer(
            &alice_account,
            &alice_account,
            &alice.pubkey(),
            amount + 1,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap_err();
    assert_eq!(
        error,
        TokenClientError::Client(Box::new(TransportError::TransactionError(
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(TokenError::InsufficientFunds as u32)
            )
        )))
    );
}

#[tokio::test]
async fn self_transfer() {
    let mut context = TestContext::new().await;
    context.init_token_with_mint(vec![]).await.unwrap();
    run_self_transfers(context, TestMode::All).await;
}

#[tokio::test]
async fn self_transfer_with_extension() {
    let mut context = TestContext::new().await;
    context
        .init_token_with_mint(vec![ExtensionInitializationParams::TransferFeeConfig {
            transfer_fee_config_authority: Some(Pubkey::new_unique()),
            withdraw_withheld_authority: Some(Pubkey::new_unique()),
            transfer_fee_basis_points: 100u16,
            maximum_fee: 1_000_000u64,
        }])
        .await
        .unwrap();
    run_self_transfers(context, TestMode::CheckedOnly).await;
}

async fn run_self_owned(context: TestContext, test_mode: TestMode) {
    let TokenContext {
        decimals,
        mint_authority,
        token,
        alice,
        bob,
        ..
    } = context.token_context.unwrap();

    let alice_account = token
        .create_auxiliary_token_account(&alice, &alice.pubkey())
        .await
        .unwrap();
    let bob_account = Keypair::new();
    let bob_account = token
        .create_auxiliary_token_account(&bob_account, &bob.pubkey())
        .await
        .unwrap();

    // mint a token
    let amount = 10;
    token
        .mint_to(
            &alice_account,
            &mint_authority.pubkey(),
            amount,
            Some(decimals),
            &vec![&mint_authority],
        )
        .await
        .unwrap();

    if test_mode == TestMode::All {
        // unchecked is ok
        token
            .transfer(
                &alice_account,
                &bob_account,
                &alice.pubkey(),
                1,
                None,
                &vec![&alice],
            )
            .await
            .unwrap();
    }

    // checked is ok
    token
        .transfer(
            &alice_account,
            &bob_account,
            &alice.pubkey(),
            1,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap();

    // self transfer is ok
    token
        .transfer(
            &alice_account,
            &alice_account,
            &alice.pubkey(),
            1,
            Some(decimals),
            &vec![&alice],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn self_owned() {
    let mut context = TestContext::new().await;
    context.init_token_with_mint(vec![]).await.unwrap();
    run_self_owned(context, TestMode::All).await;
}

#[tokio::test]
async fn self_owned_with_extension() {
    let mut context = TestContext::new().await;
    context
        .init_token_with_mint(vec![ExtensionInitializationParams::TransferFeeConfig {
            transfer_fee_config_authority: Some(Pubkey::new_unique()),
            withdraw_withheld_authority: Some(Pubkey::new_unique()),
            transfer_fee_basis_points: 100u16,
            maximum_fee: 1_000_000u64,
        }])
        .await
        .unwrap();
    run_self_owned(context, TestMode::CheckedOnly).await;
}

#[tokio::test]
async fn transfer_with_fee_on_mint_without_fee_configured() {
    let mut context = TestContext::new().await;
    context.init_token_with_mint(vec![]).await.unwrap();
    let TokenContext {
        decimals,
        mint_authority,
        token,
        alice,
        bob,
        ..
    } = context.token_context.unwrap();

    let alice_account = Keypair::new();
    let alice_account = token
        .create_auxiliary_token_account(&alice_account, &alice.pubkey())
        .await
        .unwrap();
    let bob_account = Keypair::new();
    let bob_account = token
        .create_auxiliary_token_account(&bob_account, &bob.pubkey())
        .await
        .unwrap();

    // mint some tokens
    let amount = 10;
    token
        .mint_to(
            &alice_account,
            &mint_authority.pubkey(),
            amount,
            Some(decimals),
            &vec![&mint_authority],
        )
        .await
        .unwrap();

    // success if expected fee is 0
    token
        .transfer_with_fee(
            &alice_account,
            &bob_account,
            &alice.pubkey(),
            1,
            decimals,
            0,
            &vec![&alice],
        )
        .await
        .unwrap();

    // fail for anything else
    let error = token
        .transfer_with_fee(
            &alice_account,
            &bob_account,
            &alice.pubkey(),
            2,
            decimals,
            1,
            &vec![&alice],
        )
        .await
        .unwrap_err();
    assert_eq!(
        error,
        TokenClientError::Client(Box::new(TransportError::TransactionError(
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(TokenError::FeeMismatch as u32)
            )
        )))
    );
}
