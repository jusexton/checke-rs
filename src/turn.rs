use std::convert::Infallible;

use crate::position::{Move, NotationError};

/// Represents a turn on a board. Turns are simply an abstraction around a collection of moves.
/// Multiple moves are allowed per turn due to checkers allowing multiple jumps per turn.
pub struct Turn {
    moves: Vec<Move>,
}

impl Turn {
    /// Construct a turn from an iterable of items that can be converted into [Move] instances.
    pub fn new<T, I>(moves: I) -> Result<Self, T::Error>
    where
        T: TryInto<Move>,
        I: IntoIterator<Item = T>,
    {
        let move_results = moves
            .into_iter()
            .map(|m| m.try_into())
            .collect::<Result<Vec<Move>, T::Error>>();

        move_results.map(|moves| Turn { moves })
    }

    /// Attempts tp create a [Turn] instance using checkers notation.
    pub fn from_notation(text: &str) -> Result<Self, NotationError> {
        let parse_result = text
            .split(',')
            .map(Move::try_from)
            .collect::<Result<Vec<Move>, NotationError>>();

        parse_result.map(|moves| Turn { moves })
    }

    /// Returns a reference to all moves that represent this turn in the order they should be made.
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
}

/// Allows strings of checkers notation to be easily converted into turn instances.
impl TryFrom<&str> for Turn {
    type Error = NotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Turn::from_notation(value)
    }
}

/// Allows array of any value that can be converted into a [Move]
/// to be easily converted into turn instances.
impl<T, const N: usize> TryFrom<[T; N]> for Turn
where
    T: Into<Move>,
{
    type Error = Infallible;

    fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
        Ok(Turn::new(value).unwrap())
    }
}
