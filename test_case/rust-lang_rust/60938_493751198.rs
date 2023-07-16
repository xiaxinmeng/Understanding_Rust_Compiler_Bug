plain
travis_time:end:13cadef4:start=1558265730037430317,finish=1558265830146578563,duration=100109148246
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:40]    Compiling cc v1.0.35
[00:26:40]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:40]    Compiling libc v0.2.54
[00:26:40]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:40] error: couldn't read src/libcore/../stdsimd/crates/core_arch/src/../stdsimd/crates/core_arch/src/core_arch_docs.md: No such file or directory (os error 2)
[00:26:40]    |
[00:26:40]    |
[00:26:40] 13 |     doc(include = "../stdsimd/crates/core_arch/src/core_arch_docs.md")
[00:26:40]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ couldn't read file
[00:26:41]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:26:46]    Compiling compiler_builtins v0.1.14
[00:26:46]    Compiling cmake v0.1.38
[00:26:46]    Compiling backtrace-sys v0.1.27
---
travis_time:end:170fc974:start=1558267462260514010,finish=1558267462265949059,duration=5435049
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f75f21e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e4892fe
travis_time:start:0e4892fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08b6950e
$ dmesg | grep -i kill
