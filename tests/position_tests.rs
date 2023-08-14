use checke_rs::bitboard::MonoBitBoard;
use checke_rs::board::{BoardState, Player};
use checke_rs::position::{Move, MoveError, MoveIter, MoveValidator, Square};

#[test]
fn test_capturing_move_creates_capture() {
    let m = Move::from_notation("1x10").unwrap();

    let capture = m.capture();

    let piece = capture.expect("Expected capture piece to exist.");
    assert_eq!(piece, MonoBitBoard::from(Square::Six));
}

#[test]
fn test_non_capturing_move_does_not_create_capture() {
    let m = Move::from_notation("1x6").unwrap();

    let capture = m.capture();

    assert!(capture.is_none())
}

#[test]
fn test_no_moves_are_generated_from_empty_board() {
    let board_state = BoardState::empty();
    let moves = MoveIter::new(&board_state, Player::Red);

    assert_eq!(moves.count(), 0)
}

#[test]
fn test_move_with_valid_source_and_destination_is_ok() {
    let board_state = BoardState::default();
    let validator = MoveValidator::new(&board_state);

    let result = validator.validate("10x14");

    assert!(result.is_ok())
}

#[test]
fn test_move_with_valid_source_and_invalid_destination_is_error() {
    let board_state = BoardState::default();
    let validator = MoveValidator::new(&board_state);

    let result = validator.validate("10x16");

    let err = result.expect_err("Expected error when destination was not legal.");
    assert_eq!(err, MoveError::IllegalDestination);
}

#[test]
fn test_selecting_wrong_piece_is_error() {
    let board_state = BoardState::default();
    let validator = MoveValidator::new(&board_state);

    let result = validator.validate("23x18");

    let err = result.expect_err("Expected error when incorrect piece is selected.");
    assert_eq!(err, MoveError::WrongPlayerPiece);
}

#[test]
fn test_selecting_source_with_no_piece_is_error() {
    let board_state = BoardState::default();
    let validator = MoveValidator::new(&board_state);

    let result = validator.validate("18x15");

    let err = result.expect_err("Expected error when no piece was selected.");
    assert_eq!(err, MoveError::NoPieceAtSource);
}
