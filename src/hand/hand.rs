use clap::Clap;
use std::process::exit;

use rust_poker::hand_range::HandRange;

use crate::core::{position::Position, range};

#[derive(Clap)]
pub struct Hand {}

impl Hand {
    pub fn analyse(&self) -> () {
        println!("{}", "input below.");
        loop {
            println!("================================================");
            let mut args = String::new();
            std::io::stdin().read_line(&mut args).ok();
            let vars = args.trim().split(' ').collect::<Vec<_>>();
            if vars[0] == "exit" {
                exit(0);
            } else {
                match vars.len() {
                    1 | 2 => {
                        let position = vars.get(1).and_then(|var| Position::from(var));
                        hand_analyse(position, vars[0].to_string());
                    }
                    _ => println!("Invalid args."),
                }
            }
        }
    }
}
fn hand_analyse(position: Option<Position>, hand: String) -> () {
    let sample_combos = HandRange::from_string(hand.clone()).hands;

    range::read_ranges()
        .iter()
        .filter(|range| match position {
            None => true,
            Some(position) => range.is_ip_or_oop() || range.equals_me(position),
        })
        .filter(|range| {
            sample_combos
                .iter()
                .all(|combo| range.contains_combo(combo))
        })
        .for_each(|range| println!("{}", range.to_string(None)));
}
