use std::collections::HashMap;

use uuid::Uuid;

use crate::card::Card;

pub struct Hand {
    pub cards: HashMap<Uuid, Card>,
    pub has_kept: bool,
    pub num_mulligans: u8,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            has_kept: false,
            num_mulligans: 0,
        }
    }

    pub fn keep(&mut self) {
        self.has_kept = true;
    }

    pub fn pop_card(&mut self, card_id: Uuid) -> Option<Card> {
        self.cards.remove(&card_id)
    }
}
