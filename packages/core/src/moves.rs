use crate::game::PieceKind::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::game::{Color, Game, GameBoard, Piece};
use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std::str::FromStr;
use std::{cmp::min, collections::HashSet, hash::Hash};

#[derive(Eq, Clone, Copy, Serialize)]
pub struct Position {
    /// The rank (row) of the position on the chess board. Starting from 0.
    pub rank: usize,

    /// The file (column) of the position on the chess board. Starting from 0.
    pub file: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.file == other.file
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank.hash(state);
        self.file.hash(state);
    }
}

impl Position {
    fn validate(rank: usize, file: usize) {
        if rank >= 8 {
            panic!("invalid rank: {}", rank);
        }
        if file >= 8 {
            panic!("invalid file: {}", file);
        }
    }

    pub fn new(rank: usize, file: usize) -> Position {
        Self::validate(rank, file);
        Position { rank, file }
    }

    pub fn transition(&self, rank: i8, file: i8) -> Position {
        Position::new(
            (self.rank as i8 + rank) as usize,
            (self.file as i8 - file) as usize,
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let col = char::from(self.file as u8 + b'A');
        let row = 8 - self.rank;
        write!(f, "{}{}", col, row)
    }
}

#[derive(Debug, PartialEq)]
pub struct PositionParseErr;

impl Display for PositionParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse position")
    }
}

impl FromStr for Position {
    type Err = PositionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert position string (e.g., "B2") to x and y indices
        let col: usize = s.chars().next().unwrap().to_ascii_lowercase() as usize;

        if col > ('h' as usize) {
            return Err(PositionParseErr);
        }

        let col_value = col - 'a' as usize; // 0-indexed
        let row: usize = s.chars().nth(1).unwrap().to_digit(10).unwrap() as usize; // 0-indexed

        if row > 8 {
            return Err(PositionParseErr);
        }

        let row_value = 8 - row;

        Ok(Position {
            rank: row_value,
            file: col_value,
        })
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
                let rank = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("failed to obtain rank of tuple"))?;

                let file = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("failed to obtain file of tuple"))?;

                Ok(Position { rank, file })
            }
        }

        let visitor = PositionVisitor;
        deserializer.deserialize_seq(visitor)
    }
}

impl Piece {
    pub fn get_valid_moves(&self, game: &Game, current_position: &Position) -> HashSet<Position> {
        let board = &game.board.lock().unwrap();

        match self.kind {
            King => self.explore_king(current_position, board),
            Queen => self.explore_queen(current_position, board),
            Rook => self.explore_rook(current_position, board),
            Bishop => self.look_diagonal(current_position, board),
            Knight => self.explore_knight(current_position, board),
            Pawn => self.explore_pawn(current_position, board),
        }
    }

