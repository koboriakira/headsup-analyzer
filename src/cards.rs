use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use rs_poker::core::{Card, Suit, Value};

use crate::madehand::MadeHand;
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
        if self.cards.len() != 5 {
            return Err("Invalid cards length");
        }

        let suited = self.suits().len() == 1;
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

    fn suits(&self) -> HashSet<Suit> {
        self.cards
            .clone()
            .into_iter()
            .map(|c| c.suit)
            .collect::<HashSet<Suit>>()
    }

    fn ranks(&self) -> Vec<Value> {
        self.cards.clone().into_iter().map(|c| c.value).collect()
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
        let mut lowest = false;
        let result = !ranks.windows(2).any(|x| match x[0] as i32 - x[1] as i32 {
            // In the case 'A' and '5'
            9 => {
                lowest = true;
                false
            }
            1 => false,
            _ => true,
        });
        match (result, lowest) {
            (true, true) => Some(MadeHand::Straight(Value::Five)),
            (true, false) => Some(MadeHand::Straight(ranks[0])),
            _ => None,
        }
    }
}
