plain
[00:24:18]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:24:22]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:25:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:25:33]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:26:05] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/mod.rs:4922:41
[00:26:05] 
[00:26:05] error: internal compiler error: unexpected panic
[00:26:05] 
[00:26:05] 
[00:26:05] note: the compiler unexpectedly panicked. this is a bug.
[00:26:05] 
[00:26:05] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:05] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:26:05] 
[00:26:05] 
[00:26:05] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:26:05] 
[00:26:05] note: some of the compiler flags provided by cargo are hidden
[00:26:05] error: Could not compile `rustc`.
[00:26:05] 
[00:26:05] Caused by:
[00:26:05] Caused by:
[00:26:05]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=2c4c664d38a8396f -C extra-filename=-2c4c664d38a8396f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-a77c8710dcd8c364.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-e1907f3777b8aca0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da75a1391e0819dc.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-053154eea32faea2.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-f80cd5bfeff4f735.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-44e761f8b954b499.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6bfadf3610c3a5d3.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-688e8d8d7a3790aa.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c8176dd12ca6552b.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-aecf4befc8dca4eb.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-feaaacffe8f40a01.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7c0257f6d147c4b0.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-8407b8ebe61a1581.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1f8e51c7340dd3a6.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-d441f38020b30a7e.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5281aac7329915d9.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-e0b46302cb6adfc1.rlib --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-947947ef42b5e09f.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-7e250215e5863400.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-ad6626ca6f9c7adf.rlib --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-c83988feac67e21f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-r979312,finish=1530314510995631863,duration=1566212652551
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17958578
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun 29 23:21:51 UTC 2018
Fri Jun 29 23:21:51 UTC 2018
t/modules
154100 ./.git/modules/src
149116 ./src/llvm-emscripten/test
138748 ./obj/build/bootstrap/debug/incremental
126984 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
124176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2g84i5a3c-1e0n1zi-38adk87dv7782
92568 ./obj/build/x86_64-unknown-linux-gnu/stage1
92544 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89820 ./src/llvm/test/CodeGen
79480 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
---
travis_time:end:2f4de1b2:start=1530314511410272169,finish=1530314511421460221,duration=11188052
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24e82354
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27093340
$ dmesg | grep -i kill
