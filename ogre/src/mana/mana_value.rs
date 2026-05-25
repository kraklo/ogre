use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ManaValue {
    white: u64,
    blue: u64,
    black: u64,
    red: u64,
    green: u64,
    colorless: u64,
    generic: u64,
}
