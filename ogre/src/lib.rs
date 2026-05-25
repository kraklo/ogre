use chacha20::ChaCha20Rng;
use rand::SeedableRng;

use crate::{
    action::player_action::PlayerAction, card::library::Library, errors::GameResult,
    game_state::GameState,
};

pub mod action;
pub mod card;
pub mod errors;
pub mod game_state;
pub mod lua;
pub mod mana;
pub mod phase;
pub mod player;
pub mod stack;
pub mod zone;

fn new_game(players: Vec<(String, Library)>) -> GameState {
    let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    rand::fill(&mut seed);

    let mut game_state = GameState::new(seed);

    for (name, library) in players {
        game_state.add_player(name, library);
    }

    game_state
}

fn start_game(game_state: GameState) -> GameState {
    let mut new_state = game_state;
    new_state.deal_opening_hands();

    new_state
}

fn apply_action(game_state: GameState, action: PlayerAction) -> GameResult<GameState> {
    game_state.apply_action(action)
}
