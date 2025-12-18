use anchor_lang::prelude::*; 
use crate::states::*;



pub fn create_pool(ctx: Context<CreatePoolContext>, name: String, candidates: String ) -> Result<()> {

    let created_pool = &mut ctx.accounts.created_pool; 

    created_pool.name = name; 
    created_pool.candidates = candidates;
    created_pool.creator = ctx.accounts.pool_creator.key(); 
    //created_pool.status = Status::Active; 

    Ok(())

}



#[derive(Accounts)]
pub struct CreatePoolContext<'info> {
    #[account(mut)]
    pub pool_creator: Signer<'info>,
    #[account(
        init, 
        payer = pool_creator,
        space =  8 + Pool::INIT_SPACE, 
        seeds = [b"pool", pool_creator.key().as_ref()],
        bump
    )]
    pub created_pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>
}