#!/bin/zsh

ENGINE="./target/debug/axelrot"

$ENGINE <<EOF
uci
position fen 6k1/3r1ppp/8/8/8/8/5PPP/6K1 w - - 0 1
go depth 3
EOF
