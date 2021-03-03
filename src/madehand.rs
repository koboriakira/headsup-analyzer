use rs_poker::core::Value;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MadeHand {
    HighCard(Value, Value, Value, Value, Value),
    OnePair(Value, Value, Value, Value),
    TwoPair(Value, Value, Value),
    ThreeCard(Value, Value, Value),
    Straight(Value),
    Flush(Value),
    FullHouse(Value, Value),
    FourCard(Value, Value),
    StraightFlush(Value),
    RoyalStraightFlush,
}

#[cfg(test)]
mod tests {
    use rust_poker::hand_range::HandRange;

    use super::MadeHand;

    #[test]
    fn test_madehand() {
        assert_eq!(
            MadeHand::HighCard(0, 0, 0, 0, 0),
            MadeHand::HighCard(0, 0, 0, 0, 0)
        );
    }
}
