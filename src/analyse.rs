use crate::board::Board;
use rust_poker::hand_range::{get_card_mask, HandRange};

pub fn analyse(hero_hand_range: HandRange, villain_hand_range: HandRange, board: &Board) {
    let equities = rust_poker::equity_calculator::calc_equity(
        &[hero_hand_range, villain_hand_range].to_vec(),
        get_card_mask(&board.text),
        4,
        1000,
    );
    println!("Win Rate: {:?}", (equities[0] * 100.0).round() / 100.0);
}
