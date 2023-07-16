rust
        if let Some(first_dupe) = working.windows(2).find(|(p, q)| p == q) {
            return first_dupe.to_string();
        }
    