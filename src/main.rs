use headsup_analyzer::argparse::{arg_parse, SubCommand};
use headsup_analyzer::duel::duel::Duel;
use headsup_analyzer::hand::hand::Hand;

use rust_poker::hand_range::HandRange;
fn main() {
    let opts = arg_parse();
    // println!("{:?}", opts);

    match opts.sub_command {
        SubCommand::Duel(duel) => duel.analyse(),
        SubCommand::Hand(hand) => hand.analyse(),
    }
}
