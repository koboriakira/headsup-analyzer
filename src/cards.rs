use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use rs_poker::core::{Card, Suit, Value};

use crate::{drawhand::DrawHand, madehand::MadeHand};
#[derive(Debug, Clone)]
pub struct Cards {
    pub cards: Vec<Card>,
    pub text: String,
}

impl std::ops::Add for Cards {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut cards = self.cards.to_owned();
        cards.append(&mut other.cards.clone());
        let mut text = self.text.to_owned();
        text.push_str(&mut other.text.clone());
        Self { cards, text }
    }
}

impl FromStr for Cards {
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
                Ok(Cards {
                    cards,
                    text: s.to_string(),
                })
            })
    }
}

impl Cards {
    pub fn analyze_madehand(&self) -> Result<MadeHand, &'static str> {
        if self.cards.len() < 5 {
            return Err("Invalid cards length");
        }

        self.combination()
            .into_iter()
            .try_fold(vec![], |mut acc, cards| {
                cards.sub_analyze().and_then(|madehand| {
                    acc.push(madehand);
                    Ok(acc)
                })
            })
            .and_then(|madehands| match madehands.into_iter().max() {
                Some(max) => Ok(max),
                None => Err("No madehands."),
            })
    }

    fn sub_analyze(&self) -> Result<MadeHand, &'static str> {
        let suited = self.suited();
        let rank_as_count = self.count_rank();
        let two_card = rank_as_count.get(&2);
        let three_card = rank_as_count.get(&3);
        let four_card = rank_as_count.get(&4);
        let kickers = match rank_as_count.get(&1) {
            Some(kickers) => kickers.clone(),
            None => vec![],
        };
        let straight = self.straight();

        // println!("suited: {:?}", suited);
        // println!("two card: {:?}", two_card);
        // println!("three card: {:?}", three_card);
        // println!("four card: {:?}", four_card);
        // println!("kickers: {:?}", kickers);
        // println!("straight: {:?}", straight);

        let madehand = match (four_card, three_card, two_card, straight, suited) {
            (Some(four_card), _, _, _, _) => MadeHand::FourCard(four_card[0], kickers[0]),
            (None, Some(three_card), Some(two_card), _, _) => {
                MadeHand::FullHouse(three_card[0], two_card[0])
            }
            (None, Some(three_card), None, _, _) => {
                MadeHand::ThreeCard(three_card[0], kickers[0], kickers[1])
            }
            (None, None, Some(two_card), _, _) if two_card.len() == 2 => {
                MadeHand::TwoPair(two_card[0], two_card[1], kickers[0])
            }
            (None, None, Some(two_card), _, _) if two_card.len() == 1 => {
                MadeHand::OnePair(two_card[0], kickers[0], kickers[1], kickers[2])
            }
            (_, _, _, Some(MadeHand::Straight(n)), true) if n == Value::Ace => {
                MadeHand::RoyalStraightFlush
            }
            (_, _, _, Some(MadeHand::Straight(n)), true) => MadeHand::StraightFlush(n),
            (_, _, _, Some(straight), false) => straight,
            (_, _, _, None, true) => MadeHand::Flush(kickers[0]),
            _ => MadeHand::HighCard(kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]),
        };
        Ok(madehand)
    }

    pub fn analyse_drawhand(&self) -> Vec<DrawHand> {
        let rest_card_length = 7 - self.cards.len();
        if rest_card_length == 0 {
            return vec![];
        }

        let init: Vec<DrawHand> = vec![];
        self.combination()
            .into_iter()
            .map(|cards| cards.sub_analyse_drawhand(rest_card_length))
            .fold(init, |mut acc, mut drawhands| {
                acc.append(&mut drawhands);
                acc
            })
    }

    fn sub_analyse_drawhand(&self, rest_card_length: usize) -> Vec<DrawHand> {
        let mut result: Vec<DrawHand> = vec![];
        let flash_draw = self.suits().into_iter().find(|(s, count)| *count == 4);
        let backdoor_flash_draw = self.suits().into_iter().find(|(s, count)| *count == 3);
        match (flash_draw, backdoor_flash_draw) {
            (Some(d), _) => result.push(DrawHand::FlashDraw(d.0)),
            (None, Some(d)) => result.push(DrawHand::BackdoorFlashDraw(d.0)),
            _ => {}
        };
        result
    }

    fn combination(&self) -> Vec<Cards> {
        self.cards
            .clone()
            .into_iter()
            .combinations(5)
            .map(|cards| Cards {
                text: self.text.clone(),
                cards: cards,
            })
            .collect()
    }

    fn suited(&self) -> bool {
        self.suits().keys().len() == 1
    }

    fn suits(&self) -> HashMap<Suit, u32> {
        self.cards
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c.suit).or_insert_with(|| 0) += 1;
                acc
            })
    }

    fn ranks(&self) -> Vec<Value> {
        self.cards
            .clone()
            .into_iter()
            .map(|c| c.value)
            .sorted()
            .collect()
    }

    fn count_rank(&self) -> HashMap<u32, Vec<Value>> {
        let init: HashMap<Value, u32> = HashMap::new();
        let result_init: HashMap<u32, Vec<Value>> = HashMap::new();

        self.ranks()
            .into_iter()
            .fold(init, |mut acc, rank| {
                *acc.entry(rank).or_insert_with(|| 0_u32) += 1;
                acc
            })
            .into_iter()
            .fold(result_init, |mut acc, (rank, count)| {
                acc.entry(count).or_insert_with(|| vec![]).push(rank);
                acc
            })
            .into_iter()
            .map(|(count, ranks)| {
                let sorted_ranks: Vec<Value> = ranks.into_iter().sorted().rev().collect();
                (count, sorted_ranks)
            })
            .collect()
    }

    fn straight(&self) -> Option<MadeHand> {
        let ranks = self.ranks();
        let is_straight = ranks.windows(2).all(|x| is_connect_values(x[0], x[1]));
        if !is_straight {
            return None;
        }
        match (ranks.first(), ranks.last()) {
            (Some(Value::Two), Some(Value::Ace)) => Some(MadeHand::Straight(Value::Five)),
            _ => Some(MadeHand::Straight(*ranks.last().unwrap())),
        }
    }
}

fn is_connect_values(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Ace, Value::Five) | (Value::Five, Value::Ace) => return true,
        (a, b) => (a as i32 - b as i32).abs() == 1,
    }
}
