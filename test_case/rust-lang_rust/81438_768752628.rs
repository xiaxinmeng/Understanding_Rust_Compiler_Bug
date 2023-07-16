rust
        .filter_map(|attr| {
            if ALLOWED_ATTRIBUTES.contains(&attr.name_or_empty()) {
                Some(pprust::attribute_to_string(&attr))
            } else {
                None
            }
        })
