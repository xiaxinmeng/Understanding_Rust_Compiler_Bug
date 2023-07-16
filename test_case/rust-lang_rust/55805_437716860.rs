plain
travis_time:end:05e1a1fe:start=1541973404510982924,finish=1541973461716679789,duration=57205696865
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:08:06] configure: build.locked-deps    := True
[00:08:06] configure: llvm.ccache          := sccache
[00:08:06] configure: build.cargo-native-static := True
[00:08:06] configure: dist.missing-tools   := True
[00:08:06] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:08:06] configure: writing `config.toml` in current directory
[00:08:06] configure: 
[00:08:06] configure: run `python /checkout/x.py --help`
[00:08:06] configure: 
---
[00:10:34] 
[00:10:34] error: Could not compile `syntax`.
[00:10:34] 
[00:10:34] To learn more, run the command again with --verbose.
[00:10:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:10:34] expected success, got: exit code: 101
[00:10:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:10:34] travis_fold:end:stage0-rustc

[00:10:34] travis_time:end:stage0-rustc:start=1541974067580354422,finish=1541974106543297158,duration=38962942736

---
travis_time:end:2d625494:start=1541974107277136389,finish=1541974107282637430,duration=5501041
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1df19910
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ebe3610
travis_time:start:0ebe3610
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cc5a6f3
$ dmesg | grep -i kill
