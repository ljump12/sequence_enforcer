use anchor_lang::prelude::*;


//#[cfg(not(feature = "devnet"))]
//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//#[cfg(feature = "mainnet")]
declare_id!("GDDMwNyyx8uB6zrqwBFHjLLG3TBYk2F8Az4yrQC5RzMp");

#[program]
pub mod sequence_enforcer {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _bump: u8, _sym: String) -> ProgramResult {
        let sequence_account = &mut ctx.accounts.sequence_account;
        sequence_account.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn reset_sequence_number(
        ctx: Context<ResetSequenceNumber>,
        sequence_num: u64
    ) -> ProgramResult {
        msg!("Resetting sequence number to {}", sequence_num);

        let sequence_account = &mut ctx.accounts.sequence_account;
        sequence_account.sequence_num = sequence_num;

        Ok(())
    }

    pub fn check_and_set_sequence_number(
        ctx: Context<CheckAndSetSequenceNumber>,
        sequence_num: u64
    ) -> ProgramResult {
        let sequence_account = &mut ctx.accounts.sequence_account;
        let last_known_sequence_num = sequence_account.sequence_num;
        if sequence_num > last_known_sequence_num {
            sequence_account.sequence_num = sequence_num;
            return Ok(());
        }
        
        msg!("Sequence out of order | sequence_num={} | last_known={}", sequence_num, last_known_sequence_num);
        return Err(ErrorCode::SequenceOutOfOrder.into());
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, sym: String)]
pub struct Initialize<'info> {
    #[account(init_if_needed,
        payer=authority, 
        seeds=[sym.as_bytes()], bump=bump
    )]
    pub sequence_account: Account<'info, SequenceAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ResetSequenceNumber<'info> {
    #[account(mut, has_one=authority)]
    pub sequence_account: Account<'info, SequenceAccount>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct CheckAndSetSequenceNumber<'info> {
    #[account(mut, has_one=authority)]
    pub sequence_account: Account<'info, SequenceAccount>,
    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct SequenceAccount {
    pub sequence_num: u64,
    pub authority: Pubkey
}

#[error]
pub enum ErrorCode {
    #[msg("Sequence out of order")]
    SequenceOutOfOrder
}