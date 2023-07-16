plain
[01:10:30] running 8 tests
[01:10:30] .....F..
[01:10:30] failures:
[01:10:30] 
[01:10:30] ---- format_foreign::shell::tests::test_escape stdout ----
[01:10:30] thread 'format_foreign::shell::tests::test_escape' panicked at 'assertion failed: `(left == right)`
[01:10:30]   left: `Some((Escape((0, 2)), " leading escape"))`,
[01:10:30]  right: `Some((Escape((0, 1)), " leading escape"))`', libsyntax_ext/format_foreign.rs:930:13
[01:10:30] 
[01:10:30] 
[01:10:30] failures:
[01:10:30]     format_foreign::shell::tests::test_escape
[01:10:30]     format_foreign::shell::tests::test_escape
[01:10:30] 
[01:10:30] test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:30] 
[01:10:30] error: test failed, to rerun pass '--lib'
[01:10:30] 
[01:10:30] 
[01:10:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_ext" "--" "--quiet"
[01:10:30] 
[01:10:30] 
[01:10:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:30] Build completed unsuccessfully in 0:28:26
[01:10:30] Build completed unsuccessfully in 0:28:26
[01:10:30] make: *** [check] Error 1
[01:10:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09a87500
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
