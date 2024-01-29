use checke_rs::bitboard::BitBoard;
use checke_rs::board::{
    Board, BoardBuilder, BoardCreationError, BoardState, BoardStatus, Player, INITIAL_KINGS,
    INITIAL_RED_PIECES,
};
use checke_rs::position::{MoveError, Square};

#[test]
#[ignore]
fn test_board_initialization() {
    let board = Board::default();

    assert_eq!(board.current_state().current_player, Player::Black);
    assert_eq!(board.status(), BoardStatus::OnGoing);
}

#[test]
fn test_push_turn_with_single_move() {
    let mut board = Board::default();

    let board_state = board.push_turn("11x15").unwrap();

    assert_eq!(board_state.current_player, Player::Red);
    assert_eq!(board_state.red_pieces, INITIAL_RED_PIECES);
    assert_eq!(
        board_state.black_pieces,
        BitBoard::new(0b01010101_10101010_01010001_00001000_00000000_00000000_00000000_00000000)
    );
    assert_eq!(board_state.kings, INITIAL_KINGS);
}

#[test]
fn test_push_turn_with_many_moves() {
    let mut board = Board::default();

    let board_state = board.push_turn("11x16").unwrap();
    assert_eq!(board_state.current_player, Player::Red);
    assert_eq!(board_state.red_pieces, INITIAL_RED_PIECES);
    assert_eq!(
        board_state.black_pieces,
        BitBoard::new(0b01010101_10101010_01010001_00000010_00000000_00000000_00000000_00000000)
    );
    assert_eq!(board_state.kings, INITIAL_KINGS);

    let board_state = board.push_turn("24x19").unwrap();
    assert_eq!(board_state.current_player, Player::Black);
    assert_eq!(
        board_state.red_pieces,
        BitBoard::new(0b00000000_00000000_00000000_00000000_00000100_10101000_01010101_10101010)
    );
    assert_eq!(
        board_state.black_pieces,
        BitBoard::new(0b01010101_10101010_01010001_00000010_00000000_00000000_00000000_00000000)
    );
    assert_eq!(board_state.kings, INITIAL_KINGS);
}

#[test]
fn test_pop_turn() {
    let mut board = Board::default();

    let board_state = board.push_turn("11x16").unwrap();
    assert_eq!(board_state.current_player, Player::Red);
    assert_eq!(board_state.red_pieces, INITIAL_RED_PIECES);
    assert_eq!(
        board_state.black_pieces,
        BitBoard::new(0b01010101_10101010_01010001_00000010_00000000_00000000_00000000_00000000)
    );
    assert_eq!(board_state.kings, INITIAL_KINGS);

    let board_state = board.pop_turn().unwrap();
    assert_eq!(board_state.current_player, Player::Red);
    assert_eq!(board_state.red_pieces, INITIAL_RED_PIECES);
    assert_eq!(
        board_state.black_pieces,
        BitBoard::new(0b01010101_10101010_01010001_00000010_00000000_00000000_00000000_00000000)
    );
    assert_eq!(board_state.kings, INITIAL_KINGS);

    assert_eq!(board.current_state(), &BoardState::default())
}

#[test]
fn test_pop_turn_returns_none_when_no_turns_left_to_pop() {
    let mut board = Board::default();

    let board_state = board.pop_turn();

    assert!(board_state.is_none())
}

#[test]
fn test_push_turn_with_destination_occupied_error() {
    let mut board = Board::default();

    let result = board.push_turn("1x6");

    let error = result
        .expect_err("Expected error to occur when moving a piece to an already occupied square.");
    assert_eq!(error, MoveError::DestinationOccupied)
}

#[test]
fn test_push_turn_with_wrong_player_piece_error() {
    let mut board = Board::default();

    let result = board.push_turn("23x18");

    let error = result.expect_err("Expected error to occur when selecting incorrect player piece.");
    assert_eq!(error, MoveError::WrongPlayerPiece)
}

#[test]
fn test_push_turn_with_no_player_piece_error() {
    let mut board = Board::default();

    let result = board.push_turn("18x15");

    let error = result.expect_err(
        "Expected error to occur when selecting a square that does not contain a piece.",
    );
    assert_eq!(error, MoveError::NoPieceAtSource)
}

#[test]
fn test_simple_board_creation() {
    let board = BoardBuilder::default()
        .current_player(Player::Red)
        .piece(Player::Red, Square::Six)
        .piece(Player::Black, Square::Eighteen)
        .king(Player::Black, Square::Eight)
        .build()
        .unwrap();

    let current_state = board.current_state();
    assert_eq!(current_state.current_player, Player::Red);

    let expected_black_pieces =
        BitBoard::new(0b00000000_00000010_00000000_00000000_00010000_00000000_00000000_00000000);
    assert_eq!(current_state.black_pieces(), expected_black_pieces);

    let expected_red_pieces =
        BitBoard::new(0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert_eq!(current_state.red_pieces(), expected_red_pieces);

    let expected_kings =
        BitBoard::new(0b00000000_00000010_00000000_00000000_00000000_00000000_00000000_00000000);
    assert_eq!(current_state.all_kings(), expected_kings);
}

#[test]
fn test_board_builder_returns_error_when_multiple_placements_on_same_square() {
    let build_result = BoardBuilder::default()
        .piece(Player::Red, Square::Six)
        .piece(Player::Black, Square::Six)
        .build();

    assert_eq!(
        build_result.unwrap_err(),
        BoardCreationError::DuplicateAssignments
    );
}
