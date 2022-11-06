use rsbl::{Card, TroopColor, TroopValue};

use crate::ui::format_card;

pub mod lib;
mod ui;

fn main() {
    println!("                                    ");
    println!("                                    ");
    println!("                                    ");
    println!("                                    ");
    println!(" [🚩]  [🚩]  [🚩]  [🚩]  [🚩]  [🚩] ");
    println!("                                    ");
    println!("                                    ");
    println!("                                    ");
    println!("                                    ");

    let cards = vec![
        Card::Troop(TroopValue::new(1), TroopColor::Red),
        Card::Troop(TroopValue::new(3), TroopColor::Green),
        Card::Troop(TroopValue::new(5), TroopColor::Blue),
        Card::Troop(TroopValue::new(7), TroopColor::Yellow),
        Card::Troop(TroopValue::new(9), TroopColor::Orange),
        Card::Troop(TroopValue::new(10), TroopColor::Purple),
    ];
    for card in cards {
        print!(" {} ", format_card(card));
    }
    println!();
    println!();
}
