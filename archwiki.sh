if output=$(cargo run -q -- read-page $1 2>&1); then 
    echo "$output" | less
else
    selection=$(echo "$output" | fzf) 
    cargo run -q -- read-page "$selection" | less
fi
