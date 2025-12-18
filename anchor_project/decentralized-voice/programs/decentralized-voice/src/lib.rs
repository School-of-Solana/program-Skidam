use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod instructions;
pub mod states; 
pub mod errors;

declare_id!("8o5UDNmjgCYzHkhTrL6qV3HeCADrmSRYgctxuJnSSsav");

#[program]
pub mod decentralized_voice {

    use super::*;

    pub fn pool_create(ctx: Context<CreatePoolContext>, name: String, candidates: String) -> Result<()> {
        create_pool(ctx, name, candidates)  
    }





    
}
