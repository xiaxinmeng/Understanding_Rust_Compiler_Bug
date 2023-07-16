
   Compiling rust-bug v0.1.0 (/Users/skyzh/Work/rust-bug)
warning: function is never used: `str_contains`
  --> src/main.rs:29:4
   |
29 | fn str_contains(a: &str, b: &str) -> bool {
   |    ^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: field is never read: `f`
  --> src/main.rs:37:5
   |
37 |     f: F,
   |     ^^^^

warning: function is never used: `success_case_1`
  --> src/main.rs:66:4
   |
66 | fn success_case_1() {
   |    ^^^^^^^^^^^^^^

warning: function is never used: `success_case_2`
  --> src/main.rs:70:4
   |
70 | fn success_case_2() {
   |    ^^^^^^^^^^^^^^

warning: `rust-bug` (bin "rust-bug") generated 4 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
