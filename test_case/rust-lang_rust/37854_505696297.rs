rust
        match i {
            0..=MAXN if i < MAXN => println!("Exclusive: {}", i),
            MAXN => println!("Limit: {}", i),
            _ => println!("Out of range: {}", i)
        }
