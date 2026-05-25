use uuid::Uuid;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
pub enum GameError {
    LibraryEmptyError(LibraryEmptyError),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::LibraryEmptyError(err) => err.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct LibraryEmptyError {
    player_id: Uuid,
}

impl LibraryEmptyError {
    pub fn new(player_id: Uuid) -> Self {
        Self { player_id }
    }
}

impl fmt::Display for LibraryEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {} has no cards in their library.",
            self.player_id
        )
    }
}
