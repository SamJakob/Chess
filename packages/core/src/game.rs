use crate::error::MoveError;
use crate::game::Color::{Black, White};
use crate::game::PieceKind::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::moves::Position;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter, Write};
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
            'K' => Some(King),
            'Q' => Some(Queen),
            'B' => Some(Bishop),
            'N' => Some(Knight),
            'R' => Some(Rook),
            'P' => Some(Pawn),
            _ => None,
        }
    }

    fn char(&self) -> char {
        match *self {
            King => 'K',
            Queen => 'Q',
            Bishop => 'B',
            Knight => 'N',
            Rook => 'R',
            Pawn => 'P',
        }
    }

    fn material_value(&self) -> usize {
        match *self {
            Queen => 9,
            Rook => 5,
            Bishop => 3,
            Knight => 3,
            Pawn => 1,
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
            'B' => Some(Black),
            'W' => Some(White),
            _ => None,
        }
    }

    fn char(&self) -> char {
        match *self {
            Black => 'B',
            White => 'W',
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
    /// [GameManager]).
    id: Option<String>,

    /// 8x8 grid of pieces. Rank (1-8) then file (A-H).
    pub board: Arc<Mutex<GameBoard>>,

    /// The [Instant] the game was created.
    created_at: DateTime<Utc>,

    /// The list of moves in the game.
    moves: Arc<Mutex<Vec<(Position, Position)>>>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self.board.lock().unwrap(); // Access the game board
        let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']; // File labels

        // Initial newline for spacing
        writeln!(f, "\n")?;

        // Print column labels (file labels)
        write!(f, "    ")?; // Padding before the file labels
        for file in files.iter() {
            write!(f, "{}    ", file)?; // Spacing between each file label
        }
        writeln!(f)?; // Newline after the file labels

        // Print each row of the board
        for (i, row) in board.iter().enumerate() {
            write!(f, " {}  ", 8 - i)?; // Print the rank (row number)
            for square in row.iter() {
                match square {
                    Some(piece) => write!(f, "{}   ", piece)?, // Spacing for piece
                    None => write!(f, "..   ")?,
                }
            }
            writeln!(f, " {}", 8 - i)?; // Print the rank again at the end of the row
        }

        // Print the file labels again at the bottom
        write!(f, "    ")?; // Padding before the file labels
        for file in files.iter() {
            write!(f, "{}    ", file)?; // Spacing between each file label
        }
        writeln!(f)?; // Newline at the end

        Ok(())
    }
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
            current_move: Color,
        }

        let mut is_player_in_check = BTreeMap::new();
        is_player_in_check.insert(White, self.is_player_in_check(White));
        is_player_in_check.insert(Black, self.is_player_in_check(Black));

        let moves_count = self.get_move_count();
        let current_move = self.get_current_move();

        let game = Game {
            id: &self.id,
            board: &self.board,
            created_at: &self.created_at,
            is_player_in_check: &is_player_in_check,
            moves_count,
            current_move,
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
            White
        } else {
            Black
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
        if piece.color != self.get_current_move() {
            return Err(MoveError::OutOfTurnError);
        }

        let valid_moves = piece.get_valid_moves(self, position);
        if !valid_moves.contains(new_position) {
            return Err(MoveError::IllegalMoveError);
        }

        // If the player is in check, we should eliminate any moves that will result in the player
        // still being in check afterward.
        if self.is_player_in_check(piece.color) {
            for valid_move in valid_moves {}
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

    pub fn get_move_count(&self) -> usize {
        self.moves.lock().unwrap().len()
    }

    pub fn get_current_move(&self) -> Color {
        if self.get_move_count() % 2 == 0 {
            White
        } else {
            Black
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Color::{Black, White};
    use crate::game::PieceKind::{King, Pawn, Queen};
    use crate::game::{Game, PieceKind};
    use crate::moves::Position;
    use std::str::FromStr;

    #[test]
    fn check_bishop_value() {
        assert_eq!(3, PieceKind::Bishop.material_value())
    }

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
                        piece.kind == Pawn,
                        match rank {
                            // The second and second-last ranks should have a pawn.
                            1 | 6 => true,
                            _ => false,
                        }
                    );

                    // Check that the queen is on her own color.
                    if piece.kind == Queen {
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

    #[test]
    fn test_turns() {
        let game = Game::new();

        let white_pawn_position_original = Position::from_str("C2").unwrap();
        let white_pawn_position_1 = Position::from_str("C3").unwrap();
        let white_pawn_position_2 = Position::from_str("C4").unwrap();

        let black_pawn_position_original = Position::from_str("C7").unwrap();
        let black_pawn_position_1 = Position::from_str("C6").unwrap();

        // Assert that it's white's turn.
        assert_eq!(game.get_current_move(), White);

        let white_pawn = game
            .get_piece_by_position(&white_pawn_position_original)
            .unwrap();
        assert_eq!(white_pawn.move_count, 0);

        // Move the white pawn to start the game.
        game.move_piece_at_position(&white_pawn_position_original, &white_pawn_position_1)
            .expect("failed to move white pawn");

        // Assert that a move has occurred.
        let white_pawn = game.get_piece_by_position(&white_pawn_position_1).unwrap();
        assert_eq!(white_pawn.move_count, 1);
        assert!(game
            .get_piece_by_position(&white_pawn_position_original)
            .is_none());

        // Assert that it's now black's turn.
        assert_eq!(game.get_current_move(), Black);

        // Try (and fail) to move white again.
        game.move_piece_at_position(&white_pawn_position_1, &white_pawn_position_2)
            .expect_err("expected white pawn move to fail out of turn");

        // Assert that the move failed (i.e., the state has remained the same).
        let white_pawn = game.get_piece_by_position(&white_pawn_position_1).unwrap();
        assert_eq!(white_pawn.move_count, 1);
        assert_eq!(game.get_current_move(), Black);
        assert!(game.get_piece_by_position(&white_pawn_position_2).is_none());

        // Assert that the black pawn has not yet moved.
        let black_pawn = game
            .get_piece_by_position(&black_pawn_position_original)
            .unwrap();
        assert_eq!(black_pawn.move_count, 0);

        // Move the black pawn.
        game.move_piece_at_position(&black_pawn_position_original, &black_pawn_position_1)
            .unwrap();
        let black_pawn = game.get_piece_by_position(&black_pawn_position_1).unwrap();
        assert_eq!(black_pawn.move_count, 1);

        // Assert that the black move succeeded and it's now white's turn.
        assert_eq!(game.get_current_move(), White);
        assert!(game
            .get_piece_by_position(&black_pawn_position_original)
            .is_none());

        // Assert that the white pawn still hasn't moved.
        assert_eq!(white_pawn.move_count, 1);

        // Try to move the white pawn (it should succeed).
        game.move_piece_at_position(&white_pawn_position_1, &white_pawn_position_2)
            .expect("failed to move white pawn during turn");

        // Assert that the move succeeded (i.e., the state has remained the same).
        let white_pawn = game.get_piece_by_position(&white_pawn_position_2).unwrap();
        assert_eq!(white_pawn.move_count, 2);
        assert_eq!(game.get_current_move(), Black);
        assert!(game.get_piece_by_position(&white_pawn_position_1).is_none());
    }
}
