// LINEAR
// DIAGONAL

// LIMITED
// UNLIMITED

// COLLISIONS
// NO COLLISIONS

// EN PASSANT
// CASTLING

use crate::game::Piece;
use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Position {
    /// The rank (row) of the position on the chess board. Starting from 0.
    pub rank: usize,

    /// The file (column) of the position on the chess board. Starting from 0.
    pub file: usize,
}

impl Position {
    fn validate(rank: usize, file: usize) {
        if rank >= 8 { panic!("invalid rank: {}", rank); }
        if file >= 8 { panic!("invalid file: {}", file); }
    }

    pub fn new(rank: usize, file: usize) -> Position {
        Self::validate(rank, file);
        Position { rank, file }
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionVisitor;

        impl<'de> Visitor<'de> for PositionVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple with two digits from 0 to 7: (rank, file)")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let rank = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("failed to obtain rank of tuple")
                })?;

                let file = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("failed to obtain file of tuple")
                })?;

                Ok(Position { rank, file })
            }
        }

        let visitor = PositionVisitor;
        deserializer.deserialize_seq(visitor)
    }
}

impl Piece {
    // pub fn get_valid_moves(current_position: Position) -> HashSet<Position> {}
}