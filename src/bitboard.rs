use std::ops::{BitAnd, BitOr};

use crate::square::Square;

/// Represents all black squares on a checkers board.
pub const BLACK_SQUARES: BitBoard = BitBoard(0x5555555555555555);

/// Represents all white squares on a checkers board.
pub const WHITE_SQUARES: BitBoard = BitBoard(0xAAAAAAAAAAAAAAAA);

/// Represents all corner squares on a checkers board.
pub const CORNER_SQUARES: BitBoard = BitBoard(0x8100000000000081);

/// Represents all side squares on a checkers board.
pub const SIDE_SQUARES: BitBoard = BitBoard(0x8181818181818181);

/// Represents all top and bottom squares on a checkers board.
pub const TOP_AND_BOTTOM_SQUARES: BitBoard = BitBoard(0xFF000000000000FF);

/// Bit representation of a checkers board. Backed by 64 bits and exposes various bit operations
/// that make board calculations easy and fast.
#[derive(Copy, Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Calculates whether this bit board refers to a single piece or not. A bitboard is
    /// considered to reference a single piece when only a single bit has the value of 1.
    ///
    /// Value that does not contain a single piece.
    /// ```rust
    /// use checke_rs::bitboard::BitBoard;
    ///
    /// let bb = BitBoard(0b10101);
    /// let single_piece = bb.contains_single_piece();
    /// assert!(!single_piece)
    /// ```
    ///
    /// Value that contains a single piece.
    /// ```rust
    /// use checke_rs::bitboard::BitBoard;
    ///
    /// let bb = BitBoard(0b00100);
    /// let single_piece = bb.contains_single_piece();
    /// assert!(single_piece)
    /// ```
    pub fn single_piece(&self) -> bool {
        self.0 != 0 && (self.0 & (self.0 - 1)) == 0
    }

    /// Calculates whether this bit board is empty. A bit board is considered empty if no
    /// pieces can be found on the board.
    ///
    /// ```rust
    /// use checke_rs::bitboard::BitBoard;
    ///
    /// let bb = BitBoard(0b00000);
    ///
    /// assert!(bb.empty())
    /// ```
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Provides a new bitboard isolating the given square on this bitboard instance.
    /// All bits are flipped to zero expect the specified square. If this board instance
    /// does not have a piece on the specified square the bitboard value will be zero.
    ///
    /// Isolating a square that contains a piece.
    /// ```rust
    /// use checke_rs::bitboard::BitBoard;
    /// use checke_rs::square::Square;
    ///
    /// let bb = BitBoard(0b1010);
    ///
    /// let piece = bb.isolate(Square::ThirtyTwo);
    ///
    /// assert_eq!(piece.0, 0b0010)
    /// ```
    ///
    /// Isolating a square that does not contain a value.
    /// ```rust
    /// use checke_rs::bitboard::BitBoard;
    /// use checke_rs::square::Square;
    ///
    /// let bb = BitBoard(0b1000);
    ///
    /// let piece = bb.isolate(Square::ThirtyTwo);
    ///
    /// assert_eq!(piece.0, 0b0000)
    /// ```
    pub fn isolate(&self, square: Square) -> BitBoard {
        let position = BitBoard(2 ^ square.as_number() as u64);
        *self & position
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool { self.0 == *other }
}