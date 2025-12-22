use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod instructions;
pub mod states; 
pub mod errors;

declare_id!("8o5UDNmjgCYzHkhTrL6qV3HeCADrmSRYgctxuJnSSsav");

#[program]
pub mod decentralized_voice {

    use super::*;

    pub fn pool_create(ctx: Context<CreatePoolContext>, name: String, max_candidate_number: i64) -> Result<()> {
        create_pool(ctx, name, max_candidate_number)  
    }





    
}
