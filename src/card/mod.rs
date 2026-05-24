pub mod card_types;
pub mod library;

use crate::card::card_types::{creature::Creature, land::Land};
use uuid::Uuid;

pub enum Card {
    Land(Land),
    Creature(Creature),
    Artifact,
    Enchantment,
    Instant,
    Sorcery,
    Planeswalker,
}
