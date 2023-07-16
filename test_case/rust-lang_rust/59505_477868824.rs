plain
travis_time:end:00e7e7de:start=1553829142514754549,finish=1553829144591295246,duration=2076540697
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:42]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:06]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:05:07]    Compiling synstructure v0.10.1
[00:05:20]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:05:36] error[E0609]: no field `no_deps` on type `lock_api::mutex::MutexGuard<'_, parking_lot::raw_mutex::RawMutex, dep_graph::graph::TaskDeps>`
[00:05:36]     --> src/librustc/dep_graph/graph.rs:1128:56
[00:05:36]      |
[00:05:36] 1128 |                 if cfg!(debug_assertions) && task_deps.no_deps {
[00:05:36] 
[00:05:50] error: aborting due to previous error
[00:05:50] 
[00:05:50] For more information about this error, try `rustc --explain E0609`.
---
travis_time:end:03b40d08:start=1553829507610929519,finish=1553829507616913933,duration=5984414
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18b50759
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bf191cb
travis_time:start:0bf191cb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04ceaa5b
$ dmesg | grep -i kill