    fn check_position(
        moves: &mut HashSet<Position>,
        board: &GameBoard,
        color: Color,
        current_position: &Position,
        rank_delta: isize,
        file_delta: isize,
    ) {
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

    fn explore_king(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
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

    fn explore_queen(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves.extend(self.look_diagonal(current_position, board));
        moves
    }

    fn explore_rook(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves = self.look_sideways(current_position, board);
        moves.extend(self.look_up_and_down(current_position, board));
        moves
    }

    fn explore_knight(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
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

    fn explore_pawn(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        let direction: isize = match self.color {
            Color::White => -1,
            Color::Black => 1,
        };

        Self::check_position(
            &mut moves,
            board,
            self.color,
            current_position,
            direction,
            0,
        );

        // Optional double move for first move
        if self.move_count == 0 {
            Self::check_position(
                &mut moves,
                board,
                self.color,
                current_position,
                direction * 2,
                0,
            );
        }

        //TODO: en passant???

        moves
    }

    fn explore_pos_and_break(
        position: &Position,
        color: Color,
        board: &GameBoard,
    ) -> (bool, Option<Position>) {
        let piece_at_position = Game::get_piece(board, position);

        // Unoccupied or can take
        if piece_at_position.is_none() {
            return (false, Some(*position));
        }
        if piece_at_position.unwrap().color != color {
            return (true, Some(*position));
        }
        // square is occupied so stop exploring
        (true, None)
    }

    fn explore_rank<I>(rank: usize, files: I, color: Color, board: &GameBoard) -> HashSet<Position>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut valid_moves: HashSet<Position> = HashSet::new();
        for new_col in files {
            let (break_out, valid_move) = Piece::explore_pos_and_break(
                &Position {
                    rank,
                    file: new_col,
                },
                color,
                board,
            );
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }
        valid_moves
    }

    fn explore_file<I>(file: usize, ranks: I, color: Color, board: &GameBoard) -> HashSet<Position>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut valid_moves: HashSet<Position> = HashSet::new();
        for rank in ranks {
            let (break_out, valid_move) =
                Piece::explore_pos_and_break(&Position { rank, file }, color, board);
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }
        valid_moves
    }

    fn look_sideways(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = Piece::explore_rank(
            current_position.rank,
            (0..current_position.file).rev(),
            self.color,
            board,
        );
        valid_moves.extend(Piece::explore_rank(
            current_position.rank,
            current_position.file + 1..8,
            self.color,
            board,
        ));
        valid_moves
    }

    fn look_up_and_down(
        &self,
        current_position: &Position,
        board: &GameBoard,
    ) -> HashSet<Position> {
        // Explore down
        let mut valid_moves: HashSet<Position> = Piece::explore_file(
            current_position.rank,
            current_position.rank + 1..8,
            self.color,
            board,
        );
        valid_moves.extend(Piece::explore_rank(
            current_position.rank,
            (0..current_position.file).rev(),
            self.color,
            board,
        ));
        valid_moves
    }

    fn look_diagonal(&self, current_position: &Position, board: &GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = HashSet::new();

        // Explore to top left
        for dev in 1..min(current_position.file, current_position.rank) {
            let (break_out, valid_move) = Piece::explore_pos_and_break(
                &Position {
                    rank: current_position.rank - dev,
                    file: current_position.file - dev,
                },
                self.color,
                board,
            );
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }

        // Explore to bottom left
        for dev in 1..min(current_position.file, 8 - current_position.rank) {
            let (break_out, valid_move) = Piece::explore_pos_and_break(
                &Position {
                    rank: current_position.rank + dev,
                    file: current_position.file - dev,
                },
                self.color,
                board,
            );
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }

        // Explore to top right
        for dev in 1..min(8 - current_position.file, current_position.rank) {
            let (break_out, valid_move) = Piece::explore_pos_and_break(
                &Position {
                    rank: current_position.rank - dev,
                    file: current_position.file + dev,
                },
                self.color,
                board,
            );
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }

        // Explore to bottom right
        for dev in 1..min(8 - current_position.file, 8 - current_position.rank) {
            let (break_out, valid_move) = Piece::explore_pos_and_break(
                &Position {
                    rank: current_position.rank + dev,
                    file: current_position.file + dev,
                },
                self.color,
                board,
            );
            if valid_move.is_some() {
                valid_moves.insert(valid_move.unwrap());
            }
            if break_out {
                break;
            }
        }

        valid_moves
    }
}

#[cfg(test)]
mod test {
    use crate::game::PieceKind::Bishop;
    use crate::game::{Color, Game, Piece};
    use crate::moves::Position;
    use std::str::FromStr;

    #[test]
    fn bishop_moves_test() {
        let game = Game::new();

        let bishop_position_original = Position { rank: 7, file: 2 };
        let bishop = game
            .get_piece_by_position(&bishop_position_original)
            .unwrap();

        let bishop_position = Position::from_str("C2").unwrap();
        {
            let mut board = game.board.lock().unwrap();
            board[bishop_position_original.rank][bishop_position_original.file] = None;

            board[bishop_position.rank][bishop_position.file] = Some(bishop);
        }

        let bishop = Piece {
            kind: Bishop,
            color: Color::White,
            move_count: 0,
        };

        let moves = bishop.get_valid_moves(&game, &bishop_position);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn king_moves_test() {
        let game = Game::new();

        let position = Position { rank: 0, file: 4 };
        let king = game.get_piece_by_position(&position).unwrap();

        let moves = king.get_valid_moves(&game, &position);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn king_moves_test_2() {
        let game = Game::new();

        let position = Position { rank: 0, file: 4 };
        let king = game.get_piece_by_position(&position).unwrap();

        {
            let mut board = game.board.lock().unwrap();
            board[position.rank][position.file] = None;
            board[position.rank + 3][position.file] = Some(king);
        }

        let moves = king.get_valid_moves(&game, &Position { rank: 3, file: 4 });
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn pawn_moves_test() {
        let game = Game::new();

        let position = Position { rank: 6, file: 3 };
        let new_position = Position { rank: 5, file: 3 };

        let pawn = game.get_piece_by_position(&position).unwrap();
        let moves = pawn.get_valid_moves(&game, &position);
        assert_eq!(moves.len(), 2);

        game.move_piece_at_position(&position, &new_position)
            .expect("pawn move failed");

        let pawn = game.get_piece_by_position(&new_position).unwrap();
        let moves = pawn.get_valid_moves(&game, &new_position);
        assert_eq!(moves.len(), 1);
    }
}
