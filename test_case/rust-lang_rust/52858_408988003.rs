plain
[01:03:46] .....................i..............................................................................
[01:04:12] ...................i................................................................................
] error: internal compiler error: unexpected panic
[01:04:19] 
[01:04:19] note: the compiler unexpectedly panicked. this is a bug.
[01:04:19] 
[01:04:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:19] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[01:04:19] 
[01:04:19] 
[01:04:19] ------------------------------------------
---
[01:04:19] 
[01:04:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:04:19] 
[01:04:19] 
[01:04:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host"

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:164483a6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
