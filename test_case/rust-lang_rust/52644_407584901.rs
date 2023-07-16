plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:01]    Compiling libc v0.2.42
[00:57:02]    Compiling rand v0.4.2
[00:57:04]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:57:09] error[E0636]: the feature `test` has already been declared
[00:57:09]     |
[00:57:09]     |
[00:57:09] 122 | #![cfg_attr(test, feature(test))]
[00:57:09] 
[00:57:09] error[E0635]: unknown feature `rand`
[00:57:09]   --> liballoc/lib.rs:80:27
[00:57:09]    |
[00:57:09]    |
[00:57:09] 80 | #![cfg_attr(test, feature(rand, test))]
[00:57:09] 
[00:57:09] error: aborting due to 2 previous errors
[00:57:09] 
[00:57:09] Some errors occurred: E0635, E0636.
---
[00:57:13] 
[00:57:13] To learn more, run the command again with --verbose.
[00:57:13] 
[00:57:13] 
[00:57:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[00:57:13] 
[00:57:13] 
[00:57:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:13] Build completed unsuccessfully in 0:15:18
[00:57:13] Build completed unsuccessfully in 0:15:18
[00:57:13] make: *** [check] Error 1
[00:57:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:020a84c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e1a887c:start=1532475118639533964,finish=1532475118646307583,duration=6773619
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2bd8e708
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13616780
travis_time:start:13616780
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06446fd0
$ dmesg | grep -i kill
