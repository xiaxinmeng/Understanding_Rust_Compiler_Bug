plain
travis_time:end:047c93c4:start=1560627424950581360,finish=1560627514195892917,duration=89245311557
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:37]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:52] error: unused variable: `gcx`
[00:14:52]    --> src/librustc_mir/borrow_check/nll/region_infer/mod.rs:812:13
[00:14:52]     |
[00:14:52] 812 |         let gcx = tcx.global_tcx();
[00:14:52]     |             ^^^ help: consider prefixing with an underscore: `_gcx`
[00:14:52]     = note: `-D unused-variables` implied by `-D warnings`
[00:14:52] 
[00:15:01]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:02] error: aborting due to previous error
---
travis_time:end:2f83a7c6:start=1560628597803657351,finish=1560628597809069415,duration=5412064
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12cc995e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:049b5600
travis_time:start:049b5600
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3241ba93
$ dmesg | grep -i kill
