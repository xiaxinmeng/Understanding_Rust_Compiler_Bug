rust
            Err(err) => {
                eprintln!("Couldn't read signature file: {}", err);
                None
            }
