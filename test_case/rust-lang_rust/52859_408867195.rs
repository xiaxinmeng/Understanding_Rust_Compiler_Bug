plain
[00:21:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:21:28] 
[00:21:28] error: internal compiler error: unexpected panic
[00:21:28] 
[00:21:28] note: the compiler unexpectedly panicked. this is a bug.
[00:21:28] 
[00:21:28] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:28] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:21:28] 
[00:21:28] 
[00:21:28] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:28] 
[00:21:28] note: some of the compiler flags provided by cargo are hidden
[00:21:28] error: Could not compile `core`.
[00:21:28] 
[00:21:28] Caused by:
[00:21:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
---
157076 ./.git/modules/src
149124 ./src/llvm-emscripten/test
145460 ./obj/build/bootstrap/debug/incremental
130592 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs
130588 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs/s-f3dyotjymq-eow6d-3t5kexjst7huj
97532 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89096 ./obj/build/x86_64-unknown-linux-gnu/stage1
89072 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
77604 ./.git/modules/src/tools
---
travis_time:end:0773f3cd:start=1532958086961119093,finish=1532958086968300925,duration=7181832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:165e2760
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cc7ec7e
travis_time:start:0cc7ec7e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09aaa0ce
$ dmesg | grep -i kill
