use rs_poker::core::{Suit, Value};

pub enum DrawHand {
    TwoOver(Value, Value),
    OneOver(Value),
    FlashDraw(Suit),
    BackdoorFlashDraw(Suit),
    OpenendStraightDraw(Value),
    GutshotStraightDraw(Value),
    BackDoorStraightDraw(Value, Value),
}
