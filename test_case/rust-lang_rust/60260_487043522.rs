plain
travis_time:end:27ee211c:start=1556276263052822885,finish=1556276355746389353,duration=92693566468
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:03] configure: build.locked-deps    := True
[00:07:03] configure: llvm.ccache          := sccache
[00:07:03] configure: build.cargo-native-static := True
[00:07:03] configure: dist.missing-tools   := True
[00:07:03] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:03] configure: writing `config.toml` in current directory
[00:07:03] configure: 
[00:07:03] configure: run `python /checkout/x.py --help`
[00:07:03] configure: 
---
travis_time:end:33683f54:start=1556276898876733852,finish=1556276898881398951,duration=4665099
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:015aa880
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:024c0612
travis_time:start:024c0612
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:015f999c
$ dmesg | grep -i kill
