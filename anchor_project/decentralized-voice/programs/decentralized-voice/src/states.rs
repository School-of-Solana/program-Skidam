use anchor_lang::prelude::*;     

const POOL_NAME_LENGTH : usize = 32; 
const CANDIDATE_NAME_LENGTH : usize = 32;



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
    pub total_candidate: i64,
    pub max_candidate_number: i64,
    pub creator: Pubkey, 
}


#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(CANDIDATE_NAME_LENGTH)]
    pub name: String,
    pub pool_address: Pubkey,
    pub votes: i64,
    pub creator: Pubkey
}