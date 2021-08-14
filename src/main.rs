use std::process::exit;

use headsup_analyzer::{
    analyse::analyse,
    argparse::{arg_parse, Hand, SubCommand},
    cards::Cards,
    hand,
    position::Position,
    range,
};

use rust_poker::hand_range::HandRange;
fn main() {
    let opts = arg_parse();
    // println!("{:?}", opts);

    match opts.sub_command {
        SubCommand::Duel(duel) => duel.analyse(),
        SubCommand::Hand(hand) => hand::analyse::execute(),
    }
}
