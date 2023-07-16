plain
[00:23:11]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:15]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:15]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:24:26]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:26:18] thread 'main' panicked at 'internal error: entered unreachable code', librustc/mir/interpret/mod.rs:164:10
[00:26:19] 
[00:26:19] error: internal compiler error: unexpected panic
[00:26:19] 
[00:26:19] 
[00:26:19] note: the compiler unexpectedly panicked. this is a bug.
[00:26:19] 
[00:26:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:19] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:26:19] 
[00:26:19] 
[00:26:19] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=3 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:26:19] 
[00:26:19] note: some of the compiler flags provided by cargo are hidden
[00:26:19] error: Could not compile `rustc`.
[00:26:19] 
[00:26:19] Caused by:
[00:26:19] Caused by:
[00:26:19]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=62d74f21a64f8c0c -C extra-filename=-62d74f21a64f8c0c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-450feded456a4278.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-3846f1b0424591fd.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-3720d8c52a6bc989.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-f21bfea456e2feba.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-5c863390141836fe.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-70b92be3dfddcce2.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-270afc7a968c2570.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-ff77786b985e61bc.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-9dea40d5c994cba1.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e7bbb7d6e0541d97.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3762ade15a64029b.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-575f47f158b62d9a.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64own-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:19] expected success, got: exit code: 101
[00:26:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:26:19] travis_fold:end:stage1-rustc

[00:26:19] travis_time:end:stage1-rustc:start=1527227484793947373,finish=1527227712491800476,duration=227697853103


[00:26:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:19] Build completed unsuccessfully in 0:21:36
[00:26:19] Makefile:28: recipe for target 'all' failed
[00:26:19] make: *** [all] Error 1
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68788 ./src/llvm/lib
65420 ./src/llvm-emscripten/test/CodeGen
61608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
