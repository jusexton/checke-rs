pub struct Board {
    pub to_move: PieceColor,
}

/// Represents the current status of a board instance.
#[derive(PartialEq, Debug)]
pub enum BoardStatus {
    /// The player to move still has valid moves that can be played and is therefor the game is
    /// still ongoing.
    OnGoing,

    /// The player to move no longer has any valid moves and therefore the game has been
    /// completed.
    Complete,
}

#[derive(PartialEq, Debug)]
pub enum PieceColor {
    Red,
    Black,
}

impl Default for Board {
    /// Creates a fresh board with pieces in starting positions.
    fn default() -> Self {
        Board {
            to_move: PieceColor::Black
        }
    }
}

impl Board {
    pub fn status(&self) -> BoardStatus {
        todo!()
    }

    pub fn make_move(&mut self) {
        todo!()
    }
}