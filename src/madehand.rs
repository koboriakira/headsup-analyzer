#[derive(Debug, PartialEq, PartialOrd)]
pub enum MadeHand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeCard(u8, u8, u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourCard(u8, u8),
    StraightFlush(u8),
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
