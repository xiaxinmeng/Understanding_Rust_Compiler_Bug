rust
let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
let reduced: Option<i32> = (1..10).reduce(|acc, e| acc + e);
assert_eq!(Some(folded), reduced);
