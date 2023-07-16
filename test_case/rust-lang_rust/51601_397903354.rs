plain
[01:17:42] 
[01:17:42] running 845 tests
[01:17:42] ....................................................................................................
[01:17:42] ....................................................................................................
[01:17:42] ..........................................................F.........................................
[01:17:42] ....................................................................................................
[01:17:42] ....................................................................................................
[01:17:42] ....................................................................................................
[01:17:42] ....................................................................................................
[01:17:42] ....................................................................................................
[01:17:43] .............................................
[01:17:43] failures:
[01:17:43] 
[01:17:43] ---- iter::test_range_step stdout ----
[01:17:43] thread 'iter::test_range_step' panicked at 'assertion failed: `(left == right)`
[01:17:43]   left: `[200]`,
[01:17:43]  right: `[200, 250]`', libcore/../libcore/tests/iter.rs:1605:5
[01:17:43] 
[01:17:43] failures:
[01:17:43]     iter::test_range_step
[01:17:43] 
[01:17:43] 
[01:17:43] test result: FAILED. 842 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:17:43] 
[01:17:43] error: test failed, to rerun pass '--test coretests'
[01:17:43] 
[01:17:43] 
[01:17:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:17:43] 
[01:17:43] 
[01:17:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:43] Build completed unsuccessfully in 0:29:51
[01:17:43] Build completed unsuccessfully in 0:29:51
[01:17:43] Makefile:58: recipe for target 'check' failed
[01:17:43] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2eb90abf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
