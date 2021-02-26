pub mod position {
    use std::str::FromStr;
    #[derive(Debug)]
    pub enum Position {
        UTG,
        MP,
        CO,
        BTN,
        SB,
        BB,
        OOP,
        IP,
        NONE,
    }

    impl FromStr for Position {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match &s.to_ascii_uppercase()[..] {
                "UTG" => Ok(Self::UTG),
                "MP" => Ok(Self::MP),
                "CO" => Ok(Self::CO),
                "BTN" => Ok(Self::BTN),
                "SB" => Ok(Self::SB),
                "BB" => Ok(Self::BB),
                "OOP" => Ok(Self::OOP),
                "IP" => Ok(Self::IP),
                "NONE" => Ok(Self::NONE),
                _ => return Err(format!("Invalid position: {}", s)),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;

        use super::Position;

        #[test]
        fn position_test() {
            assert_eq!(Position::from_str("btn"), Ok(Position::BTN));
        }
    }
}

pub mod action {
    use std::str::FromStr;
    #[derive(Debug)]
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
                "THREEBET" => Ok(Self::ThreeBet),
                "THREEBETCALL" => Ok(Self::ThreeBetCall),
                "FOURBET" => Ok(Self::FourBet),
                "FOURBETCALL" => Ok(Self::FourBetCall),
                _ => return Err(format!("Invalid action: {}", s)),
            }
        }
    }
}

pub mod cards {
    use rust_poker::hand_range::get_card_mask;
    use std::str::FromStr;
    #[derive(Debug)]
    pub struct Cards(u64);

    impl FromStr for Cards {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards = get_card_mask(s);
            Ok(Cards(cards))
        }
    }
}

pub mod argparse {
    use clap::Clap;
    use rust_poker::hand_range::get_card_mask;

    use crate::{action::Action, cards::Cards, position::Position};
    #[derive(Clap, Debug)]
    #[clap(
        name = "Heads-Up Analyzer",
        version = "1.0.0",
        author = "Kobori Akira",
        about = "Analyze Heads-Up of poker"
    )]
    pub struct Opts {
        #[clap(name = "YOUR_POSITION")]
        hero_position: Position,
        #[clap(name = "YOUR_CARDS")]
        hero_cards: Cards,
        #[clap(name = "VILLAIN_POSITION")]
        villain_positon: Position,
        #[clap(name = "VILLAIN_ACTION")]
        villain_action: Action,
        #[clap(name = "BOARD")]
        board: Cards,
    }

    pub fn arg_parse() {
        let opts = Opts::parse();
        println!("{:?}", opts.hero_position);
        println!("{:?}", opts.hero_cards);
        println!("{:?}", opts.villain_positon);
        println!("{:?}", opts.villain_action);
        println!("{:?}", opts.board);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
