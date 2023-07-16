
~/rmp-rpc ❯❯❯ cargo clippy                                                                                                                                                             master
lib: rmp-rpc
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling rmp-rpc v0.2.0 (file:///home/corentih/rmp-rpc)                     
error: internal compiler error: librustc/infer/canonical.rs:703: failed to lift `QueryResult { var_values: CanonicalVarValues { var_values: [] }, region_constraints: QueryRegionConstraints { region_outlives: [(ReErased, '?0), ('?0, ReErased)], ty_outlives: [] }, certainty: Proven, value: NormalizationResult { normalized_ty: std::collections::hash_map::RandomState } }`, canonicalized from `QueryResult { var_values: CanonicalVarValues { var_values: [] }, region_constraints: QueryRegionConstraints { region_outlives: [(ReErased, '_#0r), ('_#0r, ReErased)], ty_outlives: [] }, certainty: Proven, value: NormalizationResult { normalized_ty: std::collections::hash_map::RandomState } }`

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (a04b88d19 2018-03-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rmp-rpc`.

To learn more, run the command again with --verbose.
