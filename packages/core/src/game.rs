use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter, Write};
use std::sync::Arc;

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
            _ => None
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn from_char(c: char) -> Option<Color> {
        match c {
            'B' => Some(Color::Black),
            'W' => Some(Color::White),
            _ => None
        }
    }

    fn char(&self) -> char {
        match *self {
            Color::Black => 'B',
            Color::White => 'W',
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
        f.write_char(self.color.char()).and(f.write_char(self.kind.char()))
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

fn serialize_game_board<S>(board: &GameBoard, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ranks = serializer.serialize_seq(Some(8))?;
    for idx in 0..8 {
        ranks.serialize_element(&board[idx])?;
    }
    ranks.end()
}

#[derive(Serialize)]
pub struct Game {
    id: Option<String>,

    /// 8x8 grid of pieces. Rank (1-8) then file (A-H).
    #[serde(serialize_with = "serialize_game_board")]
    board: Arc<GameBoard>,
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

        Game {
            id,
            board: Arc::new(board),
        }
    }

    pub fn get_tile_color(rank: usize, file: usize) -> Color {
        if (rank % 2) == (file % 2) {
            Color::White
        } else {
            Color::Black
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, PieceKind};

    #[test]
    fn check_starting_board() {
        let game: Game = Game::new();

        // Count that we have two queens at the end.
        let mut queen_count = 0;

        for (rank, files) in game.board.iter().enumerate() {
            for (file, piece) in files.iter().enumerate() {
                let has_piece = piece.is_some();

                assert_eq!(has_piece, match rank {
                    // All files in the first two and last two ranks should have a piece.
                    0 | 1 | 6 | 7 => true,
                    _ => false
                });

                if has_piece {
                    let piece = piece.unwrap();
                    assert_eq!(piece.move_count, 0);

                    assert_eq!(piece.kind == PieceKind::Pawn, match rank {
                        // The second and second-last ranks should have a pawn.
                        1 | 6 => true,
                        _ => false
                    });

                    // Check that the queen is on her own color.
                    if (piece.kind == PieceKind::Queen) {
                        queen_count += 1;
                        assert_eq!(piece.color, Game::get_tile_color(rank, file));
                    }
                }
            }
        }

        assert_eq!(queen_count, 2);
    }
}
