use chacha20::ChaCha20Rng;
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::card::Card;
use crate::card::hand::Hand;
use crate::errors::GameError;
use crate::errors::GameResult;
use crate::game_state::STARTING_HAND_SIZE;
use crate::mana::mana_pool::ManaPool;

pub struct PlayerSet {
    pub players: Vec<Player>,
    pub active_player_index: usize,
}

impl PlayerSet {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            active_player_index: 0,
        }
    }

    pub fn active_player(&self) -> &Player {
        &self.players[self.active_player_index]
    }

    pub fn active_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.active_player_index]
    }
}

pub struct Player {
    id: Uuid,
    name: String,
    life: i64,
    mana_pool: ManaPool,
    pub hand: Hand,
    library: Vec<Card>,
    graveyard: Vec<Card>,
    exile: Vec<Card>,
}

impl Player {
    pub fn new(library: Vec<Card>, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            life: 20,
            mana_pool: ManaPool::new(),
            hand: Hand::new(),
            library,
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }

    pub fn shuffle_library(&mut self, rng: &mut ChaCha20Rng) {
        self.library.shuffle(rng);
    }

    // This is different from drawing as this is at the start of the game, so no triggers
    pub fn deal_cards(&mut self, num_cards: u8) -> GameResult<()> {
        for _ in 0..num_cards {
            let drawn_card = self
                .library
                .pop()
                .ok_or(GameError::LibraryEmptyError(self.id))?;

            self.hand.cards.insert(drawn_card.id, drawn_card);
        }

        Ok(())
    }

    pub fn keep_hand(&mut self) {
        self.hand.keep();
    }

    pub fn mulligan_hand(&mut self, rng: &mut ChaCha20Rng) -> GameResult<()> {
        if self.hand.num_mulligans > STARTING_HAND_SIZE {
            return Err(GameError::CannotMulliganError);
        }

        self.hand.num_mulligans += 1;

        self.library
            .append(&mut self.hand.cards.clone().into_values().collect());
        self.shuffle_library(rng);
        let _ = self.deal_cards(STARTING_HAND_SIZE);

        Ok(())
    }

    pub fn bottom_card(&mut self, card: Card) {
        self.library.insert(0, card);
    }

    pub fn pop_card_from_hand(&mut self, card_id: Uuid) -> GameResult<Card> {
        let popped_card = self.hand.pop_card(card_id);

        if let Some(card) = popped_card {
            return Ok(card);
        }

        Err(GameError::CardNotInHandError(self.name.clone(), card_id))
    }
}
