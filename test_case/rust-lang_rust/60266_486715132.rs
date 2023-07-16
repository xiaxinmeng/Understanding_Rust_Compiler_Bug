plain
travis_time:end:032b667f:start=1556203980226844455,finish=1556204068967232341,duration=88740387886
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:13:33]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:13:37] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:13:37]   --> src/librustc_mir/borrow_check/flows.rs:24:75
[00:13:37]    |
[00:13:37] 24 | crate type PoloniusOutput = Output<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:13:37] 
[00:13:37] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:13:37]   --> src/librustc_mir/borrow_check/nll/facts.rs:14:79
[00:13:37]    |
[00:13:37]    |
[00:13:37] 14 | crate type AllFacts = PoloniusAllFacts<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:13:37] 
[00:13:37] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:13:37]   --> src/librustc_mir/borrow_check/nll/mod.rs:85:61
[00:13:37]    |
[00:13:37]    |
[00:13:37] 85 |     Option<Rc<Output<RegionVid, BorrowIndex, LocationIndex, Local>>>,
[00:13:37] 
[00:13:37] error: aborting due to 3 previous errors
[00:13:37] 
[00:13:37] For more information about this error, try `rustc --explain E0107`.
---
travis_time:end:0f0a9c68:start=1556205055484666377,finish=1556205055489057479,duration=4391102
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2fd5141a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d6b867c
travis_time:start:0d6b867c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0106a454
$ dmesg | grep -i kill
