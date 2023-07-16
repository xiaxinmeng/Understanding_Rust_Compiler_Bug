rust
    let mut iter = 0..12; // some iterator
    for x in iter.by_ref().take(6) {
        println!("First: {}", x);
    }
    for x in iter {
        println!("Rest: {}", x);
    }
