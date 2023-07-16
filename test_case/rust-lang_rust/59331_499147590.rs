plain
travis_time:end:1d86d3d8:start=1559749900239036286,finish=1559749902583712557,duration=2344676271
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:01] error[E0433]: failed to resolve: use of undeclared type or module `TypeVariableOrigin`
[00:09:01]    --> src/librustc/infer/error_reporting/need_type_info.rs:125:24
[00:09:01]     |
[00:09:01] 125 |                 if let TypeVariableOrigin::TypeParameterDefinition(_, name) =
[00:09:01]     |                        ^^^^^^^^^^^^^^^^^^ use of undeclared type or module `TypeVariableOrigin`
[00:09:31] error: aborting due to previous error
[00:09:31] 
[00:09:31] For more information about this error, try `rustc --explain E0433`.
[00:09:31] error: Could not compile `rustc`.
---
travis_time:end:16fce8ca:start=1559750486664363530,finish=1559750486668667225,duration=4303695
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24459b44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:001b2fac
travis_time:start:001b2fac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0415a42e
$ dmesg | grep -i kill
