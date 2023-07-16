plain
[00:07:55]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:12]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:27]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:27]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:27] error: unknown start of token: `
[00:14:27]   --> librustc_mir/interpret/eval_context.rs:17:41
[00:14:27]    |
[00:14:27] 17 | ||||||| parent of a2543471ec... Removed `uninitialized_statics` field from `Memory` struct in miri.
[00:14:27]    |                                         ^
[00:14:27] help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
[00:14:27]    |
[00:14:27] 17 | ||||||| parent of a2543471ec... Removed 'uninitialized_statics` field from `Memory` struct in miri.
[00:14:27] 
[00:14:27] error: aborting due to previous error
[00:14:27] 
[00:14:27] error: Could not compile `rustc_mir`.
[00:14:27] error: Could not compile `rustc_mir`.
[00:14:27] 
[00:14:27] Caused by:
[00:14:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8fc603afb8ea9b13 -C extra-filename=-8fc603afb8ea9b13 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-d45628fe21047b42.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-09e9dbd1ef48ffa5.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9c861d36e123bec8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-2e546cbdf217aece.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
[00:16:22] error: build failed
[00:16:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:22] expected success, got: exit code: 101
[00:16:22] expected success, got: exit code: 101
[00:16:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:22] travis_fold:end:stage0-rustc

[00:16:22] travis_time:end:stage0-rustc:start=1530498741937549335,finish=1530499392256924808,duration=650319375473


[00:16:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:22] Build completed unsuccessfully in 0:11:03
[00:16:22] make: *** [all] Error 1
[00:16:22] Makefile:28: recipe for target 'all' failed
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:23b60fbb:start=1530499392872789029,finish=1530499392878969663,duration=6180634
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:05cd119e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:045b92c0
$ dmesg | grep -i kill
