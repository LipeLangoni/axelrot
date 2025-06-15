use chess::Board;
use std::str::FromStr;
use axelrot::evaluation;

#[test]
fn test_eval_black_queen_missing() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1";
    let board = Board::from_str(fen).unwrap();
    assert_eq!(evaluation(&board), -9);
}

#[test]
fn test_eval_white_queen_missing() {
    let fen = "rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_str(fen).unwrap();
    assert_eq!(evaluation(&board), 9);
}
