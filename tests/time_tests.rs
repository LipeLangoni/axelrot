use axelrot::axelrot;
use chess::Board;
use std::time::Instant;

#[test]
fn test_time_management_short_vs_long() {
    let board = Board::default();
    let depth = 10;
    let winc = 0;
    let binc = 0;
    // Short time: 50ms
    let wtime_short = 50;
    let btime_short = 50;
    // Long time: 500ms
    let wtime_long = 500;
    let btime_long = 500;

    let start_short = Instant::now();
    let _ = axelrot(&board, depth, wtime_short, btime_short, winc, binc);
    let elapsed_short = start_short.elapsed();

    let start_long = Instant::now();
    let _ = axelrot(&board, depth, wtime_long, btime_long, winc, binc);
    let elapsed_long = start_long.elapsed();

    assert!(elapsed_long > elapsed_short, "Long search should take longer than short search");

    assert!(elapsed_short.as_millis() < 1000, "Short search exceeded expected time");
    assert!(elapsed_long.as_millis() < 5000, "Long search exceeded expected time");
}
