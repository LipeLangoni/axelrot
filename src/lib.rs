use std::time::{Duration, Instant};

pub struct SearchInfo {
    pub start: Instant,
    pub time_budget: Duration,
    pub stopped: bool,
}

impl SearchInfo {
    pub fn new(time_budget_ms: u64) -> Self {
        SearchInfo {
            start: Instant::now(),
            time_budget: Duration::from_millis(time_budget_ms),
            stopped: false,
        }
    }
    pub fn should_stop(&mut self) -> bool {
        if self.stopped {
            return true;
        }
        if self.start.elapsed() >= self.time_budget {
            self.stopped = true;
            return true;
        }
        false
    }
}
pub struct PvTable {
    pub pv: Vec<chess::ChessMove>,
}

impl PvTable {
    pub fn new() -> Self {
        PvTable { pv: Vec::new() }
    }
    pub fn set_pv(&mut self, pv: &[chess::ChessMove]) {
        self.pv.clear();
        self.pv.extend_from_slice(pv);
    }
}

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
use chess::{Board, Piece, Color, ALL_PIECES, MoveGen, BoardStatus};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct TTEntry {
    pub value: i32,
    pub depth: i32,
    pub node_type: NodeType,
    pub ply: usize,
}

pub struct TranspositionTable {
    pub table: HashMap<u64, TTEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable { table: HashMap::new() }
    }
    pub fn get(&self, hash: u64, ply: usize) -> Option<&TTEntry> {
        self.table.get(&hash).filter(|entry| entry.ply <= ply)
    }
    pub fn put(&mut self, hash: u64, value: i32, depth: i32, node_type: NodeType, ply: usize) {
        let entry = TTEntry { value, depth, node_type, ply };
        self.table.insert(hash, entry);
    }
}

pub fn evaluation(board: &Board) -> i32 {
    let piece_values = [
        100, 
        300, 
        300, 
        500, 
        900, 
        0,   
    ];
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

    fn mirror_sq(sq: usize) -> usize {
        sq ^ 56
    }

    use chess::Piece::*;
    use chess::Color;
    use chess::ALL_PIECES;
    use chess::Square;

    let mut total_score = 0;
    for (piece_idx, &piece) in [Pawn, Knight, Bishop, Rook, Queen, King].iter().enumerate() {
        let value = piece_values[piece_idx];
        let our_bb = board.pieces(piece) & board.color_combined(board.side_to_move());
        let their_bb = board.pieces(piece) & board.color_combined(!board.side_to_move());

        let pst = match piece {
            Pawn => &pawn_pst,
            Knight => &knight_pst,
            Bishop => &bishop_pst,
            Rook => &rook_pst,
            Queen => &queen_pst,
            King => &king_pst,
        };

        for sq in our_bb {
            let idx = sq.to_index();
            total_score += value;
            total_score += pst[idx];
        }
        for sq in their_bb {
            let idx = mirror_sq(sq.to_index());
            total_score -= value;
            total_score -= pst[idx];
        }
    }
    total_score
}

pub fn axelrot(
    board: &Board,
    max_depth: i32,
    wtime: u64,
    btime: u64,
    winc: u64,
    binc: u64,
) -> String {

    let stm = board.side_to_move();
    let time_left = match stm {
        Color::White => wtime,
        Color::Black => btime,
    };
    let inc = match stm {
        Color::White => winc,
        Color::Black => binc,
    };
    let move_time = (time_left / 30).max(10) + inc;
    let mut info = SearchInfo::new(move_time);

    let mut best_move: Option<chess::ChessMove> = None;
    let mut best_value = i32::MIN + 1;
    let mut pv_table = PvTable::new();
    let mut board = *board;
    let mut history = Vec::new();
    let mut tt = TranspositionTable::new();
    for depth in 1..=max_depth {
        if info.should_stop() { break; }
        let mut pv = Vec::new();
        let mut pv_temp = Vec::new();
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX;

        let mut moves: Vec<_> = MoveGen::new_legal(&board).collect();
        if let Some(pv_move) = pv_table.pv.get(0) {
            if let Some(pos) = moves.iter().position(|m| m == pv_move) {
                let mv = moves.remove(pos);
                moves.insert(0, mv);
            }
        }

        let mut current_best_move: Option<chess::ChessMove> = None;
        let mut current_best_value = i32::MIN + 1;

        for &mv in &moves {
            if info.should_stop() { break; }
            history.push(board);
            board = board.make_move_new(mv);
            pv_temp.clear();
            let value = -negamax(&mut board, -beta, -alpha, depth - 1, 1, &mut history, &mut pv_temp, &mut pv, &mut info, &mut tt);
            board = history.pop().unwrap();

            if info.should_stop() { break; }
            if value > current_best_value || current_best_move.is_none() {
                current_best_value = value;
                current_best_move = Some(mv);
                pv.clear();
                pv.push(mv);
                pv.extend_from_slice(&pv_temp);
            }
            if value > alpha {
                alpha = value;
            }
        }
        pv_table.set_pv(&pv);

        if !info.should_stop() && current_best_move.is_some() {
            best_move = current_best_move;
            best_value = current_best_value;
        }
    }

    if let Some(mv) = best_move {
        mv.to_string()
    } else {
        "0000".to_string()
    }
}

pub fn negamax(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    depth: i32,
    ply: usize,
    history: &mut Vec<Board>,
    pv: &mut Vec<chess::ChessMove>,
    pv_temp: &mut Vec<chess::ChessMove>,
    info: &mut SearchInfo,
    tt: &mut TranspositionTable,
) -> i32 {
    if info.should_stop() {
        return 0;
    }
    if depth <= 0 {
        return quiesce(board, alpha, beta, ply, history);
    }
    if ply > 0 && history.iter().any(|b| b == board) {
        return 0;
    }
    let hash = board.get_hash();
    if let Some(entry) = tt.get(hash, ply) {
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return entry.value,
                NodeType::LowerBound => if entry.value > beta { return entry.value; },
                NodeType::UpperBound => if entry.value <= alpha { return entry.value; },
            }
        }
    }
    let moves: Vec<_> = MoveGen::new_legal(board).collect();
    if moves.is_empty() {
        return if board.checkers().popcnt() > 0 {
            -10000 + ply as i32
        } else {
            0
        };
    }
    let mut best_value = -10000;
    let mut found_pv = false;
    for mv in moves {
        if info.should_stop() {
            break;
        }
        history.push(*board);
        *board = board.make_move_new(mv);
        pv_temp.clear();
        let score = -negamax(board, -beta, -alpha, depth - 1, ply + 1, history, pv_temp, pv, info, tt);
        *board = history.pop().unwrap();
        if info.should_stop() {
            break;
        }
        if score >= beta {
            tt.put(hash, score, depth, NodeType::LowerBound, ply);
            return score;
        }
        if score > best_value {
            best_value = score;
            if score > alpha {
                alpha = score;
                pv.clear();
                pv.push(mv);
                pv.extend_from_slice(pv_temp);
                found_pv = true;
            }
        }
    }
    let node_type = if best_value > alpha { NodeType::Exact } else { NodeType::UpperBound };
    tt.put(hash, best_value, depth, node_type, ply);
    best_value
}
