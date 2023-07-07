use lazy_static::lazy_static;
use regex::{Captures, Regex};
use thiserror::Error;

use crate::bitboard::{
    BOTTOM_SQUARES,
    LEFT_SQUARES,
    MonoBitBoard,
    RIGHT_SQUARES,
    TOP_SQUARES,
};
use crate::board::{Board, Player};

#[derive(PartialEq, Debug)]
pub enum Position {
    Bottom,
    Left,
    BottomLeft,
    BottomRight,
    Top,
    Right,
    TopRight,
    TopLeft,
    Interior,
}

impl From<MonoBitBoard> for Position {
    fn from(bitboard: MonoBitBoard) -> Self {
        let is_left = LEFT_SQUARES.contains(bitboard);
        let is_right = RIGHT_SQUARES.contains(bitboard);
        let is_top = TOP_SQUARES.contains(bitboard);
        let is_bottom = BOTTOM_SQUARES.contains(bitboard);

        match (is_left, is_right, is_top, is_bottom) {
            (false, false, false, true) => Position::Bottom,
            (true, false, false, false) => Position::Left,
            (true, false, false, true) => Position::BottomLeft,
            (false, true, false, true) => Position::BottomRight,
            (false, false, true, false) => Position::Top,
            (false, true, false, false) => Position::Right,
            (false, true, true, false) => Position::TopRight,
            (true, false, true, false) => Position::TopLeft,
            (false, false, false, false) => Position::Interior,
            _ => panic!("This should be impossible to reach.")
        }
    }
}

/// Error denoting an issue parsing checkers notation.
#[derive(Debug, Error)]
pub enum NotationError {
    #[error("Provided value did not conform to a valid checkers notation format.")]
    InvalidFormat,

    #[error("Provided value operates outside the realm of a classical checkers board.")]
    OutOfRange,

    #[error("Provided value represented a piece standing still. The destination square was equal to the source.")]
    Idle,
}

#[derive(Debug, Error)]
#[error("Square could not be converted")]
pub struct SquareConversionError;

/// Represents every valid square on a classical checkers board.
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq)]
pub enum Square {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
    TwentyFour = 24,
    TwentyFive = 25,
    TwentySix = 26,
    TwentySeven = 27,
    TwentyEight = 28,
    TwentyNine = 29,
    Thirty = 30,
    ThirtyOne = 31,
    ThirtyTwo = 32,
}

impl Square {
    /// Provides an iterator over all square values.
    ///
    /// ```
    /// use checke_rs::position::Square;
    ///
    /// let square_count = Square::iter().count();
    ///
    /// assert_eq!(square_count, 32)
    /// ```
    pub fn iter() -> impl Iterator<Item=Self> {
        [
            Square::One, Square::Two, Square::Three, Square::Four,
            Square::Five, Square::Six, Square::Seven, Square::Eight,
            Square::Nine, Square::Ten, Square::Eleven, Square::Twelve,
            Square::Thirteen, Square::Fourteen, Square::Fifteen, Square::Sixteen,
            Square::Seventeen, Square::Eighteen, Square::Nineteen, Square::Twenty,
            Square::TwentyOne, Square::TwentyTwo, Square::TwentyThree, Square::TwentyFour,
            Square::TwentyFive, Square::TwentySix, Square::TwentySeven, Square::TwentyEight,
            Square::TwentyNine, Square::Thirty, Square::ThirtyOne, Square::ThirtyTwo
        ].iter().copied()
    }

    /// Creates a [BitCell] representing this square instance.
    pub fn to_bitboard(&self) -> MonoBitBoard {
        let value = match self {
            Square::One => 0x4000000000000000,
            Square::Two => 0x1000000000000000,
            Square::Three => 0x400000000000000,
            Square::Four => 0x100000000000000,
            Square::Five => 0x80000000000000,
            Square::Six => 0x20000000000000,
            Square::Seven => 0x8000000000000,
            Square::Eight => 0x2000000000000,
            Square::Nine => 0x400000000000,
            Square::Ten => 0x100000000000,
            Square::Eleven => 0x40000000000,
            Square::Twelve => 0x10000000000,
            Square::Thirteen => 0x8000000000,
            Square::Fourteen => 0x2000000000,
            Square::Fifteen => 0x800000000,
            Square::Sixteen => 0x200000000,
            Square::Seventeen => 0x40000000,
            Square::Eighteen => 0x10000000,
            Square::Nineteen => 0x4000000,
            Square::Twenty => 0x1000000,
            Square::TwentyOne => 0x800000,
            Square::TwentyTwo => 0x200000,
            Square::TwentyThree => 0x80000,
            Square::TwentyFour => 0x20000,
            Square::TwentyFive => 0x4000,
            Square::TwentySix => 0x1000,
            Square::TwentySeven => 0x400,
            Square::TwentyEight => 0x100,
            Square::TwentyNine => 0x80,
            Square::Thirty => 0x20,
            Square::ThirtyOne => 0x8,
            Square::ThirtyTwo => 0x2
        };
        MonoBitBoard::new(value).unwrap()
    }

    /// Returns the square instance represented as a u8.
    ///
    /// ```rust
    /// use checke_rs::position::Square;
    ///
    /// let number = Square::One.to_number();
    ///
    /// assert_eq!(number, 1)
    /// ```
    pub fn to_number(&self) -> u8 {
        num::ToPrimitive::to_u8(self).unwrap()
    }
}

