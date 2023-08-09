use std::collections::VecDeque;

use thiserror::Error;

use crate::bitboard::{BitBoard, MonoBitBoard};
use crate::position::{MoveError, MoveIter, MoveValidator, Square};
use crate::turn::Turn;

pub const INITIAL_RED_PIECES: BitBoard = BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_10101010_01010101_10101010);
pub const INITIAL_BLACK_PIECES: BitBoard = BitBoard::new(0b01010101_10101010_01010101_00000000_00000000_00000000_00000000_00000000);
pub const INITIAL_KINGS: BitBoard = BitBoard::new(0);

/// Represents the current status of a board instance.
#[derive(Debug, PartialEq)]
pub enum BoardStatus {
    /// The player to move still has valid moves that can be played and is therefor the game is
    /// still ongoing.
    OnGoing,

    /// The player to move no longer have any valid moves and therefore the game has been
    /// completed.
    Complete { winner: Player },
}

/// Represents the player disc color
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Red,
    Black,
}

/// Represents the state a classical checkers board may be in.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardState {
    pub active_player: Player,
    pub red_pieces: BitBoard,
    pub black_pieces: BitBoard,
    pub kings: BitBoard,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            active_player: Player::Black,
            red_pieces: INITIAL_RED_PIECES,
            black_pieces: INITIAL_BLACK_PIECES,
            kings: INITIAL_KINGS,
        }
    }
}

impl BoardState {
    /// Creates an empty [BoardState] instance.
    pub fn empty() -> Self {
        Self {
            active_player: Player::Black,
            red_pieces: BitBoard::new(0),
            black_pieces: BitBoard::new(0),
            kings: BitBoard::new(0),
        }
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
            Player::Red => self.red_pieces(),
            Player::Black => self.black_pieces()
        }
    }

    /// Retrieves the player that is able to take a turn on this board instance.
    pub fn active_player(&self) -> Player { self.active_player }

    /// Determines the next player that will be active once a turn is successfully completed.
    pub fn next_player(&self) -> Player {
        match self.active_player {
            Player::Red => Player::Black,
            Player::Black => Player::Red
        }
    }

    /// Retrieves a bitboard representing where all pieces of the current player are on the board.
    pub fn current_player_pieces(&self) -> BitBoard {
        self.pieces_by_player(self.active_player())
    }

    /// Returns true if there is a piece occupying the given square and it belongs to the current
    /// player. Otherwise, returns false.
    pub fn is_current_player_piece(&self, bitboard: MonoBitBoard) -> bool {
        self.current_player_pieces() & bitboard != 0
    }

    /// Returns true if there is a piece occupying the given square. Otherwise, returns false.
    pub fn is_piece(&self, bitboard: MonoBitBoard) -> bool {
        self.all_pieces() & bitboard != 0
    }

    /// Retrieves a bitboard representing where all pieces are on the board.
    pub fn all_pieces(&self) -> BitBoard {
        self.red_pieces() | self.black_pieces()
    }

    /// Returns true if there is a red piece occupying the given square. Otherwise, returns false.
    pub fn is_red_piece(&self, bitboard: MonoBitBoard) -> bool {
        self.red_pieces() & bitboard != 0
    }

    /// Returns true if there is a black piece occupying the given square. Otherwise, returns false.
    pub fn is_black_piece(&self, bitboard: MonoBitBoard) -> bool {
        self.black_pieces() & bitboard != 0
    }

    /// Retrieves a bitboard representing where the red king pieces are on the board.
    pub fn red_kings(&self) -> BitBoard {
        self.red_pieces() & self.all_kings()
    }

    /// Retrieves a bitboard representing where the black king pieces are on the board.
    pub fn black_kings(&self) -> BitBoard {
        self.black_pieces() & self.all_pieces()
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

    /// Returns true if there is a king occupying the given square. Otherwise, returns false.
    pub fn is_king(&self, bitboard: MonoBitBoard) -> bool {
        self.all_kings() & bitboard != 0
    }
}

#[derive(Debug)]
pub struct Board {
    state_stack: VecDeque<BoardState>,
}

impl Default for Board {
    /// Creates a fresh board with pieces in starting positions.
    fn default() -> Self {
        let initial_state = BoardState::default();
        Board::new(initial_state)
    }
}

impl Board {
    pub(crate) fn new(initial_state: BoardState) -> Self {
        Board { state_stack: VecDeque::from([initial_state]) }
    }

