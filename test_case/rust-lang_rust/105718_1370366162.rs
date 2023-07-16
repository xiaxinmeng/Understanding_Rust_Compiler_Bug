rs
    let i = input.parse().unwrap();
    let h = histories.get(i).unwrap();
    let command: Vec<&str> = h.command.split_ascii_whitespace().collect();
