plain
travis_time:end:009d7ee8:start=1557847101712934513,finish=1557847188606065128,duration=86893130615
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:28:59]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[00:29:02]    Compiling crossbeam-utils v0.2.2
[00:29:02]    Compiling log v0.4.6
[00:29:02]    Compiling arrayvec v0.4.7
[00:29:02] error: internal compiler error: src/librustc_mir/interpret/intern.rs:161: vtables are behind references and should thus be frozen
[00:29:02] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:29:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:29:02] error: aborting due to previous error
[00:29:02] 
[00:29:02] 
[00:29:02] 
[00:29:02] note: the compiler unexpectedly panicked. this is a bug.
[00:29:02] 
[00:29:02] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:29:02] 
[00:29:02] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:29:02] 
[00:29:02] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:29:02] note: some of the compiler flags provided by cargo are hidden
[00:29:02] 
[00:29:02] error: Could not compile `log`.
[00:29:02] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:049dc658:start=1557848947035028449,finish=1557848947039762599,duration=4734150
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b42fa1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16c5c775
travis_time:start:16c5c775
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0020931c
$ dmesg | grep -i kill
