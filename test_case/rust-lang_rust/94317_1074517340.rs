rust
fn warn_y_out_of_bounds<T>(opt: Option<T>) {
    if opt.is_none() {
        warn!("y out of bound") 
    }
}

// inspect(|opt| if opt.is_none() { ... })
    let arr2d = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let (x, y) = (1, 5);
    let el = arr2d
        .get(y)
        .inspect(warn_y_out_of_bounds)
        .and_then(|row| row.get(x).inspect(|opt| if opt.is_none() { warn!("x out of bound") }));
