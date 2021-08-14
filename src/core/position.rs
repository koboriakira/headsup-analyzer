use std::{fmt, str::FromStr};
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Position {
    SB,
    BB,
    UTG,
    MP,
    CO,
    BTN,
    OOP,
    IP,
    NONE,
}

impl Position {
    pub fn from(s: &str) -> Option<Position> {
        match Position::from_str(s) {
            Ok(pos) => Some(pos),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub fn is_none(&self) -> bool {
        self == &Position::NONE
    }
}

impl FromStr for Position {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_uppercase()[..] {
            "UTG" => Ok(Self::UTG),
            "MP" => Ok(Self::MP),
            "CO" => Ok(Self::CO),
            "BTN" => Ok(Self::BTN),
            "SB" => Ok(Self::SB),
            "BB" => Ok(Self::BB),
            "OOP" => Ok(Self::OOP),
            "IP" => Ok(Self::IP),
            "NONE" => Ok(Self::NONE),
            _ => return Err(format!("Invalid position: {}", s)),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &Position::BB => "BB",
            &Position::UTG => "UTG",
            &Position::MP => "MP",
            &Position::CO => "CO",
            &Position::BTN => "BTN",
            &Position::SB => "SB",
            &Position::OOP => "OOP",
            &Position::IP => "IP",
            &Position::NONE => "NONE",
        };
        write!(f, "{}", string)
    }
}

impl Position {
    pub fn convert_to_ip_or_oop(&self, villain_positon: Position) -> Position {
        if self > &villain_positon {
            Position::OOP
        } else {
            Position::IP
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Position::IP => Position::OOP,
            Position::OOP => Position::IP,
            other => other.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Position;

    #[test]
    fn position_test() {
        assert_eq!(Position::from_str("btn"), Ok(Position::BTN));
    }
}
