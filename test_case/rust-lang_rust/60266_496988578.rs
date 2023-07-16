plain
travis_time:end:0ff26400:start=1559142905552742939,finish=1559142908307082584,duration=2754339645
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:57]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:01] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:15:01]   --> src/librustc_mir/borrow_check/flows.rs:24:75
[00:15:01]    |
[00:15:01] 24 | crate type PoloniusOutput = Output<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:15:01] 
[00:15:01] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:15:01]   --> src/librustc_mir/borrow_check/nll/facts.rs:14:79
[00:15:01]    |
[00:15:01]    |
[00:15:01] 14 | crate type AllFacts = PoloniusAllFacts<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:15:01] 
[00:15:01] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:15:01]   --> src/librustc_mir/borrow_check/nll/mod.rs:86:61
[00:15:01]    |
[00:15:01]    |
[00:15:01] 86 |     Option<Rc<Output<RegionVid, BorrowIndex, LocationIndex, Local>>>,
[00:15:01] 
[00:15:01] error: aborting due to 3 previous errors
[00:15:01] 
[00:15:01] For more information about this error, try `rustc --explain E0107`.
---
travis_time:end:0526f76c:start=1559143986754407863,finish=1559143986758989680,duration=4581817
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:286c066b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009ded4c
travis_time:start:009ded4c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b633703
$ dmesg | grep -i kill
