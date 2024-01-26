#!/bin/sh

out=$(archwiki-rs read-page "$1" --format markdown 2>/tmp/awrs-recommendations) 

if [ $? -eq 0 ]; then
    echo "$out" | pandoc -s -f markdown -t man | man -p t -l -
elif [ $(cat /tmp/awrs-recommendations | grep '[^[:space:]]' | wc -c) -ne 0 ]; then
    selection=$(cat /tmp/awrs-recommendations | fzf) 
    $0 "$selection"
fi

