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
use std::ops::RangeBounds;
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
        board: &GameBoard,
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

    fn check_position(moves: &mut HashSet<Position>, board: &GameBoard, color: Color, current_position: Position, rank_delta: isize, file_delta: isize) {
        let (rank, file) = (current_position.rank, current_position.file);

        let new_rank = (rank as isize) + rank_delta;
        let new_file = (file as isize) + file_delta;

        if !(0..8).contains(&new_rank) {
            return;
        }

        if !(0..8).contains(&new_file) {
            return;
        }

        let rank = new_rank as usize;
        let file = new_file as usize;

        let piece_at_position = board[rank][file];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != color {
            moves.insert(Position { rank, file });
        }
    }

    fn explore_king(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        Self::check_position(&mut moves, board, self.color, current_position, -1, -1);
        Self::check_position(&mut moves, board, self.color, current_position, -1, 0);
        Self::check_position(&mut moves, board, self.color, current_position, -1, 1);
        Self::check_position(&mut moves, board, self.color, current_position, 0, -1);
        Self::check_position(&mut moves, board, self.color, current_position, 0, 0);
        Self::check_position(&mut moves, board, self.color, current_position, 0, 1);
        Self::check_position(&mut moves, board, self.color, current_position, 1, -1);
        Self::check_position(&mut moves, board, self.color, current_position, 1, 0);
        Self::check_position(&mut moves, board, self.color, current_position, 1, 1);

        moves
    }

    fn explore_queen(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves.extend(self.look_diagonal(current_position, board));
        moves
    }

    fn explore_rook(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves
    }

    fn explore_knight(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        Self::check_position(&mut moves, board, self.color, current_position, -2, -1);
        Self::check_position(&mut moves, board, self.color, current_position, -2, 1);
        Self::check_position(&mut moves, board, self.color, current_position, 2, -1);
        Self::check_position(&mut moves, board, self.color, current_position, 2, 1);
        Self::check_position(&mut moves, board, self.color, current_position, -1, -2);
        Self::check_position(&mut moves, board, self.color, current_position, -1, 2);
        Self::check_position(&mut moves, board, self.color, current_position, 1, -2);
        Self::check_position(&mut moves, board, self.color, current_position, 1, 2);

        // TODO: Castling???

        moves
    }

    fn explore_pawn(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        let starting_rank: u8 = match self.color {
            Color::White => 6,
            Color::Black => 1
        };

        let direction: isize = match self.color {
            Color::White => -1,
            Color::Black => 1
        };

        Self::check_position(&mut moves, board, self.color, current_position, direction, 0);

        // Optional double move for first move
        if (self.move_count == 0) {
            Self::check_position(&mut moves, board, self.color, current_position, direction * 2, 0);
        }

        //TODO: en passant???

        moves
    }

    fn look_sideways(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
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

    fn look_up_and_down(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
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

    fn look_diagonal(&self, current_position: Position, board: &GameBoard) -> HashSet<Position> {
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
    use crate::game::Game;
    use crate::moves::Position;

    // #[test]
    // fn king_moves_test() {
    //     let king_piece: Piece = Piece { kind: crate::game::PieceKind::King, color: Color::White, move_count: 0 };
    //     let current_position: Position = Position { rank: 3, file: 3 };
    //
    //     let board: GameBoard = [
    //         [p!("BR"), p!("BN"), p!("BB"), p!("BQ"), p!("BK"), p!("BB"), p!("BN"), p!("BR")],
    //         [p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP")],
    //         [None; 8],
    //         [None; 8],
    //         [None; 8],
    //         [None; 8],
    //         [p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP")],
    //         [p!("WR"), p!("WN"), p!("WB"), p!("WQ"), p!("WK"), p!("WB"), p!("WN"), p!("WR")],
    //     ];
    //
    //     let moves = king_piece.get_valid_moves(current_position, board);
    //
    //     assert_eq!(moves.len(), 8);
    // }

    #[test]
    fn king_moves_test() {
        let game = Game::new();

        let position = Position { rank: 0, file: 4 };
        let king = game.get_piece_by_position(position).unwrap();

        let moves = king.get_valid_moves(position, &game.board.lock().unwrap());
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pawn_moves_test() {
        let mut game = Game::new();

        let position = Position { rank: 6, file: 3 };
        let pawn = game.get_piece_by_position(position).unwrap();
        let moves = pawn.get_valid_moves(position, &game.board.lock().unwrap());
        assert_eq!(moves.len(), 2);

        let new_position = Position { rank: 6, file: 4 };
        game.move_piece_at_position(position, new_position).expect("pawn move failed");
        let pawn = game.get_piece_by_position(new_position).unwrap();
        let moves = pawn.get_valid_moves(position, &game.board.lock().unwrap());
        assert_eq!(moves.len(), 1);
    }
}