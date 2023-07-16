rust
    match x.read().unwrap().as_ref() {
        Some(xx) => { println!("x is {}", xx) },
        _ => {
            let mut xx = x.write().unwrap();
            *xx = Some(100i32);
        },
    }
