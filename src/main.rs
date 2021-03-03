use headsup_analyzer::{analyse::analyse, argparse::arg_parse, cards::Cards, player::Player};
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
    // println!("{:?}", &villain);
    let public_cards = get_card_mask(&opts.board);
    analyse(hero.hand_range, villain.hand_range, public_cards);
}
