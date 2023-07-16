plain
travis_time:end:349fafa9:start=1553532677982611730,finish=1553532833850488420,duration=155867876690
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:28:03] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:03] 
[00:28:03] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:28:03] 
[00:28:03] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:03] note: some of the compiler flags provided by cargo are hidden
[00:28:03] 
[00:28:03] error: Could not compile `alloc`.
[00:28:03] 
---
194868 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
157396 ./obj/build/bootstrap/debug/incremental
156420 ./src/llvm-project/clang
142512 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy/s-faoj6o50ih-1ugm5dv-3u618d595ya6l
137348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
134508 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123628 ./src/llvm-project/llvm/test/CodeGen
108536 ./src/llvm-project/lldb
---
travis_time:end:04003150:start=1553534527585948814,finish=1553534527590882555,duration=4933741
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09827ce8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e198600
travis_time:start:1e198600
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ce11a34
$ dmesg | grep -i kill
