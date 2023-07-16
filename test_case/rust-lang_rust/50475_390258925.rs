plain
[00:24:11]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:24:15]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:24:15]    Compiling cmake v0.1.30
[00:24:15]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:24:20] thread 'main' panicked at 'assertion failed: !value.has_escaping_regions()', librustc/ty/sty.rs:709:9
[00:24:21] 
[00:24:21] error: internal compiler error: unexpected panic
[00:24:21] 
[00:24:21] 
[00:24:21] note: the compiler unexpectedly panicked. this is a bug.
[00:24:21] 
[00:24:21] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:24:21] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:24:21] 
[00:24:21] 
[00:24:21] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=3 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:24:21] 
[00:24:21] note: some of the compiler flags provided by cargo are hidden
[00:24:21] error: Could not compile `core`.
[00:24:21] 
[00:24:21] Caused by:
[00:24:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=9b2cf3afb370a473 -C extra-filename=-9b2cf3afb370a473 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
