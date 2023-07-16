plain
[00:21:47]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:48]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:48]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:52] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:21:52]   left: `false`,
[00:21:52]  right: `true`', librustc/ty/context.rs:2283:13
[00:21:52] 
[00:21:52] error: internal compiler error: unexpected panic
[00:21:52] 
[00:21:52] 
[00:21:52] note: the compiler unexpectedly panicked. this is a bug.
[00:21:52] 
[00:21:52] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:52] 
[00:21:52] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:21:52] 
[00:21:52] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:52] 
[00:21:52] note: some of the compiler flags provided by cargo are hidden
[00:21:52] error: Could not compile `core`.
[00:21:52] 
[00:21:52] Caused by:
[00:21:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
