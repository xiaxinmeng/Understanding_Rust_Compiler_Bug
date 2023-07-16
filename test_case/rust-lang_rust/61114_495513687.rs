plain
travis_time:end:001d36cb:start=1558684546408943655,finish=1558684547220886444,duration=811942789
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:33]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:07:33] error[E0308]: mismatched types
[00:07:33]   --> src/libarena/lib.rs:91:9
[00:07:33]    |
[00:07:33] 90 |     fn start(&self) -> *mut T {
[00:07:33]    |                        ------ expected `*mut T` because of return type
[00:07:33]    |         ^^^^^^^^^^^^^^^^^^ types differ in mutability
[00:07:33]    |
[00:07:33]    = note: expected type `*mut T`
[00:07:33]               found type `*const T`
---
travis_time:end:0301103a:start=1558685030399082758,finish=1558685030403830623,duration=4747865
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:187e9707
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30c7d896
[
