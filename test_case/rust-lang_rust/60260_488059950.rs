plain
travis_time:end:1cf60c26:start=1556641649167640406,finish=1556641736659844503,duration=87492204097
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:34] configure: build.locked-deps    := True
[00:07:34] configure: llvm.ccache          := sccache
[00:07:34] configure: build.cargo-native-static := True
[00:07:34] configure: dist.missing-tools   := True
[00:07:34] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:34] configure: writing `config.toml` in current directory
[00:07:34] configure: 
[00:07:34] configure: run `python /checkout/x.py --help`
[00:07:34] configure: 
---
travis_time:end:00506de5:start=1556642319166596412,finish=1556642319172781229,duration=6184817
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c786fbf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20a5c120
travis_time:start:20a5c120
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b20c299
$ dmesg | grep -i kill