    /// Creates an empty [Board] instance.
    pub fn empty() -> Self {
        let initial_state = BoardState::empty();
        Board::new(initial_state)
    }

    /// Returns the current board state
    pub fn current_state(&self) -> &BoardState {
        match self.state_stack.back() {
            Some(state) => state,
            // Unreachable due to the board always having at least a single state
            None => unreachable!()
        }
    }

    /// Calculates the current status of the game based on if the boards currently active player
    /// has any available moves to make.
    pub fn status(&self) -> BoardStatus {
        let current_state = self.current_state();
        let mut player_moves = MoveIter::new(current_state, current_state.active_player);
        match player_moves.next() {
            Some(_) => BoardStatus::OnGoing,
            None => BoardStatus::Complete { winner: current_state.next_player() }
        }
    }

    /// Similar to [Board::status] but provides a simple yes or no answer to if the game
    /// is still in progress.
    pub fn is_game_concluded(&self) -> bool {
        match self.status() {
            BoardStatus::Complete { .. } => true,
            BoardStatus::OnGoing => false
        }
    }

    /// Attempts to apply a turn to the game board, changing the state of the board if a valid
    /// turn is provided.
    pub fn push_turn<T>(&mut self, turn: T) -> Result<&BoardState, MoveError> where T: TryInto<Turn> {
        if self.is_game_concluded() {
            return Err(MoveError::GameConcluded);
        }

        let turn = turn.try_into().map_err(|_| MoveError::InvalidConstruction)?;
        let mut board_state = self.current_state().clone();
        for m in turn.moves() {
            let validator = MoveValidator::new(&board_state);
            validator.validate(m.clone())?;

            match board_state.active_player {
                Player::Red => {
                    board_state.red_pieces ^= m.to_bitboard()
                }
                Player::Black => {
                    board_state.black_pieces ^= m.to_bitboard();
                }
            }
        }

        board_state.active_player = board_state.next_player();
        self.state_stack.push_back(board_state);
        Ok(self.current_state())
    }

    /// Removes the last turn and returns the state of the board, or None if only the
    /// initial state remains on the stack.
    pub fn pop_turn(&mut self) -> Option<BoardState> {
        match self.state_stack.len() {
            // There should always be at least one state item on the stack.
            1 => None,
            _ => self.state_stack.pop_back()
        }
    }

    /// Returns a reference to the boards state stack. Useful for viewing the history of
    /// the board.
    pub fn state_stack(&self) -> &VecDeque<BoardState> {
        &self.state_stack
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum BoardCreationError {
    #[error("Only a single piece can be placed per square.")]
    DuplicateAssignments
}

#[derive(Debug)]
struct Placement {
    player: Player,
    square: Square,
    is_king: bool,
}

/// Used to create complex but safe board configurations with a builder pattern.
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
    pub fn piece(&mut self, player: Player, square: Square) -> &mut Self {
        let placement = Placement { player, square, is_king: false };
        self.placements.push(placement);
        self
    }

    /// Adds a king piece on the board.
    pub fn king(&mut self, player: Player, square: Square) -> &mut Self {
        let placement = Placement { player, square, is_king: true };
        self.placements.push(placement);
        self
    }

    /// Attempts to construct a new [Board] instance given the previous details.
    /// An error will be returned if during the build process multiple pieces were placed
    /// on the same square.
    pub fn build(&self) -> Result<Board, BoardCreationError> {
        let mut red_pieces = BitBoard::new(0);
        let mut black_pieces = BitBoard::new(0);
        let mut kings = BitBoard::new(0);

        for placement in &self.placements {
            let piece = MonoBitBoard::from(placement.square);
            if (red_pieces | black_pieces).contains(piece) {
                return Err(BoardCreationError::DuplicateAssignments);
            }

            match placement.player {
                Player::Red => { red_pieces = red_pieces | piece }
                Player::Black => { black_pieces = black_pieces | piece }
            }

            // TODO: If piece is placed where it should be kinged, it will remain a normal piece.
            //  Builder should be smart enough to automatically update these pieces to kings.
            if placement.is_king {
                kings = kings | piece
            }
        }

        let current_player = self.current_player;
        let initial_state = BoardState {
            active_player: current_player,
            red_pieces,
            black_pieces,
            kings,
        };
        let board = Board::new(initial_state);
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
