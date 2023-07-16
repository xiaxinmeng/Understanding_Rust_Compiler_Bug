rust
        // Ensure indexes are also not malformed.
        if start_index > end_index || end_index > source_len - 1 {
            debug!("find_width_of_character_at_span: source indexes are malformed");
            return 0;  // Previous version is  `return 1`
        }
