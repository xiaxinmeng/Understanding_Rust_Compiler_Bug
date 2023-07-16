rust
        let names: Vec<String> = index_read
            .iter_hunks()
            .advance_to_after(&"/subdir".into())
            .flatten()
            .map(|entry| entry.apath.into())
            .collect();
        assert!(names.is_empty());
