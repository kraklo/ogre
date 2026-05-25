use uuid::Uuid;

use crate::card::Card;
use crate::mana::mana_pool::ManaPool;

pub struct Player {
    id: Uuid,
    life: i64,
    mana_pool: ManaPool,
    hand: Vec<Card>,
    library: Vec<Card>,
    graveyard: Vec<Card>,
    exile: Vec<Card>,
}

impl Player {
    pub fn new(library: Vec<Card>) -> Self {
        Self {
            id: Uuid::new_v4(),
            life: 20,
            mana_pool: ManaPool::new(),
            hand: Vec::new(),
            library,
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }
}
