use anchor_lang::prelude::*; 
use crate::states::*; 
use crate::errors::DecentralizedVoiceErrors;


pub fn vote (ctx: Context<VoteContext>, candidate_name: String) -> Result<()> {

    let created_pool = &mut ctx.accounts.created_pool; 
    let candidate = &mut ctx.accounts.candidate;
    if candidate_name == candidate.name &&
                candidate.votes <= created_pool.max_candidate_number {
    candidate.votes.checked_add(1).ok_or(DecentralizedVoiceErrors::OverflowOccured)?;
     }

    Ok(())
}




#[derive(Accounts)]
#[instruction(candidate_name: String)]
pub struct VoteContext<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub created_pool: Account<'info, Pool>,
    #[account(
        mut, 
               seeds = [
            b"candidate",
            created_pool.creator.as_ref(),
            {candidate_name.as_bytes()},
            created_pool.key().as_ref()], 
        bump,

    )]
    pub candidate: Account<'info, Candidate>,
    system_program: Program<'info,System>,




}