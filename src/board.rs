use thiserror::Error;

use crate::bitboard::BitBoard;
use crate::moves::MoveIter;
use crate::turn::Turn;

const INITIAL_RED_PIECES: BitBoard = BitBoard(0x2AD5AA);
const INITIAL_BLACK_PIECES: BitBoard = BitBoard(0x156AD50000000000);
const INITIAL_KINGS: BitBoard = BitBoard(0x0);

#[derive(Debug, Error)]
pub enum MoveError {

}

pub struct Board {
    current_player: Player,
    red_pieces: BitBoard,
    black_pieces: BitBoard,
    kings: BitBoard,
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

/// Represents the player disc color
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Player {
    Red,
    Black,
}

impl Default for Board {
    /// Creates a fresh board with pieces in starting positions.
    fn default() -> Self {
        Board {
            current_player: Player::Black,
            red_pieces: INITIAL_RED_PIECES,
            black_pieces: INITIAL_BLACK_PIECES,
            kings: INITIAL_KINGS,
        }
    }
}

impl Board {
    /// The current status of the game.
    pub fn status(&self) -> BoardStatus {
        let mut player_moves = MoveIter::new(self, self.current_player);
        match player_moves.next() {
            Some(_) => BoardStatus::OnGoing,
            None => BoardStatus::Complete
        }
    }

    /// Applies a turn to the game board and changes the state of the board and
    /// the position of its pieces.
    pub fn push_turn(&mut self, turn: Turn) -> Result<(), MoveError> {
        todo!()
    }

    /// Pops the most recent turn off the turn stack
    pub fn pop_turn(&mut self) {
        todo!()
    }

    /// The player who's turn it is to make a move.
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Retrieves a bitboard representing where the red pieces are on the board.
    pub fn red_pieces(&self) -> BitBoard {
        self.red_pieces
    }

    /// Retrieves a bitboard representing where the black pieces are on the board.
    pub fn black_pieces(&self) -> BitBoard {
        self.black_pieces
    }

    /// Retrieves a bitboard representing where the red king pieces are on the board.
    pub fn red_kings(&self) -> BitBoard {
        self.red_pieces & self.kings
    }

    /// Retrieves a bitboard representing where the black king pieces are on the board.
    pub fn black_kings(&self) -> BitBoard {
        self.black_pieces & self.kings
    }

    /// Retrieves a bitboard representing where all king pieces are on the board.
    pub fn all_kings(&self) -> BitBoard {
        self.kings
    }

    /// Retrieves a bitboard representing where all pieces of a specified player are on the board.
    pub fn pieces_by_player(&self, player: Player) -> BitBoard {
        match player {
            Player::Red => self.red_pieces,
            Player::Black => self.black_pieces
        }
    }

    pub fn kings_by_player(&self, player: Player) -> BitBoard {
        match player {
            Player::Red => self.red_kings(),
            Player::Black => self.black_kings()
        }
    }

    /// Retrieves a bitboard representing where all pieces are on the board.
    pub fn all_pieces(&self) -> BitBoard {
        self.red_pieces | self.black_pieces
    }
}