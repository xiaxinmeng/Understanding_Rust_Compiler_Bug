 rust
    let mut f = BufReader::new(file);
    let mut count = 0;
    let mut line = String::new();
    if let Ok(mut bytes) = f.read_line(&mut line) {
        while bytes > 0 {
            // Do something with the line
            count += 1;
            line.clear();
            match f.read_line(&mut line) {
                Ok(b) => bytes = b,
                Err(_) => {},
            }
        }
    }
