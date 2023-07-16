rust
    Entry::Occupied(mut entry) => {
        // If `child` is defined in crate `cnum`, ensure
        // that it is mapped to a parent in `cnum`.
        if child.krate == cnum && entry.get().krate != cnum {
            entry.insert(parent);
        }
    }
