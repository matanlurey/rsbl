//! UI-specific submodule for Battle Line, i.e. with colors and I/O.

use ansi_term::{ANSIString, Color, Style};
use rsbl::{Card, TroopColor};

/// Returns a card as a formatted ANSI string (i.e. for terminal output).
pub fn format_card<'a>(card: Card) -> ANSIString<'a> {
    if let Card::Troop(value, color) = card {
        let color = match color {
            TroopColor::Red => Color::Red,
            TroopColor::Green => Color::Green,
            TroopColor::Blue => Color::Blue,
            TroopColor::Yellow => Color::Yellow,
            TroopColor::Orange => Color::RGB(255, 87, 51),
            TroopColor::Purple => Color::Purple,
        };
        let style = Style::new().on(color).bold();
        let value = if value.value() == 10 {
            0
        } else {
            value.value()
        };
        return style.paint(format!(" {} ", value));
    } else {
        todo!("Not yet implemented: {:?}", card);
    }
}
