use checke_rs::bitboard::{BitBoard, MonoBitBoard};

#[test]
fn test_mono_bitboard_result_does_not_error_with_single_bit() {
    let result = MonoBitBoard::new(0b100);

    assert!(result.is_ok())
}

#[test]
fn test_mono_bitboard_result_is_error_with_many_bits() {
    let result = MonoBitBoard::new(0b101);

    assert!(result.is_err())
}

#[test]
fn test_sub_boards_are_calculated_correctly() {
    let position = MonoBitBoard::new(0b010).unwrap();
    let bitboard = BitBoard::new(0b10001000);

    assert!(!bitboard.contains(position))
}

#[test]
fn test_pieces_returns_correct_bitboards() {
    let bb = BitBoard::new(0b00100100001);

    let pieces = bb.pieces();

    let expected_pieces = vec![
        MonoBitBoard::new(0b00000000001).unwrap(),
        MonoBitBoard::new(0b00000100000).unwrap(),
        MonoBitBoard::new(0b00100000000).unwrap()

    ];
    assert_eq!(pieces, expected_pieces)
}