// use anchor_lang::{
//     prelude::Pubkey,
//     solana_program::{self},
//     system_program,
//     AccountDeserialize,
//     InstructionData,
//     ToAccountMetas,
//     context
// };
use anchor_lang::{
    prelude::*,
    solana_program::{self, system_program},
    context::{CpiContext, Context}
};
use conditional_vault:: InitializeConditionalVaultArgs;
// use anchor_spl::{associated_token, token};
use spl_token::;
use anyhow::Ok;
use solana_program::instruction::Instruction;
use solana_program_test::{tokio, ProgramTest, ProgramTestContext};
use solana_sdk::{account::Account, account_info::AccountInfo, signature::Keypair, signer::Signer, transaction::Transaction};


#[tokio::test]
async fn test_initialize_conditional_vault() {

  let SetUpTest {
    validator,
    user,
    underlying_token_mint,
    cvault_pda,
    cfinalize_pda,
    crevert_pda,
  } = SetUpTest::new();

  let mut ctx = validator.start_with_context().await;

  // let utoken_mint_account: Account = load_and_deserialize(&mut context, underlying_token_mint.pubkey()).await;

  // let mint_account = ctx
  //       .banks_client
  //       .get_account(underlying_token_mint.pubkey())
  //       .await
  //       .unwrap() //unwraps the Result into an Option<Account>
  //       .unwrap(); //unwraps the Option<Account> into an Account

//   let mint_account_info = AccountInfo::new(
//     &underlying_token_mint.pubkey(),
//     false, // is_signer
//     true,  // is_writable
//     &mut mint_account.lamports,
//     &mut mint_account.data,
//     &mint_account.owner,
//     mint_account.executable,
//     mint_account.rent_epoch,
// );

  // Setup the mint
  let mint_ix = spl_token::instruction::initialize_mint2(
                &spl_token::id(),
                &mint.pubkey(),
                &user.pubkey(),
                None,
                8,
            );
  
  context.banks_client.process_transaction(mint_tx).await.unwrap();
 
  // create mint instruction
  let token_mint = token::initialize_mint2(
    CpiContext::new(
        Context.accounts.token_program.to_account_info(),
        init_mint,
    ),
    8,
    &underlying_token_mint.pubkey(),
    None, // no freeze authority
  );



  // initialize conditional vault instruction
  // let args = InitializeConditionalVaultArgs {
  //   settlement_authority: user.pubkey(),
  // };
  // let init_ix = Instruction {
  //     program_id: conditional_vault::ID,
  //     accounts: conditional_vault::accounts::InitializeConditionalVault {
  //         vault: cvault_pda,
  //         underlying_token_mint: , // here need help
  //         conditional_on_finalize_token_mint: cfinalize_pda,
  //         conditional_on_revert_token_mint: crevert_pda,
  //         vault_underlying_token_account: Keypair:: new().pubkey(),
  //         payer: user.pubkey(),
  //         token_program: token::ID,
  //         associated_token_program: associated_token::ID,
  //         system_program: system_program::ID,
  //     }
  //     .to_account_metas(None),
  //     data: conditional_vault::instruction::InitializeConditionalVault{ args }.data(),
  // };


  // let init_tx = Transaction::new_signed_with_payer(
  //   &[init_ix],
  //   Some(&user.pubkey()),
  //   &[&user],
  //   context.last_blockhash,
  // );

  // context.banks_client.process_transaction(init_tx).await.unwrap();

  // let conditional_vault: conditional_vault::ConditionalVault = load_and_deserialize(&mut context, cvault_pda).await;

  // assert!(conditional_vault.status == conditional_vault::VaultStatus::Active);

}



pub struct SetUpTest {
  pub validator: ProgramTest,
  pub user: Keypair,
  pub underlying_token_mint: Keypair,
  pub cvault_pda: Pubkey,
  pub cfinalize_pda: Pubkey,
  pub crevert_pda: Pubkey,
}

impl SetUpTest {
  pub fn new() -> Self {
    let mut validator = ProgramTest::new("conditional_vault", conditional_vault::ID, None);

    let user = Keypair::new();
    validator.add_account(
      user.pubkey(),
      Account {
        lamports: 1_000_000_000,
        ..Account::default()
      },
    );

    let underlying_token_mint = Keypair::new();
    validator.add_account(
      underlying_token_mint.pubkey(),
      Account {
        lamports: 1_000_000_000,
        ..Account::default()
      },
    );

    let (cvault_pda, _) = Pubkey::find_program_address(&[b"conditional_vault"], &conditional_vault::ID);
    let (cfinalize_pda, _) = Pubkey::find_program_address(&[b"conditional_on_finalize_mint"], &conditional_vault::ID);
    let (crevert_pda, _) = Pubkey::find_program_address(&[b"conditional_on_revert_mint"], &conditional_vault::ID);
    
    Self {
      validator,
      user,
      underlying_token_mint,
      cvault_pda,
      cfinalize_pda,
      crevert_pda,
    }

  }
}

/// Fetch the account from the ProgramTestContext and deserialize it.
/// Taken from the MarginFi Github tests: https://github.com/mrgnlabs/marginfi-v2/blob/main/test-utils/src/test.rs#L468
pub async fn load_and_deserialize<T: AccountDeserialize>(
    ctx: &mut ProgramTestContext,
    address: Pubkey,
) -> T {
    let account = ctx
        .banks_client
        .get_account(address)
        .await
        .unwrap() //unwraps the Result into an Option<Account>
        .unwrap(); //unwraps the Option<Account> into an Account

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}