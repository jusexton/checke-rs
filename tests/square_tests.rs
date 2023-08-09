use checke_rs::bitboard::MonoBitBoard;
use checke_rs::position::{NotationError, Square};

#[test]
fn test_square_converts_from_valid_u8() {
    let square = Square::try_from(1).unwrap();

    assert_eq!(Square::One, square)
}

#[test]
fn test_square_errors_from_invalid_u8() {
    let result = Square::try_from(0);

    let err = result.expect_err("Expected NotationError when u8 is out of range.");
    assert_eq!(NotationError::OutOfRange, err)
}

#[test]
fn test_square_converts_from_valid_string_slice() {
    let square = Square::try_from("1").unwrap();

    assert_eq!(Square::One, square)
}

#[test]
fn test_to_number() {
    let square = Square::One;

    let number = square.to_number();

    assert_eq!(number, 1)
}

#[test]
fn test_square_iter() {
    let square_count = Square::iter().count();

    assert_eq!(square_count, 32)
}

#[test]
fn test_square_one_produces_correct_bitboard() {
    let bb = MonoBitBoard::from(Square::One);

    let value = 0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    let expected_bb = MonoBitBoard::new(value).unwrap();
    assert_eq!(bb, expected_bb)
}

#[test]
fn test_square_thirty_two_produces_correct_bitboard() {
    let bb = MonoBitBoard::from(Square::ThirtyTwo);

    let value = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010;
    let expected_bb = MonoBitBoard::new(value).unwrap();
    assert_eq!(bb, expected_bb)
}

mod move_tests {
    use test_case::test_case;

    use checke_rs::position::Move;

    #[test_case("9x12")]
    #[test_case("10-12")]
    #[test_case("10x14")]
    fn test_move_created_from_valid_checkers_notation(text: &str) {
        let is_valid = Move::try_from(text).is_ok();
        assert!(is_valid)
    }

    #[test_case("")]
    #[test_case("a")]
    #[test_case("a-b")]
    #[test_case("cxd")]
    #[test_case("09x10")]
    fn test_move_error_from_invalid_checkers_notation(text: &str) {
        let is_error = Move::try_from(text).is_err();
        assert!(is_error)
    }
}
