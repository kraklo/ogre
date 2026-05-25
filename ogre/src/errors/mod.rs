use std::fmt;

use uuid::Uuid;

use crate::action::player_action::PlayerAction;
use crate::phase::Phase;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
pub enum GameError {
    LibraryEmptyError(Uuid),
    WrongPhaseError(PlayerAction, Phase),
    CannotMulliganError,
    MulliganWrongNumberBottomError(u8, u8),
    CardNotInHandError(String, Uuid),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::LibraryEmptyError(player_id) => {
                write!(f, "Player {} has no cards in their library.", player_id)
            }
            GameError::WrongPhaseError(action, phase) => write!(
                f,
                "Cannot take the {:?} action in the {:?} phase.",
                action, phase
            ),
            GameError::CannotMulliganError => write!(f, "Cannot mulligan below 0"),
            GameError::MulliganWrongNumberBottomError(intended_bottomed, actual_bottomed) => {
                write!(
                    f,
                    "Wrong number of cards bottomed: {}. Number expected: {}",
                    actual_bottomed, intended_bottomed
                )
            }
            GameError::CardNotInHandError(player_name, card_id) => {
                write!(f, "{} has no card with the ID {}", player_name, card_id)
            }
        }
    }
}
