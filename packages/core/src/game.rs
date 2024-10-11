use crate::error::MoveError;
use crate::game::Color::{Black, White};
use crate::game::PieceKind::King;
use crate::moves::Position;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Write};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceKind {
    fn from_char(name: char) -> Option<PieceKind> {
        match name {
            'K' => Some(PieceKind::King),
            'Q' => Some(PieceKind::Queen),
            'B' => Some(PieceKind::Bishop),
            'N' => Some(PieceKind::Knight),
            'R' => Some(PieceKind::Rook),
            'P' => Some(PieceKind::Pawn),
            _ => None,
        }
    }

    fn char(&self) -> char {
        match *self {
            PieceKind::King => 'K',
            PieceKind::Queen => 'Q',
            PieceKind::Bishop => 'B',
            PieceKind::Knight => 'N',
            PieceKind::Rook => 'R',
            PieceKind::Pawn => 'P',
        }
    }

    fn value(&self) -> usize {
        match *self {
            PieceKind::Queen => 9,
            PieceKind::Rook => 5,
            PieceKind::Bishop => 3,
            PieceKind::Knight => 3,
            PieceKind::Pawn => 1,
            _ => 0,
        }
    }
}

impl Serialize for PieceKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_char(self.char())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn from_char(c: char) -> Option<Color> {
        match c {
            'B' => Some(Color::Black),
            'W' => Some(Color::White),
            _ => None,
        }
    }

    fn char(&self) -> char {
        match *self {
            Color::Black => 'B',
            Color::White => 'W',
        }
    }

    fn get_other(&self) -> Color {
        match *self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_char(self.char())
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct Piece {
    /// The kind of piece. This also indicates its value.
    pub kind: PieceKind,

    /// The color (owner) of the piece.
    pub color: Color,

    /// The number of moves that the Piece has made.
    pub move_count: usize,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.color.char())
            .and(f.write_char(self.kind.char()))
    }
}

impl Piece {
    fn new(kind: PieceKind, color: Color) -> Piece {
        Piece {
            kind,
            color,
            move_count: 0,
        }
    }

    pub fn new_from(notation: &'static str) -> Option<Piece> {
        if notation.len() == 2 {
            let kind_notation = notation.as_bytes()[1] as char;
            let color_notation = notation.as_bytes()[0] as char;

            let kind = PieceKind::from_char(kind_notation);
            let color = Color::from_char(color_notation);

            if let (Some(kind), Some(color)) = (kind, color) {
                return Some(Piece::new(kind, color));
            }
        }

        panic!("Invalid piece notation: {}", notation);
    }
}

#[macro_export]
macro_rules! p {
    ($x: literal) => {
        Piece::new_from($x)
    };
}

pub type GameBoard = [[Option<Piece>; 8]; 8];

fn serialize_game_board<S>(board: &Arc<Mutex<GameBoard>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ranks = serializer.serialize_seq(Some(8))?;
    for rank in board.lock().unwrap().iter() {
        ranks.serialize_element(&rank)?;
    }
    ranks.end()
}

pub struct Game {
    /// ID of the game in the [crate::game_manager::GameManager] (if the game belongs to a
    /// GameManager]).
    id: Option<String>,

    /// 8x8 grid of pieces. Rank (1-8) then file (A-H).
    pub board: Arc<Mutex<GameBoard>>,

    /// The [Instant] the game was created.
    created_at: DateTime<Utc>,

    /// The list of moves in the game.
    moves: Arc<Mutex<Vec<(Position, Position)>>>,
}

impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Game<'a> {
            id: &'a Option<String>,
            #[serde(serialize_with = "serialize_game_board")]
            board: &'a Arc<Mutex<GameBoard>>,
            #[serde(with = "ts_milliseconds")]
            created_at: &'a DateTime<Utc>,
            is_player_in_check: &'a BTreeMap<Color, bool>,
            moves_count: usize,
        }

        let mut is_player_in_check = BTreeMap::new();
        is_player_in_check.insert(White, self.is_player_in_check(White));
        is_player_in_check.insert(Black, self.is_player_in_check(Black));

        let game = Game {
            id: &self.id,
            board: &self.board,
            created_at: &self.created_at,
            is_player_in_check: &is_player_in_check,
            moves_count: self.moves.lock().unwrap().len(),
        };

        game.serialize(serializer)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game::new_with_id(None)
    }

    pub fn new_with_id(id: Option<String>) -> Game {
        #[rustfmt::skip]
        let board = Arc::new(Mutex::new([
            [p!("BR"), p!("BN"), p!("BB"), p!("BQ"), p!("BK"), p!("BB"), p!("BN"), p!("BR")],
            [p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP"), p!("BP")],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP"), p!("WP")],
            [p!("WR"), p!("WN"), p!("WB"), p!("WQ"), p!("WK"), p!("WB"), p!("WN"), p!("WR")],
        ]));

        Game {
            id,
            board,
            created_at: Utc::now(),
            moves: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_tile_color(rank: usize, file: usize) -> Color {
        if (rank % 2) == (file % 2) {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_piece_by_position(&self, position: &Position) -> Option<Piece> {
        self.board.lock().unwrap()[position.rank][position.file]
    }

    pub fn move_piece_at_position(
        &self,
        position: &Position,
        new_position: &Position,
    ) -> Result<(), MoveError> {
        let piece = self.get_piece_by_position(position);
        if piece.is_none() {
            return Err(MoveError::PieceNotFoundError);
        }

        let mut piece = piece.unwrap();
        let valid_moves = piece.get_valid_moves(&self, position);
        if !valid_moves.contains(new_position) {
            return Err(MoveError::IllegalMoveError);
        }

        piece.move_count += 1;

        let mut board = self.board.lock().unwrap();
        board[position.rank][position.file] = None;
        board[new_position.rank][new_position.file] = Some(piece);

        self.moves.lock().unwrap().push((*position, *new_position));
        Ok(())
    }

    pub fn get_piece(board: &GameBoard, position: &Position) -> Option<Piece> {
        board[position.rank][position.file]
    }

    pub fn is_player_in_check(&self, color: Color) -> bool {
        for rank in 0..8 {
            for file in 0..8 {
                let position = &Position { rank, file };
                let piece = self.get_piece_by_position(position);
                if piece.is_none() {
                    continue;
                }

                let piece = piece.unwrap();
                let moves = piece.get_valid_moves(self, position);
                if moves.is_empty() {
                    continue;
                }

                for position in moves {
                    let target = self.get_piece_by_position(&position);
                    if target.is_none() {
                        continue;
                    }

                    let target = target.unwrap();
                    if target.color == color && target.kind == King {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use crate::game::Color::{Black, White};
    use crate::game::PieceKind::{King, Queen};
    use crate::game::{Game, PieceKind};
    use crate::moves::Position;
    use std::str::FromStr;

    #[test]
    fn check_starting_board() {
        let game: Game = Game::new();

        // Count that we have two queens at the end.
        let mut queen_count = 0;

        for (rank, files) in game.board.lock().unwrap().iter().enumerate() {
            for (file, piece) in files.iter().enumerate() {
                let has_piece = piece.is_some();

                assert_eq!(
                    has_piece,
                    match rank {
                        // All files in the first two and last two ranks should have a piece.
                        0 | 1 | 6 | 7 => true,
                        _ => false,
                    }
                );

                if has_piece {
                    let piece = piece.unwrap();
                    assert_eq!(piece.move_count, 0);

                    assert_eq!(
                        piece.kind == PieceKind::Pawn,
                        match rank {
                            // The second and second-last ranks should have a pawn.
                            1 | 6 => true,
                            _ => false,
                        }
                    );

                    // Check that the queen is on her own color.
                    if piece.kind == PieceKind::Queen {
                        queen_count += 1;
                        assert_eq!(piece.color, Game::get_tile_color(rank, file));
                    }
                }
            }
        }

        assert_eq!(queen_count, 2);
    }

    #[test]
    fn test_is_king_in_check() {
        let game = Game::new();
        assert!(!game.is_player_in_check(White));
        assert!(!game.is_player_in_check(Black));

        let white_queen_position_original = Position::from_str("D1").unwrap();
        let black_king_position_original = Position::from_str("E8").unwrap();

        let white_queen_position = Position::from_str("D5").unwrap();
        let black_king_position = Position::from_str("E5").unwrap();

        // Obtain the white_queen and black_king.
        let white_queen = game
            .get_piece_by_position(&white_queen_position_original)
            .unwrap();
        let black_king = game
            .get_piece_by_position(&black_king_position_original)
            .unwrap();
        assert_eq!(white_queen.kind, Queen);
        assert_eq!(white_queen.color, White);
        assert_eq!(black_king.kind, King);
        assert_eq!(black_king.color, Black);

        {
            // Move the white queen and the black king next to each other in the middle of the board.
            let mut board = game.board.lock().unwrap();
            board[white_queen_position_original.rank][white_queen_position_original.file] = None;
            board[black_king_position_original.rank][black_king_position_original.file] = None;
            board[white_queen_position.rank][white_queen_position.file] = Some(white_queen);
            board[black_king_position.rank][black_king_position.file] = Some(black_king);
        }

        assert!(!game.is_player_in_check(White));
        assert!(game.is_player_in_check(Black));
    }
}
