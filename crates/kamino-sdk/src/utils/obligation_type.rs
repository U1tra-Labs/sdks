use solana_sdk::pubkey::Pubkey;

use crate::error::KaminoError;
// use crate::classes::{KaminoMarket, KaminoObligation};

pub enum ObligationType {
    VanillaObligation {
        tag: u8,
        id: u8,
        program_id: Pubkey
    },
    MultiplyObligation {
        tag: u8,
        id: u8,
        program_id: Pubkey,
        coll_token: Pubkey,
        debt_token: Pubkey
    },
    LendingObligation {
        tag: u8,
        id: u8,
        program_id: Pubkey,
        token: Pubkey
    },
    LeverageObligation {
        tag: u8,
        id: u8,
        program_id: Pubkey,
        coll_token: Pubkey,
        debt_token: Pubkey
    }
}

pub struct InitObligationArgsModel {
    pub tag: u8,
    pub id: u8,
    pub seed1: Pubkey,
    pub seed2: Pubkey
}

impl ObligationType {
    // pub fn get_obligation_type_by_obligation() -> Result<Self, KaminoError> {}
    
    pub fn get_obligation_by_type(
        // kaminoMarket: KaminoMarket,
        obligation_tag: u8,
        mint_address_1: Pubkey,
        mint_address_2: Pubkey
    ) -> Result<Self, KaminoError> {
        match obligation_tag {
            // replace pubkey defaults with kaminoMarket.programId
            0 => Ok(Self::new_vanilla(Pubkey::default(), None)),
            1 => Ok(Self::new_multiply(mint_address_1, mint_address_2, Pubkey::default(), None)),
            2 => Ok(Self::new_lending(mint_address_1, Pubkey::default(), None)),
            3 => Ok(Self::new_leverage(mint_address_1, mint_address_2, Pubkey::default(), None)),
            _ => Err(KaminoError::InvalidObligationType)
        }
    }
    
    pub fn new_vanilla(program_id: Pubkey, id: Option<u8>) -> Self {
        let id = match id {
            Some(id) => id,
            None => 0
        };
        Self::VanillaObligation { 
            tag: 0, 
            id,
            program_id
        }
    }
    
    pub fn new_multiply(mint_address_1: Pubkey, mint_address_2: Pubkey, program_id: Pubkey, id: Option<u8>) -> Self {
        let id = match id {
            Some(id) => id,
            None => 0
        };
        Self::MultiplyObligation { 
            tag: 1, 
            id, 
            program_id, 
            coll_token: mint_address_1, 
            debt_token: mint_address_2
        }
    }
    
    pub fn new_leverage(mint_address_1: Pubkey, mint_address_2: Pubkey, program_id: Pubkey, id: Option<u8>) -> Self {
        let id = match id {
            Some(id) => id,
            None => 0
        };
        Self::LeverageObligation { 
            tag: 2, 
            id, 
            program_id, 
            coll_token: mint_address_1, 
            debt_token: mint_address_2
        }
    }
    
    pub fn new_lending(token: Pubkey, program_id: Pubkey, id: Option<u8>) -> Self {
        let id = match id {
            Some(id) => id,
            None => 0
        };
        Self::LendingObligation { 
            tag: 3, 
            id, 
            program_id, 
            token
        }
    }
    
    fn get_program_id(&self) -> Pubkey {
        match self {
            Self::VanillaObligation { program_id, .. } => *program_id,
            Self::MultiplyObligation { program_id, .. } => *program_id,
            Self::LeverageObligation { program_id, .. } => *program_id,
            Self::LendingObligation { program_id, .. } => *program_id,
        }
    }
    
    pub fn to_pda(self, market: Pubkey, user: Pubkey) -> Pubkey {
        let program_id = self.get_program_id();
        get_obligation_pda_with_args(market, user, self.to_args(), &program_id)
    }
    
    pub fn to_args(self) -> InitObligationArgsModel {
        match self {
            Self::VanillaObligation { tag, id, .. } => InitObligationArgsModel { tag, id, seed1: Pubkey::default(), seed2: Pubkey::default() },
            Self::MultiplyObligation { tag, id, coll_token, debt_token, .. } => InitObligationArgsModel { tag, id, seed1: coll_token, seed2: debt_token },
            Self::LeverageObligation { tag, id, coll_token, debt_token, .. } => InitObligationArgsModel { tag, id, seed1: coll_token, seed2: debt_token },
            Self::LendingObligation { tag, id, token, .. } => InitObligationArgsModel { tag, id, seed1: token, seed2: token }
        }
    }
}

pub fn get_obligation_pda_with_args(
    market: Pubkey,
    user: Pubkey,
    args: InitObligationArgsModel,
    program_id: &Pubkey
) -> Pubkey {
    let seed = [
        &[args.tag][..],
        &[args.id][..],
        &user.to_bytes()[..],
        &market.to_bytes()[..],
        &args.seed1.to_bytes()[..],
        &args.seed2.to_bytes()[..],
    ];
    let (pda, _) = Pubkey::find_program_address(&seed, program_id);
    return pda
}