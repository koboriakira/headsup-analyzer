use crate::core::{
    action::Action, board::Board, cards::Cards, hand_wrapper::HandWrapper, player::Player,
    position::Position,
};
use rust_poker::hand_range::{get_card_mask, HandRange};

use clap::Clap;

#[derive(Clap)]
pub struct Duel {
    #[clap(name = "YOUR_POSITION")]
    pub hero_position: Position,
    #[clap(name = "YOUR_CARDS")]
    pub hand: HandWrapper,
    #[clap(name = "VILLAIN_POSITION")]
    pub villain_positon: Position,
    #[clap(name = "VILLAIN_ACTION")]
    pub villain_action: Action,
    #[clap(name = "BOARD")]
    pub board: Board,
}

impl Duel {
    pub fn analyse(&self) {
        let (hero, villain) = self.create_hero_and_villain().unwrap();

        let available_cards: Cards = self.create_available_cards().unwrap();
        let madehand = available_cards.analyze_madehand();
        println!("{:?}", madehand);

        let drawhands = available_cards.analyse_drawhand();
        println!("{:?}", drawhands);

        analyse(hero.hand_range, villain.hand_range, &self.board);
    }

    fn create_hero_and_villain(&self) -> Result<(Player, Player), String> {
        let hero = Player::new(
            Some(self.hand.hand.clone()),
            self.hero_position.clone(),
            self.villain_positon.clone(),
            self.villain_action.clone(),
        );
        let villain = Player::new(
            None,
            self.villain_positon,
            self.hero_position,
            self.villain_action.to_hero_action(),
        );
        match (hero, villain) {
            (Ok(hero), Ok(villain)) => Ok((hero, villain)),
            (Err(error), _) => Err(error.to_string()),
            (_, Err(error)) => Err(error.to_string()),
        }
    }

    fn create_available_cards(&self) -> Result<Cards, String> {
        Ok(self.hand.cards.clone() + self.board.cards.clone())
    }
}

fn analyse(hero_hand_range: HandRange, villain_hand_range: HandRange, board: &Board) {
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
