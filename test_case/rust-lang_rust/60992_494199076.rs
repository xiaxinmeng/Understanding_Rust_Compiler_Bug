plain
[00:03:52] --- stdout
[00:03:52] TARGET = Some("x86_64-unknown-linux-gnu")
[00:03:52] OPT_LEVEL = Some("2")
[00:03:52] HOST = Some("x86_64-unknown-linux-gnu")
[00:03:52] CC_x86_64-unknown-linux-gnu = Some("sccache cc")
[00:03:52] CFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
[00:03:52] CRATE_CC_NO_DEFAULTS = None
[00:03:52] DEBUG = Some("false")
[00:03:52] CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
[00:03:52] running: "sccache" "cc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-98e28db5f13c3bec/out/GCDAProfiling.o" "-c" "/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.15/compiler-rt/lib/profile/GCDAProfiling.c"
[00:03:52] cargo:warning=cc: error: /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.15/compiler-rt/lib/profile/GCDAProfiling.c: No such file or directory
[00:03:52] cargo:warning=cc: warning: '-x c' after last input file has no effect
[00:03:52] cargo:warning=cc: fatal error: no input files
[00:03:52] cargo:warning=compilation terminated.
[00:03:52] 
[00:03:52] --- stderr
[00:03:52] thread 'main' panicked at '
[00:03:52] 
[00:03:52] 
[00:03:52] Internal error occurred: Command "sccache" "cc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-98e28db5f13c3bec/out/GCDAProfiling.o" "-c" "/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.15/compiler-rt/lib/profile/GCDAProfiling.c" with args "cc" did not execute successfully (status code exit code: 1).
[00:03:52] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
[00:03:52] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:03:52] 
[00:03:52] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:1e2e92de:start=1558399699876263235,finish=1558399699890322238,duration=14059003
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a4ee891
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a0a8324
travis_time:start:0a0a8324
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00bbd071
$ dmesg | grep -i kill
