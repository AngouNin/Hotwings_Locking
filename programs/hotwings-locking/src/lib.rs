// Importing necessary modules from Anchor and SPL Token libraries
use anchor_lang::prelude::*; // For general Anchor utilities and types
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer}; // For token-related functionalities

// Program ID for SPL-Token program
declare_id!("FuML3MpeXtoKgZY1nBJUCJyvtQZdBcSt2Kb7GjqGW8SR");

// Defining the main module for the `hotwings_locking` program
#[program]
mod hotwings_locking {
    use super::*;

    // Entry point function to initialize a lock on tokens
    pub fn initialize_lock(
        ctx: Context<InitializeLock>, // Context object holding all accounts and information needed
        amount: u64, // Amount of tokens to lock 
    ) -> Result<()> {
        // Constructing the accounts needed to perform a token transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.project_wallet.to_account_info(), // The token account from which to transfer (Project Wallet)
            to: ctx.accounts.locked_account.to_account_info(), // The token account to which tokens will be transferred (Locked Account - PDA)
            authority: ctx.accounts.project_wallet_authority.to_account_info(), // The authority that approves the transfer
        };

        // Creating a CPI context which enables interaction with the token program
        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        // Calling the token transfer function from the SPL Token program
        token::transfer(cpi_context, amount)?;

        // Updating the locked tokens balance for the user's locked tokens account
        let locked_account = &mut ctx.accounts.investor_locked_tokens; // Getting mutable reference to investor's locked tokens
        locked_account.locked_amount += amount; // Incrementing the locked amount by the transferred amount

        // Indicating successful completion of the function
        Ok(())
    }
}

// Context for InitializeLock Instruction
#[derive(Accounts)]
pub struct InitializeLock<'info> {
    // Token Mint Address (HotWings Token Only)
    pub token_mint: Account<'info, Mint>,

    // Project Wallet (Holds initial supply to lock tokens from)
    #[account(mut)]
    pub project_wallet: Account<'info, TokenAccount>,

    // Project Wallet Authority (Signer for project wallet)
    pub project_wallet_authority: Signer<'info>,

    // PDA Account for Locked Tokens (Program Derived Account)
    #[account(
        init_if_needed, // Create PDA if it doesn't already exist
        seeds = [b"locked-tokens", user.key().as_ref()], // Unique Seed for PDA
        bump,
        payer = user,
        token::mint = token_mint,
        token::authority = program_pda,
    )]
    pub locked_account: Account<'info, TokenAccount>,

    // Program PDA (controls the locked token account)
    /// CHECK: PDA authority
    pub program_pda: AccountInfo<'info>,

    // Storage for tracking locked tokens for the investor
    #[account(
        init_if_needed,
        seeds = [b"locked-balance", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 8 // Space for u64 locked_amount
    )]
    pub investor_locked_tokens: Account<'info, LockedTokens>,

    // Investor Account (Signer who will have tokens locked)
    #[account(mut)]
    pub user: Signer<'info>,

    // System Program (Required for creating PDAs)
    pub system_program: Program<'info, System>,

    // SPL Token Program (Required for SPL interactions)
    pub token_program: Program<'info, Token>,

    // Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

// Locked Tokens Account Data Structure (Tracks locked tokens per user)
#[account]
pub struct LockedTokens {
    pub locked_amount: u64,
}