plain
    Checking tempfile v3.1.0
    Checking regex v1.4.3
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: associated function is never used: `is_dummy`
    --> src/librustdoc/clean/types.rs:1855:14
     |
1855 |     crate fn is_dummy(&self) -> bool {
     |
     |
     = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

