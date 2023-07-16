 rust
.\local\boom.rs:13:23: 13:48 error: mismatched types:
 expected `core::marker::Sized`,
    found `core::marker::Sized`
(lifetime mismatch) [E0308]
.\local\boom.rs:13     fn scan_from() -> Result<(), ScanErrorKind> {
                                         ^~~~~~~~~~~~~~~~~~~~~~~~~
.\local\boom.rs:13:23: 13:48 help: run `rustc --explain E0308` to see a detailed explanation
.\local\boom.rs:13:49: 15:6 note: the lifetime 'a as defined on the block at 13:48...
.\local\boom.rs:13     fn scan_from() -> Result<(), ScanErrorKind> {
.\local\boom.rs:14         Ok(())
.\local\boom.rs:15     }
note: ...does not necessarily outlive the static lifetime
error: aborting due to previous error
