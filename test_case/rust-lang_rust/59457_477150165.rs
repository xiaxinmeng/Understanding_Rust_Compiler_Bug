plain
[00:46:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:51] 
[00:46:51] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:46:51] 
[00:46:51] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C linker=clang -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:46:51] note: some of the compiler flags provided by cargo are hidden
[00:46:51] 
[00:46:51] [RUSTC-TIMING] core test:false 34.757
[00:46:51] error: Could not compile `core`.
---
travis_time:end:3445cad0:start=1553692733603495524,finish=1553692733639510340,duration=36014816
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:222741a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00594c08
travis_time:start:00594c08
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00321d74
$ dmesg | grep -i kill
