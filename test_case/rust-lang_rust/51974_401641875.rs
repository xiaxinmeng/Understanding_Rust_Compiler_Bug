plain
[01:16:23] 
[01:16:23]    Doc-tests std
[01:16:25] 
[01:16:25] running 931 tests
[01:16:58] iiFi................................................................................................
[01:17:29] ......i......i...i......i...........................................................................
[01:17:38] ....................................................................................................
[01:17:54] .....................iiii........ii.................................................................
[01:18:03] ....................................................................................................
---
[01:18:59] ---- alloc.rs - alloc (line 28) stdout ----
[01:18:59] error[E0432]: unresolved import `super::GLOBAL`
[01:18:59]  --> alloc.rs:30:1
[01:18:59]   |
[01:18:59] 5 | static GLOBAL: std::alloc::System = std::alloc::System;
[01:18:59]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `GLOBAL` in the root
[01:18:59] 
[01:18:59] thread 'alloc.rs - alloc (line 28)' panicked at 'assertion failed: prev.is_none()', librustc/middle/region.rs:509:13
[01:18:59] thread 'alloc.rs - alloc (line 28)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:18:59] 
[01:18:59] 
[01:18:59] failures:
[01:18:59] failures:
[01:18:59]     alloc.rs - alloc (line 28)
[01:18:59] 
[01:18:59] test result: FAILED. 906 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:18:59] 
[01:18:59] error: test failed, to rerun pass '--doc'
[01:18:59] 
[01:18:59] 
[01:18:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:18:59] 
[01:18:59] 
[01:18:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:59] Build completed unsuccessfully in 0:36:34
[01:18:59] Build completed unsuccessfully in 0:36:34
[01:18:59] make: *** [check] Error 1
[01:18:59] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:182bbb78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
