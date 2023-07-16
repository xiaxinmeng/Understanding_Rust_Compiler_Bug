rust
LENGTHS.flat_map(|length| {
    seeds
        .par_iter()
        .find_any(|&&seed| test_password(length, seed))
        .map(|&seed| (length, seed))
}).next()
