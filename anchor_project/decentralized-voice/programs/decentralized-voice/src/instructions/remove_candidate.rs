use anchor_lang::prelude::*; 
use crate::states::*; 
use crate::errors::DecentralizedVoiceErrors;

pub fn remove_candidate (ctx: Context<RemoveCandidateContext>, candidate_name: String) -> Result<()> {


    let candidate_creator = &mut ctx.accounts.candidate_creator; 
    let candidate = &mut ctx.accounts.candidate; 
    let created_pool = &mut ctx.accounts.created_pool; 

    if candidate_creator.key() != created_pool.creator {
        return Err(DecentralizedVoiceErrors::NotPoolOwner.into())
    }


    created_pool.total_candidate -=1;


    Ok(())

}


#[derive(Accounts)]
#[instruction(candidate_name:String)]
pub struct RemoveCandidateContext<'info> {
    #[account(mut)]
    pub candidate_creator: Signer<'info>,
    #[account(mut)]
    pub created_pool: Account<'info, Pool>,
    #[account(
        mut, 
        close = candidate_creator,
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