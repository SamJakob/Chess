// LINEAR
// DIAGONAL

// LIMITED
// UNLIMITED

// COLLISIONS
// NO COLLISIONS

// EN PASSANT
// CASTLING

use crate::game::{Color, GameBoard, Piece, PieceKind};
use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt::Formatter;
use std::{cmp::min, collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    /// The rank (rank) of the position on the chess board. Starting from 0.
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
    pub fn get_valid_moves(
        &self,
        current_position: Position,
        board: GameBoard,
    ) -> HashSet<Position> {
        match self.kind {
            PieceKind::King => self.explore_king(current_position, board),
            PieceKind::Queen => self.explore_queen(current_position, board),
            PieceKind::Rook => self.explore_rook(current_position, board),
            PieceKind::Bishop => self.look_diagonal(current_position, board),
            PieceKind::Knight => self.explore_knight(current_position, board),
            PieceKind::Pawn => self.explore_pawn(current_position, board)
        }
    }

    fn explore_king(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();
        let piece_at_position = board[current_position.rank - 1][current_position.file - 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 1, file: current_position.file - 1 });
        }

        let piece_at_position = board[current_position.rank - 1][current_position.file];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 1, file: current_position.file });
        }

        let piece_at_position = board[current_position.rank - 1][current_position.file + 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 1, file: current_position.file + 1 });
        }

        let piece_at_position = board[current_position.rank][current_position.file - 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank, file: current_position.file - 1 });
        }

        let piece_at_position = board[current_position.rank][current_position.file + 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank, file: current_position.file + 1 });
        }

        let piece_at_position = board[current_position.rank + 1][current_position.file - 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 1, file: current_position.file - 1 });
        }

        let piece_at_position = board[current_position.rank + 1][current_position.file];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 1, file: current_position.file });
        }

        let piece_at_position = board[current_position.rank + 1][current_position.file + 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 1, file: current_position.file + 1 });
        }

        moves
    }

    fn explore_queen(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves.extend(self.look_diagonal(current_position, board));
        moves
    }

    fn explore_rook(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves
    }

    fn explore_knight(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();
        let piece_at_position = board[current_position.rank - 2][current_position.file - 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 2, file: current_position.file - 1 });
        }

        let piece_at_position = board[current_position.rank - 2][current_position.file + 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 2, file: current_position.file + 1 });
        }

        let piece_at_position = board[current_position.rank + 2][current_position.file - 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 2, file: current_position.file - 1 });
        }

        let piece_at_position = board[current_position.rank + 2][current_position.file + 1];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 2, file: current_position.file + 1 });
        }

        let piece_at_position = board[current_position.rank - 1][current_position.file - 2];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 1, file: current_position.file - 2 });
        }

        let piece_at_position = board[current_position.rank - 1][current_position.file + 2];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank - 1, file: current_position.file + 2 });
        }

        let piece_at_position = board[current_position.rank + 1][current_position.file - 2];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 1, file: current_position.file - 2 });
        }

        let piece_at_position = board[current_position.rank + 1][current_position.file + 2];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: current_position.rank + 1, file: current_position.file + 2 });
        }

        // TODO: Castling???

        moves
    }

    fn explore_pawn(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        let starting_rank: u8 = match self.color {
            Color::White => 6,
            Color::Black => 1
        };

        let direction: i8 = match self.color {
            Color::White => -1,
            Color::Black => 1
        };

        let new_rank = ((current_position.rank as i8) + direction) as usize;

        let piece_at_position = board[new_rank][current_position.file];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
            moves.insert(Position { rank: new_rank, file: current_position.file });
        }

        // Double move for first move
        let new_rank1 = ((current_position.rank as i8) + (direction * 2)) as usize;

        let piece_at_position1 = board[new_rank1][current_position.file];
        if piece_at_position1.is_none() || piece_at_position1.unwrap().color != self.color {
            moves.insert(Position { rank: new_rank1, file: current_position.file });
        }

        //TODO: en passant???

        moves
    }

    fn look_sideways(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = HashSet::new();

        // Explore left
        for new_col in (0..current_position.file).rev() {
            let position = Position {
                rank: current_position.rank,
                file: new_col,
            };

            let piece_at_position = board[current_position.rank][new_col];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        // explore right
        for new_col in current_position.file + 1..8 {
            let position = Position {
                rank: current_position.rank,
                file: new_col,
            };

            let piece_at_position = board[current_position.rank][new_col];

            // Unoccupied or can take
            if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            if piece_at_position.is_none() {
                continue;
            }
            // square is occupied so stop exploring
            break;
        }

        valid_moves
    }

    fn look_up_and_down(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = HashSet::new();

        // Explore down
        for new_rank in current_position.rank + 1..8 {
            let position = Position {
                rank: new_rank,
                file: current_position.file,
            };

            let piece_at_position = board[new_rank][current_position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        // explore up
        for new_rank in (0..current_position.file).rev() {
            let position = Position {
                rank: new_rank,
                file: current_position.file,
            };

            let piece_at_position = board[new_rank][current_position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            if piece_at_position.is_none() {
                continue;
            }
            // square is occupied so stop exploring
            break;
        }

        valid_moves
    }

    fn look_diagonal(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = HashSet::new();

        // Explore to top left
        for dev in 1..min(current_position.file, current_position.rank) {
            let position = Position {
                rank: current_position.rank - dev,
                file: current_position.file - dev,
            };

            let piece_at_position = board[position.rank][position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        // Explore to bottom left
        for dev in 1..min(current_position.file, current_position.rank) {
            let position = Position {
                rank: current_position.rank + dev,
                file: current_position.file - dev,
            };

            let piece_at_position = board[position.rank][position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        // Explore to top right
        for dev in 1..min(current_position.file, current_position.rank) {
            let position = Position {
                rank: current_position.rank - dev,
                file: current_position.file + dev,
            };

            let piece_at_position = board[position.rank][position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        // Explore to bottom right
        for dev in 1..min(current_position.file, current_position.rank) {
            let position = Position {
                rank: current_position.rank + dev,
                file: current_position.file + dev,
            };

            let piece_at_position = board[position.rank][position.file];

            // Unoccupied or can take
            if piece_at_position.is_none() {
                valid_moves.insert(position);
                continue;
            }
            if piece_at_position.unwrap().color != self.color {
                valid_moves.insert(position);
            }
            // square is occupied so stop exploring
            break;
        }

        valid_moves
    }
}

mod test {
    use crate::{game::{Color, GameBoard, Piece}, moves::Position, p};

    #[test]
    fn king_moves_test() {
        let king_piece: Piece = Piece { kind: crate::game::PieceKind::King, color: Color::White, move_count: 0 };
        let current_position: Position = Position { rank: 3, file: 3 };

        let board: GameBoard = [
            [p!("BR"), p!("BN"), p!("BB"), p!("BQ"), p!("BK"), p!("BB"), p!("BN"), p!("BR")],
            [p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP")],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP")],
            [p!("WR"), p!("WN"), p!("WB"), p!("WQ"), p!("WK"), p!("WB"), p!("WN"), p!("WR")],
        ];

        let moves = king_piece.get_valid_moves(current_position, board);

        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn pawn_moves_test() {
        let king_piece: Piece = Piece { kind: crate::game::PieceKind::Pawn, color: Color::White, move_count: 0 };
        let current_position: Position = Position { rank: 3, file: 3 };

        let board: GameBoard = [
            [p!("BR"), p!("BN"), p!("BB"), p!("BQ"), p!("BK"), p!("BB"), p!("BN"), p!("BR")],
            [p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP")],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP")],
            [p!("WR"), p!("WN"), p!("WB"), p!("WQ"), p!("WK"), p!("WB"), p!("WN"), p!("WR")],
        ];

        let moves = king_piece.get_valid_moves(current_position, board);

        assert_eq!(moves.len(), 2);
    }
}