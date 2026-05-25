use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}
