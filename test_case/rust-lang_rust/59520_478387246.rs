plain
travis_time:end:2294b5d0:start=1554066478057838555,finish=1554066479041370778,duration=983532223
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:25:16]    Compiling rustc-demangle v0.1.10
[00:25:22]    Compiling memmap v0.6.2
[00:25:22]    Compiling num_cpus v1.8.0
[00:25:24]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:25:28] error[E0425]: cannot find value `niche` in this scope
[00:25:28]      |
[00:25:28]      |
[00:25:28] 1389 |                             Some(truncate(value, niche.value.size(cx)))
[00:25:28] 
[00:25:31] error: aborting due to previous error
[00:25:31] 
[00:25:31] For more information about this error, try `rustc --explain E0425`.
---
travis_time:end:11e5cbff:start=1554068021542969296,finish=1554068021547713097,duration=4743801
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06c0b63d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0166a57a
travis_time:start:0166a57a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00778f02
$ dmesg | grep -i kill
