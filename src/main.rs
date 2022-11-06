use rand;
use rsbl::{Card, Deck, Field, Hand, Side, TroopColor, TroopValue};

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

    println!("{}", ui::draw_field(field));
    println!();

    let mut troops = Deck::of_troops(&mut rand::thread_rng());
    let mut player = Hand::new();

    for _ in 0..7 {
        player.add(&troops.draw().unwrap());
    }

    println!("{}", ui::draw_hand(player));
    println!();
    println!("{} cards remaining in the Troop Deck", troops.len());
}
