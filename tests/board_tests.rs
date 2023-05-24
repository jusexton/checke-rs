use checke_rs::board::{Board, BoardStatus, PieceColor};

#[test]
fn test_board_initialization() {
    let board = Board::default();

    assert_eq!(board.to_move, PieceColor::Black);
    assert_eq!(board.status(), BoardStatus::OnGoing);
}