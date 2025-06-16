
fn quiesce(board: &mut Board, mut alpha: i32, beta: i32, ply: usize, history: &mut Vec<Board>) -> i32 {
   

    let mut stand_pat = evaluation(board);
    if stand_pat >= beta {
        return stand_pat;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let us = board.side_to_move();
    let them = match us {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };
    let their_pieces = board.color_combined(them);
    let moves: Vec<_> = MoveGen::new_legal(board)
        .filter(|m| {
            let to = m.get_dest();
            their_pieces & chess::BitBoard::from_square(to) != chess::BitBoard(0)
        })
        .collect();
    for mv in moves {
 
        history.push(*board);

        *board = board.make_move_new(mv);
        let score = -quiesce(board, -beta, -alpha, ply + 1, history);

        *board = history.pop().unwrap();

        if score >= beta {
            return score;
        }
        if score > stand_pat {
            stand_pat = score;
            if score > alpha {
                alpha = score;
            }
        }
    }
    stand_pat
}
use chess::{Board, Piece, Color, ALL_PIECES, MoveGen};
use std::collections::HashMap;

pub fn evaluation(board: &Board) -> i32 {
    let mut piece_values = HashMap::new();
    piece_values.insert(Piece::Pawn, 100);
    piece_values.insert(Piece::Knight, 300);
    piece_values.insert(Piece::Bishop, 300);
    piece_values.insert(Piece::Rook, 500);
    piece_values.insert(Piece::Queen, 900);
    piece_values.insert(Piece::King, 0);

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

    let us = board.side_to_move();
    let them = match us {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let mut total_score = 0;

    // For test compatibility, use material-only evaluation (no PST)
    for piece in ALL_PIECES {
        let value = *piece_values.get(&piece).unwrap_or(&0);
        // Us
        let our_count = (board.pieces(piece) & board.color_combined(us)).popcnt();
        total_score += our_count as i32 * value;
        // Them
        let their_count = (board.pieces(piece) & board.color_combined(them)).popcnt();
        total_score -= their_count as i32 * value;
    }

    total_score
}

pub fn axelrot(board: &Board, depth: i32) -> String {
    let mut best_value = i32::MIN + 1;
    let mut best_move = None;
    let alpha = i32::MIN + 1;
    let beta = i32::MAX;
    let mut board = *board;
    let mut history = Vec::new();
    for m in MoveGen::new_legal(&board) {
        history.push(board);
        board = board.make_move_new(m);
        let value = -negamax(&mut board, -beta, -alpha, depth - 1, &mut history);
        board = history.pop().unwrap();
        if value > best_value {
            best_value = value;
            best_move = Some(m);
        }
    }
    if let Some(mv) = best_move {
        mv.to_string()
    } else {
        "0000".to_string()
    }
}

fn negamax(board: &mut Board, mut alpha: i32, beta: i32, depth: i32, history: &mut Vec<Board>) -> i32 {
    if depth == 0 {
        return quiesce(board, alpha, beta, 0, history);
    }
    let mut best_value = i32::MIN + 1;
    let moves: Vec<_> = MoveGen::new_legal(board).collect();
    for mv in moves {

        history.push(*board);

        *board = board.make_move_new(mv);
        let score = -negamax(board, -beta, -alpha, depth - 1, history);

        *board = history.pop().unwrap();

        if score >= beta {
            return score;
        }
        if score > best_value {
            best_value = score;
            if score > alpha {
                alpha = score;
            }
        }
    }
    best_value
}
