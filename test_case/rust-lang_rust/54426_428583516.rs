plain
[00:07:49]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:09]    Compiling rustc_metadata_utils v0.0.0 (/checkout/src/librustc_metadata_utils)
[00:13:09]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:09]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:09] error: unexpected close delimiter: `}`
[00:13:09]    --> librustc_mir/dataflow/impls/borrows.rs:413:1
[00:13:09] 413 | }
[00:13:09] 413 | }
[00:13:09]     | ^ unexpected close delimiter
[00:13:09] error: aborting due to previous error
[00:13:09] 
[00:13:09] error: Could not compile `rustc_mir`.
[00:13:09] warning: build failed, waiting for other jobs to finish...
[00:13:09] warning: build failed, waiting for other jobs to finish...
[00:14:18] error: build failed
[00:14:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:18] expected success, got: exit code: 101
[00:14:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:14:18] travis_fold:end:stage0-rustc

[00:14:18] travis_time:end:stage0-rustc:start=1539179552753576700,finish=1539180072046235467,duration=519292658767


[00:14:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:18] Build completed unsuccessfully in 0:09:34
[00:14:18] make: *** [all] Error 1
[00:14:18] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:067f7be3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:002639cd:start=1539180072749896045,finish=1539180072756516228,duration=6620183
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22e4f8aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0756e8a5
travis_time:start:0756e8a5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0511eab0
$ dmesg | grep -i kill
