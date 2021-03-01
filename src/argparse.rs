use clap::Clap;

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
