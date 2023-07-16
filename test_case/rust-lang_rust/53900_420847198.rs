plain
[00:12:34]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:34]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:12:35] [RUSTC-TIMING] rustc_metadata_utils test:false 0.731
[00:12:35]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:12:40] error[E0599]: no variant named `Anon` found for type `rustc::ty::TyKind<'_>` in the current scope
[00:12:40]    --> librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:492:22
[00:12:40]     |
[00:12:40] 492 |                 sty: ty::TyKind::Anon(did, substs),
[00:12:40] 
[00:12:48] error: aborting due to previous error
[00:12:48] 
[00:12:48] For more information about this error, try `rustc --explain E0599`.
[00:12:48] For more information about this error, try `rustc --explain E0599`.
[00:12:48] [RUSTC-TIMING] rustc_mir test:false 14.150
[00:12:48] error: Could not compile `rustc_mir`.
[00:12:48] 
[00:12:48] Caused by:
[00:12:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=9389ef21a7981207 -C extra-filename=-9389ef21a7981207 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-93ff3b002b18b7ed.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-09c0f3a89ad0d6b5.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-96aac12abd62414d.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05301c67193a930e.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ac364f854372fe.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4a08b81d2b6640c1.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-0651ffc5a9129db1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-47b99ffec2efbd05.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:13:04] [RUSTC-TIMING] rustc_traits test:false 28.615
[00:13:23] [RUSTC-TIMING] rustc_incremental test:false 49.088
[00:13:57] [RUSTC-TIMING] rustc_typeck test:false 83.116
[00:13:57] error: build failed
[00:13:57] error: build failed
[00:13:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:57] expected success, got: exit code: 101
[00:13:57] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:13:57] travis_fold:end:stage0-rustc

[00:13:57] travis_time:end:stage0-rustc:start=1536799845921628135,finish=1536800395025321501,duration=549103693366

---
travis_time:end:0a23c238:start=1536800395853058268,finish=1536800395860147998,duration=7089730
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00163726
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06e528dc
travis_time:start:06e528dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01eb6ee6
$ dmesg | grep -i kill
