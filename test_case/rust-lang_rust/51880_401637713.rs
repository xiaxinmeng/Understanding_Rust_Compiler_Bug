plain
[00:20:32]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:20:35]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:21:30]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:21:41]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:22:09] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/mod.rs:4922:41
[00:22:09] 
[00:22:09] error: internal compiler error: unexpected panic
[00:22:09] 
[00:22:09] 
[00:22:09] note: the compiler unexpectedly panicked. this is a bug.
[00:22:09] 
[00:22:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:22:09] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:22:09] 
[00:22:09] 
[00:22:09] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:22:09] 
[00:22:09] note: some of the compiler flags provided by cargo are hidden
[00:22:09] error: Could not compile `rustc`.
[00:22:09] 
[00:22:09] Caused by:
[00:22:09] Caused by:
[00:22:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=4d03a96572f416f4 -C extra-filename=-4d03a96572f416f4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-998809afc03a8106.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknow2887864 .
1595516 ./obj/build
1004196 ./obj/build/x86_64-unknown-linux-gnu
729584 ./src
562228 ./.git
---
159788 ./.git/modules/src
149116 ./src/llvm-emscripten/test
143020 ./obj/build/bootstrap/debug/incremental
128512 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
128508 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2ie5gec1y-1mfu6vc-1n73vja9bcp2q
97520 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89808 ./src/llvm/test/CodeGen
89008 ./obj/build/x86_64-unknown-linux-gnu/stage1
88984 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
