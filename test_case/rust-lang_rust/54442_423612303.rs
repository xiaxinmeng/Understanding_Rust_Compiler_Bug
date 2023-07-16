plain
[00:06:24]    Compiling chalk-engine v0.7.0
[00:06:24]    Compiling env_logger v0.5.12
[00:06:25]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:06:25]    Compiling parking_lot_core v0.3.0
[00:06:26] error[E0277]: no implementation for `u128 & i32`
[00:06:26]     --> librustc_apfloat/ieee.rs:2608:58
[00:06:26]      |
[00:06:26] 2608 |         let select = |limb, i| (limb >> (i * HALF_BITS)) & ((1 << HALF_BITS) - 1);
[00:06:26]      |                                                          ^ no implementation for `u128 & i32`
[00:06:26]      |
[00:06:26]      = help: the trait `std::ops::BitAnd<i32>` is not implemented for `u128`
[00:06:26] error: aborting due to previous error
[00:06:26] 
[00:06:26] For more information about this error, try `rustc --explain E0277`.
[00:06:26] error: Could not compile `rustc_apfloat`.
[00:06:26] error: Could not compile `rustc_apfloat`.
[00:06:26] warning: build failed, waiting for other jobs to finish...
[00:06:28] error: build failed
[00:06:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:28] expected success, got: exit code: 101
[00:06:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:06:28] travis_fold:end:stage0-rustc

[00:06:28] travis_time:end:stage0-rustc:start=1537550638622924466,finish=1537550666003542093,duration=27380617627


[00:06:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:28] Build completed unsuccessfully in 0:01:21
[00:06:28] Makefile:28: recipe for target 'all' failed
[00:06:28] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1135272c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1350a1e0
$ echo "#### Build failed; Di ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
64960 ./src/test
62448 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
60840 ./src/llvm-emscripten/lib
56436 ./src/llvm/test/MC
