rust
macro_rules! mac {
    ($e:expr) => {};
    (label: $v:expr, MARKER) => {};
}

mac!(label: try!(), MARKER)
