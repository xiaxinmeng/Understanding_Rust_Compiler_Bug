plain
    Checking clippy_lints v0.1.54 (/checkout/src/tools/clippy/clippy_lints)
error: unused variable: `edition`
   --> src/tools/clippy/clippy_lints/src/doc.rs:551:38
    |
551 |     fn has_needless_main(code: &str, edition: Edition) -> bool {
    |                                      ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_edition`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `clippy_lints`

