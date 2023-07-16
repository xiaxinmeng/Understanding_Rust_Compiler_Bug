plain
travis_time:end:03752b1b:start=1543909813110345578,finish=1543909868985148801,duration=55874803223
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
184272 ./obj/build/cache/2018-10-30
153272 ./src/tools/clang
150332 ./obj/build/bootstrap/debug/incremental
134740 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134736 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f79qgjj7ni-1l6t9lv-4qyniat88vxc
134060 ./.git/modules/src
115340 ./src/llvm/test/CodeGen
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
---
travis_time:end:25f9ff50:start=1543910111560666043,finish=1543910111565902426,duration=5236383
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1eddb1e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:044559c0
travis_time:start:044559c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12bf5050
$ dmesg | grep -i kill
