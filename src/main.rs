use headsup_analyzer::argparse::{arg_parse, SubCommand};

fn main() {
    let opts = arg_parse();
    // println!("{:?}", opts);

    match opts.sub_command {
        SubCommand::Duel(duel) => duel.analyse(),
        SubCommand::Hand(hand) => hand.analyse(),
    }
}
