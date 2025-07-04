use chess::{Board, Color};
use std::io;
use axelrot::{evaluation, axelrot};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut board = Board::default();
    let mut debug_mode = false;
    let options = [
        "option name Nullmove type check default true",
        "option name Selectivity type spin default 2 min 0 max 4",
        "option name Style type combo default Normal var Solid var Normal var Risky",
        "option name NalimovPath type string default c:\\n",
        "option name Clear Hash type button",
        "option name UCI_ShowCurrLine type check default false",
        "option name UCI_ShowRefutations type check default false",
        "option name UCI_LimitStrength type check default false",
        "option name UCI_Elo type spin default 2000 min 1350 max 2850",
        "option name UCI_AnalyseMode type check default false",
        "option name UCI_Opponent type string default none none computer Unknown",
        "option name UCI_EngineAbout type string default Axelrot by felipelangoni, see github.com/felipelangoni/axelrot",
        "option name UCI_ShredderbasesPath type string default <empty>",
        "option name UCI_SetPositionValue type string default <empty>"
    ];
    let mut sent_registration = false;
    let mut sent_copyprotection = false;
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        if debug_mode {
            println!("info string received: {}", input);
        }
        if input == "d" || input == "D" {
            println!("{}", board);
            continue;
        } else if input == "uci" {
            println!("id name axelrot");
            println!("id author felipelangoni");
            for opt in &options {
                println!("{}", opt);
            }
            if !sent_copyprotection {
                println!("copyprotection ok");
                sent_copyprotection = true;
            }
            if !sent_registration {
                println!("registration ok");
                sent_registration = true;
            }
            println!("uciok");
        } else if input.starts_with("debug ") {
            let arg = input[6..].trim();
            debug_mode = arg.eq_ignore_ascii_case("on");
            println!("info string debug mode {}", if debug_mode {"on"} else {"off"});
        } else if input == "isready" {
            println!("readyok");
        } else if input.starts_with("setoption ") {
            println!("info string setoption received: {}", input);
        } else if input.starts_with("register") {
            println!("registration checking");
            println!("registration ok");
        } else if input == "ucinewgame" {
            board = Board::default();
            println!("info string ucinewgame received");
        } else if input.starts_with("position ") {
            let rest = input.strip_prefix("position ").unwrap();
            if rest.starts_with("startpos") {
                board = Board::default();
                if let Some(moves_str) = rest.strip_prefix("startpos moves ") {
                    for mv_str in moves_str.split_whitespace() {
                        if let Ok(mv) = mv_str.parse() {
                            board = board.make_move_new(mv);
                        }
                    }
                }
            } else if rest.starts_with("fen ") {
                let mut parts = rest[4..].splitn(2, " moves ");
                let fen = parts.next().unwrap().trim();
                if let Ok(fen_board) = Board::from_str(fen) {
                    board = fen_board;
                }
                if let Some(moves_str) = parts.next() {
                    for mv_str in moves_str.split_whitespace() {
                        if let Ok(mv) = mv_str.parse() {
                            board = board.make_move_new(mv);
                        }
                    }
                }
            }
        } else if input == "eval" {
            let stm = match board.side_to_move() {
                Color::White => "White",
                Color::Black => "Black",
            };
            let score = evaluation(&board);
            println!("info string eval: side to move: {}, score: {}", stm, score);
        } else if input.starts_with("go") {

            let mut wtime = 300_000u64;
            let mut btime = 300_000u64;
            let mut winc = 0u64;
            let mut binc = 0u64;
            let mut depth = 20;
            for token in input.split_whitespace() {
                if token == "wtime" {
                    wtime = input.split_whitespace().skip_while(|&t| t != "wtime").nth(1).and_then(|v| v.parse().ok()).unwrap_or(wtime);
                } else if token == "btime" {
                    btime = input.split_whitespace().skip_while(|&t| t != "btime").nth(1).and_then(|v| v.parse().ok()).unwrap_or(btime);
                } else if token == "winc" {
                    winc = input.split_whitespace().skip_while(|&t| t != "winc").nth(1).and_then(|v| v.parse().ok()).unwrap_or(winc);
                } else if token == "binc" {
                    binc = input.split_whitespace().skip_while(|&t| t != "binc").nth(1).and_then(|v| v.parse().ok()).unwrap_or(binc);
                } else if token == "depth" {
                    depth = input.split_whitespace().skip_while(|&t| t != "depth").nth(1).and_then(|v| v.parse().ok()).unwrap_or(depth);
                }
            }
            let best_move = axelrot(&board, depth, wtime, btime, winc, binc);
            println!("bestmove {}", best_move);
        } else if input == "stop" {
            println!("info string stop received");
        } else if input == "ponderhit" {
            println!("info string ponderhit received");
        } else if input == "quit" {
            break;
        }
    }
}





