plain
2019-08-28T11:08:19.1766441Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-28T11:08:19.1766503Z 
2019-08-28T11:08:19.1767303Z   git checkout -b <new-branch-name>
2019-08-28T11:08:19.1767394Z 
2019-08-28T11:08:19.1767803Z HEAD is now at f970d3d04 Auto merge of #63825 - nathanwhit:check-run-results, r=Mark-Simulacrum
2019-08-28T11:08:19.1990385Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-28T11:08:19.1994216Z ==============================================================================
2019-08-28T11:08:19.1994342Z Task         : Bash
2019-08-28T11:08:19.1994423Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-28T13:40:02.0775712Z 
2019-08-28T13:40:02.0775896Z failures:
2019-08-28T13:40:02.0823701Z 
2019-08-28T13:40:02.0824877Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2019-08-28T13:40:02.0825319Z normalized run.stdout:
2019-08-28T13:40:02.0829161Z uploaded "$TEST_BUILD_DIR/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a", waiting for result
2019-08-28T13:40:02.0829597Z 
2019-08-28T13:40:02.0829662Z 
2019-08-28T13:40:02.0829773Z The actual run.stdout differed from the expected run.stdout.
2019-08-28T13:40:02.0829773Z The actual run.stdout differed from the expected run.stdout.
2019-08-28T13:40:02.0830543Z Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stdout
2019-08-28T13:40:02.0830819Z error: 1 errors occured comparing run output.
2019-08-28T13:40:02.0830920Z status: exit code: 0
2019-08-28T13:40:02.0830920Z status: exit code: 0
2019-08-28T13:40:02.0831434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2019-08-28T13:40:02.0831885Z ------------------------------------------
2019-08-28T13:40:02.0831885Z ------------------------------------------
2019-08-28T13:40:02.0832317Z uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a", waiting for result
2019-08-28T13:40:02.0832709Z ------------------------------------------
2019-08-28T13:40:02.0832791Z stderr:
2019-08-28T13:40:02.0833081Z ------------------------------------------
2019-08-28T13:40:02.0833081Z ------------------------------------------
2019-08-28T13:40:02.0833429Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:23] Unit = Unit
2019-08-28T13:40:02.0833806Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:24] a = Unit
2019-08-28T13:40:02.0834195Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:30] Point{x: 42, y: 24,} = Point {
2019-08-28T13:40:02.0834317Z     x: 42,
2019-08-28T13:40:02.0834385Z     y: 24,
2019-08-28T13:40:02.0834478Z }
2019-08-28T13:40:02.0834817Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:31] b = Point {
2019-08-28T13:40:02.0834933Z     x: 42,
2019-08-28T13:40:02.0835001Z     y: 24,
2019-08-28T13:40:02.0835672Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:39]
2019-08-28T13:40:02.0835672Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:39]
2019-08-28T13:40:02.0836149Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:43] &a = NoCopy(
2019-08-28T13:40:02.0836329Z )
2019-08-28T13:40:02.0836329Z )
2019-08-28T13:40:02.0836681Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:43] dbg!(& a) = NoCopy(
2019-08-28T13:40:02.0836866Z )
2019-08-28T13:40:02.0836866Z )
2019-08-28T13:40:02.0837579Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:48] f(&42) = 42
2019-08-28T13:40:02.0837683Z before
2019-08-28T13:40:02.0838079Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:53] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-28T13:40:02.0838696Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] ("Yeah",) = (
2019-08-28T13:40:02.0838812Z     "Yeah",
2019-08-28T13:40:02.0838877Z )
2019-08-28T13:40:02.0839200Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:64] 1 = 1
2019-08-28T13:40:02.0839533Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:64] 2 = 2
2019-08-28T13:40:02.0839885Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] 1u8 = 1
2019-08-28T13:40:02.0840232Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] 2u32 = 2
2019-08-28T13:40:02.0840569Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] "Yeah" = "Yeah"
2019-08-28T13:40:02.0840910Z ------------------------------------------
2019-08-28T13:40:02.0840962Z 
2019-08-28T13:40:02.0840999Z 
2019-08-28T13:40:02.0841055Z 
---
2019-08-28T13:40:02.0891443Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-28T13:40:02.0891994Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-28T13:40:02.0918096Z 
2019-08-28T13:40:02.0918540Z 
2019-08-28T13:40:02.0921216Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-28T13:40:02.0922057Z 
2019-08-28T13:40:02.0922122Z 
2019-08-28T13:40:02.0932223Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
2019-08-28T13:40:02.0932823Z Build completed unsuccessfully in 2:26:45
2019-08-28T13:40:02.0932823Z Build completed unsuccessfully in 2:26:45
2019-08-28T13:40:02.1000476Z == clock drift check ==
2019-08-28T13:40:02.1019075Z   local time: Wed Aug 28 13:40:02 UTC 2019
2019-08-28T13:40:02.1751209Z   network time: Wed, 28 Aug 2019 13:40:02 GMT
2019-08-28T13:40:02.1755768Z == end clock drift check ==
2019-08-28T13:40:02.9521818Z ##[error]Bash exited with code '1'.
2019-08-28T13:40:02.9564433Z ##[section]Starting: Upload CPU usage statistics
2019-08-28T13:40:02.9571751Z ==============================================================================
2019-08-28T13:40:02.9571835Z Task         : Bash
2019-08-28T13:40:02.9571913Z Description  : Run a Bash script on macOS, Linux, or Windows
