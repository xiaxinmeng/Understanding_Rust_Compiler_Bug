plain
travis_time:end:05b801f8:start=1544224442596761929,finish=1544224512640402127,duration=70043640198
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:26] configure: build.locked-deps    := True
[00:07:26] configure: llvm.ccache          := sccache
[00:07:26] configure: build.cargo-native-static := True
[00:07:26] configure: dist.missing-tools   := True
[00:07:26] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:26] configure: writing `config.toml` in current directory
[00:07:26] configure: 
[00:07:26] configure: run `python /checkout/x.py --help`
[00:07:26] configure: 
---
[00:12:27]     Checking rustc-demangle v0.1.9
[00:12:27]     Checking num_cpus v1.8.0
[00:12:31]     Checking memmap v0.6.2
[00:12:31]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:12:33] error[E0463]: can't find crate for `rustc_macros` which `rustc` depends on
[00:12:33]    |
[00:12:33] 45 | #[macro_use] extern crate rustc;
[00:12:33]    |              ^^^^^^^^^^^^^^^^^^^ can't find crate
[00:12:33] 
[00:12:33] 
[00:12:33] error: aborting due to previous error
[00:12:33] 
[00:12:33] For more information about this error, try `rustc --explain E0463`.
[00:12:33] error: Could not compile `rustc_codegen_llvm`.
[00:12:33] 
[00:12:33] To learn more, run the command again with --verbose.
[00:12:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--message-format" "json"
[00:12:33] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:12:33] Build completed unsuccessfully in 0:03:56
---
travis_time:end:100940bc:start=1544225275217129397,finish=1544225275222627296,duration=5497899
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04205a02
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04627458
travis_time:start:04627458
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:099010d8
$ dmesg | grep -i kill
