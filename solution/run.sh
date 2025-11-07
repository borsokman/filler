#!/bin/bash

mkdir -p logs

P1_WINS=0
P2_WINS=0
ROUNDS=100

for i in $(seq 1 $ROUNDS); do
    ../m1_game_engine -f ../maps/map02 -p1 ./bot -p2 ../m1_robots/h2_d2 > logs/h2_d2.txt 2>&1

    WINNER=$(grep "won!" logs/h2_d2.txt)
    if echo "$WINNER" | grep -q "Player1"; then
        P1_WINS=$((P1_WINS+1))
    elif echo "$WINNER" | grep -q "Player2"; then
        P2_WINS=$((P2_WINS+1))
    fi
    echo "Round $i: $WINNER"
done

echo "=========================="
echo "Player1 wins: $P1_WINS"
echo "Player2 wins: $P2_WINS"