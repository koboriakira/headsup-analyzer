use headsup_analyzer::{analyse::analyse, argparse::arg_parse, cards::Cards, player::Player};
use rust_poker::hand_range::get_card_mask;
fn main() {
    let opts = arg_parse();
    let hero = Player::new(
        Some(opts.hand.0),
        opts.hero_position.clone(),
        opts.villain_positon.clone(),
        opts.villain_action.clone(),
    )
    .unwrap();
    let villain = Player::new(
        None,
        opts.villain_positon,
        opts.hero_position,
        opts.villain_action.to_hero_action(),
    )
    .unwrap();

    analyse(hero.hand_range, villain.hand_range, &opts.board);
}
