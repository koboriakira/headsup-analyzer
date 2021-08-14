use std::str::FromStr;

use clap::Clap;
use rust_poker::hand_range::HandRange;

use crate::core::{
    action::Action, board::Board, cards::Cards, hand_wrapper::HandWrapper, player, player::Player,
    position::Position, range,
};
use crate::duel::duel::Duel;
use crate::hand::hand::Hand;
#[derive(Clap)]
#[clap(
    name = "Heads-Up Analyzer",
    version = "1.0.0",
    author = "Kobori Akira",
    about = "Analyze Heads-Up of poker"
)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "1.0")]
    Hand(Hand),
    #[clap(version = "1.0")]
    Duel(Duel),
}

pub fn arg_parse() -> Opts {
    Opts::parse()
}
