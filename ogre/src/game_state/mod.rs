use chacha20::{ChaCha20Rng, Seed};
use rand::SeedableRng;
use uuid::Uuid;

use crate::action::player_action::PlayerAction;
use crate::card::Card;
use crate::card::library::Library;
use crate::errors::{GameError, GameResult};
use crate::phase::{Phase, PreGamePhase};
use crate::player::{Player, PlayerSet};
use crate::stack::Stack;

pub const STARTING_HAND_SIZE: u8 = 7;

pub struct GameState {
    pub rng: ChaCha20Rng,
    players: PlayerSet,
    turn_number: u64,
    phase: Phase,
    battlefield: Vec<Card>,
    stack: Stack,
}

impl GameState {
    pub fn new(seed: Seed) -> Self {
        let rng = ChaCha20Rng::from_seed(seed);

        Self {
            rng,
            players: PlayerSet::new(),
            turn_number: 0,
            phase: Phase::FirstMain,
            battlefield: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn apply_action(self, action: PlayerAction) -> GameResult<GameState> {
        let mut new_state = self;

        let action_result: GameResult<()> = match action {
            PlayerAction::KeepHand => new_state.keep_hand(),
            PlayerAction::MulliganHand => new_state.mulligan_hand(),
            PlayerAction::MulliganChooseBottom(cards_to_bottom) => {
                new_state.mulligan_choose_bottom(cards_to_bottom)
            }
        };

        if let Err(err) = action_result {
            return Err(err);
        }

        Ok(new_state)
    }

    fn advance_to_next_player(&mut self) -> GameResult<()> {
        self.players.active_player_index += 1;

        if self.players.active_player_index >= self.players.players.len() {
            self.advance_phase()?;
        }

        Ok(())
    }

    fn pregame_advance_to_next_player(&mut self) {
        let current_player_index = self.players.active_player_index;
        self.players.active_player_index += 1;

        // Loop to next player that hasn't kept
        while !self.players.active_player().hand.has_kept
            && current_player_index != self.players.active_player_index
        {
            self.players.active_player_index += 1;

            if self.players.active_player_index >= self.players.players.len() {
                self.players.active_player_index = 0;
            }
        }
    }

    fn advance_phase(&mut self) -> GameResult<()> {
        Ok(())
    }

    pub fn add_player(&mut self, name: String, library: Library) {
        let mut player = Player::new(library, name);
        player.shuffle_library(&mut self.rng);

        self.players.players.push(player);
    }

    pub fn deal_opening_hands(&mut self) {
        for player in self.players.players.iter_mut() {
            let _ = player.deal_cards(STARTING_HAND_SIZE);
        }
    }

    fn keep_hand(&mut self) -> GameResult<()> {
        if self.phase != Phase::PreGame(PreGamePhase::KeepOrMulligan) {
            return Err(GameError::WrongPhaseError(
                PlayerAction::KeepHand,
                self.phase.clone(),
            ));
        }

        let active_player = self.players.active_player_mut();

        active_player.keep_hand();

        if active_player.hand.num_mulligans > 0 {
            self.phase = Phase::PreGame(PreGamePhase::MulliganChooseBottom);
        } else {
            let current_player_index = self.players.active_player_index;
            self.pregame_advance_to_next_player();

            // This means everyone has kept
            // Go time
            if self.players.active_player_index == current_player_index {
                // Time to start the first turn of the game. Skip to first player's main 1
                self.players.active_player_index = 0;
                self.phase = Phase::FirstMain;
            }
        }

        Ok(())
    }

    fn mulligan_hand(&mut self) -> GameResult<()> {
        if self.phase != Phase::PreGame(PreGamePhase::KeepOrMulligan) {
            return Err(GameError::WrongPhaseError(
                PlayerAction::MulliganHand,
                self.phase.clone(),
            ));
        }

        let active_player = self.players.active_player_mut();

        active_player.mulligan_hand(&mut self.rng)?;
        self.pregame_advance_to_next_player();

        Ok(())
    }

    /// Puts cards on the bottom of the library in the order given
    fn mulligan_choose_bottom(&mut self, cards_to_bottom: Vec<Uuid>) -> GameResult<()> {
        let active_player = self.players.active_player_mut();

        for card_id in cards_to_bottom {
            let card = active_player.pop_card_from_hand(card_id)?;

            active_player.bottom_card(card);
        }

        let current_player_index = self.players.active_player_index;
        self.pregame_advance_to_next_player();

        // This means everyone has kept
        // Go time
        if self.players.active_player_index == current_player_index {
            // Time to start the first turn of the game. Skip to first player's main 1
            self.players.active_player_index = 0;
            self.phase = Phase::FirstMain;
        }

        Ok(())
    }
}
