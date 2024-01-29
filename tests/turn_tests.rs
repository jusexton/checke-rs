use checke_rs::position::{Move, Square};
use checke_rs::turn::Turn;

#[test]
fn test_turn_created_correctly_from_strings() {
    let moves = ["11x15", "15x18"];
    let turn = Turn::new(moves).unwrap();

    let expected_moves = &vec![
        Move::from_squares(Square::Eleven, Square::Fifteen),
        Move::from_squares(Square::Fifteen, Square::Eighteen),
    ];
    assert_eq!(turn.moves(), expected_moves)
}

#[test]
fn test_turn_created_correctly_from_squares() {
    let moves = [
        (Square::Eleven, Square::Fifteen),
        (Square::Fifteen, Square::Eighteen),
    ];
    let turn = Turn::new(moves).unwrap();

    let expected_moves = &vec![
        Move::from_squares(Square::Eleven, Square::Fifteen),
        Move::from_squares(Square::Fifteen, Square::Eighteen),
    ];
    assert_eq!(turn.moves(), expected_moves)
}

#[test]
fn test_turn_created_correctly_from_notation() {
    let notation = "11x15,15x18";
    let turn = Turn::from_notation(notation).unwrap();

    let expected_moves = &vec![
        Move::from_squares(Square::Eleven, Square::Fifteen),
        Move::from_squares(Square::Fifteen, Square::Eighteen),
    ];
    assert_eq!(turn.moves(), expected_moves)
}
