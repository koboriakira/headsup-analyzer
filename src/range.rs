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
    pub me: Position,
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
    // let json_model = read_json_file();
    let json_model = read_json_data();
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

fn read_json_data() -> JsonModel {
    let data = r#"
{
  "patterns": [
    {
      "name": "UTGオープン",
      "action": "open",
      "me": "utg",
      "opponent": "none",
      "hands": "ATo+,A8s+,A4s,A5s,77+,KJo+,K9s+,QTs+,JTs"
    },
    {
      "name": "MPオープン",
      "action": "open",
      "me": "mp",
      "opponent": "none",
      "hands": "ATo+,A2s+,66+,KJo+,K8s+,QJo+,Q9s+,J9s+,T9s+"
    },
    {
      "name": "COオープン",
      "action": "open",
      "me": "co",
      "opponent": "none",
      "hands": "A9o+,A2s+,44+,KTo+,K6s+,QTo+,Q9s+,J9s+,JTo,T8s+,98s,87s,76s,65s"
    },
    {
      "name": "BTNオープン",
      "action": "open",
      "me": "btn",
      "opponent": "none",
      "hands": "22+,A2s+,A4o+,K2s+,K9o+,Q6s+,QTo+,J7s+,J9o+,T7s+,T9o+,97s+,86s+,75s+,64s+,54s+"
    },
    {
      "name": "SBオープン",
      "action": "open",
      "me": "sb",
      "opponent": "none",
      "hands": "22+,A2s+,A3o+,K2s+,K8o+,Q2s+,Q9o+,J5s+,J9o+,T6s+,T8o+,96s+,98o+,85s+,74s+,64s+,53s+,43s+"
    },
    {
      "name": "MPコール vs UTG",
      "action": "call",
      "me": "mp",
      "opponent": "utg",
      "hands": "JJ,TT,99,AQs,AJs,KQs"
    },
    {
      "name": "COコール vs UTG(EP)",
      "action": "call",
      "me": "co",
      "opponent": "utg",
      "hands": "JJ,TT,99,AQs,AJs,KQs"
    },
    {
      "name": "COコール vs MP(EP)",
      "action": "call",
      "me": "co",
      "opponent": "mp",
      "hands": "JJ,99,AQs,AJs,KQs"
    },
    {
      "name": "BTNコール vs UTG(EP)",
      "action": "call",
      "me": "btn",
      "opponent": "utg",
      "hands": "TT,99,88,AQs,AJs,ATs,AQo,AJo,KQs,KJs,KQo,QJs,JTs"
    },
    {
      "name": "BTNコール vs MP(EP)",
      "action": "call",
      "me": "btn",
      "opponent": "mp",
      "hands": "TT,99,88,AQs,AJs,ATs,AQo,AJo,KQs,KJs,KQo,QJs,JTs"
    },
    {
      "name": "BTNコール vs CO",
      "action": "call",
      "me": "btn",
      "opponent": "co",
      "hands": "99,88,77,AJs,ATs,AQo,QJs,JTs"
    },
    {
      "name": "SBコール vs UTG(EP)",
      "action": "call",
      "me": "sb",
      "opponent": "utg",
      "hands": "99,88,77,KJs"
    },
    {
      "name": "SBコール vs MP(EP)",
      "action": "call",
      "me": "sb",
      "opponent": "mp",
      "hands": "99,88,77,KJs"
    },
    {
      "name": "BBコール vs UTG(EP)",
      "action": "call",
      "me": "bb",
      "opponent": "utg",
      "hands": "JJ,TT,99,88,77,66,55,44,33,22,AQs,AJs,ATs,A9s,A8s,A7s,A6s,A5s,A4s,A3s,A2s,AQo,AJo,ATo,K9s+,KQo,Q9s+,J9s+,T8s+,97s+,86s+,75s+,64s+,54s"
    },
    {
      "name": "BBコール vs MP(EP)",
      "action": "call",
      "me": "bb",
      "opponent": "mp",
      "hands": "JJ,TT,99,88,77,66,55,44,33,22,AQs,AJs,ATs,A9s,A8s,A7s,A6s,A5s,A4s,A3s,A2s,AQo,AJo,ATo,K9s+,KQo,Q9s+,J9s+,T8s+,97s+,86s+,75s+,64s+,54s"
    },
    {
      "name": "BBコール vs CO",
      "action": "call",
      "me": "bb",
      "opponent": "co",
      "hands": "TT,99,88,77,66,55,44,33,22,AJs,ATs,A9s,A8s,A7s,A6s,A3s,A2s,AQo,AJo,ATo,K7s+,KJo+,Q8s+,QJo,J8s+,T8s+,97s+,86s+,75s+,64s+,54s"
    },
    {
      "name": "BBコール vs BTN",
      "action": "call",
      "me": "bb",
      "opponent": "btn",
      "hands": "77,66,55,44,33,22,A9s,A8s,A7s,A6s,AQo,AJo,ATo,A9o,K5s+,KT+o,Q7s+,QTo+,J8s+,JTo,T7s+,97s+,86s+,75s+,64s+,54s,43s"
    },
    {
      "name": "BBコール vs SB",
      "action": "call",
      "me": "bb",
      "opponent": "sb",
      "hands": "77,66,55,44,33,22,A9s,A8s,A7s,A6s,A5s,A4s,A3s,A2s,AJo,ATo,K9s,K8s,K7s,K6s,K5s,K4s,K3s,K2s,K7o+,QTs,Q9s,Q8s,Q7s,Q6s,Q5s,Q4s,Q3s,Q2s,Q8o+,J9s,J8s,J7s,J6s,J5s,J4s,J3s,J8o+,T8s,T7s,T6s,T5s,T4s,T8o+,94s+,97o+,84s+,87o,74s+,63s+,52s+,42s+,32s"
    },
    {
      "name": "MP3bet vs UTG",
      "action": "3bet",
      "me": "mp",
      "opponent": "utg",
      "hands": "AA,KK,QQ,ATs+,A5s,A4s,AQo+,KQs,KJs"
    },
    {
      "name": "CO3bet vs UTG(EP)",
      "action": "3bet",
      "me": "co",
      "opponent": "utg",
      "hands": "AA,KK,QQ,ATs+,A5s,A4s,AQo+,KJs,65s,54s"
    },
    {
      "name": "CO3bet vs MP(EP)",
      "action": "3bet",
      "me": "co",
      "opponent": "mp",
      "hands": "AA,KK,QQ,ATs+,A5s,A4s,AQo+,KJs,65s,54s"
    },
    {
      "name": "BTN3bet vs UTG(EP)",
      "action": "3bet",
      "me": "btn",
      "opponent": "utg",
      "hands": "JJ+,AKs,A5s,A4s,AKo,AJo,KTs,KQo,76s,65s,54s"
    },
    {
      "name": "BTN3bet vs MP(EP)",
      "action": "3bet",
      "me": "btn",
      "opponent": "mp",
      "hands": "JJ+,AKs,A5s,A4s,AKo,AJo,KTs,KQo,76s,65s,54s"
    },
    {
      "name": "BTN3bet vs CO",
      "action": "3bet",
      "me": "btn",
      "opponent": "co",
      "hands": "TT+,AQs+,A9s,A8s,A5s,A4s,A3s,A2s,AKo,AJo,KTs,KQo,76s,65s,54s"
    },
    {
      "name": "SB3bet vs UTG(EP)",
      "action": "3bet",
      "me": "sb",
      "opponent": "utg",
      "hands": "TT+,ATs+,A5s,A4s,AQo+,KQs"
    },
    {
      "name": "SB3bet vs MP(EP)",
      "action": "3bet",
      "me": "sb",
      "opponent": "mp",
      "hands": "TT+,ATs+,A5s,A4s,AQo+,KQs"
    },
    {
      "name": "SB3bet vs CO",
      "action": "3bet",
      "me": "sb",
      "opponent": "co",
      "hands": "66+,A8s+,A5s,A4s,A3s,AJo+,KTs+,KQo"
    },
    {
      "name": "SB3bet vs BTN",
      "action": "3bet",
      "me": "sb",
      "opponent": "btn",
      "hands": "55+,A2s+,ATo+,K9s+,KJo+,QTs+,QJo+,J9s+,T9s+"
    },
    {
      "name": "BB3bet vs UTG(EP)",
      "action": "3bet",
      "me": "bb",
      "opponent": "utg",
      "hands": "QQ+,AKs,ATs,A5s,A4s,AKo,ATo,K9s"
    },
    {
      "name": "BB3bet vs MP(EP)",
      "action": "3bet",
      "me": "bb",
      "opponent": "mp",
      "hands": "QQ+,AKs,ATs,A5s,A4s,AKo,ATo,K9s"
    },
    {
      "name": "BB3bet vs CO",
      "action": "3bet",
      "me": "bb",
      "opponent": "co",
      "hands": "JJ+,AQs+,A5s,A4s,AKo,A5o,K6s,K5s,K4s,KTo,Q7s,Q6s"
    },
    {
      "name": "BB3bet vs BTN",
      "action": "3bet",
      "me": "bb",
      "opponent": "btn",
      "hands": "88+,ATs+,A5s,A4s,AQo+,A8o,A5o,KJs+,K4s,K3s,K2s,K9o,QJs+,JTs+,J7s,T9o"
    },
    {
      "name": "BB3bet vs SB",
      "action": "3bet",
      "me": "bb",
      "opponent": "sb",
      "hands": "88+,ATs+,AQo+,A5o,A4o,KTs+,QJs,JTs,J2s,T9s,T3s,T2s,93s,83s,73s,72s,62s"
    },
    {
      "name": "IP3betコール",
      "action": "3betcall",
      "me": "ip",
      "opponent": "oop",
      "hands": "QQ,JJ,TT,99,88,77,66,55,A9s+,A5s,A4s,AQo+,KTs+,QTs+,JTs,T9s,76s,65s,54s"
    },
    {
      "name": "IP3betコール(nit)",
      "action": "3betcall",
      "me": "ip",
      "opponent": "oop",
      "option": "nit",
      "hands": "KK,QQ,JJ,TT,99,88,77,66,AKo,ATs+,KQs,QJs,JTs"
    },
    {
      "name": "OOP3betコール",
      "action": "3betcall",
      "me": "oop",
      "opponent": "ip",
      "hands": "QQ,JJ,TT,99,88,77,66,AQs,ATs,AKo,KJs,QJs,JTs"
    },
    {
      "name": "OOP3betコール(nit)",
      "action": "3betcall",
      "me": "oop",
      "opponent": "ip",
      "option": "nit",
      "hands": "QQ,JJ,AKo"
    },
    {
      "name": "IP4bet",
      "action": "4bet",
      "me": "ip",
      "opponent": "oop",
      "hands": "KK+,A7s,A6s,K9s"
    },
    {
      "name": "IP4bet(nit)",
      "action": "4bet",
      "me": "ip",
      "opponent": "oop",
      "option": "nit",
      "hands": "AA,A5s"
    },
    {
      "name": "OOP4bet",
      "action": "4bet",
      "me": "oop",
      "opponent": "ip",
      "option": "nit",
      "hands": "KK+,AKs,AJs,KQs"
    },
    {
      "name": "OOP4bet(nit)",
      "action": "4bet",
      "me": "oop",
      "opponent": "ip",
      "option": "nit",
      "hands": "KK+,AQs+"
    },
    {
      "name": "IP4betコール",
      "action": "4betcall",
      "me": "ip",
      "opponent": "oop",
      "hands": "KK+"
    },
    {
      "name": "IP4betコール(nit)",
      "action": "4betcall",
      "me": "ip",
      "opponent": "oop",
      "option": "nit",
      "hands": "AA"
    },
    {
      "name": "OOP4betコール",
      "action": "4betcall",
      "me": "oop",
      "opponent": "ip",
      "hands": "KK+,AKs"
    },
    {
      "name": "OOP4betコール(nit)",
      "action": "4betcall",
      "me": "oop",
      "opponent": "ip",
      "option": "nit",
      "hands": "KK+,AKs"
    },
    {
      "name": "リンパー",
      "action": "limp",
      "me": "none",
      "opponent": "none",
      "hands": "22+,A2s+,KJ+,QJ+,T9s,98s,87s,76s,65s,54s"
    }
  ]
}
    "#;
    serde_json::from_str(data).ok().unwrap()
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
