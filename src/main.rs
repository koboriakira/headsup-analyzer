use std::fmt::Debug;

use headsup_analyzer::{argparse::arg_parse, cards::Cards, player::Player};
use rust_poker::hand_range::get_card_mask;
fn main() {
    let opts = arg_parse();
    let hero = Player::new(
        opts.cards,
        opts.hero_position.clone(),
        opts.villain_positon.clone(),
        opts.villain_action.clone(),
    )
    .unwrap();
    let villain = Player::new(
        Cards::new(0),
        opts.villain_positon,
        opts.hero_position,
        opts.villain_action.to_hero_action(),
    )
    .unwrap();
    println!("{}", &hero.hand_range.hands);
    let test_combo = &hero.hand_range.hands.first().unwrap();
    println!("{}", test_combo);
    println!("{:?}", test_combo);
    // println!("{:?}", &villain);
    let public_cards = get_card_mask(&opts.board);
    let equities = rust_poker::equity_calculator::calc_equity(
        &[hero.hand_range, villain.hand_range].to_vec(),
        public_cards,
        4,
        1000,
    );
    println!("Win Rate: {:?}", (equities[0] * 100.0).round() / 100.0);
}
