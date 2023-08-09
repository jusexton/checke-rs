use checke_rs::bitboard::{BitBoard, MonoBitBoard, CellIter};

#[test]
fn test_bitboard_equals_u64() {
    let value = 0b101;
    let bitboard = BitBoard::new(value);
    assert_eq!(bitboard, value)
}

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

    let pieces = bb.used_cells().collect::<Vec<MonoBitBoard>>();

    let expected_pieces = vec![
        MonoBitBoard::new(0b00000000001).unwrap(),
        MonoBitBoard::new(0b00000100000).unwrap(),
        MonoBitBoard::new(0b00100000000).unwrap()
    ];
    assert_eq!(pieces, expected_pieces)
}

#[test]
fn test_cell_iter() {
    let bitboard = BitBoard::new(0b10000000_00000000_00000000_00000000_00000000_00000000_00000001);
    let pieces = CellIter::new(bitboard);

    assert_eq!(pieces.count(), 2)
}

#[test]
fn test_cell_with_all_empty_cells() {
    let bitboard = BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    let pieces = CellIter::new(bitboard);

    assert_eq!(pieces.count(), 0)
}
