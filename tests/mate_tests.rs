// tests/mate_tests.rs
// Test that axelrot finds mate in one
use axelrot::axelrot;
use chess::Board;
use std::str::FromStr;

#[test]
fn test_mate_in_one() {

    let fen = "6k1/4Rppp/8/8/8/8/5PPP/6K1 w - - 0 1";
    let board = Board::from_str(fen).unwrap();

    let best_move = axelrot(&board, 2, 1000, 1000, 0, 0);
    assert_eq!(best_move, "e7e8", "Engine should find mate in one");
}
