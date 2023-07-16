plain
travis_time:end:22de1f80:start=1559271248658007330,finish=1559271251098476393,duration=2440469063
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:04]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:05:16] error: method is never used: `cloned`
[00:05:16]    --> src/libcore/ops/range.rs:712:5
[00:05:16]     |
[00:05:16] 712 |     fn cloned(&self) -> Bound<T> {
[00:05:16]     |
[00:05:16]     = note: `-D dead-code` implied by `-D warnings`
[00:05:16] 
[00:05:16] error: aborting due to previous error
---
travis_time:end:039f5175:start=1559271579559208580,finish=1559271579564993409,duration=5784829
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01d7865b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06aa4c30
travis_time:start:06aa4c30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15597f6e
$ dmesg | grep -i kill
