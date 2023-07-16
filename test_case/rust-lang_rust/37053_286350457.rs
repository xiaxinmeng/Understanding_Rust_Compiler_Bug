rust
        let mut leg = compare(value, &buffer[trial]);
        if leg == Ordering::Equal {
            leg = when_equal;
        };
