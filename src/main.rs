use headsup_analyzer::{analyse::analyse, argparse::arg_parse, cards::Cards};
fn main() {
    let opts = arg_parse();
    let (hero, villain) = opts.create_hero_and_villain().unwrap();

    let available_cards: Cards = opts.create_available_cards().unwrap();
    let madehand = available_cards.analyze_madehand();
    println!("{:?}", madehand);

    let drawhands = available_cards.analyse_drawhand();
    println!("{:?}", drawhands);

    analyse(hero.hand_range, villain.hand_range, &opts.board);
}
