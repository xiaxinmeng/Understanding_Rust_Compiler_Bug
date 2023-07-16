rust
use std::io::Write;

// The cfg result depends on whether the `io::Write` trait is in scope here
#[cfg(::std::fs::File::write)]
