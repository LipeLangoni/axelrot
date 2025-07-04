#!/bin/bash
# Dynamic Elo estimation script for Axelrot vs Stockfish (or another engine)
# Usage: ./estimate_elo_cutechess_dynamic.sh [ENGINE_PATH]
# If ENGINE_PATH is not provided, defaults to Stockfish and tests all ELO_LEVELS.

CUTECHESS="cutechess-cli"
STOCKFISH_PATH="stockfish"
GAMES_PER_LEVEL=20
AXELROT_PATH="$(pwd)/target/release/axelrot"
AXELROT_NAME="axelrot"

if [ -z "$1" ]; then
    OPPONENT_PATH="stockfish"
    OPPONENT_NAME="stockfish"
    ELO_LEVELS=(1350 1400 1600 1800)
    for ELO in "${ELO_LEVELS[@]}"; do
        echo "\n=== Testing $AXELROT_NAME vs $OPPONENT_NAME Elo $ELO ==="
        "$CUTECHESS" \
            -engine name="$AXELROT_NAME" cmd="$AXELROT_PATH" proto=uci \
            -engine name="$OPPONENT_NAME" cmd="$OPPONENT_PATH" proto=uci option.UCI_LimitStrength=true option.UCI_Elo=$ELO \
            -games $GAMES_PER_LEVEL \
            -each proto=uci tc=300+0 \
            -repeat \
            -concurrency 5 \
            -openings file=./Perfect_2023/BIN/Perfect2023.bin \
            -pgnout ${AXELROT_NAME}_vs_${OPPONENT_NAME}_${ELO}.pgn \
            -ratinginterval 1 \
            -draw movenumber=40 movecount=8 score=5 \
            -recover > cutechess_${AXELROT_NAME}_vs_${OPPONENT_NAME}_${ELO}.log 2>&1
        echo "Results for $AXELROT_NAME vs $OPPONENT_NAME Elo $ELO:"
        grep -E "Result|$AXELROT_NAME|$OPPONENT_NAME" ${AXELROT_NAME}_vs_${OPPONENT_NAME}_${ELO}.pgn | grep -v "\[Event" | grep -v "\[Site" | grep -v "\[Date" | grep -v "\[Round" | grep -v "\[WhiteElo" | grep -v "\[BlackElo"
    done
else
    OPPONENT_PATH="$1"
    OPPONENT_NAME=$(basename "$OPPONENT_PATH")
    OPPONENT_NAME="${OPPONENT_NAME%%.*}"
    echo "\n=== Testing $AXELROT_NAME vs $OPPONENT_NAME ==="
    "$CUTECHESS" \
        -engine name="$AXELROT_NAME" cmd="$AXELROT_PATH" proto=uci \
        -engine name="$OPPONENT_NAME" cmd="$OPPONENT_PATH" proto=uci \
        -games $GAMES_PER_LEVEL \
        -each proto=uci tc=300+0 \
        -repeat \
        -concurrency 5 \
        -openings file=./Perfect_2023/BIN/Perfect2023.bin \
        -pgnout ${AXELROT_NAME}_vs_${OPPONENT_NAME}.pgn \
        -ratinginterval 1 \
        -draw movenumber=40 movecount=8 score=5 \
        -recover > cutechess_${AXELROT_NAME}_vs_${OPPONENT_NAME}.log 2>&1
    echo "Results for $AXELROT_NAME vs $OPPONENT_NAME:"
    grep -E "Result|$AXELROT_NAME|$OPPONENT_NAME" ${AXELROT_NAME}_vs_${OPPONENT_NAME}.pgn | grep -v "\[Event" | grep -v "\[Site" | grep -v "\[Date" | grep -v "\[Round" | grep -v "\[WhiteElo" | grep -v "\[BlackElo"
fi

echo "\n=== Done! ==="
echo "Check the PGN files and summary above to estimate $AXELROT_NAME's Elo (look for the level where it scores ~50%)."
