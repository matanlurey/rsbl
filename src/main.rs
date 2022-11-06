use rsbl::{Card, Field, Side, TroopColor, TroopValue};
use ui::draw_field;

mod display;
pub mod lib;
mod ui;

fn main() {
    let mut field = Field::new();

    // Demo, add {1, 2} to {North, 1}.
    field.add_card(
        1,
        Side::North,
        Card::Troop(TroopValue::new(1), TroopColor::Red),
    );
    field.add_card(
        1,
        Side::North,
        Card::Troop(TroopValue::new(2), TroopColor::Red),
    );

    // Demo, add {3} to {South, 4} and {10} to {South, 6}.
    field.add_card(
        4,
        Side::South,
        Card::Troop(TroopValue::new(3), TroopColor::Blue),
    );
    field.add_card(
        6,
        Side::South,
        Card::Troop(TroopValue::new(10), TroopColor::Green),
    );

    println!("{}", draw_field(field));
}
