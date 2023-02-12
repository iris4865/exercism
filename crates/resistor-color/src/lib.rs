use enum_iterator::Sequence;
use std::fmt::{Display, Error, Formatter};

mod tests;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl TryFrom<u32> for ResistorColor {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResistorColor::Black),
            1 => Ok(ResistorColor::Brown),
            2 => Ok(ResistorColor::Red),
            3 => Ok(ResistorColor::Orange),
            4 => Ok(ResistorColor::Yellow),
            5 => Ok(ResistorColor::Green),
            6 => Ok(ResistorColor::Blue),
            7 => Ok(ResistorColor::Violet),
            8 => Ok(ResistorColor::Grey),
            9 => Ok(ResistorColor::White),
            _ => Err(String::from("value out of range")),
        }
    }
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
        // match self {
        //     ResistorColor::Black => write!(f, "Black"),
        //     ResistorColor::Blue => write!(f, "Blue"),
        //     ResistorColor::Brown => write!(f, "Brown"),
        //     ResistorColor::Green => write!(f, "Green"),
        //     ResistorColor::Grey => write!(f, "Grey"),
        //     ResistorColor::Orange => write!(f, "Orange"),
        //     ResistorColor::Red => write!(f, "Red"),
        //     ResistorColor::Violet => write!(f, "Violet"),
        //     ResistorColor::White => write!(f, "White"),
        //     ResistorColor::Yellow => write!(f, "Yellow"),
        // }
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::try_from(value) {
        Ok(color) => color.to_string(),
        Err(err) => err,
    }
}

pub fn colors() -> Vec<ResistorColor> {
    (0..=9)
        .map(|i| ResistorColor::try_from(i).unwrap())
        .collect()
}
