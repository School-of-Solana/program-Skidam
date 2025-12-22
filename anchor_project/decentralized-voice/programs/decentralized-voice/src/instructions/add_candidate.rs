use anchor_lang::prelude::*; 
use crate::states::*; 
use crate::errors::DecentralizedVoiceErrors;

pub fn add_candidate (ctx: Context<AddCandidateContext>, candidate_name: String) -> Result<()> {


    let candidate_creator = &mut ctx.accounts.candidate_creator; 
    let candidate = &mut ctx.accounts.candidate; 
    let created_pool = &mut ctx.accounts.created_pool; 

    if candidate_creator.key() != created_pool.creator {
        return Err(DecentralizedVoiceErrors::NotPoolOwner.into())
    }

    if created_pool.total_candidate == created_pool.max_candidate_number {
        return Err(DecentralizedVoiceErrors::MaximumCandidateReached.into())
    }

    candidate.name = candidate_name; 
    candidate.pool_address = created_pool.key();
    candidate.votes = 0;
    candidate.creator = candidate_creator.key();


    Ok(())

}


#[derive(Accounts)]
#[instruction(candidate_name:String)]
pub struct AddCandidateContext<'info> {
    #[account(mut)]
    pub candidate_creator: Signer<'info>,
    #[account(mut)]
    pub created_pool: Account<'info, Pool>,
    #[account(
        init, 
        payer = candidate_creator,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [
            b"candidate",
            candidate_creator.key().as_ref(),
            candidate_name.as_bytes(),
            created_pool.key().as_ref()], 
        bump,

    )]
    pub candidate : Account<'info, Candidate>, 
    system_program: Program<'info,System>,

}