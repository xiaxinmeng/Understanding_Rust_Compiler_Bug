rust
    let mut v = Vec::new();
    
    match v.get(0) {
        Some(&x) => v.push(x + x),
        None => v.push(1),
    }
