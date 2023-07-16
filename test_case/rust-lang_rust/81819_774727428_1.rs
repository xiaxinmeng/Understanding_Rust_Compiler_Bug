rust
let folded = (1..10).fold(0, |acc, e| acc + e);
let reduced = (1..10).reduce(|acc, e| acc + e).unwrap_or(0);
assert_eq!(folded, reduced);
