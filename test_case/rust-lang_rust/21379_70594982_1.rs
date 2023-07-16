 rust
    loop {
        match (*iter).next() {
            Some(ref i) => println!("{}", i),
            None => break
        }
    }
