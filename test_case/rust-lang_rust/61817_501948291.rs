plain
travis_time:end:15b6a49c:start=1560474201958782050,finish=1560474204539132835,duration=2580350785
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:37]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:39]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:39]     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:06:31]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:36] error[E0496]: lifetime name `'tcx` shadows a lifetime name that is already in scope
[00:06:36]   --> src/librustc/ty/query/job.rs:78:27
[00:06:36]    |
[00:06:36] 65 | impl<'tcx> QueryJob<'tcx> {
[00:06:36]    |      ---- first declared here
[00:06:36] ...
[00:06:36] 78 |     pub(super) fn r#await<'tcx>(
[00:06:36]    |                           ^^^^ lifetime 'tcx already in scope
[00:06:37] error: aborting due to previous error
[00:06:37] 
[00:06:37] For more information about this error, try `rustc --explain E0496`.
[00:06:37] error: Could not compile `rustc`.
---
travis_time:end:1756ed13:start=1560474614970359914,finish=1560474614975954303,duration=5594389
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0650cb08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:040f3eae
travis_time:start:040f3eae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02aa63c0
$ dmesg | grep -i kill
