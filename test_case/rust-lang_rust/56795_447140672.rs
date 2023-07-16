plain
travis_time:end:038fceaa:start=1544738046643114821,finish=1544738049604723556,duration=2961608735
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:20:14]    Compiling rustc-demangle v0.1.9
[00:20:17]    Compiling memmap v0.6.2
[00:20:17]    Compiling num_cpus v1.8.0
[00:20:22]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:20:26] error[E0463]: can't find crate for `serde_derive` which `rustc` depends on
[00:20:26]    |
[00:20:26] 45 | #[macro_use] extern crate rustc;
[00:20:26]    |              ^^^^^^^^^^^^^^^^^^^ can't find crate
[00:20:26] 
---
[00:20:26] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:26] Build completed unsuccessfully in 0:16:54
[00:20:26] make: *** [all] Error 1
[00:20:26] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21c35976
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 13 22:14:45 UTC 2018
---
travis_time:end:067d019c:start=1544739286205368245,finish=1544739286211151194,duration=5782949
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24315e88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1831b318
travis_time:start:1831b318
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07a629da
$ dmesg | grep -i kill
