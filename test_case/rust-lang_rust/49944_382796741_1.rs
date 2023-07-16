rust
    let closest_to = |d| {
        let gt = perfs.range(d..).next();
        let lt = perfs.range(..d).next_back();
