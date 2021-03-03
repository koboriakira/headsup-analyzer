use rust_poker::hand_range::HandRange;
pub fn analyse(hero_hand_range: HandRange, villain_hand_range: HandRange, public_cards: u64) {
    // println!("{}", "villain_combos");
    // let villain_combos = villain_hand_range
    //     .hands
    //     .clone()
    //     .into_iter()
    //     .map(|combo| combo.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");
    // println!("{}", villain_combos);
    let equities = rust_poker::equity_calculator::calc_equity(
        &[hero_hand_range, villain_hand_range].to_vec(),
        public_cards,
        4,
        1000,
    );
    println!("Win Rate: {:?}", (equities[0] * 100.0).round() / 100.0);
}
