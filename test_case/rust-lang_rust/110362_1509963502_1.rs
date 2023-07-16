rust
    {
        let extend_receiver = &mut left; // (*)
        let arg = right.strip_prefix(duplicate_part).unwrap();
        extend_receiver.extend_from_slice(arg);
    }
