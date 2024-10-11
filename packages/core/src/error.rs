use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum MoveError {
    PieceNotFoundError,
    IllegalMoveError,
    OutOfTurnError,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            MoveError::PieceNotFoundError => write!(f, "no piece found at the specified position"),
            MoveError::IllegalMoveError => write!(f, "illegal move"),
            MoveError::OutOfTurnError => write!(f, "cannot move out of turn"),
        }
    }
}
