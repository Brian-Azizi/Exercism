use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, IntoEnumIterator, PartialEq, IntEnum)]
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

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value().into()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::into_enum_iter().find(|color| color.int_value() as usize == value) {
        Some(color) => {
            format!("{:?}", color)
        }
        None => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
