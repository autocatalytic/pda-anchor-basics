use anchor_lang::prelude::*;
use anchor_lang::
    solana_program::{program::invoke_signed, system_instruction};


declare_id!("8fLMd4e4rrhUoxUw67uLvZiYkfo2GTF87WnJyXxEjVmV");

const TREASURY_PDA_SEED: &[u8] = b"treasure";

#[program]
pub mod pda_treasure {
    use super::*;

    // Super simple, no Accounts, just to pull a PDA from within backend.
    pub fn print_funds(ctx: Context<NoAccount>) -> Result<()> {
        let (pda, bump_seed) = Pubkey::find_program_address(&[TREASURY_PDA_SEED], ctx.program_id);
        msg!("found pda: {}, with bump {}", pda, bump_seed);
        Ok(())
    }

    // Still think it's amazing you don't need the receiver here!
    pub fn withdrawal_handler(ctx: Context<Withdraw>, bump_seed: u8, /* receiver: Pubkey,*/ lamports: u64) -> Result<()> {
        ctx.accounts.withdraw(bump_seed, lamports)?;
        msg!("Moving {} Lamports", lamports);
        Ok(())
    }  
}

#[account]
pub struct Treasury {
    balance: Pubkey,
}

#[derive(Accounts)]
pub struct NoAccount<> {}

/// Vestige from previous testing, but leaving in for the interesting results:
/// 
/// 1. didn't need this AT ALL for testing basic functionality, duh
/// 2. PDA's are super cool
/// 3. using this structure exposed the uniqueness constraint resulting from
///    a static seed without, say, a counter or new account each time.
/// 4. switching between system program, PDA, authorities, still confusing me.
/*
#[derive(Accounts)]
pub struct PrintTreasuryBalance<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// CHECK: don't care
    #[account(init, payer=initializer, seeds=[TREASURY_PDA_SEED], space = 8 + 8, bump)]
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
*/

// Don't need a signer here because PDA's are not cryptographically signed
#[derive(Accounts)]
pub struct Withdraw<'info> {
    // treasury PDA
    /// CHECK: ToDo
    #[account(mut, seeds=[TREASURY_PDA_SEED], bump)]
    pub treasury: AccountInfo<'info>,

    // destination for withdrawal
    /// CHECK: ToDo
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    // misc - need this so invoke_signed can rederive our treasury PDA
    // and probably other reasons I'm ignorant of
    pub system_program: Program<'info, System>,
}

// Cleverly pull invoke_signed into an impl block so we can use mutable
// references to self, which cannot be done within ctx: Context (at least with
// anchor's framework), in #[program] up top.
impl<'info> Withdraw<'info> {
    /// The withdraw function within the Withdraw impl block does not require 
    /// either the PDA or receiver, as PDA is rederived with 
    /// invoke_signed(account_infos) and to: we readily access with &self. 
    /// 
    /// Most surprising absence is from:, which comes in from derive(Accounts), 
    /// because we don't pass them as signature variables but in the .accounts
    /// section when client calls the instruction (program.method).
    fn withdraw(&self, bump_seed: u8, lamports: u64) -> Result<()> {

        msg!("impl block from: {}, to: {}, bump: {}", self.treasury.key(), self.destination.key(), bump_seed);

        invoke_signed(
            &system_instruction::transfer(&self.treasury.key(), &self.destination.key(), lamports),
            &[
                self.treasury.to_account_info(),
                self.destination.clone(),
                self.system_program.to_account_info(),
            ],
            &[&[
                // Signers seeds
                TREASURY_PDA_SEED, //.as_ref(),
                &[bump_seed],
            ]],
        )
        .map_err(Into::into)
    }
}
