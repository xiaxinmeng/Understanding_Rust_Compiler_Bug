plain
travis_time:end:030e7b9c:start=1544734309553568350,finish=1544734368222419390,duration=58668851040
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:16:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:16:11] tidy error: /checkout/src/test/rustdoc/issue-19190.rs: missing trailing newline
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
\; -exec echo travis_fold":"end:crashlog \; || true
\; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:056ad326:start=1544735349503162431,finish=1544735349507862520,duration=4700089
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0027d080
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03c8ba88
travis_time:start:03c8ba88
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0076a0e8
$ dmesg | grep -i kill
