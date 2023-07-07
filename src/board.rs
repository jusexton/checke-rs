use thiserror::Error;

use crate::bitboard::BitBoard;
use crate::position::{MoveIter, Square};
use crate::turn::Turn;

const INITIAL_RED_PIECES: BitBoard = BitBoard::new(0x2AD5AA);
const INITIAL_BLACK_PIECES: BitBoard = BitBoard::new(0x156AD50000000000);
const INITIAL_KINGS: BitBoard = BitBoard::new(0x0);

#[derive(Debug, Error)]
pub enum MoveError {}

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

#[derive(Debug)]
pub struct Board {
    current_player: Player,
    red_pieces: BitBoard,
    black_pieces: BitBoard,
    kings: BitBoard,
}

impl Board {
    /// Creates an empty [Board] instance.
    pub fn empty() -> Self {
        Board {
            red_pieces: BitBoard::new(0),
            black_pieces: BitBoard::new(0),
            kings: BitBoard::new(0),
            ..Board::default()
        }
    }

    /// Retrieves the player that is able to take a turn on this board instance.
    pub fn current_player(&self) -> Player { self.current_player }

    /// Calculates the current status of the game based on if the [Board::current_player] currently
    /// has any available moves to make.
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

    /// Retrieves a bitboard representing where all red pieces are on the board.
    pub fn red_pieces(&self) -> BitBoard {
        self.red_pieces
    }

    /// Retrieves a bitboard representing where all black pieces are on the board.
    pub fn black_pieces(&self) -> BitBoard {
        self.black_pieces
    }

    /// Retrieves a bitboard representing where all pieces of a specified player are on the board.
    pub fn pieces_by_player(&self, player: Player) -> BitBoard {
        match player {
            Player::Red => self.red_pieces,
            Player::Black => self.black_pieces
        }
    }

    /// Retrieves a bitboard representing where all pieces are on the board.
    pub fn all_pieces(&self) -> BitBoard {
        self.red_pieces | self.black_pieces
    }

    /// Retrieves a bitboard representing where the red king pieces are on the board.
    pub fn red_kings(&self) -> BitBoard {
        self.red_pieces & self.kings
    }

    /// Retrieves a bitboard representing where the black king pieces are on the board.
    pub fn black_kings(&self) -> BitBoard {
        self.black_pieces & self.kings
    }

    /// Retrieves a bitboard representing where all kings of a specified player are on the board.
    pub fn kings_by_player(&self, player: Player) -> BitBoard {
        match player {
            Player::Red => self.red_kings(),
            Player::Black => self.black_kings()
        }
    }

    /// Retrieves a bitboard representing all king pieces are on the board.
    pub fn all_kings(&self) -> BitBoard { self.kings }

    /// Provides either a positive or negative 1 denoting which direction the current player
    /// should be progressing on the board. A positive 1 represents the current player should be
    /// moving down the board or towards the larger square values. Vice versa with -1.
    pub fn direction(&self) -> i8 {
        match self.current_player {
            Player::Red => 1,
            Player::Black => -1
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum BoardCreationError {
    #[error("Only a single piece can be placed per square.")]
    DuplicateSquareAssignments
}

#[derive(Debug)]
struct Placement {
    player: Player,
    square: Square,
    is_king: bool,
}

/// Used to create complex board configurations with reasonable safety.
#[derive(Debug)]
pub struct BoardBuilder {
    current_player: Player,
    placements: Vec<Placement>,
}

impl BoardBuilder {
    /// Sets the current player the board will be built with.
    pub fn current_player(&mut self, player: Player) -> &mut Self {
        self.current_player = player;
        self
    }

    /// Adds a normal piece on the board.
    pub fn place_piece(&mut self, player: Player, square: Square) -> &mut Self {
        let placement = Placement { player, square, is_king: false };
        self.placements.push(placement);
        self
    }

    /// Adds a king piece on the board.
    pub fn place_king(&mut self, player: Player, square: Square) -> &mut Self {
        let placement = Placement { player, square, is_king: true };
        self.placements.push(placement);
        self
    }

    /// Attempts to construct a new [Board] instance given the previous details.
    /// An error will be returned if during the build process multiple pieces were placed
    /// on the same square.
    pub fn build(&self) -> Result<Board, BoardCreationError> {
        let current_player = self.current_player;
        let mut red_pieces = BitBoard::new(0);
        let mut black_pieces = BitBoard::new(0);
        let mut kings = BitBoard::new(0);
        for placement in &self.placements {
            let piece = placement.square.to_bitboard();
            if (red_pieces | black_pieces).contains(piece) {
                return Err(BoardCreationError::DuplicateSquareAssignments);
            }

            match placement.player {
                Player::Red => { red_pieces = red_pieces | piece }
                Player::Black => { black_pieces = black_pieces | piece }
            }

            if placement.is_king {
                kings = kings | piece
            }
        }

        let board = Board { current_player, red_pieces, black_pieces, kings };
        Ok(board)
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        BoardBuilder {
            current_player: Player::Black,
            placements: vec![],
        }
    }
}
