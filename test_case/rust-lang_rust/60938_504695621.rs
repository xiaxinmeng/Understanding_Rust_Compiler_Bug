plain
travis_time:end:236d0ff0:start=1561233267195445824,finish=1561233269315700261,duration=2120254437
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:22]    Compiling cc v1.0.35
[00:26:22]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:22]    Compiling libc v0.2.54
[00:26:22]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:26:22] error: couldn't read src/libcore/../stdsimd/crates/core_arch/src/../stdsimd/crates/core_arch/src/core_arch_docs.md: No such file or directory (os error 2)
[00:26:22]    |
[00:26:22]    |
[00:26:22] 13 |     doc(include = "../stdsimd/crates/core_arch/src/core_arch_docs.md")
[00:26:22]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ couldn't read file
[00:26:22]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:23]    Compiling autocfg v0.1.4
[00:26:25]    Compiling backtrace v0.3.29
[00:26:28]    Compiling compiler_builtins v0.1.16
---
travis_time:end:013a209c:start=1561234884298851755,finish=1561234884303616034,duration=4764279
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:090dde4a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1391eac5
travis_time:start:1391eac5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06dbc17e
$ dmesg | grep -i kill
