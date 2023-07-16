rust
let odds = iter.filter(is_even.negate());
let odds = iter.filter(|v| !is_even(v));
