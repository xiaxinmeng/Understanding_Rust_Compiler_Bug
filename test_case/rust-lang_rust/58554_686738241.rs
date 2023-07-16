rust
let skipped_before = frames.iter().position(|frame|
    unimplemented!("somehow determine whether `frame` is a call to `begin_panic`")
).map(|i|
    // Exclude the call to `begin_panic()` as well.
    i + 1
).unwrap_or(0);
