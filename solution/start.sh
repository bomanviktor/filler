#!/bin/bash

../m1_game_engine -f ../maps/map02 -p1 target/debug/filler -p2 ../m1_robots/terminator


#if [[ $(uname) == "Linux" ]]; then
 #   echo "LINUX"
  #  ../linux_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../linux_robots/bender
#else
 #   echo "MAC"

#fi