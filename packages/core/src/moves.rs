// LINEAR
// DIAGONAL

// LIMITED
// UNLIMITED

// COLLISIONS
// NO COLLISIONS

// EN PASSANT
// CASTLING

<<<<<<< Updated upstream
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
=======
use crate::game::{Color, Game, GameBoard, Piece, PieceKind};
use std::collections::HashSet;

impl Piece {
    // get the span of a piece (all moves within the board, ignoring collisions)
    fn get_span(&self, current_position: Position) -> HashSet<Position> {
        match (*self).kind {
            PieceKind::King => current_position
                .transition(-1, -1)
                .into_iter()
                .chain(current_position.transition(-1, 0))
                .chain(current_position.transition(-1, 1))
                .chain(current_position.transition(0, -1))
                .chain(current_position.transition(0, 1))
                .chain(current_position.transition(1, -1))
                .chain(current_position.transition(1, 0))
                .chain(current_position.transition(1, 1))
                .collect::<HashSet<_>>(),
            PieceKind::Queen => {
                let mut span: HashSet<Position> = HashSet::new();
                span.extend(Self::span_col(current_position));
                span.extend(Self::span_row(current_position));
                span.extend(Self::span_diagonal(current_position));
                span
            }
            PieceKind::Bishop => Self::span_diagonal(current_position),
            PieceKind::Knight => current_position
                .transition(-2, -1)
                .into_iter()
                .chain(current_position.transition(-2, 1))
                .chain(current_position.transition(-1, -2))
                .chain(current_position.transition(-1, 2))
                .chain(current_position.transition(1, -2))
                .chain(current_position.transition(1, 2))
                .chain(current_position.transition(2, -1))
                .chain(current_position.transition(2, 1))
                .collect::<HashSet<_>>(),
            PieceKind::Rook => {
                let mut span: HashSet<Position> = HashSet::new();
                span.extend(Self::span_col(current_position));
                span.extend(Self::span_row(current_position));
                span
            }
            PieceKind::Pawn => Self::span_pawn(&self, current_position),
        }
    }

    fn span_pawn(piece: &Piece, current_position: Position) -> HashSet<Position> {
        let delta: i8 = if piece.color == Color::White { -1 } else { 1 };
        let start_row: u8 = if piece.color == Color::White { 6 } else { 1 };

        let mut span: HashSet<Position> = HashSet::new();

        if current_position.row == start_row {
            // first move
            span.insert(current_position.transition(delta * 2, 0).unwrap());
        }

        if current_position.row == (start_row as i16 + (delta as i16 * 3)) as u8 {
            // en passant
            span.extend(
                [
                    current_position.transition(1, -1),
                    current_position.transition(1, 1),
                ]
                .into_iter()
                .filter_map(|pos| pos), // Filters out `None`, keeps `Some`
            );
        }

        // default
        span.insert(current_position.transition(delta * 1, 0).unwrap());

        span
    }

    fn span_row(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(n, 0);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }

