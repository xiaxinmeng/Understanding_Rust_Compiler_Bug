plain
2019-07-20T18:01:57.8769268Z ---- [run-pass] run-pass/command-uid-gid.rs stdout ----
2019-07-20T18:01:57.8769385Z 
2019-07-20T18:01:57.8769477Z error: test run failed!
2019-07-20T18:01:57.8769542Z status: exit code: 101
2019-07-20T18:01:57.8770038Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-uid-gid/a"
2019-07-20T18:01:57.8770715Z ------------------------------------------
2019-07-20T18:01:57.8770715Z ------------------------------------------
2019-07-20T18:01:57.8771407Z uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-uid-gid/a", waiting for result
2019-07-20T18:01:57.8771698Z ------------------------------------------
2019-07-20T18:01:57.8771777Z stderr:
2019-07-20T18:01:57.8771989Z ------------------------------------------
2019-07-20T18:01:57.8771989Z ------------------------------------------
2019-07-20T18:01:57.8772356Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:1051:5
2019-07-20T18:01:57.8772523Z 
2019-07-20T18:01:57.8772738Z ------------------------------------------
2019-07-20T18:01:57.8772795Z 
2019-07-20T18:01:57.8772824Z 
---
2019-07-20T18:01:57.8781507Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-20T18:01:57.8781647Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T18:01:57.8791618Z 
2019-07-20T18:01:57.8794232Z 
2019-07-20T18:01:57.8796880Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
2019-07-20T18:01:57.8798059Z 
2019-07-20T18:01:57.8798098Z 
2019-07-20T18:01:57.8809462Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
2019-07-20T18:01:57.8809612Z Build completed unsuccessfully in 1:53:30
2019-07-20T18:01:57.8809612Z Build completed unsuccessfully in 1:53:30
2019-07-20T18:01:58.8190425Z ##[error]Bash exited with code '1'.
2019-07-20T18:01:58.8224068Z ##[section]Starting: Upload CPU usage statistics
2019-07-20T18:01:58.8230387Z ==============================================================================
2019-07-20T18:01:58.8230501Z Task         : Bash
2019-07-20T18:01:58.8230572Z Description  : Run a Bash script on macOS, Linux, or Windows
