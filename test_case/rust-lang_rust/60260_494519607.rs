plain
travis_time:end:1aeb1740:start=1558459100467927468,finish=1558459258745564063,duration=158277636595
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
travis_time:end:064f4bc8:start=1558459815039189654,finish=1558459815044752829,duration=5563175
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1145b796
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bec206c
travis_time:start:0bec206c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:011aac8f
$ dmesg | grep -i kill
