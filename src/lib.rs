pub mod player {
    use crate::{
        action::Action,
        cards::{self, Cards},
        position::Position,
    };
    use anyhow::{bail, Context, Result};
    use rust_poker::{hand_evaluator::Hand, hand_range::HandRange};
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, fs::File, io::BufReader, str::FromStr};

    #[derive(Debug)]
    pub struct Player {
        cards: Cards,
        pub hand_range: HandRange,
        action: Action,
        position: Position,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Patterns {
        patterns: Vec<Pattern>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Pattern {
        position: String,
        villain: String,
        action: String,
        hands: String,
    }

    impl Player {
        pub fn new(
            cards: Cards,
            hero_position: Position,
            villain_positon: Position,
            villain_action: Action,
        ) -> Result<Self> {
            let (hero_position, villain_positon, hero_action) =
                collect_position_and_action(hero_position, villain_positon, villain_action);
            get_hand_range(&hero_position, &villain_positon, &hero_action).and_then(|hand_range| {
                Ok(Self {
                    cards,
                    hand_range,
                    action: hero_action,
                    position: hero_position,
                })
            })
        }
    }

    fn collect_position_and_action(
        hero_position: Position,
        villain_position: Position,
        villain_action: Action,
    ) -> (Position, Position, Action) {
        let hero_action = villain_action.to_hero_action();
        if hero_action.is_after_three_bet() {
            let hero_position = hero_position.convert_to_ip_or_oop(villain_position);
            let villain_position = hero_position.invert();
            return (hero_position, villain_position, hero_action);
        }
        if hero_action.eq(&Action::Open) {
            return (hero_position, Position::NONE, hero_action);
        }
        (hero_position, villain_position, hero_action)
    }

    fn get_hand_range(
        hero_position: &Position,
        villain_positon: &Position,
        hero_action: &Action,
    ) -> Result<HandRange> {
        let json = load_json()?;
        let pattern = json.patterns.into_iter().find(|p| {
            if !Position::from_str(&p.position).unwrap().eq(hero_position) {
                return false;
            }
            if !Position::from_str(&p.villain).unwrap().eq(villain_positon) {
                return false;
            }
            if !Action::from_str(&p.action).unwrap().eq(hero_action) {
                return false;
            }
            true
        });
        match pattern {
            Some(pattern) => Ok(HandRange::from_string(pattern.hands)),
            None => Err(anyhow::anyhow!(format!(
                "can't find HandRange. {:?} vs. {:?} : {:?}",
                hero_position, villain_positon, hero_action
            ))),
        }
    }

    fn load_json() -> Result<Patterns> {
        let file = File::open("range.json").context("can't load json file".to_string())?;
        let reader = BufReader::new(file);
        let deserialized: Patterns =
            serde_json::from_reader(reader).with_context(|| "can't read json file".to_string())?;
        Ok(deserialized)
    }
}

pub mod position {
    use std::str::FromStr;
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub enum Position {
        SB,
        BB,
        UTG,
        MP,
        CO,
        BTN,
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

    impl Position {
        pub fn convert_to_ip_or_oop(&self, villain_positon: Position) -> Position {
            if self > &villain_positon {
                Position::OOP
            } else {
                Position::IP
            }
        }

        pub fn invert(&self) -> Self {
            match self {
                Position::IP => Position::OOP,
                Position::OOP => Position::IP,
                other => other.clone(),
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
}

pub mod cards {
    use rust_poker::hand_range::get_card_mask;
    use std::str::FromStr;
    #[derive(Debug)]
    pub struct Cards(u64);

    impl Cards {
        pub fn new(value: u64) -> Self {
            Self(value)
        }
    }

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
        pub hero_position: Position,
        #[clap(name = "YOUR_CARDS")]
        pub cards: Cards,
        #[clap(name = "VILLAIN_POSITION")]
        pub villain_positon: Position,
        #[clap(name = "VILLAIN_ACTION")]
        pub villain_action: Action,
        #[clap(name = "BOARD")]
        pub board: String,
    }

    pub fn arg_parse() -> Opts {
        let opts = Opts::parse();
        // println!("{:?}", opts.hero_position);
        // println!("{:?}", opts.cards);
        // println!("{:?}", opts.villain_positon);
        // println!("{:?}", opts.villain_action);
        // println!("{:?}", opts.board);
        opts
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
