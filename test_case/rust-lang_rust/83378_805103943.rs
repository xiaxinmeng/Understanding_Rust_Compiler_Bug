rust
let len = self.len();
assert!(
    mid <= len,
    "split index (is {}) should be <= len (is {}))",
    mid,
    len
);
