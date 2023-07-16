
augie% python3 x.py test
    Updating crates.io index
[...]
    Finished release [optimized] target(s) in 5.98s
tidy check
Checking which error codes lack tests...
tidy error: `rust/src/stdarch/examples/hex.rs:198` contains `#[test]`; unit tests and benchmarks must be placed into separate files or directories named `tests.rs`, `benches.rs`, `tests` or `benches`
tidy error: rust/src/stdarch/examples/Cargo.toml doesn't have `edition = "2018"` on a separate line
tidy error: rust/src/stdarch/crates/stdarch-test/Cargo.toml doesn't have `edition = "2018"` on a separate line
tidy error: `rust/src/stdarch/crates/simd-test-macro/src/lib.rs:121` contains `#[test]`; unit tests and benchmarks must be placed into separate files or directories named `tests.rs`, `benches.rs`, `tests` or `benches`
