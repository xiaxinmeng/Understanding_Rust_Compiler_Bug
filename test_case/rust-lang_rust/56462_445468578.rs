plain
travis_time:end:00c700f5:start=1544278669563959679,finish=1544278723777822350,duration=54213862671
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:32] configure: build.locked-deps    := True
[00:07:32] configure: llvm.ccache          := sccache
[00:07:32] configure: build.cargo-native-static := True
[00:07:32] configure: dist.missing-tools   := True
[00:07:32] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:32] configure: writing `config.toml` in current directory
[00:07:32] configure: 
[00:07:32] configure: run `python /checkout/x.py --help`
[00:07:32] configure: 
---
[00:12:39]     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:12:39]     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:12:44]     Checking rustc-main v0.0.0 (/checkout/src/rustc)
[00:12:45]     Finished release [optimized] target(s) in 3m 21s
[00:12:45] thread 'main' panicked at 'failed to copy `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-9f012a3c4a8560db.so` to `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_macros-9f012a3c4a8560db.so`: No such file or directory (os error 2)', src/bootstrap/lib.rs:1180:17
[00:12:45] travis_fold:end:stage0-rustc

[00:12:45] travis_time:end:stage0-rustc:start=1544279296114616927,finish=1544279497326681879,duration=201212064952

---
travis_time:end:09b0810d:start=1544279498047769029,finish=1544279498053695603,duration=5926574
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bee4ee4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0914b014
travis_time:start:0914b014
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a96e650
$ dmesg | grep -i kill
