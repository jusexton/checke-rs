use crate::error::NotationError;
use crate::moves::Move;

/// Construct representing many moves at once. Necessary due to checkers allowing multiple
/// jumps per turn.
pub struct Turn {
    pub moves: Vec<Move>,
}

impl Turn {
    /// Creates a [Turn] instance from the given move vector.
    pub fn new(moves: Vec<Move>) -> Self {
        Turn { moves }
    }
}

impl TryFrom<&str> for Turn {
    type Error = NotationError;

    /// Create a [Turn] instance using checkers notation.
    /// ```rust
    /// use checke_rs::turn::Turn;
    ///
    /// let turn = Turn::try_from("9x18,18x25").unwrap();
    ///
    /// assert_eq!(turn.moves.len(), 2)
    /// ```
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let parse_result: Result<Vec<Move>, NotationError> = text.split(',')
            .map(Move::try_from)
            .collect();

        parse_result.map(|moves| Turn { moves })
    }
}
