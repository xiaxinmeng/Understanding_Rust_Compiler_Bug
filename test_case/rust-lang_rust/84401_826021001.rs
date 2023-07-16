plain
    Checking semver v0.11.0
    Checking toml v0.5.7
    Checking url v2.1.1
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
error[E0599]: no method named `to_def_id` found for struct `DefId` in the current scope
    |
    |
587 |         .map_or(false, |(entry_fn_def_id, _)| def_id == entry_fn_def_id.to_def_id())
    |                                                                         ^^^^^^^^^ method not found in `DefId`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_utils`
