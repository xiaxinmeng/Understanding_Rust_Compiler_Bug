plain
travis_time:end:044c3220:start=1544049912417853147,finish=1544049995460392015,duration=83042538868
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:33]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:33]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:49]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:03:49]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:49] error: expected one of `=>`, `if`, or `|`, found `{`
[00:03:49]      |
[00:03:49] 1754 |             Some(cow) {
[00:03:49] 1754 |             Some(cow) {
[00:03:49]      |                       ^ expected one of `=>`, `if`, or `|` here
[00:03:50]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2958716 .
1236624 ./obj
1236584 ./obj/build
---
184276 ./obj/build/cache/2018-10-30
153276 ./src/tools/clang
150340 ./obj/build/bootstrap/debug/incremental
134748 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134744 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f7bitqd6t3-asai72-jerzeai9r9z3
134544 ./.git/modules/src
115344 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
---
travis_time:end:3c5356b0:start=1544050236676700128,finish=1544050236682471681,duration=5771553
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0705b5d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f8958f8
travis_time:start:2f8958f8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:st
