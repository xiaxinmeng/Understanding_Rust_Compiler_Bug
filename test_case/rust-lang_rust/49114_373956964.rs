
error: internal compiler error: librustc/infer/canonical.rs:703: failed to lift `QueryResult { var_values: CanonicalVarValues { var_values: [] }, region_constraints: QueryRegionConstraints { region_outlives: [(ReErased, '?0), ('?0, ReErased)], ty_outlives: [] }, certainty: Proven, value: NormalizationResult { normalized_ty: std::result::Result<futures::Async<security_module::messages::ResponseSignature>, futures::Canceled> } }`, canonicalized from `QueryResult { var_values: CanonicalVarValues { var_values: [] }, region_constraints: QueryRegionConstraints { region_outlives: [(ReErased, '_#0r), ('_#0r, ReErased)], ty_outlives: [] }, certainty: Proven, value: NormalizationResult { normalized_ty: std::result::Result<futures::Async<security_module::messages::ResponseSignature>, futures::Canceled> } }`
thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:540:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.26.0-nightly (55c984ee5 2018-03-16) running on x86_64-unknown-linux-gnu
note: compiler flags: -C debuginfo=2 -C linker=gcc-6 -C incremental
note: some of the compiler flags provided by cargo are hidden
