plain
[00:13:28]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:28]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:28]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:29]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:33] error[E0621]: explicit lifetime required in the type of `infcx`
[00:13:33]    --> librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:504:60
[00:13:33]     |
[00:13:33] 487 |         infcx: &InferCtxt<'_, '_, 'tcx>,
[00:13:33]     |         ----- consider changing the type of `infcx` to `&rustc::infer::InferCtxt<'tcx, '_, 'tcx>`
[00:13:33] ...
[00:13:33] 504 |                     let bounds = predicates_of.instantiate(infcx.tcx, substs);
[00:13:33]     |                                                            ^^^^^^^^^ lifetime `'tcx` required
[00:13:40] error: aborting due to previous error
[00:13:40] 
[00:13:40] For more information about this error, try `rustc --explain E0621`.
[00:13:40] error: Could not compile `rustc_mir`.
[00:13:40] error: Could not compile `rustc_mir`.
[00:13:40] 
[00:13:40] Caused by:
[00:13:40]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=7952c237ec0d4952 -C extra-filename=-7952c237ec0d4952 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4ee92b74dcb65ca6.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-cfbc17aa3c766576.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4a211d9e23f5aeb5.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-61336079186baa43.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f882aab6100635ab.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-51fd1bd0441a9815.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-26b6009735d1b07c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ef45b71e578357b1.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:14:48] error: build failed
[00:14:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:48] expected success, got: exit code: 101
[00:14:48] expected success, got: exit code: 101
[00:14:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:14:48] travis_fold:end:stage0-rustc

[00:14:48] travis_time:end:stage0-rustc:start=1536080690856832469,finish=1536081220814804188,duration=529957971719


[00:14:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:48] Build completed unsuccessfully in 0:09:43
[00:14:48] Makefile:28: recipe for target 'all' failed
[00:14:48] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08470a00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151200 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148964 ./obj/build/bootstrap/debug/incremental
134532 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134528 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4htnk0fx7-15eu063-29k09wkatlb53
111068 ./src/llvm/test/CodeGen
103868 ./src/tools/lldb
98948 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
---
travis_time:end:0cbc3880:start=1536081221498329716,finish=1536081221505843206,duration=7513490
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0caee348
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0003e104
travis_time:start:0003e104
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07dc2ede
$ dmesg | grep -i kill
