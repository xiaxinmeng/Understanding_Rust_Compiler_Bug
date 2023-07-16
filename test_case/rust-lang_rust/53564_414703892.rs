plain
[00:19:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:19:43] 
[00:19:43] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:19:43] 
[00:19:43] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:19:43] note: some of the compiler flags provided by cargo are hidden
[00:19:43] 
[00:19:43] error: Could not compile `core`.
[00:19:43] 
---
151200 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f42a075x8w-h9mzy1-1iplbejydu42a
103868 ./src/tools/lldb
98956 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
90576 ./obj/build/x86_64-unknown-linux-gnu/stage1
---
travis_time:end:0e9c4e8e:start=1534863313532289125,finish=1534863313540257491,duration=7968366
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:32bb0106
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:002678e8
travis_time:start:002678e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i38
