use crate::card::Card;
use crate::card::library::Library;
use crate::player::Player;

pub struct GameState {
    players: Vec<Player>,
    active_player_index: u8,
    battlefield: Vec<Card>,
}

impl GameState {
    pub fn new(libraries: Vec<Library>) -> Self {
        let players = libraries
            .into_iter()
            .map(|library| Player::new(library))
            .collect();

        Self {
            players,
            active_player_index: 0,
            battlefield: Vec::new(),
        }
    }
}
