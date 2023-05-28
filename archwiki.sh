command=$HOME/.cargo/bin/archwiki-rs
if output=$($command read-page $1 2>&1); then 
    echo "$output" | less
else
    selection=$(echo "$output" | fzf) 
    $command read-page "$selection" | less
fi
