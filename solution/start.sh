#!/bin/bash

if [[ $(uname) == "Linux" ]]; then
    ../linux_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../linux_robots/bender
else
    ../m1_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../m1_robots/bender
fi