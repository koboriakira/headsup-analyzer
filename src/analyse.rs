use crate::board::Board;
use rust_poker::hand_range::{get_card_mask, HandRange};

pub fn analyse(hero_hand_range: HandRange, villain_hand_range: HandRange, board: &Board) {
    let result = rust_poker::equity_calculator::approx_equity(
        &[hero_hand_range, villain_hand_range].to_vec(),
        get_card_mask(&board.cards.text),
        4,
        1000.0,
    );
    match result {
        Ok(equities) => println!("Win Rate: {:?}", (equities[0] * 100.0).round() / 100.0),
        Err(err) => println!("{}", err),
    }
}
