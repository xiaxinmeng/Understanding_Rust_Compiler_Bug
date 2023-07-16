plain
[00:04:41]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:04:41]    Compiling cmake v0.1.33
[00:04:41]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:04:44]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:04:45] error[E0606]: casting `usize` as `*const T` is invalid
[00:04:45]   --> libcore/ptr.rs:77:46
[00:04:45]    |
[00:04:45] 77 | pub const fn null<T: ?Sized>() -> *const T { 0 as *const T }
[00:04:45] 
[00:04:45] 
[00:04:45] error[E0606]: casting `usize` as `*mut T` is invalid
[00:04:45]   --> libcore/ptr.rs:91:48
[00:04:45]    |
[00:04:45] 91 | pub const fn null_mut<T: ?Sized>() -> *mut T { 0 as *mut T }
[00:04:45] 
[00:04:46]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:47]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:47]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
---
[00:04:50] warning: build failed, waiting for other jobs to finish...
[00:04:58] error: build failed
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:58] expected success, got: exit code: 101
[00:04:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:04:58] travis_fold:end:stage0-std

[00:04:58] travis_time:end:stage0-std:start=1536090588188875505,finish=1536090611497103197,duration=23308227692


[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:58] Build completed unsuccessfully in 0:00:24
[00:04:58] Makefile:28: recipe for target 'all' failed
[00:04:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dd13f90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0828d65e:start=1536090612271972762,finish=1536090612279805142,duration=7832380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16123a10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:029773f2
travis_time:start:029773f2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:133f3f50
$ dmesg | grep -i kill
