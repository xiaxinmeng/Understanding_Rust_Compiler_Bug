plain
travis_time:end:054da1d2:start=1545297245717990325,finish=1545297303072910640,duration=57354920315
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:05:51] configure: build.locked-deps    := True
[00:05:51] configure: llvm.ccache          := sccache
[00:05:51] configure: build.cargo-native-static := True
[00:05:51] configure: dist.missing-tools   := True
[00:05:51] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:05:51] configure: writing `config.toml` in current directory
[00:05:51] configure: 
[00:05:51] configure: run `python /checkout/x.py --help`
[00:05:51] configure: 
---
[00:08:15]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:18] error: unused import: `mem`
[00:08:18]   --> src/libsyntax/tokenstream.rs:35:22
[00:08:18]    |
[00:08:18] 35 | use std::{fmt, iter, mem};
[00:08:18]    |
[00:08:18]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:18] 
[00:08:23] error: aborting due to previous error
[00:08:23] error: aborting due to previous error
[00:08:23] 
[00:08:23] error: Could not compile `syntax`.
[00:08:23] 
[00:08:23] To learn more, run the command again with --verbose.
[00:08:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:23] Build completed unsuccessfully in 0:01:21
travis_time:end:0015db30:start=1545297311720268660,finish=1545297815425529278,duration=503705260618
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0bb6472a:start=1545297815908039571,finish=1545297815914791154,duration=6751583
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:111da833
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a7ae108
travis_time:start:1a7ae108
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:062b8bd3
$ dmesg | grep -i kill
