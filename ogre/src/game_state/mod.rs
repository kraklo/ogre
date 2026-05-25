use chacha20::ChaCha20Rng;
use rand::SeedableRng;

use crate::card::Card;
use crate::card::library::Library;
use crate::phase::Phase;
use crate::player::Player;
use crate::stack::Stack;

const STARTING_HAND_SIZE: u64 = 7;

pub struct GameState {
    pub rng: ChaCha20Rng,
    seed: <ChaCha20Rng as SeedableRng>::Seed,
    players: Vec<Player>,
    active_player_index: u8,
    turn_number: u64,
    phase: Phase,
    battlefield: Vec<Card>,
    stack: Stack,
}

impl GameState {
    pub fn new(libraries: Vec<Library>) -> Self {
        let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
        rand::fill(&mut seed);
        let mut rng = ChaCha20Rng::from_seed(seed);

        let mut players = libraries
            .into_iter()
            .map(|library| Player::new(library))
            .collect();

        let mut game_state = Self {
            rng,
            seed,
            players,
            active_player_index: 0,
            turn_number: 0,
            phase: Phase::FirstMain,
            battlefield: Vec::new(),
            stack: Vec::new(),
        };

        for player in players.iter_mut() {
            player.shuffle_library(&mut game_state);
            player.draw_cards(STARTING_HAND_SIZE);
        }

        game_state
    }
}
