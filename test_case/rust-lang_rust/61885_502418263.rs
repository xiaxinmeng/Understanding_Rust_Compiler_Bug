plain
travis_time:end:1e68762b:start=1560657785906660410,finish=1560657875056541206,duration=89149880796
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:03]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:04] error[E0432]: unresolved import `crate::intrinsics::unchecked_sub`
[00:04:04]   --> src/libcore/slice/mod.rs:28:44
[00:04:04]    |
[00:04:04] 28 | use crate::intrinsics::{assume, exact_div, unchecked_sub};
[00:04:04]    |                                            |
[00:04:04]    |                                            |
[00:04:04]    |                                            no `unchecked_sub` in `intrinsics`
[00:04:04]    |                                            help: a similar name exists in the module: `unchecked_shl`
[00:04:05]    Compiling backtrace v0.3.29
[00:04:08]    Compiling compiler_builtins v0.1.16
[00:04:08]    Compiling backtrace-sys v0.1.27
[00:04:08]    Compiling cmake v0.1.38
---
travis_time:end:168cd4d0:start=1560658145141400416,finish=1560658145146148839,duration=4748423
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12d0c46e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f38059e
travis_time:start:1f38059e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
[0
