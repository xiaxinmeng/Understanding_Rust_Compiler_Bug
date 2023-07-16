plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:20:20] error: unused variable: `arg_0`
[00:20:20]    --> libcore/fmt/mod.rs:104:43
[00:20:20]     |
[00:20:20] 104 | #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
[00:20:20]     |                                           ^^^^ help: consider using `_arg_0` instead
---
[00:20:20]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:20:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:20] expected success, got: exit code: 101
[00:20:20] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1075:9
[00:20:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:20] travis_fold:end:stage1-std
[00:20:20] travis_time:end:stage1-std:start=1522892524791158261,finish=1522892560294824940,duration=35503666679
[00:20:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:20] Build completed unsuccessfully in 0:15:40
[00:20:20] make: *** [all] Error 1
[00:20:20] Makefile:28: recipe for target 'all' failed
