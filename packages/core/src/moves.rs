// LINEAR
// DIAGONAL

// LIMITED
// UNLIMITED

// COLLISIONS
// NO COLLISIONS

// EN PASSANT
// CASTLING

use std::{cmp::min, collections::HashSet, hash::Hash};

use serde::Deserialize;

use crate::game::{Color, GameBoard, Piece, PieceKind};

#[derive(PartialEq, Eq, Hash, Deserialize, Clone, Copy)]
pub struct Position {
    row: u8,
    col: u8,
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
        let piece_at_position =  board[(current_position.row - 1) as usize][(current_position.col - 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-1, col: current_position.col-1});
        }

        let piece_at_position =  board[(current_position.row - 1) as usize][current_position.col as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-1, col: current_position.col});
        }

        let piece_at_position =  board[(current_position.row - 1) as usize][(current_position.col + 1) as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-1, col: current_position.col+1});
        }

        let piece_at_position =  board[(current_position.row) as usize][(current_position.col - 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row, col: current_position.col-1});
        }

        let piece_at_position =  board[(current_position.row) as usize][(current_position.col + 1) as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row, col: current_position.col+1});
        }

        let piece_at_position =  board[(current_position.row + 1) as usize][(current_position.col - 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+1, col: current_position.col-1});
        }

        let piece_at_position =  board[(current_position.row + 1) as usize][current_position.col as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+1, col: current_position.col});
        }

        let piece_at_position =  board[(current_position.row + 1) as usize][(current_position.col + 1) as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+1, col: current_position.col+1});
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
        let piece_at_position =  board[(current_position.row - 2) as usize][(current_position.col - 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-2, col: current_position.col-1});
        }

        let piece_at_position =  board[(current_position.row - 2) as usize][(current_position.col + 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-2, col: current_position.col+1});
        }

        let piece_at_position =  board[(current_position.row + 2) as usize][(current_position.col - 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+2, col: current_position.col-1});
        }

        let piece_at_position =  board[(current_position.row + 2) as usize][(current_position.col + 1)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+2, col: current_position.col+1});
        }

        let piece_at_position =  board[(current_position.row - 1) as usize][(current_position.col - 2)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-1, col: current_position.col-2});
        }

        let piece_at_position =  board[(current_position.row - 1) as usize][(current_position.col + 2)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row-1, col: current_position.col+2});
        }

        let piece_at_position =  board[(current_position.row + 1) as usize][(current_position.col - 2)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+1, col: current_position.col-2});
        }

        let piece_at_position =  board[(current_position.row + 1) as usize][(current_position.col + 2)as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: current_position.row+1, col: current_position.col+2});
        }

        // TODO: Castling???

        moves
    }

    fn explore_pawn(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut moves: HashSet<Position> = HashSet::new();

        let starting_row:u8 = match self.color{
            Color::White => 6,
            Color::Black => 1
        };

        let direction:i8 = match self.color{
            Color::White => -1,
            Color::Black => 1
        };

        let new_row = ((current_position.row as i8) + direction) as u8;

        let piece_at_position =  board[new_row as usize][current_position.col as usize];
        if piece_at_position.is_none() || piece_at_position.unwrap().color != self.color{
            moves.insert(Position{row: new_row, col: current_position.col});
        }

        // Double move for first move
        let new_row1 = ((current_position.row as i8) + (direction * 2)) as u8;

        let piece_at_position1 =  board[new_row1 as usize][current_position.col as usize];
        if piece_at_position1.is_none() || piece_at_position1.unwrap().color != self.color{
            moves.insert(Position{row: new_row1, col: current_position.col});
        }

        //TODO: en passant???

        moves
    }

    fn look_sideways(&self, current_position: Position, board: GameBoard) -> HashSet<Position> {
        let mut valid_moves: HashSet<Position> = HashSet::new();

        // Explore left
        for new_col in (0..current_position.col).rev() {
            let position = Position {
                row: current_position.row,
                col: new_col as u8,
            };

            let piece_at_position = board[current_position.row as usize][new_col as usize];

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
        for new_col in (current_position.col+1..8) {
            let position = Position {
                row: current_position.row,
                col: new_col as u8,
            };

            let piece_at_position = board[current_position.row as usize][new_col as usize];

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
        for new_row in (current_position.row+1..8) {
            let position = Position {
                row: new_row as u8,
                col: current_position.col,
            };

            let piece_at_position = board[new_row as usize][current_position.col as usize];

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
        for new_row in (0..current_position.col).rev() {
            let position = Position {
                row: new_row as u8,
                col: current_position.col,
            };

            let piece_at_position = board[new_row as usize][current_position.col as usize];

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
        for dev in 1..min(current_position.col, current_position.row) {
            let position = Position {
                row: current_position.row - dev,
                col: current_position.col - dev,
            };

            let piece_at_position = board[position.row as usize][position.col as usize];

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
        for dev in 1..min(current_position.col, current_position.row) {
            let position = Position {
                row: current_position.row + dev,
                col: current_position.col - dev,
            };

            let piece_at_position = board[position.row as usize][position.col as usize];

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
        for dev in 1..min(current_position.col, current_position.row) {
            let position = Position {
                row: current_position.row - dev,
                col: current_position.col + dev,
            };

            let piece_at_position = board[position.row as usize][position.col as usize];

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
        for dev in 1..min(current_position.col, current_position.row) {
            let position = Position {
                row: current_position.row + dev,
                col: current_position.col + dev,
            };

            let piece_at_position = board[position.row as usize][position.col as usize];

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

mod test{
    use crate::{game::{Color, GameBoard, Piece}, moves::Position, p};

    #[test]
    fn king_moves_test(){
        let king_piece: Piece = Piece { kind: crate::game::PieceKind::King, color: Color::White, move_count: 0 };
        let current_position: Position = Position{row: 3, col: 3};

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
    fn pawn_moves_test(){
        let king_piece: Piece = Piece { kind: crate::game::PieceKind::Pawn, color: Color::White, move_count: 0 };
        let current_position: Position = Position{row: 3, col: 3};

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