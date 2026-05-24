use uuid::Uuid;

use crate::card::Card;

pub struct Creature {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    is_tapped: bool,

    // Creature specific fields
    power: i64,
    toughness: i64,
}

impl Creature {
    pub fn new(owner_id: Uuid, name: String, power: i64, toughness: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name,
            is_tapped: false,
            power,
            toughness,
        }
    }
}
