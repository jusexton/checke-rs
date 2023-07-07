use checke_rs::bitboard::BitBoard;
use checke_rs::board::{Board, BoardBuilder, BoardCreationError, BoardStatus, Player};
use checke_rs::position::Square;
use checke_rs::turn::Turn;

#[test]
#[ignore]
fn test_board_initialization() {
    let board = Board::default();

    assert_eq!(board.current_player(), Player::Black);
    assert_eq!(board.status(), BoardStatus::OnGoing);
}

#[test]
#[ignore]
fn test_push_turn() {
    let mut board = Board::default();

    let turn = Turn::new(vec![]);
    let result = board.push_turn(turn);
}

#[test]
fn test_simple_board_creation() {
    let board = BoardBuilder::default()
        .place_piece(Player::Red, Square::Six)
        .place_piece(Player::Black, Square::Eighteen)
        .place_king(Player::Black, Square::Eight)
        .build().unwrap();

    let expected_black_pieces = BitBoard::new(0b00000000_00000010_00000000_00000000_00010000_00000000_00000000_00000000);
    assert_eq!(board.black_pieces(), expected_black_pieces);

    let expected_red_pieces = BitBoard::new(0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert_eq!(board.red_pieces(), expected_red_pieces);

    let expected_kings = BitBoard::new(0b00000000_00000010_00000000_00000000_00000000_00000000_00000000_00000000);
    assert_eq!(board.all_kings(), expected_kings);
}

#[test]
fn test_board_builder_returns_error_when_multiple_placement_on_same_square() {
    let build_result = BoardBuilder::default()
        .place_piece(Player::Red, Square::Six)
        .place_piece(Player::Black, Square::Six)
        .build();

    assert_eq!(build_result.unwrap_err(), BoardCreationError::DuplicateSquareAssignments);
}