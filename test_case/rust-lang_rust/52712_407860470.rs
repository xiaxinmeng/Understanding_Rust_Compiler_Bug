plain
[00:25:27] error: aborting due to 2 previous errors
[00:25:27] 
[00:25:27] For more information about this error, try `rustc --explain E0080`.
[00:25:27] 
[00:25:27] note: the compiler unexpectedly panicked. this is a bug.
[00:25:27] 
[00:25:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:25:27] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:25:27] 
[00:25:27] 
[00:25:27] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:25:27] 
[00:25:27] note: some of the compiler flags provided by cargo are hidden
[00:25:27] error: Could not compile `rustc`.
[00:25:27] 
[00:25:27] Caused by:
[00:25:27] Caused by:
[00:25:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=a9489390ece37231 -C extra-filename=-a9489390ece37231 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/lit/modules
149116 ./src/llvm-emscripten/test
145440 ./obj/build/bootstrap/debug/incremental
130572 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130572 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130568 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f38ovn7vgy-1ivc33o-2rpcqp3yi7261
97528 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
97464 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
88260 ./obj/build/x86_64-unknown-linux-gnu/stage1
88236 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
