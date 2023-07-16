plain
travis_time:end:057be65f:start=1552942958137602078,finish=1552942960617471838,duration=2479869760
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:32:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:32:26] error: incorrect close delimiter: `}`
[00:32:26]    --> src/librustc/ty/relate.rs:637:13
[00:32:26]     |
[00:32:26] 630 |             ConstValue::Unevaluated(b_def_id, b_substs)) if a_def_id == b_def_id => {
[00:32:26] ...
[00:32:26] ...
[00:32:26] 633 |                 Ok(tcx.mk_const(ty::Const {
[00:32:26]     |                   - un-closed delimiter
[00:32:26] 637 |             }
[00:32:26]     |             ^ incorrect close delimiter
[00:32:26] 
[00:33:09] error: aborting due to previous error
---
travis_time:end:08468a57:start=1552944961548020918,finish=1552944961552755370,duration=4734452
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14182040
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09296e1d
travis_time:start:09296e1d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1799bc1f
$ dmesg | grep -i kill
