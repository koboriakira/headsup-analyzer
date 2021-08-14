use std::str::FromStr;

use rs_poker::core::Hand;

use crate::core::cards::Cards;

#[derive(Debug, Clone)]
pub struct HandWrapper {
    pub hand: Hand,
    pub cards: Cards,
}

impl FromStr for HandWrapper {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = Cards::from_str(s);
        match cards {
            Err(message) => Err(message.to_string()),
            Ok(cards) => Hand::new_from_str(s).and_then(|hand| {
                let hand_wrapper = HandWrapper { hand, cards };
                Ok(hand_wrapper)
            }),
        }
    }
}
