use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Creature {
    power: i64,
    toughness: i64,
}

impl Creature {
    pub fn new(power: i64, toughness: i64) -> Self {
        Self { power, toughness }
    }
}
