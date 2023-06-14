use checke_rs::bitboard::BitBoard;

#[test]
fn test_single_piece_bitboard() {
    let bb = BitBoard(8);

    assert!(bb.contains_single_piece())
}

#[test]
fn test_non_single_piece_bitboard() {
    let bb = BitBoard(14);

    assert!(!bb.contains_single_piece())
}