plain
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00ms


running 65 tests
...............F.F...............................................
failures:


---- errors::verify_ast_passes_equality_in_where_58 stdout ----
thread 'errors::verify_ast_passes_equality_in_where_58' panicked at 'ast_passes_equality_in_where_refs: `ident` not found, `["span", "assoc", "assoc2"]`', compiler/rustc_ast_passes/src/errors.rs:602:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `-p rustc_ast_passes --lib`

---- errors::verify_ast_passes_feature_on_non_nightly_60 stdout ----
thread 'errors::verify_ast_passes_feature_on_non_nightly_60' panicked at 'ast_passes_feature_on_non_nightly_refs: `name` not found, `["span", "channel", "stable_features", "sugg"]`', compiler/rustc_ast_passes/src/errors.rs:649:10

failures:
    errors::verify_ast_passes_equality_in_where_58
    errors::verify_ast_passes_feature_on_non_nightly_60
