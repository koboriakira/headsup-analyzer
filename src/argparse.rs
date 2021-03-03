use std::str::FromStr;

use clap::Clap;
use rs_poker::core::Hand;

use crate::{action::Action, board::Board, position::Position};
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

#[derive(Debug)]
pub struct HandWrapper(pub Hand);

impl FromStr for HandWrapper {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hand::new_from_str(s).and_then(|hand| Ok(HandWrapper(hand)))
    }
}

pub fn arg_parse() -> Opts {
    Opts::parse()
}
