use num_derive::{FromPrimitive, ToPrimitive};

use crate::bitboard::BitBoard;
use crate::error::NotationError;

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
            Square::TwentyNine, Square::Thirty, Square::ThirtyOne, Square::TwentyTwo
        ].iter().copied()
    }

    pub fn as_bitboard(&self) -> BitBoard {
        todo!()
    }

    pub fn as_number(&self) -> u8 {
        unsafe {
            num::ToPrimitive::to_u8(self).unwrap_unchecked()
        }
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

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        value.as_bitboard()
    }
}
