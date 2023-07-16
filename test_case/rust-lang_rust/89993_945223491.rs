
cd src/tools/compiletest
cargo c
    Checking compiletest v0.0.0 (repos/rust/src/tools/compiletest)
warning: field is never read: `explanation`
  --> src/tools/compiletest/src/json.rs:78:5
   |
78 |     explanation: Option<String>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `compiletest` (bin "compiletest") generated 1 warning
    Finished dev [unoptimized] target(s) in 27.81s