    fn span_col(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(0, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }

    fn span_diagonal(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(n, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }

            let new_pos = current_position.transition(-n, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }

    fn get_legal_moves(&self, en_passant_col: Option<u8>, board: GameBoard) -> HashSet<Position> {
        HashSet::new()
>>>>>>> Stashed changes
    }
}

impl Piece {
<<<<<<< Updated upstream
    // pub fn get_valid_moves(current_position: Position) -> HashSet<Position> {}
}
=======

    fn is_legal_no_block_check(&self, new_pos: &Position, game: &Game) -> bool {
        // find piece at square
        (*game).get_piece(new_pos);
        // can take opponent piece
        // cannot take own piece
        false
    }

    fn get_legal_moves(&self, en_passant_col: Option<u8>, board: &GameBoard) -> HashSet<Position> {
        match (*self).kind {
            PieceKind::King => current_position
                .transition(-1, -1)
                .into_iter()
                .chain(current_position.transition(-1, 0))
                .chain(current_position.transition(-1, 1))
                .chain(current_position.transition(0, -1))
                .chain(current_position.transition(0, 1))
                .chain(current_position.transition(1, -1))
                .chain(current_position.transition(1, 0))
                .chain(current_position.transition(1, 1))
                .collect::<HashSet<_>>(),
            PieceKind::Queen => {
                let mut span: HashSet<Position> = HashSet::new();
                span.extend(Self::span_col(current_position));
                span.extend(Self::span_row(current_position));
                span.extend(Self::span_diagonal(current_position));
                span
            }
            PieceKind::Bishop => Self::span_diagonal(current_position),
            PieceKind::Knight => current_position
                .transition(-2, -1)
                .into_iter()
                .chain(current_position.transition(-2, 1))
                .chain(current_position.transition(-1, -2))
                .chain(current_position.transition(-1, 2))
                .chain(current_position.transition(1, -2))
                .chain(current_position.transition(1, 2))
                .chain(current_position.transition(2, -1))
                .chain(current_position.transition(2, 1))
                .collect::<HashSet<_>>(),
            PieceKind::Rook => {
                let mut span: HashSet<Position> = HashSet::new();
                span.extend(Self::span_col(current_position));
                span.extend(Self::span_row(current_position));
                span
            }
            PieceKind::Pawn => Self::span_pawn(&self, current_position),
        }
    }

    fn span_pawn(piece: &Piece, current_position: Position) -> HashSet<Position> {
        let delta: i8 = if piece.color == Color::White { -1 } else { 1 };
        let start_row: u8 = if piece.color == Color::White { 6 } else { 1 };

        let mut span: HashSet<Position> = HashSet::new();

        if current_position.row == start_row {
            // first move
            span.insert(current_position.transition(delta * 2, 0).unwrap());
        }

        if current_position.row == (start_row as i16 + (delta as i16 * 3)) as u8 {
            // en passant
            span.extend(
                [
                    current_position.transition(1, -1),
                    current_position.transition(1, 1),
                ]
                .into_iter()
                .filter_map(|pos| pos), // Filters out `None`, keeps `Some`
            );
        }

        // default
        span.insert(current_position.transition(delta * 1, 0).unwrap());

        span
    }

    fn span_row(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(n, 0);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }

    fn span_col(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(0, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }

    fn span_diagonal(current_position: Position) -> HashSet<Position> {
        let mut span: HashSet<Position> = HashSet::new();

        for n in -8..8 {
            if n == 0 {
                continue;
            }

            let new_pos = current_position.transition(n, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }

            let new_pos = current_position.transition(-n, n);
            if new_pos.is_some() {
                span.insert(new_pos.unwrap());
            }
        }

        span
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Color;

    use super::*;

    #[test]
    fn test_to_string() {
        let pos = Position::new(7, 0).unwrap(); // This should correspond to "A1"
        assert_eq!(pos.to_string(), "A1");

        let pos = Position::new(0, 4).unwrap(); // This should correspond to "E8"
        assert_eq!(pos.to_string(), "E8");

        let pos = Position::new(4, 3).unwrap(); // This should correspond to "D4"
        assert_eq!(pos.to_string(), "D4");
    }

    #[test]
    fn test_from_string() {
        let pos: Option<Position> = Position::from_string("A1".to_string());
        assert_eq!(pos.unwrap(), Position { col: 0, row: 7 });

        let pos = Position::from_string("E8".to_string());
        assert_eq!(pos.unwrap(), Position { col: 4, row: 0 });

        let pos = Position::from_string("D4".to_string());
        assert_eq!(pos.unwrap(), Position { col: 3, row: 4 });
    }

    #[test]
    fn test_from_string_out_of_bounds() {
        // This should return None since "I9" is not a valid chessboard position
        assert_eq!(None, Position::from_string("I9".to_string()));
    }

    fn log_span(piece: &Piece) {
        (*piece)
            .get_span(Position::from_string("B2".to_string()).unwrap())
            .iter()
            .for_each(|pos| println!("{}", (*pos).to_string()));
    }

    #[test]
    fn test_king_span() {
        let king: Piece = Piece {
            kind: PieceKind::King,
            color: Color::Black,
            move_count: 0,
        };
        log_span(&king);
    }

    #[test]
    fn test_white_pawn_span() {
        let pawn: Piece = Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
            move_count: 0,
        };
        log_span(&pawn);
    }

    #[test]
    fn test_black_pawn_span() {
        let pawn: Piece = Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
            move_count: 0,
        };
        log_span(&pawn);
    }

    #[test]
    fn test_knight_span() {
        let knight: Piece = Piece {
            kind: PieceKind::Knight,
            color: Color::Black,
            move_count: 0,
        };
        log_span(&knight);
    }
}
>>>>>>> Stashed changes
