use std::ops::{BitAnd, BitOr, BitXor, BitXorAssign, Range};

use thiserror::Error;

/// Represents all black squares on a checkers board.
pub const BLACK_SQUARES: BitBoard = BitBoard(0x5555555555555555);

/// Represents all white squares on a checkers board.
pub const WHITE_SQUARES: BitBoard = BitBoard(0xAAAAAAAAAAAAAAAA);

/// Represents all corner squares on a checkers board.
pub const CORNER_SQUARES: BitBoard = BitBoard(0x8100000000000081);

/// Represents all left squares on a checkers board.
pub const LEFT_SQUARES: BitBoard = BitBoard(0x8080808080808080);

/// Represents all right squares on a checkers board.
pub const RIGHT_SQUARES: BitBoard = BitBoard(0x101010101010101);

/// Represents all left and right squares on a checkers board.
pub const LEFT_AND_RIGHT_SQUARES: BitBoard = BitBoard(0x8181818181818181);

/// Represents all top squares on a checkers board.
pub const TOP_SQUARES: BitBoard = BitBoard(0xFF00000000000000);

/// Represents all bottom squares on a checkers board.
pub const BOTTOM_SQUARES: BitBoard = BitBoard(0x00000000000000FF);

/// Represents all top and bottom squares on a checkers board.
pub const TOP_AND_BOTTOM_SQUARES: BitBoard = BitBoard(0xFF000000000000FF);

/// Bit representation of a checkers board. Backed by 64 bits; a bitboard uses bits to represent
/// each square on the board. Multiple bitboards are generally used to track different information.
/// These boards can then make use of bit operations to quickly determine the full context of the
/// board. This bitboard implementation not only exposes various bit operations but other domain
/// specific helpers closely related to bitboard calculation that make board
/// calculations easy and fast.
#[derive(Copy, Clone, Debug)]
pub struct BitBoard(u64);

impl BitBoard {
    /// Constructs a new [BitBoard] instance with the given value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Calculates whether this bitboard is empty. A bitboard is considered empty if no
    /// bits have the value of 1. In other words, the value is 0.
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns a [CellIter] that will be capable of iterating over every active bit on this board.
    pub fn used_cells(&self) -> CellIter {
        CellIter::new(*self)
    }

    /// Calculates whether the given [MonoBitBoard] overlaps with this bitboard instance.
    /// A bitboard overlaps with another when they have at least one bit in common.
    pub fn contains(&self, bitboard: MonoBitBoard) -> bool {
        !(*self & bitboard).empty()
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd<MonoBitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: MonoBitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr<MonoBitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: MonoBitBoard) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

#[derive(Debug, Error)]
#[error("MonoBitBoard can only be constructed with a value that contains a single bit with the value of 1.")]
pub struct MonoBitBoardError;

/// Special type of bitboard that enforces that only a single bit has the value of 1.
/// This can be useful when representing a piece or single cell using bitboard and type safety.
#[derive(Copy, Clone, Debug)]
pub struct MonoBitBoard(u64);

impl MonoBitBoard {
    /// Attempt to create a [MonoBitBoard] with the given value.
    pub fn new(value: u64) -> Result<Self, MonoBitBoardError> {
        let is_single_piece = value != 0 && (value & (value - 1)) == 0;
        match is_single_piece {
            true => Ok(MonoBitBoard(value)),
            false => Err(MonoBitBoardError)
        }
    }
}

impl BitAnd for MonoBitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for MonoBitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl TryFrom<BitBoard> for MonoBitBoard {
    type Error = MonoBitBoardError;

    /// Attempts to convert a [BitBoard] into a [MonoBitBoard]
    fn try_from(value: BitBoard) -> Result<Self, Self::Error> {
        MonoBitBoard::new(value.0)
    }
}

macro_rules! impl_equals {
    ($x:ident, $y:ident) => {
        impl PartialEq for $x {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl PartialEq<u64> for $x {
            fn eq(&self, other: &u64) -> bool { self.0 == *other }
        }

        impl PartialEq<$y> for $x {
            fn eq(&self, other: &$y) -> bool {
                self.0 == other.0
            }
        }
    }
}

impl_equals!(MonoBitBoard, BitBoard);
impl_equals!(BitBoard, MonoBitBoard);

/// Iterator capable of producing a [MonoBitBoard] for each active cell of a given [BitBoard].
pub struct CellIter {
    bitboard: BitBoard,
    iter: Range<usize>,
}

impl CellIter {
    /// Creates a new iterator instance with the given [BitBoard]
    pub fn new(bitboard: BitBoard) -> Self {
        CellIter { bitboard, iter: 0..64 }
    }
}

impl Iterator for CellIter {
    type Item = MonoBitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref()
            .find(|index| self.bitboard.0 & (1 << index) != 0)
            .map(|index| MonoBitBoard::new(1 << index).unwrap())
    }
}
