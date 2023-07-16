plain
[01:24:25] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:25]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[01:24:27] error[E0425]: cannot find function `parse_item_from_source_str` in this scope
[01:24:27]     --> libsyntax/parse/mod.rs:1051:24
[01:24:27]      |
[01:24:27] 1051 |             let item = parse_item_from_source_str(name.clone(), source, &sess)
[01:24:27]      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `parse_stream_from_source_str`?
[01:24:27] 
[01:24:27] error[E0425]: cannot find function `parse_item_from_source_str` in this scope
[01:24:27]     --> libsyntax/parse/mod.rs:1057:24
[01:24:27]      |
[01:24:27] 1057 |             let item = parse_item_from_source_str(name.clone(), source, &sess)
[01:24:27]      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `parse_stream_from_source_str`?
[01:24:27] 
[01:24:27] error[E0425]: cannot find function `parse_item_from_source_str` in this scope
[01:24:27]     --> libsyntax/parse/mod.rs:1065:24
[01:24:27]      |
[01:24:27] 1065 |             let item = parse_item_from_source_str(name, source, &sess).unwrap().unwrap();
[01:24:27]      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `parse_stream_from_source_str`?
[01:24:27] 
[01:24:27] error[E0425]: cannot find function `parse_expr_from_source_str` in module `parse`
[01:24:27]     --> libsyntax/parse/mod.rs:1075:31
[01:24:27]      |
[01:24:27] 1075 |             let expr = parse::parse_expr_from_source_str(PathBuf::from("foo").into(),
[01:24:27]      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `parse_crate_from_source_str`?
[01:24:27] 
[01:24:27] error[E0425]: cannot find function `parse_item_from_source_str` in this scope
[01:24:27]     --> libsyntax/parse/mod.rs:1099:24
[01:24:27]      |
[01:24:27] 1099 |             let item = parse_item_from_source_str(
[01:24:27]      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ did you mean `parse_stream_from_source_str`?
[01:24:27] 
[01:24:28] error[E0599]: no method named `abort_if_errors` found for type `parse::parser::Parser<'_>` in the current scope
[01:24:28]    --> libsyntax/util/parser_testing.rs:38:7
[01:24:28]     |
[01:24:28] 38  |     p.abort_if_errors();
[01:24:28]     | 
[01:24:28]    ::: libsyntax/parse/parser.rs:219:1
[01:24:28]     |
[01:24:28]     |
[01:24:28] 219 | pub struct Parser<'a> {
[01:24:28]     | --------------------- method `abort_if_errors` not found for this
[01:24:35] error: aborting due to 6 previous errors
[01:24:35] 
[01:24:35] Some errors occurred: E0425, E0599.
[01:24:35] For more information about an error, try `rustc --explain E0425`.
[01:24:35] For more information about an error, try `rustc --explain E0425`.
[01:24:35] error: Could not compile `syntax`.
[01:24:35] 
[01:24:35] To learn more, run the command again with --verbose.
[01:24:35] 
[01:24:35] 
[01:24:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:24:35] 
[01:24:35] 
[01:24:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:35] Build completed unsuccessfully in 0:39:54
[01:24:35] Build completed unsuccessfully in 0:39:54
[01:24:35] make: *** [check] Error 1
[01:24:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:130472d9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
