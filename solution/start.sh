#!/bin/bash

if [[ $(uname -m) == "aarch64" ]]; then
  ../m1_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../m1_robots/bender
else
  ../linux_game_engine -f ../maps/map01 -p2 target/debug/filler -p1 ../linux_robots/bender
fi