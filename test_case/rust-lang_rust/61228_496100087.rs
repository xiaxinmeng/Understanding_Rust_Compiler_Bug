rust
    if let Some(xx) = x.read().unwrap().as_ref() {
        println!("x is {}", xx);
    } else {
        let mut xx = x.write().unwrap();
        *xx = Some(100i32);
    }
