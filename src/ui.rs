//! UI-specific submodule for Battle Line, i.e. with colors and I/O.

use ansi_term::Color;
use rsbl::{Card, Field, Flag, Hand, TroopColor};
use std::fmt::Display;

use crate::display::{Buffer, Cell};

fn draw_card(card: &Card) -> Cell {
    let mut cell = Cell::new();
    if let Card::Troop(value, color) = card {
        cell.bg(match color {
            TroopColor::Red => Color::Red,
            TroopColor::Green => Color::Green,
            TroopColor::Blue => Color::Blue,
            TroopColor::Yellow => Color::Yellow,
            TroopColor::Orange => Color::RGB(255, 87, 51),
            TroopColor::Purple => Color::Purple,
        });
        cell.render(format!(
            "{}",
            if value.value() == 10 {
                0
            } else {
                value.value()
            }
        ));
    } else {
        todo!("Not yet implemented: {:?}", card);
    }
    return cell;
}

fn draw_flag(flag: &Flag) -> Cell {
    let mut cell = Cell::new();
    if let Flag::Claimed = flag {
        todo!("Use ⬆ or ⬇ based on who claimed");
    } else {
        cell.render(String::from("⚑"));
        cell.fg(Color::Red);
    }
    cell
}

/// Returns a [`Display`]-able buffer representing output from the `field`.
pub fn draw_field(field: Field) -> impl Display {
    assert_eq!(field.columns().len(), 7);

    let mut buffer = Buffer::new(7 * 4, 9);

    for c in 0..7 {
        let column = &field.columns()[c];

        // Draw flag.
        buffer.set(c * 4, 5, draw_flag(column.flag()));

        // Draw north set of cards for this column.
        let mut i = 4;
        for card in column.formations()[0].cards().iter() {
            buffer.set(c * 4, i, draw_card(card));
            i -= 1;
        }

        // Draw south set of cards for this column.
        let mut i = 6;
        for card in column.formations()[1].cards().iter() {
            buffer.set(c * 4, i, draw_card(card));
            i += 1;
        }
    }

    return buffer;
}

pub fn draw_hand(hand: Hand) -> impl Display {
    let mut buffer = Buffer::new(10 * 3, 3);
    let mut i = 0;

    buffer.print(&String::from("Hand:"), 0, 0, None, None);

    for card in hand.iter() {
        buffer.print(&format!("{}", i + 1), i * 4, 1, None, None);
        buffer.set(i * 4, 2, draw_card(card));
        i += 1;
    }

    return buffer;
}
