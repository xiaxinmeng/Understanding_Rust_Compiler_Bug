
let d = Duration::milliseconds(1500);

let a: i64 = TimeSpan::num_seconds(&d);
assert_eq!(a, 1);

let b: f64 = TimeSpan::num_seconds(&d);
assert_eq!(b, 1.5);
