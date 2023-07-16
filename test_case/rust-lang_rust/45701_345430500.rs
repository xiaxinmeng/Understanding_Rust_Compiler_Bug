
[01:15:57]    Compiling clippy_lints v0.0.171 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[01:16:03] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[01:16:03]    --> src/tools/clippy/clippy_lints/src/lifetimes.rs:328:13
[01:16:03]     |
[01:16:03] 328 |             TyImplTraitExistential(ref param_bounds) |
[01:16:03]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
[01:16:03] 
[01:16:04] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[01:16:04]    --> src/tools/clippy/clippy_lints/src/lifetimes.rs:328:13
[01:16:04]     |
[01:16:04] 328 |             TyImplTraitExistential(ref param_bounds) |
[01:16:04]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
[01:16:04] 
[01:16:04] error: aborting due to previous error
[01:16:04] 
[01:16:04] error: Could not compile `clippy_lints`.
[01:16:04] warning: build failed, waiting for other jobs to finish...
[01:16:06] error: aborting due to previous error
[01:16:06] 
[01:16:06] error: Could not compile `clippy_lints`.
