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

// #[cfg(test)]
// mod tests {
//     use rs_poker::core::Value;
//     use rust_poker::hand_range::HandRange;

//     use super::MadeHand;
// }
