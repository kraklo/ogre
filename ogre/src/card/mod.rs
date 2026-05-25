pub mod card_types;
pub mod hand;
pub mod library;

use crate::{
    card::card_types::{creature::Creature, land::Land},
    zone::Zone,
};
use uuid::Uuid;

#[derive(Clone)]
pub enum CardData {
    Land(Land),
    Creature(Creature),
    // Artifact,
    // Enchantment,
    // Instant,
    // Sorcery,
    // Planeswalker,
}

#[derive(Clone)]
pub struct Card {
    pub id: Uuid,
    owner_id: Uuid,
    name: String,
    zone: Zone,
    is_tapped: bool,
    data: CardData,
}

impl Card {
    pub fn new(owner_id: Uuid, name: String, data: CardData) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name,
            zone: Zone::Library,
            is_tapped: false,
            data,
        }
    }
}
