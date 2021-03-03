use std::str::FromStr;

use rs_poker::core::{Card, Suit, Value};

#[derive(Debug)]
pub struct Board {
    pub cards: Vec<Card>,
    pub text: String,
}

impl FromStr for Board {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .into_iter()
            .collect::<Vec<char>>()
            .chunks(2)
            .try_fold(vec![], |mut cards, chars| {
                let card = Value::from_char(chars[0]).and_then(|value| {
                    Suit::from_char(chars[1]).and_then(|suit| Some(Card { value, suit }))
                });
                match card {
                    Some(card) => {
                        cards.push(card);
                        Ok(cards)
                    }
                    None => Err("Can't convert chars to Card"),
                }
            })
            .and_then(|cards| {
                Ok(Board {
                    cards,
                    text: s.to_string(),
                })
            })
    }
}
