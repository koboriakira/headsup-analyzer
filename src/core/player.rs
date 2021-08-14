use crate::core::{action::Action, position::Position};
use anyhow::{Context, Result};
use rs_poker::core::Hand;
use rust_poker::hand_range::HandRange;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, str::FromStr};

#[derive(Debug)]
pub struct Player {
    hand: Option<Hand>,
    pub hand_range: HandRange,
    action: Action,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Patterns {
    patterns: Vec<Pattern>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pattern {
    action: String,
    opponent: String,
    me: String,
    hands: String,
    name: String,
}

impl Player {
    pub fn new(
        hand: Option<Hand>,
        hero_position: Position,
        villain_positon: Position,
        villain_action: Action,
    ) -> Result<Self> {
        let (hero_position, villain_positon, hero_action) =
            collect_position_and_action(hero_position, villain_positon, villain_action);
        get_hand_range(&hero_position, &villain_positon, &hero_action).and_then(|hand_range| {
            Ok(Self {
                hand,
                hand_range,
                action: hero_action,
                position: hero_position,
            })
        })
    }
}

fn collect_position_and_action(
    hero_position: Position,
    villain_position: Position,
    villain_action: Action,
) -> (Position, Position, Action) {
    let hero_action = villain_action.to_hero_action();
    if hero_action.is_after_three_bet() {
        let hero_position = hero_position.convert_to_ip_or_oop(villain_position);
        let villain_position = hero_position.invert();
        return (hero_position, villain_position, hero_action);
    }
    if hero_action.eq(&Action::Open) {
        return (hero_position, Position::NONE, hero_action);
    }
    (hero_position, villain_position, hero_action)
}

fn get_hand_range(
    hero_position: &Position,
    villain_positon: &Position,
    hero_action: &Action,
) -> Result<HandRange> {
    let json = load_json()?;
    let pattern = json.patterns.into_iter().find(|p| {
        if !Position::from_str(&p.me).unwrap().eq(hero_position) {
            return false;
        }
        if !Position::from_str(&p.opponent).unwrap().eq(villain_positon) {
            return false;
        }
        if !Action::from_str(&p.action).unwrap().eq(hero_action) {
            return false;
        }
        true
    });
    match pattern {
        Some(pattern) => Ok(HandRange::from_string(pattern.hands)),
        None => Err(anyhow::anyhow!(format!(
            "can't find HandRange. {:?} vs. {:?} : {:?}",
            hero_position, villain_positon, hero_action
        ))),
    }
}

fn load_json() -> Result<Patterns> {
    let file = File::open("range.json").context("can't load json file".to_string())?;
    let reader = BufReader::new(file);
    let deserialized: Patterns =
        serde_json::from_reader(reader).with_context(|| "can't read json file".to_string())?;
    Ok(deserialized)
}
