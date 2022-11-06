/// Possible cards.
#[derive(Debug, PartialEq)]
pub enum Card {
    Tactics,
    Troop(TroopValue, TroopColor),
}

/// Possible colors for a troop card.
#[derive(Debug, PartialEq)]
pub enum TroopColor {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
}

/// Possible values for a troop card, i.e. between 1 and 10.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TroopValue {
    value: u8,
}

impl TroopValue {
    /// Creates a new troop-representing value.
    ///
    /// # Panics
    ///
    /// If [`value`] is not between 1 and 10.
    pub fn new(value: u8) -> TroopValue {
        if value < 1 || value > 10 {
            panic!("Expected a value between 1 and 10, got {value}");
        }
        TroopValue { value }
    }

    /// Returns the value represented, between 1 and 10.
    pub fn value(&self) -> u8 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn troop_value_too_low() {
        TroopValue::new(0);
    }

    #[should_panic]
    #[test]
    fn troop_value_too_high() {
        TroopValue::new(11);
    }

    #[test]
    fn troop_value_just_right() {
        let value = TroopValue::new(5);
        assert_eq!(value.value(), 5);
    }
}
