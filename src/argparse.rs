use std::str::FromStr;

use clap::Clap;
use rust_poker::hand_range::HandRange;

use crate::{
    action::Action, analyse, board::Board, cards::Cards, hand_wrapper::HandWrapper, player::Player,
    position::Position, range,
};
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

#[derive(Clap)]
pub struct Duel {
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

#[derive(Clap)]
pub struct Hand {
    #[clap(name = "YOUR_POSITION")]
    pub hero_position: Position,
    #[clap(name = "YOUR_CARDS")]
    pub hand: String,
}

impl Hand {
    pub fn analyse(&self) {
        let sample_combos = HandRange::from_string(self.hand.clone()).hands;
        // println!("{:?}", sample_combos);
        // let hand_model = rs_poker::core::Hand::new_from_str(&self.hand).unwrap();
        // println!("Position: {:?}", self.hero_position);
        // println!("sample_combo: {:?}", &sample_combo);
        let ip_or_oop = [Position::IP, Position::OOP];
        range::read_ranges()
            .iter()
            .filter(|range| ip_or_oop.contains(&range.me) || self.hero_position == range.me)
            .filter(|range| {
                // println!("{:?}", &range.name);
                // // println!("{:?}", &range.hand_range.hands);
                // range
                //     .hand_range
                //     .hands
                //     .iter()
                //     .for_each(|combo| print!("{}, ", &combo.to_string()));
                // println!("");
                sample_combos
                    .iter()
                    .all(|combo| range.hand_range.hands.contains(&combo))
                // range
                //     .hand_range
                //     .hands
                //     .iter()
                //     .any(|combo| combo == &sample_combo)
            })
            .for_each(|range| println!("{}", range.name));
    }
}

impl Duel {
    pub fn analyse(&self) {
        let (hero, villain) = self.create_hero_and_villain().unwrap();

        let available_cards: Cards = self.create_available_cards().unwrap();
        let madehand = available_cards.analyze_madehand();
        println!("{:?}", madehand);

        let drawhands = available_cards.analyse_drawhand();
        println!("{:?}", drawhands);

        analyse::analyse(hero.hand_range, villain.hand_range, &self.board);
    }

    fn create_hero_and_villain(&self) -> Result<(Player, Player), String> {
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
            (Err(error), _) => Err(error.to_string()),
            (_, Err(error)) => Err(error.to_string()),
        }
    }

    fn create_available_cards(&self) -> Result<Cards, String> {
        Ok(self.hand.cards.clone() + self.board.cards.clone())
    }
}

pub fn arg_parse() -> Opts {
    Opts::parse()
}
