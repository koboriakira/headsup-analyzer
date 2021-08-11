use std::str::FromStr;

use clap::Clap;
use rs_poker::core::Hand;

use crate::{action::Action, board::Board, cards::Cards, player::Player, position::Position};
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
    pub hand: HandWrapper,
    #[clap(name = "VILLAIN_POSITION")]
    pub villain_positon: Position,
    #[clap(name = "VILLAIN_ACTION")]
    pub villain_action: Action,
    #[clap(name = "BOARD")]
    pub board: Board,
}

impl Opts {
    pub fn create_hero_and_villain(&self) -> Result<(Player, Player), String> {
        let hero = Player::new(
            Some(self.hand.hand.clone()),
            self.hero_position.clone(),
            self.villain_positon.clone(),
            self.villain_action.clone(),
        );
        let villain = Player::new(
            None,
            self.villain_positon,
            self.hero_position,
            self.villain_action.to_hero_action(),
        );
        match (hero, villain) {
            (Ok(hero), Ok(villain)) => Ok((hero, villain)),
            (_, _) => Err(String::from("Can't create Player model.")),
        }
    }

    pub fn create_available_cards(&self) -> Result<Cards, ()> {
        Ok(self.hand.cards.clone() + self.board.cards.clone())
    }
}

#[derive(Debug)]
pub struct HandWrapper {
    pub hand: Hand,
    pub cards: Cards,
}

impl FromStr for HandWrapper {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = Cards::from_str(s);
        match cards {
            Err(message) => Err(message.to_string()),
            Ok(cards) => Hand::new_from_str(s).and_then(|hand| {
                let hand_wrapper = HandWrapper { hand, cards };
                Ok(hand_wrapper)
            }),
        }
    }
}

pub fn arg_parse() -> Opts {
    Opts::parse()
}
