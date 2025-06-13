use chess::{Board, Piece, Color, ALL_PIECES, MoveGen, ChessMove};

fn main() {
    uci();
}
use std::io::{self, BufRead, Write};

fn uci() {
    let stdin = io::stdin();
    let mut board = Board::default();
    println!("id name axelrot");
    println!("id author felipelangoni");
    println!("uciok");
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        if input == "uci" {
            println!("id name axelrot");
            println!("id author felipelangoni");
            println!("uciok");
        } else if input == "isready" {
            println!("readyok");
        } else if input.starts_with("position startpos") {
            board = Board::default();
            if let Some(moves_str) = input.strip_prefix("position startpos moves ") {
                for mv_str in moves_str.split_whitespace() {
                    if let Ok(mv) = mv_str.parse() {
                        board = board.make_move_new(mv);
                    }
                }
            }
        } else if input.starts_with("go") {
            let best_move = axelrot(&board, 3);
            println!("bestmove {}", best_move);
        } else if input == "quit" {
            break;
        }
    }
}

fn axelrot(board: &Board, depth: i32) -> String {
    let mut best_value = i32::MIN + 1;
    let mut best_move = None;
    let mut alpha = i32::MIN + 1;
    let beta = i32::MAX;
    for m in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(m);
        let value = -negamax(&new_board, -beta, -alpha, depth - 1);
        if value > best_value {
            best_value = value;
            best_move = Some(m);
        }
        if best_value > alpha {
            alpha = best_value;
        }
    }
    if let Some(mv) = best_move {
        mv.to_string()
    } else {
        "0000".to_string()
    }
}

use std::collections::HashMap;


fn negamax(board: &Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    if depth == 0 {
        return evaluation(board);
    }
    let mut best_value = i32::MIN + 1;
    for mv in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mv);
        let value = -negamax(&new_board, -beta, -alpha, depth - 1);
        if value > best_value {
            best_value = value;
        }
        if best_value > alpha {
            alpha = best_value;
        }
        if alpha >= beta {
            break;
        }
    }
    best_value
}



fn evaluation(board: &Board) -> i32 {
    // Piece values for evaluation
    let mut piece_values = HashMap::new();
    piece_values.insert(Piece::Pawn, 1);
    piece_values.insert(Piece::Knight, 3);
    piece_values.insert(Piece::Bishop, 3);
    piece_values.insert(Piece::Rook, 5);
    piece_values.insert(Piece::Queen, 9);
    piece_values.insert(Piece::King, 0); // King is not counted in evaluation

    // Piece-square tables for all pieces (values for White; mirror for Black)
    let pawn_pst: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        5, 10, 10, -40, -40, 10, 10, 5,
        5, -5, -10, 0, 0, -10, -5, 5,
        0, 0, 0, 50, 50, 0, 0, 0,
        5, 5, 10, 25, 25, 10, 5, 5,
        10, 10, 20, 30, 30, 20, 10, 10,
        50, 50, 50, 50, 50, 50, 50, 50,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let knight_pst: [i32; 64] = [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20, 0, 5, 5, 0, -20, -40,
        -30, 5, 10, 15, 15, 10, 5, -30,
        -30, 0, 15, 20, 20, 15, 0, -30,
        -30, 5, 15, 20, 20, 15, 5, -30,
        -30, 0, 10, 15, 15, 10, 0, -30,
        -40, -20, 0, 0, 0, 0, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50
    ];
    let bishop_pst: [i32; 64] = [
        -20, -10, -40, -10, -10, -40, -10, -20,
        -10, 5, 0, 0, 0, 0, 5, -10,
        -10, 10, 10, 10, 10, 10, 10, -10,
        -10, 0, 20, 10, 10, 20, 0, -10,
        -10, 5, 5, 10, 10, 5, 5, -10,
        -10, 0, 5, 10, 10, 5, 0, -10,
        -10, 0, 0, 0, 0, 0, 0, -10,
        -20, -10, -40, -10, -10, -40, -10, -20
    ];
    let rook_pst: [i32; 64] = [
        0, 0, 0, 5, 5, 0, 0, 0,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        -5, 0, 0, 0, 0, 0, 0, -5,
        5, 10, 10, 10, 10, 10, 10, 5,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let queen_pst: [i32; 64] = [
        -20, -10, -10, -5, -5, -10, -10, -20,
        -10, 0, 0, 0, 0, 0, 0, -10,
        -10, 5, 5, 5, 5, 5, 0, -10,
        0, 0, 5, 5, 5, 5, 0, -5,
        -5, 0, 5, 5, 5, 5, 0, -5,
        -10, 0, 5, 5, 5, 5, 0, -10,
        -10, 0, 0, 0, 0, 0, 0, -10,
        -20, -10, -10, -5, -5, -10, -10, -20
    ];
    let king_pst: [i32; 64] = [
        20, 30, 10, 0, 0, 10, 30, 20,
        20, 20, -10, -10, -10, -10, 20, 20,
        -10, -20, -20, -20, -20, -20, -20, -10,
        -20, -30, -30, -40, -40, -30, -30, -20,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30
    ];

    // Commented out to avoid printing during search
    // println!("\nPiece counts:");
    let mut eval_white = 0;
    let mut eval_black = 0;
    for &color in &[Color::White, Color::Black] {
        let color_name = match color {
            Color::White => "White",
            Color::Black => "Black",
        };
        // print!("{}: ", color_name);
        for piece in ALL_PIECES {
            let count = board.pieces(piece) & board.color_combined(color);
            let num = count.popcnt();
            let piece_name = match piece {
                Piece::King => "K",
                Piece::Queen => "Q",
                Piece::Rook => "R",
                Piece::Bishop => "B",
                Piece::Knight => "N",
                Piece::Pawn => "P",
            };
            // print!("{}:{} ", piece_name, num);
            let value = *piece_values.get(&piece).unwrap_or(&0);
            // Material value
            let mut piece_score = value * num as i32;
            // Piece-square table bonus for all pieces
            let mut pst_bonus = 0;
            let mut bitboard = count;
            while bitboard != chess::BitBoard(0) {
                let sq = bitboard.to_square();
                let idx = sq.to_index();
                let pst_idx = if color == Color::White { idx } else { 63 - idx };
                pst_bonus += match piece {
                    Piece::Pawn => pawn_pst[pst_idx],
                    Piece::Knight => knight_pst[pst_idx],
                    Piece::Bishop => bishop_pst[pst_idx],
                    Piece::Rook => rook_pst[pst_idx],
                    Piece::Queen => queen_pst[pst_idx],
                    Piece::King => king_pst[pst_idx],
                };
                bitboard = bitboard & !chess::BitBoard::from_square(sq);
            }
            piece_score += pst_bonus;
            if color == Color::White {
                eval_white += piece_score;
            } else {
                eval_black += piece_score;
            }
        }
        println!("");
    }
    let evaluation = eval_white - eval_black;
    // println!("\nMaterial + PST evaluation (White - Black): {}", evaluation);
    evaluation
}

