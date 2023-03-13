use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black  = 0,
    Brown  = 1,
    Red    = 2,
    Orange = 3,
    Yellow = 4,
    Green  = 5,
    Blue   = 6,
    Violet = 7,
    Grey   = 8,
    White  = 0x9,
}

use ResistorColor::*;
pub fn color_to_value(color: ResistorColor) -> u32 {
    // unimplemented!("convert color {color:?} into a numerical representation")
    match color {
        Black  => Black.int_value(),
        Brown  => Brown.int_value(),
        Red    => Red.int_value(),
        Orange => Orange.int_value(),
        Yellow => Yellow.int_value(),
        Green  => Green.int_value(),
        Blue   => Blue.int_value(),
        Violet => Violet.int_value(),
        Grey   => Grey.int_value(),
        White  => White.int_value(),
    }
}

pub fn value_to_color_string(value: u32) -> String {
    // unimplemented!("convert the value {value} into a string representation of color")
    if value > 9 {
        return String::from("value out of range");
    }
    let color = ResistorColor::from_int(value).unwrap();
    match color {
        Black  => return "Black".to_string(),
        Brown  => return "Brown".to_string(),
        Red    => return "Red".to_string(),
        Orange => return "Orange".to_string(),
        Yellow => return "Yellow".to_string(),
        Green  => return "Green".to_string(),
        Blue   => return "Blue".to_string(),
        Violet => return "Violet".to_string(),
        Grey   => return "Grey".to_string(),
        White  => return "White".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    all::<ResistorColor>().collect::<Vec<_>>()
}
