if output=$(cargo run -q -- $1 2>&1); then 
    echo "$output" | less
else
    selection=$(echo "$output" | fzf) 
    cargo run -q "$selection" | less
fi
