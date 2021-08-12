use rs_poker::core::{Suit, Value};

use crate::straight;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum DrawHand {
    TwoOver(Value, Value),
    OneOver(Value),
    FlashDraw(Suit),
    BackdoorFlashDraw(Suit),
    StraightDraw(Value),
    BackDoorStraightDraw(Value, Value),
}

pub fn calculate_straight_draws(values: Vec<Value>) -> Vec<DrawHand> {
    let straight_draws = straight::get_straight_patterns()
        .into_iter()
        .map(|pattern| {
            let (matched, unmatched): (Vec<Value>, Vec<Value>) = pattern
                .into_iter()
                .partition(|value| values.contains(value));
            match matched.len() {
                4 => Some(DrawHand::StraightDraw(unmatched[0])),
                3 => Some(DrawHand::BackDoorStraightDraw(unmatched[0], unmatched[1])),
                _ => None,
            }
        })
        .filter_map(|el| el)
        .collect::<Vec<_>>();
    let straight_draw: Vec<DrawHand> = straight_draws
        .clone()
        .into_iter()
        .filter(|&draw_hand| match draw_hand {
            DrawHand::StraightDraw(n) => true,
            _ => false,
        })
        .collect();
    if straight_draw.len() > 0 {
        return straight_draw;
    }
    let back_door_straight_draw: Vec<DrawHand> = straight_draws
        .into_iter()
        .filter(|&draw_hand| match draw_hand {
            DrawHand::BackDoorStraightDraw(n, m) => true,
            _ => false,
        })
        .collect();
    if back_door_straight_draw.len() > 0 {
        return back_door_straight_draw;
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::cards::Cards;

    use super::*;

    #[test]
    fn test_openend_straight_draws() {
        let values = Cards::from_str("4s5cad6h7c").ok().unwrap().values();

        let actual = calculate_straight_draws(values);

        let straight_draw_three = DrawHand::StraightDraw(Value::Three);
        let straight_draw_eight = DrawHand::StraightDraw(Value::Eight);

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], straight_draw_three);
        assert_eq!(actual[1], straight_draw_eight);
    }

    #[test]
    fn test_gutshot_straight_draws() {
        let values = Cards::from_str("2hQh9sJcKd").ok().unwrap().values();

        let actual = calculate_straight_draws(values);

        let straight_draw = DrawHand::StraightDraw(Value::Ten);

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], straight_draw);
    }

    #[test]
    fn test_backdoor_straight_draws() {
        let values = Cards::from_str("2h3h9sJcKd").ok().unwrap().values();

        let actual = calculate_straight_draws(values);

        let back_door_straight_draw = DrawHand::BackDoorStraightDraw(Value::Ten, Value::Queen);

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], back_door_straight_draw);
    }
}
