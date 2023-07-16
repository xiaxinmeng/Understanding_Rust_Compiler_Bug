rust
macro_rules! mac {
    ($e:expr) => {}; // Removing this line makes it compile again.
    (label: $v:expr, MARKER) => {};
}

mac!(label: 0, MARKER);
