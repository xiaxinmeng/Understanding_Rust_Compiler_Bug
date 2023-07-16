plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:083c56c4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:49:22]    Compiling cmake v0.1.33
[00:49:22]    Compiling backtrace-sys v0.1.27
[00:49:25]    Compiling profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)
[00:49:25]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:49:26] thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:997:5
[00:49:26] 
[00:49:26] error: internal compiler error: unexpected panic
[00:49:26] 
[00:49:26] note: the compiler unexpectedly panicked. this is a bug.
[00:49:26] note: the compiler unexpectedly panicked. this is a bug.
[00:49:26] 
[00:49:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:49:26] 
[00:49:26] note: rustc 1.35.0-nightly (640017302 2019-03-28) running on x86_64-unknown-linux-gnu
[00:49:26] 
[00:49:26] note: compiler flags: -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C linker=clang -C prefer-dynamic -C linker=clang -C debuginfo=1 -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:49:26] note: some of the compiler flags provided by cargo are hidden
[00:49:26] 
[00:49:27] error: Could not compile `core`.
[00:49:27] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:0efffd3f:start=1553807847387479696,finish=1553807847402592018,duration=15112322
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14647747
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0081a10f
travis_time:start:0081a10f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27a67a1e
$ dmesg | grep -i kill
