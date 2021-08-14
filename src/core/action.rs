use std::str::FromStr;
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Action {
    Open,
    Limp,
    Call,
    ThreeBet,
    ThreeBetCall,
    FourBet,
    FourBetCall,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_uppercase()[..] {
            "OPEN" => Ok(Self::Open),
            "LIMP" => Ok(Self::Limp),
            "CALL" => Ok(Self::Call),
            "THREEBET" | "3BET" => Ok(Self::ThreeBet),
            "THREEBETCALL" | "3BETCALL" => Ok(Self::ThreeBetCall),
            "FOURBET" | "4BET" => Ok(Self::FourBet),
            "FOURBETCALL" | "4BETCALL" => Ok(Self::FourBetCall),
            _ => return Err(format!("Invalid action: {}", s)),
        }
    }
}

impl Action {
    pub fn to_hero_action(&self) -> Self {
        match self {
            Self::Open => Self::Call,
            Self::Limp => Self::Limp,
            Self::Call => Self::Open,
            Self::ThreeBet => Self::ThreeBetCall,
            Self::ThreeBetCall => Self::ThreeBet,
            Self::FourBet => Self::FourBetCall,
            Self::FourBetCall => Self::FourBet,
        }
    }

    pub fn is_after_three_bet(&self) -> bool {
        match self {
            Self::ThreeBetCall | Self::FourBet | Self::FourBetCall => true,
            _ => false,
        }
    }
}
