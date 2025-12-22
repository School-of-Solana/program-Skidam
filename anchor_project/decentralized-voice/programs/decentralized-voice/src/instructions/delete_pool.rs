use anchor_lang::prelude::*; 
use crate::states::*;
use crate::errors::DecentralizedVoiceErrors;



pub fn delete_pool(ctx: Context<DeletePoolContext>) -> Result<()> {


    let created_pool = &mut ctx.accounts.created_pool; 
    let pool_deleter = &mut ctx.accounts.pool_deleter; 

    if pool_deleter.key() != created_pool.creator {
        return Err(DecentralizedVoiceErrors::NotVoteOwner.into())
    }


    Ok(())

}



#[derive(Accounts)]
pub struct DeletePoolContext<'info> {
    #[account(mut)]
    pub pool_deleter: Signer<'info>,
    #[account(mut,
        close = pool_deleter, 
        seeds = [b"pool", pool_deleter.key().as_ref()],
        bump
    )]
    pub created_pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>
}