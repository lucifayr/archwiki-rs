if output=$(/usr/bin/archwiki-rs read-page $1 2>&1); then 
    echo "$output" | less
else
    selection=$(echo "$output" | fzf) 
    /usr/bin/archwiki-rs read-page "$selection" | less
fi
