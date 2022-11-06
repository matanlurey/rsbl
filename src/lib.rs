//! Game logic and data structures.

/// The field of cards, i.e. seven flags with cards being played at each.
pub struct Field {
    columns: Vec<Column>,
}

impl Field {
    pub fn new() -> Field {
        let mut columns: Vec<Column> = Vec::with_capacity(7);

        for _ in 0..7 {
            columns.push(Column {
                flag: Flag::Unclaimed,
                formations: [Formation::new(), Formation::new()],
            })
        }

        Field { columns }
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn add_card(&mut self, column: usize, side: Side, troop: Card) {
        let column = &mut self.columns[column];
        let formation = if let Side::North = side {
            &mut column.formations[0]
        } else {
            &mut column.formations[1]
        };
        formation.0.push(troop);
    }
}

/// Which player, the north or south player.
pub enum Side {
    North,
    South,
}

/// A column of cards, i.e. a flag with cards being played for each player.
pub struct Column {
    /// Flag state for a given column.
    flag: Flag,

    /// Formations of cards belonging to either player.
    formations: [Formation; 2],
}

impl Column {
    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn formations(&self) -> &[Formation; 2] {
        &self.formations
    }
}

/// Possible states for a flag in a [`Column`].
pub enum Flag {
    Unclaimed,
    Claimed,
}

/// Possible cards in a formation for a player.
pub struct Formation(Vec<Card>);

impl Formation {
    pub fn new() -> Formation {
        Formation(Vec::with_capacity(4))
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }
}

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
