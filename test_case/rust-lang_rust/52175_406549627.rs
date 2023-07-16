plain
[00:46:57] ....................................................................................................
[00:47:10] .......................................................................i............................
[00:47:20] ....................................................................................................
[00:47:31] .............................................................................................i......
[00:47:40] ........................F...........................................................................
[00:48:04] ....................................................................................................
[00:48:14] ..........................................................................i.........................
[00:48:24] .........................................................................i..........................
[00:48:41] ....................................................................................................
---
[00:50:22] ---- [run-pass] run-pass/issue-51848.rs stdout ----
[00:50:22] 
[00:50:22] error: compilation failed!
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-51848.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-51848/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-51848/auxiliary"
[00:50:22] ------------------------------------------
[00:50:22] 
[00:50:22] ------------------------------------------
[00:50:22] stderr:
[00:50:22] stderr:
[00:50:22] ------------------------------------------
[00:50:22] error: invalid format string: expected `'}'` but string was terminated
[00:50:22]    |
[00:50:22]    |
[00:50:22] 21 |     println!("{"); //~ ERROR invalid
[00:50:22]    |     ^^^^^^^^^^^^^^ expected `'}'` in format string
[00:50:22]    |
[00:50:22]    = note: if you intended to print `{`, you can escape it using `{{`
[00:50:22] 
[00:50:22] error: aborting due to previous error
[00:50:22] 
[00:50:22] 
---
[00:50:22] 
[00:50:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:22] 
[00:50:22] 
[00:50:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu/release
121716 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107612 ./src/llvm/test/CodeGen
107460 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
107456 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
---
travis_time:end:04077bd7:start=1532080095549842568,finish=1532080095629394498,duration=79551930
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fbdfc20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09bc6bce
$ dmesg | grep -i kill
