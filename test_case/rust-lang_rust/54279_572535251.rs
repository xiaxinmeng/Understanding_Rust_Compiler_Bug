rust
        let (_, dupes) = working.partition_dedup();
        if !dupes.is_empty() {
            assert_eq!(dupes.len(), 1);
            return dupes.first().unwrap().to_string();
        }
    