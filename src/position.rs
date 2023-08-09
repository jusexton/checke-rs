use std::convert::Infallible;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use thiserror::Error;

use crate::bitboard::{BitBoard, CellIter, MonoBitBoard};
use crate::board::{BoardState, Player};

/// Error denoting an issue parsing checkers notation.
#[derive(Debug, Error, PartialEq)]
pub enum NotationError {
    #[error("Provided value did not conform to a valid checkers notation format.")]
    InvalidFormat,

    #[error("Provided value operates outside the realm of a classical checkers board.")]
    OutOfRange,
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

    /// Returns the square instance represented as a u8.
    pub fn to_number(&self) -> u8 {
        num::ToPrimitive::to_u8(self).unwrap()
    }
}

impl From<Square> for MonoBitBoard {
    fn from(value: Square) -> Self {
        let value = match value {
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
}

impl TryFrom<u8> for Square {
    type Error = NotationError;

    /// Converts a number to its square representation.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        num::FromPrimitive::from_u8(value).ok_or(NotationError::OutOfRange)
    }
}

impl TryFrom<&str> for Square {
    type Error = NotationError;

    /// Converts a number in string format to its square representation
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let value = text.parse::<u8>().map_err(|_| NotationError::InvalidFormat)?;
        Square::try_from(value)
    }
}

impl TryFrom<MonoBitBoard> for Square {
    type Error = SquareConversionError;

    /// Converts the given [MonoBitBoard] to a [Square] instance. Results in an error when
    /// the given bitboard did not represent one of the 32 classical checker squares.
    fn try_from(bitboard: MonoBitBoard) -> Result<Self, Self::Error> {
        Square::iter()
            .find(|square| MonoBitBoard::from(*square) == bitboard)
            .ok_or(SquareConversionError)
    }
}

/// Represents a move that can occur on a board. A move is represented by a source and destination.
#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    source: MonoBitBoard,
    destination: MonoBitBoard,
}

impl Move {
    /// Creates a new [Move] instance from two given [MonoBitBoard]s.
    /// Move instances have no context of a board or any checkers rules. Moves are simply
    /// a source and destination. Move validation is expected to be done via other mechanisms.
    pub fn new(source: MonoBitBoard, destination: MonoBitBoard) -> Self {
        Move { source, destination }
    }

    /// Creates a new [Move] instance from two given squares. Move instances have no context of
    /// a board or any checkers rules. Moves are simply a source and destination square.
    /// Move validation is expected to be done via other mechanisms.
    pub fn from_squares(source: Square, destination: Square) -> Self {
        let source = MonoBitBoard::from(source);
        let destination = MonoBitBoard::from(destination);
        Move { source, destination }
    }

    /// Create a new [Move] instance using the given checkers notation text.
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

        Ok(Move::from_squares(source, dest))
    }

    /// Retrieves a copy of this moves source.
    pub fn source(&self) -> MonoBitBoard {
        self.source
    }

    /// Retrieves a copy of this moves destination.
    pub fn destination(&self) -> MonoBitBoard {
        self.destination
    }

    /// Returns a bitboard representing the squares that will change if the move is applied.
    /// This value will be useful when updating a bitboard with a move by applying an xor.
    pub fn to_bitboard(&self) -> BitBoard { self.source | self.destination }
}

impl TryFrom<&str> for Move {
    type Error = NotationError;

    /// Converts a string slice representing checkers notation into a [Move] instance.
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Move::from_notation(text)
    }
}

impl TryFrom<(MonoBitBoard, MonoBitBoard)> for Move {
    type Error = Infallible;

    /// Converts a tuple of [MonoBitBoard] into a [Move] instance.
    fn try_from(value: (MonoBitBoard, MonoBitBoard)) -> Result<Self, Self::Error> {
        Ok(Move::new(value.0, value.1))
    }
}

impl TryFrom<(Square, Square)> for Move {
    type Error = Infallible;

    /// Converts a tuple of [Square] into a [Move] instance.
    fn try_from(value: (Square, Square)) -> Result<Self, Self::Error> {
        let m = Self {
            source: MonoBitBoard::from(value.0),
            destination: MonoBitBoard::from(value.1),
        };
        Ok(m)
    }
}

const RED_PIECE_MOVES: &[BitBoard; 32] = &[
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b01010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00010100_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000101_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00010000_10100000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b01000100_00101000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00010001_00001010_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000100_00000010_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00100000_01000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_10001000_01010000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00100010_00010100_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00001000_00000101_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00010000_10100000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_01000100_00101000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00010001_00001010_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000100_00000010_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00100000_01000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_10001000_01010000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00100010_00010100_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00001000_00000101_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00010000_10100000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_01000100_00101000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00010001_00001010_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000100_00000010_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00100000_01000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_10001000_01010000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00100010_00010100_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00001000_00000101_00000000)
];

const BLACK_PIECE_MOVES: &[BitBoard; 32] = &[
    BitBoard::new(0b00000000_10100000_00010000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00101000_01000100_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00001010_00010001_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000010_00000100_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_01000000_00100000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_01010000_10001000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00010100_00100010_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000101_00001000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_10100000_00010000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00101000_01000100_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00001010_00010001_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000010_00000100_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_01000000_00100000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_01010000_10001000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00010100_00100010_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000101_00001000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_10100000_00010000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00101000_01000100_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00001010_00010001_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000010_00000100_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00100000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_01010000_10001000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00010100_00100010),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000101_00001000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10100000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00101000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001010),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000)
];

