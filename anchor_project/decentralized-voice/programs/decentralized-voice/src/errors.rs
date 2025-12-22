use anchor_lang::prelude::*; 

#[error_code]
pub enum DecentralizedVoiceErrors {
    #[msg("Only vote owner can delete a vote")] NotVoteOwner, 
    #[msg("Maximum pool name length exceeded")] PoolLenghExceeded, 
    #[msg("Maximum length for candidate names exceeded")] PoolCandidateLength, 
    #[msg("You have to be the pool owner to create candidates")] NotPoolOwner,
    
}