rust
    let arr2d = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let (x, y) = (1, 5);
    let el = arr2d
        .get(y)
        .inspect_none(|| warn!("y out of bound"))
        .and_then(|row| row.get(x).inspect_none(|| warn!("x out of bound")));
