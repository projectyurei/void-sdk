use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;

const COMP_DEF_OFFSET_INIT_ACCOUNT: u32 = comp_def_offset("init_account");

declare_id!("9oqbvYkKhFA2EFrJKGujRqzHnCRGuGnzTD6dyXuxo6oo");

#[arcium_program]
pub mod void_protocol {
    use super::*;

    /// Initialize the computation definition for init_account circuit.
    /// Must be called once before creating any private accounts.
    pub fn init_account_comp_def(ctx: Context<InitAccountCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    /// Create a new private account with encrypted state.
    /// The account state is initialized via MPC computation.
    pub fn create_private_account(
        ctx: Context<CreatePrivateAccount>,
        computation_offset: u64,
        nonce: u128,
    ) -> Result<()> {
        let account = &mut ctx.accounts.private_account;
        account.bump = ctx.bumps.private_account;
        account.owner = ctx.accounts.owner.key();
        account.state_nonce = nonce;
        account.encrypted_state = [[0u8; 32]; 4];

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        let args = ArgBuilder::new().plaintext_u128(nonce).build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![InitAccountCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[CallbackAccount {
                    pubkey: ctx.accounts.private_account.key(),
                    is_writable: true,
                }],
            )?],
            1,
            0,
        )?;

        Ok(())
    }

    /// Callback invoked by Arcium after init_account MPC computation completes.
    /// Stores the encrypted state in the account.
    #[arcium_callback(encrypted_ix = "init_account")]
    pub fn init_account_callback(
        ctx: Context<InitAccountCallback>,
        output: SignedComputationOutputs<InitAccountOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(InitAccountOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        let account_key = ctx.accounts.private_account.key();
        let owner = ctx.accounts.private_account.owner;

        let account = &mut ctx.accounts.private_account;
        account.encrypted_state = o.ciphertexts;
        account.state_nonce = o.nonce;

        emit!(AccountCreatedEvent {
            account: account_key,
            owner,
        });

        Ok(())
    }
}

// ============================================================================
// Account Structs
// ============================================================================

/// On-chain account storing encrypted private DeFi state.
/// The encrypted_state contains 4 encrypted fields: owner_lo, owner_hi, balance, token_mint
#[account]
#[derive(InitSpace)]
pub struct VoidPrivateAccount {
    pub bump: u8,
    pub owner: Pubkey,
    pub state_nonce: u128,
    pub encrypted_state: [[u8; 32]; 4],
}

// ============================================================================
// Instruction Account Contexts
// ============================================================================

#[init_computation_definition_accounts("init_account", payer)]
#[derive(Accounts)]
pub struct InitAccountCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("init_account", owner)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct CreatePrivateAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + VoidPrivateAccount::INIT_SPACE,
        seeds = [b"private_account", owner.key().as_ref()],
        bump,
    )]
    pub private_account: Account<'info, VoidPrivateAccount>,
    #[account(
        init_if_needed,
        space = 9,
        payer = owner,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(mut, address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_ACCOUNT))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("init_account")]
#[derive(Accounts)]
pub struct InitAccountCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_ACCOUNT))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,
    /// CHECK: computation_account, checked by arcium program via constraints in the callback context.
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub private_account: Account<'info, VoidPrivateAccount>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct AccountCreatedEvent {
    pub account: Pubkey,
    pub owner: Pubkey,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Cluster not set")]
    ClusterNotSet,
}
