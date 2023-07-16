plain
travis_time:end:035ea7e5:start=1561404569756646119,finish=1561404647815978248,duration=78059332129
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:39:50]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:40:24] error: outlives requirements can be inferred
[00:40:24]   --> src/librustc_mir/dataflow/impls/storage_liveness.rs:89:38
[00:40:24]    |
[00:40:24] 89 | pub struct RequiresStorage<'mir, 'tcx: 'mir, 'b> {
[00:40:24]    |
[00:40:24] note: lint level defined here
[00:40:24]   --> src/librustc_mir/lib.rs:30:9
[00:40:24]    |
[00:40:24]    |
[00:40:24] 30 | #![deny(rust_2018_idioms)]
[00:40:24]    |         ^^^^^^^^^^^^^^^^
[00:40:24]    = note: #[deny(explicit_outlives_requirements)] implied by #[deny(rust_2018_idioms)]
[00:40:24] error: outlives requirements can be inferred
[00:40:24]    --> src/librustc_mir/dataflow/impls/storage_liveness.rs:196:36
[00:40:24]     |
[00:40:24]     |
[00:40:24] 196 | struct MoveVisitor<'a, 'b, 'c, 'mir: 'a, 'tcx> {
[00:40:24] 
[00:40:24] error: aborting due to 2 previous errors
[00:40:24] 
[00:40:24] error: Could not compile `rustc_mir`.
---
travis_time:end:02b0fa00:start=1561407273504890228,finish=1561407273509872935,duration=4982707
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d4b0c62
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a978828
travis_time:start:2a978828
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c180491
$ dmesg | grep -i kill
