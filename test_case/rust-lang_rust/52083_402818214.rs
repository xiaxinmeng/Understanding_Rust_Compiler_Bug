plain
[00:18:06]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:18:06] error: unused import: `rustc::session::config::BorrowckMode`
[00:18:06]   --> librustc_borrowck/borrowck/mod.rs:39:5
[00:18:06]    |
[00:18:06] 39 | use rustc::session::config::BorrowckMode;
[00:18:06]    |
[00:18:06]    = note: `-D unused-imports` implied by `-D warnings`
[00:18:06] 
[00:18:07] error: aborting due to previous error
[00:18:07] error: aborting due to previous error
[00:18:07] 
[00:18:07] error: Could not compile `rustc_borrowck`.
[00:18:07] 
[00:18:07] Caused by:
[00:18:07]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_borrowck librustc_borrowck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=b63ed12ca231f4e3 -C extra-filename=-b63ed12ca231f4e3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-8fc603afb8ea9b13.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
[00:18:19] error: build failed
[00:18:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:19] expected success, got: exit code: 101
[00:18:19] expected success, got: exit code: 101
[00:18:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:18:19] travis_fold:end:stage0-rustc

[00:18:19] travis_time:end:stage0-rustc:start=1530815735888662491,finish=1530816541396870522,duration=805508208031


[00:18:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:19] Build completed unsuccessfully in 0:13:37
[00:18:19] Makefile:28: recipe for target 'all' failed
[00:18:19] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01bde1eb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c94e1b0:start=1530816541999796236,finish=1530816542006246005,duration=6449769
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dee3931
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13cfb675
$ dmesg | grep -i kill