const KING_MOVES: &[BitBoard; 32] = &[
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000)
];

/// Capable of generating all possible moves. The key different between [MoveGenerator]
/// and [MoveIter] is that [MoveIter] only yields valid moves in the context of the provided
/// [BoardState]. [MoveGenerator] blindly yields all the legal moves a particular piece can have.
struct MoveGenerator<'a> {
    board_state: &'a BoardState,
    player: Player,
}

impl<'a> MoveGenerator<'a> {
    /// Creates a new [MoveGenerator] instance given a [BoardState] reference and [Player].
    pub fn new(board_state: &'a BoardState, player: Player) -> Self {
        MoveGenerator { board_state, player }
    }

    /// Provides an iterator of moves given a specific board cell.
    pub fn by_cell(&self, cell: MonoBitBoard) -> impl Iterator<Item=Move> {
        let inf_cell_iter = [cell].into_iter().cycle();
        let moves_by_cell = self.moves_by_cell(cell);
        inf_cell_iter.zip(moves_by_cell).map(|m| Move::try_from(m).unwrap())
    }

    fn moves_by_cell(&self, cell: MonoBitBoard) -> CellIter {
        self.get_move_bitboard(self.board_state, self.player, cell)
            .unwrap_or(&BitBoard::new(0))
            .used_cells()
    }

    fn get_move_bitboard(&self,
                         board_state: &BoardState,
                         player: Player,
                         cell: MonoBitBoard) -> Option<&BitBoard> {
        let Ok(square) = Square::try_from(cell) else { return None; };

        let move_index = (square.to_number() - 1) as usize;
        let is_king = board_state.is_king(cell);
        let move_bitboard = match is_king {
            true => KING_MOVES.get(move_index),
            false => match player {
                Player::Red => RED_PIECE_MOVES.get(move_index),
                Player::Black => BLACK_PIECE_MOVES.get(move_index)
            }
        };

        move_bitboard
    }
}

/// Error that can occur while performing a move action.
#[derive(Debug, Error, PartialEq)]
pub enum MoveError {
    #[error("Move data was incorrectly formed. Ensure that the source and destination values are valid squares.")]
    InvalidConstruction,

    #[error("A piece did not exist at the provided source square.")]
    NoPieceAtSource,

    #[error("The selected piece to move did not belong to the player color that was allowed to move.")]
    WrongPlayerPiece,

    #[error("The provided destination was illegal. The selected piece can not legally move to the provided destination.")]
    IllegalDestination,

    #[error("Moves can no longer be made on a board that been completed.")]
    GameConcluded,

    #[error("The destination was already occupied by a player piece.")]
    DestinationOccupied,
}

/// Capable of validating that a given move is valid provided additional [BoardState] context.
pub struct MoveValidator<'a> {
    board_state: &'a BoardState,
}

impl<'a> MoveValidator<'a> {
    /// Creates a new [MoveValidator] instance from the given [BoardState].
    pub fn new(board_state: &'a BoardState) -> Self {
        MoveValidator { board_state }
    }

    /// Validates a given move is valid per this validator's board state.
    pub fn validate<T>(&self, m: T) -> Result<(), MoveError> where T: TryInto<Move> {
        let m = m.try_into().map_err(|_| MoveError::InvalidConstruction)?;

        self.valid_piece_selection(&m)?;
        self.valid_destination(&m)?;

        Ok(())
    }

    fn valid_piece_selection(&self, m: &Move) -> Result<(), MoveError> {
        if !self.board_state.is_piece(m.source) {
            return Err(MoveError::NoPieceAtSource);
        }
        if !self.board_state.is_current_player_piece(m.source) {
            return Err(MoveError::WrongPlayerPiece);
        }
        Ok(())
    }

    fn valid_destination(&self, m: &Move) -> Result<(), MoveError> {
        let generator = MoveGenerator::new(self.board_state, self.board_state.active_player);
        let mut destinations = generator.by_cell(m.source).map(|m| m.destination);

        // TODO: If a destination is an attack, it needs to be verified that an opponent
        //  piece is between the source and destination.
        if destinations.all(|dest| dest != m.destination) {
            return Err(MoveError::IllegalDestination);
        }
        if self.board_state.all_pieces() & m.destination != 0 {
            return Err(MoveError::DestinationOccupied);
        }
        Ok(())
    }
}

/// Iterator capable of generating all possible moves for a given [BoardState]
/// and [Player] of that board.
pub struct MoveIter<'a> {
    player_pieces: CellIter,
    generator: MoveGenerator<'a>,
    validator: MoveValidator<'a>,
}

impl<'a> MoveIter<'a> {
    /// Creates a new [MoveIter] instance from given board reference and player.
    pub fn new(board_state: &'a BoardState, player: Player) -> Self {
        let player_pieces = board_state.pieces_by_player(player).used_cells();
        let generator = MoveGenerator::new(board_state, player);
        let validator = MoveValidator::new(board_state);

        MoveIter { player_pieces, generator, validator }
    }
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.player_pieces
            .by_ref()
            .flat_map(|piece| self.generator.by_cell(piece))
            .find(|m| self.validator.validate(m.clone()).is_ok())
    }
}
