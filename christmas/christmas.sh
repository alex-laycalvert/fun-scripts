#!/bin/bash
trap "clear; tput reset; tput cnorm; exit" 2
MAX_SIZE=50
MIN_SIZE=5
USE_NERD_STAR=1
XMAS_MESSAGE="Merry Christmas You Filthy Animal!"
DURATION=10
USAGE=" usage: christmas.sh [options]
 
 A simple ASCII Christmas tree with a few customizable options.

 This is NOT the malicious one, check the script for yourself.

 options:
    -h, --help          show this message

    -s N, --size N      set the size of the christmas tree to integer N
                        max value is 50, min value is 5
    -S, --star          use a Nerd Font star on top of the tree (default)
                        if a Nerd Font is not installed this will appear 
                        to be a broken character
    -N, --no-star       do not use a Nerd Font star on top of the tree
    -m, --xmas-message  message to display directly under the Christmas Tree
                        multi-worded message should be in quotes or using
                        an escape space character

 source: https://github.com/alex-laycalvert/fun-scripts

 modified from ChristBASHTree by sergiolepore at https://github.com/sergiolepore/ChristBASHTree

"

col=$(($(tput cols) / 2))
c=$((col-1))
est=$((c-2))
color=0
size=20

for i in $( seq 1 $# ) 
{
    option=${@:i:1} 
    value=${@:i+1:1}
    if [ "$option" = "--star" ] || [ "$option" = "-S" ]; then
        USE_NERD_STAR=1
    elif [ "$option" = "--no-star" ] || [ "$option" = "-N" ]; then
        USE_NERD_STAR=0
    elif [ "$option" = "--help" ] || [ "$option" = "-h" ]; then
        echo "$USAGE"
        exit
    elif [ "$option" = "--size" ] || [ "$option" = "-s" ]; then
        if [ $value -gt $MAX_SIZE ]; then
            size=$MAX_SIZE
        elif [ $value -lt $MIN_SIZE ]; then
            size=$MIN_SIZE
        else
            size=$value
        fi
    elif [ "$option" = "--xmas-message" ] || [ "$option" = "-m" ]; then
        XMAS_MESSAGE=$value
    fi
}
clear

if [ $size -le 20 ]; then
    row=$(($(tput lines) / 2 - ${size} / 2 + 0))
elif [ $size -lt 47 ]; then
    row=$(($(tput lines) / 2 - ${size} / 2 + 5))
else
    row=$(($(tput lines) / 2 - ${size} / 2 + 8))
fi
lin=$row

XMAS_MESSAGE_SIZE=$((${#XMAS_MESSAGE} / 2))
tput civis
tput setaf 2; tput bold

# nerd font star option
if [ $USE_NERD_STAR = 1 ]; then
    tput cup $((lin - 1)) $col
    echo 六
fi

# Tree
for ((i=1; i<size; i+=2))
{
    tput cup $lin $col
    for ((j=1; j<=i; j++))
    {
        echo -n \*
    }
    let lin++
    let col--
}

tput sgr0; tput setaf 3

# Trunk
if [ $size -ge 47 ]; then
    for ((i=1; i<=size/10 + 3; i++))
    {
        tput cup $((lin++)) $((c - 3))
        echo 'mWmWmWmWm'
    }
elif [ $size -ge 37 ]; then
    for ((i=1; i<=size/10 + 3; i++))
    {
        tput cup $((lin++)) $((c - 2))
        echo 'mWmWmWm'
    }
elif [ $size -ge 27 ]; then
    for ((i=1; i<=size/10 + 2; i++))
    {
        tput cup $((lin++)) $((c - 1))
        echo 'mWmWm'
    }
elif [ $size -gt 7 ]; then
    for ((i=1; i<=size/10 + 1; i++))
    {
        tput cup $((lin++)) $c
        echo 'mWm'
    }
else
    tput cup $((lin++)) $c
    echo ' W '
fi
tput cup $((lin++)) $c
new_year=$(date +'%Y')
let new_year++
tput setaf 1; tput bold
tput cup $((lin + 1)) $((c - XMAS_MESSAGE_SIZE + 2)); echo "$XMAS_MESSAGE"
tput cup $((lin+ 2)) $((c - 10)); echo And lots of CODE in $new_year
let c++
k=1

decorate_loop () {
    for ((i=1; i<=DURATION; i++)) {
        # Turn off the lights
        [ $k -gt 1 ] && {
            tput setaf 2; tput bold
            tput cup ${line[$[k-1]$i]} ${column[$[k-1]$i]}; echo \*
            unset line[$[k-1]$i]; unset column[$[k-1]$i]  # Array cleanup
        }

        li=$((RANDOM % (size / 2 - 1) + row + 1))
        start=$((c-li+row))
        co=$((RANDOM % (li-row) * 2 + 1 + start))
        tput setaf $color; tput bold   # Switch colors
        tput cup $li $co
        echo o
        line[$k$i]=$li
        column[$k$i]=$co
        color=$(((color+1)%8))
        # Flashing text
        sh=1
        for l in C O D E
        do
            tput cup $((lin+2)) $((c+sh))
            echo $l
            let sh++
            sleep 0.01
        done
    }
    k=$((k % 2 + 1))
}

decorate() {
    while true; do
        decorate_loop
    done
}

# Lights and decorations
decorate &
decorate_pid=$!
while true; do
    read -n1 reply
    if [ "$reply" = "q" ]; then
        break
    fi
done
kill -9 $decorate_pid
wait
tput reset
tput cnorm
clear
