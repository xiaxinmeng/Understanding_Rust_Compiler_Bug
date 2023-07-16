plain
[00:07:19] 
[00:07:19] error: Could not compile `syntax_ext`.
[00:07:19] 
[00:07:19] Caused by:
[00:07:19]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=152376803e8694f9 -C extra-filename=-152376803e8694f9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-6ad9c4f0e3eb0853.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-c5970c5befc4b045.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so` (exit code: 101)
[00:13:02] error: build failed
[00:13:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:02] expected success, got: exit code: 101
[00:13:02] expected success, got: exit code: 101
[00:13:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:13:02] travis_fold:end:stage0-rustc

[00:13:02] travis_time:end:stage0-rustc:start=1530709661509315972,finish=1530710156097007040,duration=494587691068


[00:13:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:02] Build completed unsuccessfully in 0:08:26
[00:13:02] Makefile:28: recipe for target 'all' failed
[00:13:02] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02212070
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:018d5c1f:start=1530710156837729300,finish=1530710156843856072,duration=6126772
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02b33e7f
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003d4a87
$ dmesg | grep -i kill
