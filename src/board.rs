use std::str::FromStr;

use crate::cards::Cards;

#[derive(Debug, Clone)]
pub struct Board {
    pub cards: Cards,
}

impl FromStr for Board {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cards::from_str(s).and_then(|cards| Ok(Board { cards }))
    }
}
