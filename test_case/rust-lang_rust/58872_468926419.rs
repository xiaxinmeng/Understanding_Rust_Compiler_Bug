plain
travis_time:end:070aa3f4:start=1551537281086150125,finish=1551537365809847584,duration=84723697459
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:35]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:48] error: incorrect close delimiter: `}`
[00:05:48]     --> src/libsyntax/parse/parser.rs:6728:9
[00:05:48]      |
[00:05:48] 6722 |         let ty_first = self.parse_ty().map_err(|mut err|
[00:05:48]      |                                               - un-closed delimiter
[00:05:48] 6728 |         })?;
[00:05:48]      |         ^ incorrect close delimiter
[00:05:48] 
[00:05:48] error: incorrect close delimiter: `)`
---
travis_time:end:10d63392:start=1551537724633672625,finish=1551537724638779306,duration=5106681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25326619
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03c81ce0
travis_time:start:03c81ce0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:35d5f34c
$ dmesg | grep -i kill
