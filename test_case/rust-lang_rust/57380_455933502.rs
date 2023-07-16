plain
travis_time:end:1df7e3fd:start=1548032876937771204,finish=1548032951321227676,duration=74383456472
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:28] configure: build.locked-deps    := True
[00:06:28] configure: llvm.ccache          := sccache
[00:06:28] configure: build.cargo-native-static := True
[00:06:28] configure: dist.missing-tools   := True
[00:06:28] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:28] configure: writing `config.toml` in current directory
[00:06:28] configure: 
[00:06:28] configure: run `python /checkout/x.py --help`
[00:06:28] configure: 
---
[00:08:22] For more information about this error, try `rustc --explain E0308`.
[00:08:22] error: Could not compile `std`.
[00:08:22] 
[00:08:22] To learn more, run the command again with --verbose.
[00:08:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:22] Build completed unsuccessfully in 0:00:39
travis_time:end:22f12edc:start=1548032961733702640,finish=1548033464844018427,duration=503110315787
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:003e05dc:start=1548033465663676873,finish=1548033465671055043,duration=7378170
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:322070ce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08f93f9a
travis_time:start:08f93f9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:059e3818
$ dmesg | grep -i kill
