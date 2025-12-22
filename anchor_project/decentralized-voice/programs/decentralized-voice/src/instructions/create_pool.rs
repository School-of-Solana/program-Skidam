use anchor_lang::prelude::*; 
use crate::states::*;



pub fn create_pool(ctx: Context<CreatePoolContext>, name: String, max_candidate_number: i64 ) -> Result<()> {

    let created_pool = &mut ctx.accounts.created_pool; 

    created_pool.name = name; 
    created_pool.status = Status::Active;
    created_pool.total_candidate = 0;
    created_pool.max_candidate_number = max_candidate_number; 
    created_pool.creator = ctx.accounts.pool_creator.key(); 


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