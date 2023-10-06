#!/bin/bash

if [[ $(uname) == "Linux" ]]; then
    ../linux_game_engine -f ../maps/map01 -p1 ../linux_robots/bender -p2 target/debug/filler
elif [[ $(uname) == "Darwin" ]]; then
    ../m1_game_engine -f ../maps/map01 -p1 ../m1_robots/bender -p2 target/debug/filler
else
    echo "Unsupported operating system"
fi
