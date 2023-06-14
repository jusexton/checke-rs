mod tests {
    use test_case::test_case;

    use checke_rs::moves::Move;

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
