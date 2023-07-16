plain
    Checking semver v0.11.0
    Checking url v2.1.1
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking toml v0.5.7
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:215:9
     |
215  |         StatementKind::FakeRead(_, place) |
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:1485:5
     |
     |
1485 |     FakeRead(Box<(FakeReadCause, Place<'tcx>)>),
     |     ------------------------------------------- tuple variant defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils`
