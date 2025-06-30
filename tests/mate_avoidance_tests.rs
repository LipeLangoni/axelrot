// tests/mate_avoidance_tests.rs
// Test that axelrot avoids immediate mate threats
use axelrot::axelrot;
use chess::Board;
use std::str::FromStr;

#[test]
fn test_avoid_mate_in_one() {

    let fen = "6k1/5ppp/8/8/8/6R1/1r3PPP/6K1 w - - 0 1";
    let board = Board::from_str(fen).unwrap();

    let best_move = axelrot(&board, 2, 1000, 1000, 0, 0);

    let legal_saves = ["f2f3", "h1h3", "g1f1"];
    assert!(legal_saves.contains(&best_move.as_str()), "Engine should avoid mate, got {}", best_move);
}
