rust
    if let Some(first) = bytes.iter().position(|&b| b == 0) {
        bytes.truncate(first);
    }
