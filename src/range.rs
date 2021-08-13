use itertools::Itertools;
use rs_poker::core::Hand;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::BufReader, str::FromStr};

use rust_poker::hand_range::HandRange;

use crate::{action::Action, position::Position};

#[derive(Debug)]
pub struct Range {
    pub name: String,
    action: Action,
    me: Position,
    opponent: Position,
    pub hand_range: HandRange,
    pub hands: Vec<Hand>,
}

impl Range {}

#[derive(Serialize, Deserialize)]
struct JsonModel {
    patterns: Vec<Pattern>,
}

#[derive(Serialize, Deserialize)]
struct Pattern {
    name: String,
    action: String,
    me: String,
    opponent: String,
    hands: String,
}

impl Pattern {
    fn to_range(&self) -> Range {
        let action = Action::from_str(&self.action);
        let me = Position::from_str(&self.me);
        let opponent = Position::from_str(&self.opponent);
        let hand_range = HandRange::from_string(self.hands.clone());
        let hands: Vec<Hand> = hand_range
            .hands
            .clone()
            .into_iter()
            .map(|combo| Hand::new_from_str(&combo.to_string()).unwrap())
            .collect();
        match (action, me, opponent, hand_range) {
            (Ok(action), Ok(me), Ok(opponent), hand_range) => Range {
                name: self.name.clone(),
                action: action,
                me: me,
                opponent: opponent,
                hand_range: hand_range,
                hands: hands,
            },
            (_, _, _, _) => panic!("invalid pattern"),
        }
    }
}

pub fn read_ranges() -> Vec<Range> {
    let json_model = read_json_file();
    json_model
        .patterns
        .into_iter()
        .map(|pattern| pattern.to_range())
        .collect()
}

fn read_json_file() -> JsonModel {
    let file = File::open("range.json").unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader);

    // Return the `User`.
    u.ok().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_json_file() {
        read_json_file();
    }

    #[test]
    fn test_read_ranges() {
        let ranges = read_ranges();
        println!("{:#?}", ranges[0].hand_range);
        assert_eq!(ranges.len(), 46);
    }
}