impl TryFrom<u8> for Square {
    type Error = NotationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        num::FromPrimitive::from_u8(value).ok_or(NotationError::OutOfRange)
    }
}

impl TryFrom<&str> for Square {
    type Error = NotationError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let value = text.parse::<u8>().map_err(|_| NotationError::InvalidFormat)?;
        Square::try_from(value)
    }
}

impl TryFrom<MonoBitBoard> for Square {
    type Error = SquareConversionError;

    /// Converts the given [MonoBitBoard] to a [Square] instance.
    ///
    /// ```
    /// use checke_rs::bitboard::MonoBitBoard;
    /// use checke_rs::position::Square;
    ///
    /// let expected_square = Square::ThirtyTwo;
    /// let bitboard = expected_square.to_bitboard();
    /// let converted_square = Square::try_from(bitboard).unwrap();
    ///
    /// assert_eq!(converted_square, expected_square)
    /// ```
    ///
    /// Raises an error when the given [MonoBitBoard] could not be converted
    /// to a classical checkers square.
    ///
    /// ```
    /// use checke_rs::bitboard::MonoBitBoard;
    /// use checke_rs::position::Square;
    ///
    /// let bitboard = MonoBitBoard::new(0b1).unwrap();
    /// let result = Square::try_from(bitboard);
    ///
    /// assert!(result.is_err())
    /// ```
    fn try_from(bitboard: MonoBitBoard) -> Result<Self, Self::Error> {
        Square::iter()
            .find(|square| square.to_bitboard() == bitboard)
            .ok_or(SquareConversionError)
    }
}

/// Represents a move that can occur on a board. A move is represented by the source square
/// and the destination square.
#[derive(Copy, Clone, Debug)]
pub struct Move(pub Square, pub Square);

impl Move {
    /// Creates a new [Move] instance from two given squares.
    /// Move instances have no context of a board or any checkers rules. Moves are simply
    /// a source and destination square. Move validation is expected to be done via other
    /// mechanisms.
    ///
    /// Basic usage:
    /// ```
    /// use checke_rs::position::{Move, Square};
    ///
    /// let Move(source, dest) = Move::new(Square::Eight, Square::Nine).unwrap();
    ///
    /// assert_eq!(source, Square::Eight);
    /// assert_eq!(dest, Square::Nine);
    /// ```
    pub fn new(source: Square, destination: Square) -> Result<Self, NotationError> {
        match source != destination {
            true => Ok(Move(source, destination)),
            false => Err(NotationError::Idle)
        }
    }

    /// Create a new [Move] instance using the given checkers notation text.
    ///
    /// ```rust
    /// use checke_rs::position::{Move, Square};
    ///
    /// let Move(source, dest) = Move::from_notation("18x25").unwrap();
    ///
    /// assert_eq!(source, Square::Eighteen);
    /// assert_eq!(dest, Square::TwentyFive);
    /// ```
    pub fn from_notation(text: &str) -> Result<Self, NotationError> {
        lazy_static! {
            static ref CN_PATTERN: Regex = Regex::new(r"^([1-9]+[0-9]*)([-xX])([1-9]+[0-9]*)$").unwrap();
        }

        match CN_PATTERN.captures(text) {
            Some(captures) => Move::parse_captures(captures),
            None => Err(NotationError::InvalidFormat)
        }
    }

    fn parse_captures(captures: Captures) -> Result<Move, NotationError> {
        let source_text = captures.get(1).unwrap().as_str();
        let dest_text = captures.get(3).unwrap().as_str();

        let source = Square::try_from(source_text)?;
        let dest = Square::try_from(dest_text)?;

        Move::new(source, dest)
    }
}

impl TryFrom<&str> for Move {
    type Error = NotationError;

    /// Converts a string slice representing checkers notation into a [Move] instance.
    ///
    /// ```rust
    /// use checke_rs::position::{Move, Square};
    ///
    /// let Move(source, dest) = Move::try_from("18x25").unwrap();
    ///
    /// assert_eq!(source, Square::Eighteen);
    /// assert_eq!(dest, Square::TwentyFive);
    /// ```
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Move::from_notation(text)
    }
}

/// Iterator capable of generating all possible moves for a given board and player of that board.
pub struct MoveIter<'a> {
    board: &'a Board,
    player: Player,
    piece_index: u8,
}

impl<'a> MoveIter<'a> {
    /// Creates a new [MoveIter] instance from board reference and player type.
    /// ```
    /// use checke_rs::board::{Board, Player};
    /// use checke_rs::position::MoveIter;
    ///
    /// let board = Board::default();
    /// let moves = MoveIter::new(&board, Player::Black);
    /// ```
    pub fn new(board: &'a Board, player: Player) -> Self {
        MoveIter { board, player, piece_index: 0 }
    }

    fn get_current_piece(&self) -> Option<MonoBitBoard> {
        let player_bitboard = self.board.pieces_by_player(self.player);
        let player_pieces = player_bitboard.pieces();
        return player_pieces.get(self.piece_index as usize).cloned();
    }
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let current_piece = self.get_current_piece()?;
        let position = Position::from(current_piece);

        match position {
            Position::Bottom => {}
            Position::Left => {}
            Position::BottomLeft => {}
            Position::BottomRight => {}
            Position::Top => {}
            Position::Right => {}
            Position::TopRight => {}
            Position::TopLeft => {}
            Position::Interior => {}
        }

        self.piece_index += 1;
        Some(Move::new(Square::Two, Square::Nine).unwrap())
    }
}