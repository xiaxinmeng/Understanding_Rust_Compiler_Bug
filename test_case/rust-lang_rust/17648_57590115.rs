 rust
let ints: &[i32] = [-3, 0, 1, 5, -10];
assert_eq!(*ints.iter().min_by(|x, y| x.abs().cmp(&y.abs())).unwrap(), 0);

let floats: &[f64] = [-3., 0., 1., 5., -10.];
assert_eq!(*floats.iter().min_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap()).unwrap(), 0.);
