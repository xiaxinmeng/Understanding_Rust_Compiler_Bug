plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:30effc33
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:07:19]    Compiling tempfile v3.0.5
[00:07:20]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:07:21]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:38]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:55] error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:07:55] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
[00:07:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:07:55] error: aborting due to previous error
[00:07:55] 
[00:07:55] 
[00:07:55] 
[00:07:55] note: the compiler unexpectedly panicked. this is a bug.
[00:07:55] 
[00:07:55] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:07:55] 
[00:07:55] note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu
[00:07:55] 
[00:07:55] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C linker=clang -C prefer-dynamic -C linker=clang -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:07:55] note: some of the compiler flags provided by cargo are hidden
[00:07:55] 
[00:07:55] error: Could not compile `syntax`.
[00:07:55] 
---
travis_time:end:022d48a0:start=1550231809808669205,finish=1550231809816914709,duration=8245504
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01453848
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3d98a0d4
travis_time:start:3d98a0d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:086d9cd9
$ dmesg | grep -i kill
