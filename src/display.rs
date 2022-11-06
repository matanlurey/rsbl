//! A ligthweight display library, built similar to [TermPixels][termpixels].
//!
//! [termpixels]: https://github.com/loganzartman/termpixels
//!
//! Fundamentally, you have a 2D-matrix of cells, each of which can be filled in
//! a single character (or character equivalent, i.e. symbols like emoji), and
//! optionally a background or foreground color.
//!
//! The reason this module exists is to avoid learning a full TUI framework for
//! Rust. In the future, it likely makes sense to adopt an existing framework
//! and delete this module.

use std::fmt::Display;

use ansi_term::{Color, Style};

/// Stores a 2D buffer of cells.
///
/// A buffer maintains a two-dimensional representation of [`Cell`] instances,
/// and provides methods for querying and editing them programatically.
pub struct Buffer {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Buffer {
    /// Creates a new buffer of empty cells of the provided dimensions.
    ///
    /// # Panics
    ///
    /// If width or height is not at least 1.
    pub fn new(width: usize, height: usize) -> Buffer {
        assert!(width >= 1);
        assert!(height >= 1);
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut column: Vec<Cell> = Vec::with_capacity(width);
            for _ in 0..width {
                column.push(Cell::new());
            }
            rows.push(column);
        }

        Buffer {
            cells: rows,
            width,
            height,
        }
    }

    /// Sets the characters representing `text` to a particular location.
    pub fn print<'a>(
        &mut self,
        render: &'a String,
        column: usize,
        row: usize,
        foreground: Option<Color>,
        background: Option<Color>,
    ) {
        if render.is_empty() {
            return;
        }
        let lines = &mut render.lines();
        for i in 0..lines.count() {
            let line = if lines.count() == 0 {
                render
            } else {
                lines.next().expect("String was length checked")
            };
            for n in 0..line.chars().count() {
                if column + n >= self.width || row + i >= self.height {
                    continue;
                }
                let mut cell = Cell::new();
                let char = line.chars().nth(n).unwrap();
                cell.render(String::from(char));
                if let Some(foreground) = foreground {
                    cell.fg(foreground);
                }
                if let Some(background) = background {
                    cell.bg(background);
                }
                self.set(column + n, row + i, cell);
            }
        }
    }

    /// Sets a reference to the cell at the provided `column` and `row`.
    ///
    /// # Panics
    ///
    /// If the column or row is out of bounds.
    pub fn set(&mut self, column: usize, row: usize, cell: Cell) {
        let replace = &mut self.cells[row][column];
        *replace = cell;
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        })
    }
}

pub struct Cell {
    render: String,
    background: Option<Color>,
    foreground: Option<Color>,
}

impl Cell {
    /// Creates a new empty cell.
    pub fn new() -> Cell {
        Cell {
            render: String::from(" "),
            background: None,
            foreground: None,
        }
    }

    /// Sets the background color of the cell.
    pub fn bg(&mut self, color: Color) {
        self.background = Some(color);
    }

    /// Sets the foreground color of the cell.
    pub fn fg(&mut self, color: Color) {
        self.foreground = Some(color);
    }

    /// Sets the render text of the cell.
    ///
    /// # Panics
    ///
    /// If the visible length of the string is not exactly 1.
    pub fn render(&mut self, render: String) {
        assert_eq!(render.chars().count(), 1);
        self.render = render;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = Style::new();
        if let Some(background) = self.background {
            style = style.on(background);
        }
        if let Some(foreground) = self.foreground {
            style = style.fg(foreground);
        }
        write!(f, "{}", style.paint(&self.render))
    }
}
