#!/bin/bash
# alex-laycalvert
# 
# discoball.sh
# 
# https://github.com/alex-laycalvert/fun-scripts

# default values
USAGE="usage"
MAX_SIZE=50
MIN_SIZE=4
SIZE=10

for i in $( seq 1 $# ); do
    option=${@:i:1}
    value=${@:i+1:1}
    if [ "$option" = "--help" ] || [ "$option" = "-h" ]; then
        echo $USAGE
        exit
    elif [ "$option" = "--size" ] || [ "$option" = "-s" ]; then
        echo
    fi
done

trap 'tput cnorm; tput reset; clear; exit' 2
tput reset
tput setaf 1
tput bold
tput civis

CENTER_COL=$(($(tput cols) / 2))
CENTER_ROW=$(($(tput lines) / 2))

# tput cup $CENTER_ROW $CENTER_COL
cat test.txt
sleep 10

tput cnorm
tput reset
clear
exit
