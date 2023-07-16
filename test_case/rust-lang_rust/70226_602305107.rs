rust
let x: u128 = ...; // Some potentially large value...
let x_as_u64 = x as u64; // ... that we want to fit in a smaller type
assert_eq!(x_as_u64 as u128, x);
// Use `x_as_u64` now that it's been validated.
