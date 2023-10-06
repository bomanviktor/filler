#!/bin/bash

if [[ $(uname) == "Linux" ]]; then
    ../linux_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../linux_robots/bender
elif [[ $(uname) == "Darwin" ]]; then
    ../m1_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../linux_robots/bender
else
    echo "Unsupported operating system"
fi
