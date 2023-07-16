 rust
        if names.peek().map(|s| s.as_slice()) == Some("self") {
            let _ = names.next();
        }
