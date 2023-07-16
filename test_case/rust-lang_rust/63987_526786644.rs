
    match 0usize {
        1 ..= 8 => {}
        5 ..= 20 => {} // This should probably have been `8 ..= 20`
        20 ..= 100 => {} // This should probably have been `21 ..= 100`
        _ => {}
    }
