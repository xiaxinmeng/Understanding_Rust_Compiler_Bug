plain
travis_time:end:020e107c:start=1550134608713528647,finish=1550134609681685895,duration=968157248
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:14]    Compiling cc v1.0.28
[00:06:14]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:06:14]    Compiling libc v0.2.46
[00:06:14]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:06:15] error[E0658]: #[doc(include = "...")] is experimental (see issue #44732)
[00:06:15]    |
[00:06:15]    |
[00:06:15] 10 |     doc(include = "../stdsimd/crates/core_arch/src/core_arch_docs.md")
[00:06:15]    |
[00:06:15]    = help: add #![feature(external_doc)] to the crate attributes to enable
[00:06:15] 
[00:06:15]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
---
[00:06:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:33] expected success, got: exit code: 101
[00:06:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:33] Build completed unsuccessfully in 0:00:21
[00:06:33] make: *** [all] Error 1
[00:06:33] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b6e98ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 09:03:35 UTC 2019
---
travis_time:end:1c9ef4c2:start=1550135016041353710,finish=1550135016046187806,duration=4834096
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06cb6ea7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04fbdb78
travis_time:start:04fbdb78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0659b392
$ dmesg | grep -i kill
