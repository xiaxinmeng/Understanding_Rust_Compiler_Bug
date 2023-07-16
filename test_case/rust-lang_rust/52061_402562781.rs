plain
[01:07:17] ....................................................................................................
[01:07:17] thread '<unnamed>' panicked at 'explicit panic', liballoc/../liballoc/tests/slice.rs:1315:17
[01:07:17] ....................................................................................................
[01:07:17] ....................................................................................................
[01:07:17] .....................................................................FF.............................
[01:07:20] ..................................
[01:07:20] failures:
[01:07:20] 
[01:07:20] ---- str::to_lowercase stdout ----
[01:07:20] ---- str::to_lowercase stdout ----
[01:07:20] thread 'str::to_lowercase' panicked at 'assertion failed: `(left == right)`
[01:07:20]   left: `"aéǅaé "`,
[01:07:20]  right: `"aéǆaé "`', liballoc/../liballoc/tests/str.rs:1534:5
[01:07:20] ---- str::to_uppercase stdout ----
[01:07:20] thread 'str::to_uppercase' panicked at 'assertion failed: `(left == right)`
[01:07:20] thread 'str::to_uppercase' panicked at 'assertion failed: `(left == right)`
[01:07:20]   left: `"AÉǅSSFIἈΙ"`,
[01:07:20]  right: `"AÉǄSSFIἈΙ"`', liballoc/../liballoc/tests/str.rs:1567:5
[01:07:20] 
[01:07:20] failures:
[01:07:20]     str::to_lowercase
[01:07:20]     str::to_uppercase
[01:07:20]     str::to_uppercase
[01:07:20] 
[01:07:20] test result: FAILED. 532 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:07:20] 
[01:07:20] error: test failed, to rerun pass '--test collectionstests'
[01:07:20] 
[01:07:20] 
[01:07:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:07:20] 
[01:07:20] 
[01:07:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:20] Build completed unsuccessfully in 0:25:18
[01:07:20] Build completed unsuccessfully in 0:25:18
[01:07:20] Makefile:58: recipe for target 'check' failed
[01:07:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:314f20b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
