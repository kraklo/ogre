use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::card::Card;
use crate::errors::GameError::{self, LibraryEmptyError};
use crate::errors::GameResult;
use crate::game_state::GameState;
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

    pub fn shuffle_library(&mut self, game_state: &mut GameState) {
        self.library.shuffle(&mut game_state.rng);
    }

    pub fn draw_cards(&mut self, num_cards: u64) -> GameResult<()> {
        for _ in 0..num_cards {
            let drawn_card =
                self.library
                    .pop()
                    .ok_or(GameError::LibraryEmptyError(LibraryEmptyError::new(
                        self.id,
                    )))?;

            self.hand.push(drawn_card);
        }

        Ok(())
    }
}
