plain
[00:21:53]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:53]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:54]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:54]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:57] MISMATCH TRUE TyRef(ReSkolemized(0, BrAnon(0)), TypeAndMut { ty: <I as iter::iterator::Iterator>::Item, mutbl: MutImmutable })
[00:21:57] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:21:57]   left: `false`,
[00:21:57]  right: `true`', librustc/ty/context.rs:2287:13
[00:21:58] 
[00:21:58] error: internal compiler error: unexpected panic
[00:21:58] 
[00:21:58] 
[00:21:58] note: the compiler unexpectedly panicked. this is a bug.
[00:21:58] 
[00:21:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:58] 
[00:21:58] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:21:58] 
[00:21:58] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:58] 
[00:21:58] note: some of the compiler flags provided by cargo are hidden
[00:21:58] error: Could not compile `core`.
[00:21:58] 
[00:21:58] Caused by:
[00:21:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:58] warning: build failed, waiting for other jobs to finish...
[00:21:59] error: build failed
[00:21:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:59] expected success, got: exit code: 101
[00:21:59] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:59] travis_fold:end:stage1-std

[00:21:59] travis_time:end:stage1-std:start=1525249420143288248,finish=1525249439063348836,duration=18920060588


[00:21:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:59] Build completed unsuccessfully in 0:17:02
[00:21:59] Makefile:28: recipe for target 'all' failed
[00:21:59] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f602418
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
