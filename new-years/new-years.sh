#!/bin/bash
trap "tput reset; tput cnorm; clear; exit" 2
clear

now() {
    echo "$(date +'%H:%M:%S %D')"
}

nows() {
    echo "$(date +'%s')"
}

TARGET_DATE="00:00:00 01/01/22"
TARGET_DATE="20:27:50 12/25/21"

COL=$(($(tput cols) / 2))
ROW=$(($(tput lines) / 2))

tput civis
tput setaf 1
tput bold

loop_date() {
    while true; do
        msg=$(now)
        msgs=$(nows)
        if [ "$msg" = "$TARGET_DATE" ]; then
            msg=" TARGET ACQUIRED "
        elif [ "$msg" -lt "$TARGET_DATE" ]; then
            msg="    BEFORE    "
        fi
        msg_len=${#msg}
        tput cup $ROW $((COL - msg_len / 2)) 
        echo "$msg"
    done
}

loop_date &
date_pid=$!
while read -n1 reply && [ q != "$reply" ]
do  :
done </dev/stdin &
input_pid=$!
wait -n
kill -9 $date_pid $input_pid
wait

tput reset
tput cnorm
clear
