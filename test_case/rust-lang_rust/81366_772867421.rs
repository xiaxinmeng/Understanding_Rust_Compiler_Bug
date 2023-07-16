plain
    Checking toml v0.5.7
    Checking url v2.1.1
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0408]: variable `exp` is not bound in all patterns
  --> src/tools/clippy/clippy_lints/src/collapsible_match.rs:89:18
   |
89 |             Some(Guard::If(expr) | Guard::IfLet(_, exp, _)) => {
   |                  ^^^^^^^^^^^^^^^                   --- variable not in all patterns
   |                  |
   |                  pattern doesn't bind `exp`

error[E0408]: variable `expr` is not bound in all patterns
  --> src/tools/clippy/clippy_lints/src/collapsible_match.rs:89:36
   |
89 |             Some(Guard::If(expr) | Guard::IfLet(_, exp, _)) => {
   |                            ----    ^^^^^^^^^^^^^^^^^^^^^^^ pattern doesn't bind `expr`
   |                            variable not in all patterns

error: aborting due to 2 previous errors

