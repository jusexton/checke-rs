use checke_rs::board::{Board, BoardStatus, Player};
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