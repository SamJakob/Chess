// LINEAR
// DIAGONAL

// LIMITED
// UNLIMITED

// COLLISIONS
// NO COLLISIONS

// EN PASSANT
// CASTLING

use crate::game::Piece;
use std::collections::HashSet;

pub type Position = (usize, usize);

impl Piece {
    pub fn get_valid_moves(current_position: Position) -> HashSet<Position> {}
}