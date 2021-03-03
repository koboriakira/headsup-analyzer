use headsup_analyzer::{analyse::analyse, argparse::arg_parse, cards::Cards, player::Player};
fn main() {
    let opts = arg_parse();
    let hero = Player::new(
        Some(opts.hand.hand),
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

    let available_cards: Cards = opts.hand.cards + opts.board.cards.clone();
    let madehand = available_cards.analyze_madehand();
    println!("{:?}", madehand);

    analyse(hero.hand_range, villain.hand_range, &opts.board);
}
