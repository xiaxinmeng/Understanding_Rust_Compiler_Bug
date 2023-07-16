plain
travis_time:end:17c68bd5:start=1545860357357052204,finish=1545860358412652664,duration=1055600460
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:08]    Compiling cc v1.0.25
[00:03:08]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:08]    Compiling libc v0.2.45
[00:03:08]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:08] warning: unknown lint: `deprecated_in_future`
[00:03:08]    |
[00:03:08] 63 | #![warn(deprecated_in_future)]
[00:03:08]    |         ^^^^^^^^^^^^^^^^^^^^
[00:03:08]    |
---
[00:03:40]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:03:42]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:42]    Compiling rustc-demangle v0.1.10
[00:03:42]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:42] warning: unknown lint: `deprecated_in_future`
[00:03:42]    |
[00:03:42] 66 | #![warn(deprecated_in_future)]
[00:03:42]    |         ^^^^^^^^^^^^^^^^^^^^
[00:03:42]    |
---
[00:23:00]    Compiling compiler_builtins v0.1.2
[00:23:00]    Compiling cmake v0.1.33
[00:23:00]    Compiling backtrace-sys v0.1.27
[00:23:02]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:23:03] error: use of item 'mem::zeroed' that will be deprecated in future version 2.0.0: use `mem::MaybeUninit::zeroed` instead
[00:23:03]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:17:24
[00:23:03]    |
[00:23:03] 17 |     let zero: i32x16 = mem::zeroed();
[00:23:03]    |
[00:23:03]    = note: `-D deprecated-in-future` implied by `-D warnings`
[00:23:03] 
[00:23:03] 
[00:23:03] error: use of item 'mem::zeroed' that will be deprecated in future version 2.0.0: use `mem::MaybeUninit::zeroed` instead
[00:23:03]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:57:5
[00:23:03] 57 |     mem::zeroed()
[00:23:03]    |     ^^^^^^^^^^^
[00:23:03] 
[00:23:03]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
[00:23:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:17] expected success, got: exit code: 101
[00:23:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:17] Build completed unsuccessfully in 0:20:11
[00:23:17] Makefile:18: recipe for target 'all' failed
[00:23:17] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:071a91d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 26 22:02:44 UTC 2018
---
travis_time:end:034eb440:start=1545861764505307330,finish=1545861764511783214,duration=6475884
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09499598
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:192467bf
travis_time:start:192467bf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:012fde9e
$ dmesg | grep -i kill
