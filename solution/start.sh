#!/bin/bash

../m1_game_engine -f ../maps/map01 -p2 target/debug/filler -p1 ../m1_robots/bender


#if [[ $(uname) == "Linux" ]]; then
 #   echo "LINUX"
  #  ../linux_game_engine -f ../maps/map01 -p1 target/debug/filler -p2 ../linux_robots/bender
#else
 #   echo "MAC"

#fi