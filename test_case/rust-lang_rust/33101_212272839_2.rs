 rust
// src/main.rs
include!("prelude.rs");

fn main() {
    bonjour!();
}

// $ cargo run
//    Compiling macro-include-prelude v0.1.0 (...)
// src/main.rs:4:5: 4:12 error: macro undefined: 'bonjour!'
// src/main.rs:4     bonjour!();
//                   ^~~~~~~
// error: aborting due to previous error
// error: Could not compile `macro-include-prelude`.
