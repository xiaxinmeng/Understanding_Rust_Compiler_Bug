plain
travis_time:end:11337081:start=1558188948855316174,finish=1558189040715591761,duration=91860275587
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:28:02]    Compiling cc v1.0.35
[00:28:02]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:28:02]    Compiling libc v0.2.54
[00:28:02]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:28:03] error: couldn't read src/libcore/../stdsimd/crates/core_arch/src/../stdsimd/crates/core_arch/src/core_arch_docs.md: No such file or directory (os error 2)
[00:28:03]    |
[00:28:03]    |
[00:28:03] 13 |     doc(include = "../stdsimd/crates/core_arch/src/core_arch_docs.md")
[00:28:03]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ couldn't read file
[00:28:03]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:28:08]    Compiling compiler_builtins v0.1.14
[00:28:08]    Compiling cmake v0.1.38
[00:28:08]    Compiling backtrace-sys v0.1.27
---
travis_time:end:126666c3:start=1558190757854960618,finish=1558190757860294430,duration=5333812
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:184fe4a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d97eae4
travis_time:start:0d97eae4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c7bc017
$ dmesg | grep -i kill
