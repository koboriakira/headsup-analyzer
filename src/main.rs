use std::process::exit;

use headsup_analyzer::{
    analyse::analyse,
    argparse::{arg_parse, Hand, SubCommand},
    cards::Cards,
    position::Position,
    range,
};

use rust_poker::hand_range::HandRange;
fn main() {
    let opts = arg_parse();
    // println!("{:?}", opts);

    match opts.sub_command {
        SubCommand::Duel(duel) => duel.analyse(),
        SubCommand::Hand(hand) => {
            println!("{}", "input below.");
            loop {
                println!("================================================");
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).ok();
                let vars = args.trim().split(' ').collect::<Vec<_>>();
                if vars[0] == "exit" {
                    exit(0);
                } else if vars.len() != 2 {
                    println!("Invalid args.");
                } else {
                    let position = Position::from(&vars[0]);
                    if position.is_ok() {
                        hand_analyse(position.unwrap(), vars[1].to_string());
                    } else {
                        println!("Invalid args.");
                    }
                }
            }
        }
    }
}

fn hand_analyse(position: Position, hand: String) -> () {
    let sample_combos = HandRange::from_string(hand.clone()).hands;
    // println!("{:?}", sample_combos);
    // let hand_model = rs_poker::core::Hand::new_from_str(&self.hand).unwrap();
    // println!("Position: {:?}", self.hero_position);
    // println!("sample_combo: {:?}", &sample_combo);
    let ip_or_oop = [Position::IP, Position::OOP];
    range::read_ranges()
        .iter()
        .filter(|range| ip_or_oop.contains(&range.me) || position == range.me)
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
