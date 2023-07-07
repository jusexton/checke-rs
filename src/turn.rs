use crate::position::{Move, NotationError};

/// Construct representing many moves at once. Necessary due to checkers allowing multiple
/// jumps per turn.
pub struct Turn {
    moves: Vec<Move>,
}

impl Turn {
    /// Creates a [Turn] instance from the given move vector.
    pub fn new(moves: Vec<Move>) -> Self {
        Turn { moves }
    }

    /// Attempts tp create  a [Turn] instance using checkers notation.
    /// ```rust
    /// use checke_rs::turn::Turn;
    ///
    /// let turn = Turn::try_from("9x18,18x25").unwrap();
    ///
    /// assert_eq!(turn.moves().len(), 2)
    /// ```
    pub fn from_notation(text: &str) -> Result<Self, NotationError> {
        let parse_result: Result<Vec<Move>, NotationError> = text.split(',')
            .map(Move::try_from)
            .collect();

        parse_result.map(|moves| Turn { moves })
    }

    pub fn moves(&self) -> Vec<Move> {
        self.moves.to_vec()
    }
}

impl TryFrom<&str> for Turn {
    type Error = NotationError;

    /// Attempts to convert string slice into a [Turn] instance using checkers notation.
    /// ```rust
    /// use checke_rs::turn::Turn;
    ///
    /// let turn = Turn::try_from("9x18,18x25").unwrap();
    ///
    /// assert_eq!(turn.moves().len(), 2)
    /// ```
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Turn::from_notation(text)
    }
}
