use colored::*;
use std::fmt::Display;

pub struct Console;

impl Console {
    pub fn print<T: Display>(value: T) {
        println!("{}", value);
    }

    pub fn println<T: Display>(value: T) {
        println!("{}", value);
    }

    pub fn print_colored<T: Display>(value: T, color: Color) {
        println!("{}", value.to_string().color(color));
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn format<T: Display>(value: T) -> String {
        format!("{}", value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<Color> for colored::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => colored::Color::Black,
            Color::Red => colored::Color::Red,
            Color::Green => colored::Color::Green,
            Color::Yellow => colored::Color::Yellow,
            Color::Blue => colored::Color::Blue,
            Color::Magenta => colored::Color::Magenta,
            Color::Cyan => colored::Color::Cyan,
            Color::White => colored::Color::White,
        }
    }
} 