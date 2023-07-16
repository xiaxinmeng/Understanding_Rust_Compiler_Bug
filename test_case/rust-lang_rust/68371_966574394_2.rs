rust
let max1 = r.iter().scan_after(0, |acc, x| acc + *x).map(|(x, _)| x).max().unwrap();
