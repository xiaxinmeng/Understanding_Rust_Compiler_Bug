plain
2019-09-02T00:24:47.0255789Z 
2019-09-02T00:24:47.0255879Z failures:
2019-09-02T00:24:47.0289739Z 
2019-09-02T00:24:47.0290473Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2019-09-02T00:24:47.0290591Z normalized run.stdout:
2019-09-02T00:24:47.0291077Z uploaded "$TEST_BUILD_DIR/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a", waiting for result
2019-09-02T00:24:47.0292371Z 
2019-09-02T00:24:47.0292405Z 
2019-09-02T00:24:47.0292503Z The actual run.stdout differed from the expected run.stdout.
2019-09-02T00:24:47.0292503Z The actual run.stdout differed from the expected run.stdout.
2019-09-02T00:24:47.0293042Z Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stdout
2019-09-02T00:24:47.0293230Z error: 1 errors occured comparing run output.
2019-09-02T00:24:47.0293387Z status: exit code: 0
2019-09-02T00:24:47.0293387Z status: exit code: 0
2019-09-02T00:24:47.0293868Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2019-09-02T00:24:47.0294435Z ------------------------------------------
2019-09-02T00:24:47.0294435Z ------------------------------------------
2019-09-02T00:24:47.0294866Z uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a", waiting for result
2019-09-02T00:24:47.0295493Z ------------------------------------------
2019-09-02T00:24:47.0295556Z stderr:
2019-09-02T00:24:47.0295783Z ------------------------------------------
2019-09-02T00:24:47.0295783Z ------------------------------------------
2019-09-02T00:24:47.0296048Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:20] Unit = Unit
2019-09-02T00:24:47.0296343Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:21] a = Unit
2019-09-02T00:24:47.0296829Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2019-09-02T00:24:47.0296910Z     x: 42,
2019-09-02T00:24:47.0296981Z     y: 24,
2019-09-02T00:24:47.0297035Z }
2019-09-02T00:24:47.0297327Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:28] b = Point {
2019-09-02T00:24:47.0297401Z     x: 42,
2019-09-02T00:24:47.0297472Z     y: 24,
2019-09-02T00:24:47.0297795Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2019-09-02T00:24:47.0297795Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2019-09-02T00:24:47.0298086Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2019-09-02T00:24:47.0298230Z )
2019-09-02T00:24:47.0298230Z )
2019-09-02T00:24:47.0298524Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2019-09-02T00:24:47.0298671Z )
2019-09-02T00:24:47.0298671Z )
2019-09-02T00:24:47.0298936Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2019-09-02T00:24:47.0299027Z before
2019-09-02T00:24:47.0299333Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-09-02T00:24:47.0299657Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2019-09-02T00:24:47.0299731Z     "Yeah",
2019-09-02T00:24:47.0299800Z )
2019-09-02T00:24:47.0300061Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 1 = 1
2019-09-02T00:24:47.0300355Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 2 = 2
2019-09-02T00:24:47.0300641Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2019-09-02T00:24:47.0300940Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2019-09-02T00:24:47.0301855Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2019-09-02T00:24:47.0302204Z ------------------------------------------
2019-09-02T00:24:47.0302271Z 
2019-09-02T00:24:47.0302305Z 
2019-09-02T00:24:47.0302338Z 
---
2019-09-02T00:24:47.0344865Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-02T00:24:47.0345162Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-02T00:24:47.0363412Z 
2019-09-02T00:24:47.0363552Z 
2019-09-02T00:24:47.0366255Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
2019-09-02T00:24:47.0367393Z 
2019-09-02T00:24:47.0367530Z 
2019-09-02T00:24:47.0375124Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
2019-09-02T00:24:47.0375958Z Build completed unsuccessfully in 2:09:12
2019-09-02T00:24:47.0375958Z Build completed unsuccessfully in 2:09:12
2019-09-02T00:24:47.0434074Z == clock drift check ==
2019-09-02T00:24:47.0455085Z   local time: Mon Sep  2 00:24:47 UTC 2019
2019-09-02T00:24:47.3176369Z   network time: Mon, 02 Sep 2019 00:24:47 GMT
2019-09-02T00:24:47.3182244Z == end clock drift check ==
2019-09-02T00:24:48.4924561Z ##[error]Bash exited with code '1'.
2019-09-02T00:24:48.4959129Z ##[section]Starting: Upload CPU usage statistics
2019-09-02T00:24:48.4963693Z ==============================================================================
2019-09-02T00:24:48.4963807Z Task         : Bash
2019-09-02T00:24:48.4963879Z Description  : Run a Bash script on macOS, Linux, or Windows
