pub mod card_types;
pub mod hand;
pub mod library;

use crate::{
    card::card_types::{creature::Creature, land::Land},
    mana::{colors::Color, mana_value::ManaValue},
    zone::Zone,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub enum CardData {
    Land(Land),
    Creature(Creature),
    // Artifact,
    // Enchantment,
    // Instant,
    // Sorcery,
    // Planeswalker,
}

#[derive(Clone, Debug)]
pub struct Card {
    pub id: Uuid,
    owner_id: Uuid,
    mana_value: ManaValue,
    color_identity: Vec<Color>,
    name: String,
    zone: Zone,
    is_tapped: bool,
    data: CardData,
}

impl Card {
    pub fn from_card_json(owner_id: Uuid, card_json: CardJson) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            mana_value: card_json.mana_value,
            color_identity: card_json.color_identity,
            name: card_json.name,
            zone: Zone::Library,
            is_tapped: false,
            data: card_json.card_type,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct CardJson {
    name: String,
    mana_value: ManaValue,
    color_identity: Vec<Color>,
    card_type: CardData,
}
