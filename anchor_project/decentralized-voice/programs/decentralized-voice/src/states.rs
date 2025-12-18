use anchor_lang::prelude::*;     

const POOL_NAME_LENGTH : usize = 32; 
const POOL_CANDIDATE_LENGTH : usize = 500;

// Seems every variable size needs to be known beforehand
// Clone trait is needed for Status so it can be used in Pool

#[derive(AnchorDeserialize, AnchorSerialize, InitSpace, Clone)]

pub enum Status {
    Active, 
    Inactive
}

pub enum Candidates {
    Mathew, 
    Mark, 
    Luke, 
    John
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    #[max_len(POOL_NAME_LENGTH)]
    pub name: String,
    pub status: Status,
    #[max_len(POOL_CANDIDATE_LENGTH)]
    pub candidates: String,
    pub creator: Pubkey, 

}
