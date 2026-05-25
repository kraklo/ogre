use chacha20::{ChaCha20Rng, Seed};
use rand::SeedableRng;

use crate::{
    action::player_action::PlayerAction, card::library::Library, errors::GameResult,
    game_state::GameState,
};

mod action;
pub mod card;
mod errors;
mod game_state;
mod lua;
mod mana;
mod phase;
mod player;
mod stack;
mod zone;

pub fn new_game(players: Vec<(String, Library)>) -> GameState {
    let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    rand::fill(&mut seed);

    let mut game_state = GameState::new(seed);

    for (name, library) in players {
        game_state.add_player(name, library);
    }

    game_state
}

pub fn start_game(game_state: GameState) -> GameState {
    let mut new_state = game_state;
    new_state.deal_opening_hands();

    new_state
}

pub fn apply_action(game_state: GameState, action: PlayerAction) -> GameResult<GameState> {
    game_state.apply_action(action)
}
