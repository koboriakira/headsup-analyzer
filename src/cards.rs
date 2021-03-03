use rs_poker::core::Hand;
use rust_poker::hand_range::get_card_mask;
use std::str::FromStr;
#[derive(Debug)]
pub struct Cards(u64);

impl Cards {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Cards {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = get_card_mask(s);
        Ok(Cards(cards))
    }
}
