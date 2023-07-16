plain
[00:25:49]    Compiling polonius-engine v0.6.2
[00:25:49] [RUSTC-TIMING] smallvec test:false 0.835
[00:25:50]    Compiling rls-span v0.4.0
[00:25:50] [RUSTC-TIMING] polonius_engine test:false 1.241
[00:25:50] error[E0658]: use of unstable library feature 'stdsimd' (see issue #27731)
[00:25:50]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.1.2/src/specialized/pclmulqdq.rs:13:12
[00:25:50]    |
[00:25:50] 13 |         if is_x86_feature_detected!("pclmulqdq")
[00:25:50]    |
[00:25:50]    |
[00:25:50]    = help: add #![feature(stdsimd)] to the crate attributes to enable
[00:25:50] 
[00:25:51] error: aborting due to previous error
[00:25:51] 
[00:25:51] For more information about this error, try `rustc --explain E0658`.
[00:25:51] For more information about this error, try `rustc --explain E0658`.
[00:25:51] [RUSTC-TIMING] crc32fast test:false 0.257
[00:25:51] error: Could not compile `crc32fast`.
[00:25:51] [RUSTC-TIMING] libc test:false 1.939
[00:25:51] [RUSTC-TIMING] crossbeam_epoch test:false 2.181
[00:25:51] [RUSTC-TIMING] rls_span test:false 1.030
[00:25:51] error: build failed
[00:25:51] error: build failed
[00:25:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:51] expected success, got: exit code: 101
[00:25:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:51] Build completed unsuccessfully in 0:22:08
[00:25:51] make: *** [all] Error 1
[00:25:51] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:008848c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 17:35:54 UTC 2019
---
travis_time:end:067ef0c0:start=1550165755403521669,finish=1550165755415902033,duration=12380364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aafa360
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30efbd4c
travis_time:start:30efbd4c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03f6abf5
$ dmesg | grep -i kill
