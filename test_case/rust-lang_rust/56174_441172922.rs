plain
travis_time:end:01f22068:start=1542951164650899776,finish=1542951220996369949,duration=56345470173
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:48:48] test test::test_goto_def ... FAILED
[01:49:48] test test::test_infer_bin ... test test::test_infer_bin has been running for over 60 seconds
[01:50:08] test test::test_highlight ... FAILED
[01:51:08] test test::test_infer_custom_bin ... test test::test_infer_custom_bin has been running for over 60 seconds
[01:52:50] ERROR 2018-11-23T07:26:41Z: rls::actions: failed to fetch project model, using fallback: failed to load source for a dependency on `fnv`
[01:53:50] test test::test_infer_lib ... test test::test_infer_lib has been running for over 60 seconds
[01:53:50] test test::test_infer_lib ... test test::test_infer_lib has been running for over 60 seconds
[01:55:32] ERROR 2018-11-23T07:29:22Z: rls::actions: failed to fetch project model, using fallback: failed to load source for a dependency on `fnv`
[01:56:32] test test::test_multiple_binaries ... test test::test_multiple_binaries has been running for over 60 seconds
[01:56:52] test test::test_infer_bin ... FAILED
[01:57:52] test test::test_no_default_features ... test test::test_no_default_features has been running for over 60 seconds
[01:58:13] test test::test_infer_custom_bin ... FAILED
---
[02:10:20] test test::test_workspace_symbol_duplicates ... FAILED
[02:10:20] 
[02:10:20] failures:
[02:10:20] 
[02:10:20] ---- test::lens::test_lens_run stdout ----
[02:10:20] thread 'test::lens::test_lens_run' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:355:21
[02:10:20] 
[02:10:20] ---- test::test_bin_lib_project stdout ----
[02:10:20] ---- test::test_bin_lib_project stdout ----
[02:10:20] thread 'test::test_bin_lib_project' panicked at 'Assert failed: Could not find `bin_lib/tests/tests.rs` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/bin_lib/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_all_targets stdout ----
[02:10:20] ---- test::test_all_targets stdout ----
[02:10:20] thread 'test::test_all_targets' panicked at 'Assert failed: Could not find `bin_lib/tests/tests.rs` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/bin_lib/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_borrow_error stdout ----
[02:10:20] ---- test::test_borrow_error stdout ----
[02:10:20] thread 'test::test_borrow_error' panicked at 'Assert failed: Could not find `"message":"cannot borrow `x` as mutable more than once at a time` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/borrow_error/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_deglob stdout ----
[02:10:20] thread 'test::test_deglob' panicked at 'assertion failed: `(left == right)`
[02:10:20]   left: `Null`,
[02:10:20]  right: `100`', tools/rls/src/test/mod.rs:1472:9
[02:10:20]  right: `100`', tools/rls/src/test/mod.rs:1472:9
[02:10:20] 
[02:10:20] ---- test::test_features stdout ----
[02:10:20] thread 'test::test_features' panicked at 'Assert failed: Could not find `"message":"cannot find struct, variant or union type `Bar` in this scope` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/features/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_find_all_refs_no_cfg_test stdout ----
[02:10:20] ---- test::test_find_all_refs_no_cfg_test stdout ----
[02:10:20] thread 'test::test_find_all_refs_no_cfg_test' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_find_all_refs stdout ----
[02:10:20] ---- test::test_find_all_refs stdout ----
[02:10:20] thread 'test::test_find_all_refs' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_find_impls stdout ----
[02:10:20] ---- test::test_find_impls stdout ----
[02:10:20] thread 'test::test_find_impls' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_goto_def stdout ----
[02:10:20] ---- test::test_goto_def stdout ----
[02:10:20] thread 'test::test_goto_def' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_highlight stdout ----
[02:10:20] ---- test::test_highlight stdout ----
[02:10:20] thread 'test::test_highlight' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_hover stdout ----
[02:10:20] ---- test::test_hover stdout ----
[02:10:20] thread 'test::test_hover' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_hover_after_src_line_change stdout ----
[02:10:20] ---- test::test_hover_after_src_line_change stdout ----
[02:10:20] thread 'test::test_hover_after_src_line_change' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_infer_bin stdout ----
[02:10:20] ---- test::test_infer_bin stdout ----
[02:10:20] thread 'test::test_infer_bin' panicked at 'Assert failed: Could not find `struct is never constructed: `UnusedBin`` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/infer_bin/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_infer_custom_bin stdout ----
[02:10:20] ---- test::test_infer_custom_bin stdout ----
[02:10:20] thread 'test::test_infer_custom_bin' panicked at 'Assert failed: Could not find `struct is never constructed: `UnusedCustomBin`` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/infer_custom_bin/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_infer_lib stdout ----
[02:10:20] ---- test::test_infer_lib stdout ----
[02:10:20] thread 'test::test_infer_lib' panicked at 'Assert failed: Could not find `struct is never constructed: `UnusedLib`` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/infer_lib/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_multiple_binaries stdout ----
[02:10:20] ---- test::test_multiple_binaries stdout ----
[02:10:20] thread 'test::test_multiple_binaries' panicked at 'Assert failed: Could not find `unused variable: `bin_name` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/multiple_bins/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_no_default_features stdout ----
[02:10:20] ---- test::test_no_default_features stdout ----
[02:10:20] thread 'test::test_no_default_features' panicked at 'Assert failed: Could not find `"message":"cannot find struct, variant or union type `Baz` in this scope` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"failed to load source for a dependency on `fnv`\nUnable to update registry `https://github.com/rust-lang/crates.io-index`\nfailed to fetch `https://github.com/rust-lang/crates.io-index`\ncurl error: Could not resolve host: github.com\n; class=Net (12)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///checkout/obj/build/rls-test-data/test_data/features/Cargo.toml"}}`', tools/rls/src/test/harness.rs:227:9
[02:10:20] ---- test::test_reformat_with_range stdout ----
[02:10:20] ---- test::test_reformat_with_range stdout ----
[02:10:20] thread 'test::test_reformat_with_range' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_reformat stdout ----
[02:10:20] ---- test::test_reformat stdout ----
[02:10:20] thread 'test::test_reformat' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_rename stdout ----
[02:10:20] ---- test::test_rename stdout ----
[02:10:20] thread 'test::test_rename' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_shutdown stdout ----
[02:10:20] ---- test::test_shutdown stdout ----
[02:10:20] thread 'test::test_shutdown' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_workspace_symbol stdout ----
[02:10:20] ---- test::test_workspace_symbol stdout ----
[02:10:20] thread 'test::test_workspace_symbol' panicked at 'Missing id field', libcore/option.rs:1008:5
[02:10:20] ---- test::test_workspace_symbol_duplicates stdout ----
[02:10:20] ---- test::test_workspace_symbol_duplicates stdout ----
[02:10:20] thread 'test::test_workspace_symbol_duplicates' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:355:21
[02:10:20] 
[02:10:20] failures:
[02:10:20]     test::lens::test_lens_run
[02:10:20]     test::test_all_targets
