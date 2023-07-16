plain
2019-09-22T18:30:31.1252218Z ---- [ui] ui/panic-runtime/libtest-unwinds.rs stdout ----
2019-09-22T18:30:31.1252301Z 
2019-09-22T18:30:31.1252400Z error: ui test compiled successfully!
2019-09-22T18:30:31.1252477Z status: exit code: 0
2019-09-22T18:30:31.1253694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/libtest-unwinds.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/libtest-unwinds" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/libtest-unwinds/auxiliary" "-A" "unused"
2019-09-22T18:30:31.1254299Z ------------------------------------------
2019-09-22T18:30:31.1254355Z 
2019-09-22T18:30:31.1254621Z ------------------------------------------
2019-09-22T18:30:31.1254703Z stderr:
---
2019-09-22T18:30:31.1306201Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-22T18:30:31.1306829Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-22T18:30:31.1324645Z 
2019-09-22T18:30:31.1324778Z 
2019-09-22T18:30:31.1331802Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "ui" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-22T18:30:31.1333103Z 
2019-09-22T18:30:31.1333162Z 
2019-09-22T18:30:31.1345589Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-22T18:30:31.1345756Z Build completed unsuccessfully in 1:40:26
2019-09-22T18:30:31.1345756Z Build completed unsuccessfully in 1:40:26
2019-09-22T18:30:31.1407868Z == clock drift check ==
2019-09-22T18:30:31.1420662Z   local time: Sun Sep 22 18:30:31 UTC 2019
2019-09-22T18:30:31.2467157Z   network time: Sun, 22 Sep 2019 18:30:31 GMT
2019-09-22T18:30:31.2467687Z == end clock drift check ==
2019-09-22T18:30:32.3014157Z ##[error]Bash exited with code '1'.
2019-09-22T18:30:32.3058114Z ##[section]Starting: Upload CPU usage statistics
2019-09-22T18:30:32.3071181Z ==============================================================================
2019-09-22T18:30:32.3071311Z Task         : Bash
2019-09-22T18:30:32.3071412Z Description  : Run a Bash script on macOS, Linux, or Windows
