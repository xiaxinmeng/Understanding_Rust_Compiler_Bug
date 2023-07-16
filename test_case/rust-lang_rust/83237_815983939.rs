plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unreachable pattern
   --> src/librustdoc/clean/types.rs:238:33
    |
238 | ...                   Some(&(_, _, ExternalLocation::Remote(ref s))) => s.to_string(),
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

