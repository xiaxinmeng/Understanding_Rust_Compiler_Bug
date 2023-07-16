rust
    let mut v = Vec::new();
    
    match v.get(0) {
        Some(&x) => v.push(x + 1),  // A-OK
        None => v.push(1),
    }
