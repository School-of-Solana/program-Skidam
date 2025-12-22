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

    candidate.name = candidate_name; 
    candidate.votes = 0;
    candidate.creator = candidate_creator.key();

    Ok(())

}


#[derive(Accounts)]
pub struct AddCandidateContext<'info> {
    #[account(mut)]
    candidate_creator: Signer<'info>,

    #[account(
        init, 
        payer = candidate_creator,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [b"candidate", candidate_creator.key().as_ref()], 
        bump,

    )]
    candidate : Account<'info, Candidate>, 
    created_pool: Account<'info, Pool>,
    system_program: Program<'info,System>,

}