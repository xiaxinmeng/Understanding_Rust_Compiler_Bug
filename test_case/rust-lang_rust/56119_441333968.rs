plain
travis_time:end:04e39171:start=1543014800354910897,finish=1543014866121579110,duration=65766668213
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:14] configure: build.locked-deps    := True
[00:07:14] configure: llvm.ccache          := sccache
[00:07:14] configure: build.cargo-native-static := True
[00:07:14] configure: dist.missing-tools   := True
[00:07:14] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:14] configure: writing `config.toml` in current directory
[00:07:14] configure: 
[00:07:14] configure: run `python /checkout/x.py --help`
[00:07:14] configure: 
---
[00:08:48]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:49] error[E0425]: cannot find value `x` in this scope
[00:08:49]   --> src/libstd/sys/windows/path.rs:94:57
[00:08:49]    |
[00:08:49] 94 |         let first = &path.iter().position(|x| f(*x))?[..x];
[00:08:49]    |                                                         ^ did you mean `f`?
[00:08:52] error[E0608]: cannot index into a value of type `usize`
[00:08:52]   --> src/libstd/sys/windows/path.rs:94:22
[00:08:52]    |
[00:08:52]    |
[00:08:52] 94 |         let first = &path.iter().position(|x| f(*x))?[..x];
[00:08:52] 
[00:08:52] error: aborting due to 2 previous errors
[00:08:52] 
[00:08:52] Some errors occurred: E0425, E0608.
[00:08:52] Some errors occurred: E0425, E0608.
[00:08:52] For more information about an error, try `rustc --explain E0425`.
[00:08:52] error: Could not compile `std`.
[00:08:52] 
[00:08:52] To learn more, run the command again with --verbose.
[00:08:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:52] Build completed unsuccessfully in 0:00:31
travis_time:end:09932a78:start=1543014874373203760,finish=1543015407467304310,duration=533094100550
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:05f46f30:start=1543015407994107871,finish=1543015407998622841,duration=4514970
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b3a078
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:012f0b01
travis_time:start:012f0b01
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ab66422
$ dmesg | grep -i kill
