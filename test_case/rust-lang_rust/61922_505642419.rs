plain
travis_time:end:0b6476e3:start=1561498206946919488,finish=1561498210505857504,duration=3558938016
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:38:29]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:39:03] error: outlives requirements can be inferred
[00:39:03]   --> src/librustc_mir/dataflow/impls/storage_liveness.rs:80:38
[00:39:03]    |
[00:39:03] 80 | pub struct RequiresStorage<'mir, 'tcx: 'mir, 'b> {
[00:39:03]    |
[00:39:03] note: lint level defined here
[00:39:03]   --> src/librustc_mir/lib.rs:30:9
[00:39:03]    |
[00:39:03]    |
[00:39:03] 30 | #![deny(rust_2018_idioms)]
[00:39:03]    |         ^^^^^^^^^^^^^^^^
[00:39:03]    = note: #[deny(explicit_outlives_requirements)] implied by #[deny(rust_2018_idioms)]
[00:39:03] error: outlives requirements can be inferred
[00:39:03]    --> src/librustc_mir/dataflow/impls/storage_liveness.rs:178:32
[00:39:03]     |
[00:39:03]     |
[00:39:03] 178 | struct MoveVisitor<'a, 'b, 'mir: 'a, 'tcx> {
[00:39:03] 
[00:39:03] error: aborting due to 2 previous errors
[00:39:03] 
[00:39:04] error: Could not compile `rustc_mir`.
---
travis_time:end:1700ae9a:start=1561500734100287484,finish=1561500734105280759,duration=4993275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03d0adc9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0698e263
travis_time:start:0698e263
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02630400
$ dmesg | grep -i kill
