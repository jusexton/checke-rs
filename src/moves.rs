use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::bitboard::{BitBoard, CORNER_SQUARES, SIDE_SQUARES, TOP_AND_BOTTOM_SQUARES};
use crate::board::{Board, Player};
use crate::error::NotationError;
use crate::square::Square;

/// Represents a move that can occur on a board. A move is represented by the source square
/// and the destination square.
pub struct Move {
    pub source: Square,
    pub dest: Square,
}

impl Move {
    /// Creates a new [Move] instance from two given squares.
    /// Move instances have no context of a board or any checkers rules. Moves are simply
    /// a source and destination square. Move validation is expected to be done via other
    /// mechanisms.
    ///
    /// Basic usage:
    /// ```
    /// use checke_rs::moves::Move;
    /// use checke_rs::square::Square;
    ///
    /// let m = Move::new(Square::Eight, Square::Nine).unwrap();
    ///
    /// assert_eq!(m.source, Square::Eight);
    /// assert_eq!(m.dest, Square::Nine);
    /// ```
    pub fn new(source: Square, dest: Square) -> Result<Self, NotationError> {
        match source != dest {
            true => Ok(Move { source, dest }),
            false => Err(NotationError::Idle)
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = NotationError;

    /// Create a [Move] instance using checkers notation.
    /// ```rust
    /// use checke_rs::moves::Move;
    /// use checke_rs::square::Square;
    ///
    /// let m = Move::try_from("18x25").unwrap();
    ///
    /// assert_eq!(m.source, Square::Eighteen);
    /// assert_eq!(m.dest, Square::TwentyFive);
    /// ```
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref CN_PATTERN: Regex = Regex::new(r"^([1-9]+[0-9]*)([-xX])([1-9]+[0-9]*)$").unwrap();
        }

        match CN_PATTERN.captures(text) {
            Some(captures) => parse_captures(captures),
            None => Err(NotationError::InvalidFormat)
        }
    }
}

fn parse_captures(captures: Captures) -> Result<Move, NotationError> {
    let source_text = captures.get(1).unwrap().as_str();
    let dest_text = captures.get(3).unwrap().as_str();

    let source = Square::try_from(source_text)?;
    let dest = Square::try_from(dest_text)?;

    Move::new(source, dest)
}

/// Iterator capable of generating all possible moves for a given board and player of that board.
///
/// ```rust
/// use checke_rs::board::{Board, Player};
/// use checke_rs::moves::MoveIter;
///
/// let board = Board::default();
/// let moves = MoveIter::new(&board, Player::Black);
///
/// let move_count = moves.count();
///
/// assert_eq!(move_count, 7)
/// ```
pub struct MoveIter<'a> {
    board: &'a Board,
    player: Player,
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let pieces = self.board.pieces_by_player(self.player);
        let isolated_pieces = Square::iter()
            .map(|square| pieces.isolate(square))
            .filter(|piece| piece.single_piece());

        for piece in isolated_pieces {
            let is_side_piece = !(piece & SIDE_SQUARES).empty();
            let is_top_or_bottom_piece = !(piece & TOP_AND_BOTTOM_SQUARES).empty();

            match (is_side_piece, is_top_or_bottom_piece) {
                (false, false) => {}
                (true, false) => {}
                (false, true) => {}
                (true, true) => {}
            }

            return Some();
        }

        None
    }
}

impl<'a> MoveIter<'a> {
    /// Creates a new [MoveIter] instance from board reference and player type.
    ///
    /// Basic usage:
    /// ```
    /// use checke_rs::board::{Board, Player};
    /// use checke_rs::moves::MoveIter;
    ///
    /// let board = Board::default();
    /// let moves = MoveIter::new(&board, Player::Black);
    /// ```
    pub fn new(board: &'a Board, player: Player) -> Self {
        MoveIter { board, player }
    }
}