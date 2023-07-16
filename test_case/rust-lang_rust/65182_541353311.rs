plain
2019-10-12T17:47:49.5709008Z 
2019-10-12T17:47:49.5709252Z 6    |
2019-10-12T17:47:49.5709487Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-10-12T17:47:49.5709747Z 8 
2019-10-12T17:47:49.5710300Z - thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', $SRC_DIR/libcore/slice/mod.rs:LL:COL
2019-10-12T17:47:49.5711926Z + thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/3cf3b1551e0708700e705d89f02b7ebeaf9579d0/src/libcore/slice/mod.rs:2716:10
2019-10-12T17:47:49.5712750Z 11 
2019-10-12T17:47:49.5713036Z 12 error: internal compiler error: unexpected panic
2019-10-12T17:47:49.5713271Z 
2019-10-12T17:47:49.5713487Z 
2019-10-12T17:47:49.5713487Z 
2019-10-12T17:47:49.5713764Z The actual stderr differed from the expected stderr.
2019-10-12T17:47:49.5714445Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer/taking-fn-pointer.stderr
2019-10-12T17:47:49.5715098Z To update references, rerun the tests and pass the `--bless` flag
2019-10-12T17:47:49.5716134Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/taking-fn-pointer.rs`
2019-10-12T17:47:49.5716725Z error: 1 errors occurred comparing output.
2019-10-12T17:47:49.5717175Z status: exit code: 101
2019-10-12T17:47:49.5717175Z status: exit code: 101
2019-10-12T17:47:49.5718696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer/auxiliary" "-A" "unused"
2019-10-12T17:47:49.5721951Z ------------------------------------------
2019-10-12T17:47:49.5722425Z 
2019-10-12T17:47:49.5723021Z ------------------------------------------
2019-10-12T17:47:49.5723286Z stderr:
2019-10-12T17:47:49.5723286Z stderr:
2019-10-12T17:47:49.5723715Z ------------------------------------------
2019-10-12T17:47:49.5724229Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-12T17:47:49.5725003Z   --> /checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs:5:12
2019-10-12T17:47:49.5725366Z    |
2019-10-12T17:47:49.5725534Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-12T17:47:49.5726014Z    |
2019-10-12T17:47:49.5726180Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-12T17:47:49.5726307Z 
2019-10-12T17:47:49.5726307Z 
2019-10-12T17:47:49.5726795Z thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/3cf3b1551e0708700e705d89f02b7ebeaf9579d0/src/libcore/slice/mod.rs:2716:10
2019-10-12T17:47:49.5727201Z 
2019-10-12T17:47:49.5727364Z error: internal compiler error: unexpected panic
2019-10-12T17:47:49.5727487Z 
2019-10-12T17:47:49.5727639Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-12T17:47:49.5727639Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-12T17:47:49.5727783Z 
2019-10-12T17:47:49.5728190Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-12T17:47:49.5728383Z 
2019-10-12T17:47:49.5728762Z note: rustc 1.40.0-nightly (3cf3b1551 2019-10-12) running on x86_64-unknown-linux-gnu
2019-10-12T17:47:49.5728944Z 
2019-10-12T17:47:49.5729350Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=cc
2019-10-12T17:47:49.5729810Z 
2019-10-12T17:47:49.5730143Z ------------------------------------------
2019-10-12T17:47:49.5730293Z 
2019-10-12T17:47:49.5730427Z 
---
2019-10-12T17:47:49.5749178Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-12T17:47:49.5749544Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-12T17:47:49.5767781Z 
2019-10-12T17:47:49.5771417Z 
2019-10-12T17:47:49.5774169Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-12T17:47:49.5775227Z 
2019-10-12T17:47:49.5775389Z 
2019-10-12T17:47:49.5777902Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-12T17:47:49.5778215Z Build completed unsuccessfully in 1:19:39
2019-10-12T17:47:49.5778215Z Build completed unsuccessfully in 1:19:39
2019-10-12T17:47:49.5835321Z == clock drift check ==
2019-10-12T17:47:49.5850253Z   local time: Sat Oct 12 17:47:49 UTC 2019
2019-10-12T17:47:49.7444595Z   network time: Sat, 12 Oct 2019 17:47:49 GMT
2019-10-12T17:47:49.7445387Z == end clock drift check ==
2019-10-12T17:47:50.2989597Z ##[error]Bash exited with code '1'.
2019-10-12T17:47:50.3047781Z ##[section]Starting: Upload CPU usage statistics
2019-10-12T17:47:50.3074588Z ==============================================================================
2019-10-12T17:47:50.3074710Z Task         : Bash
2019-10-12T17:47:50.3074789Z Description  : Run a Bash script on macOS, Linux, or Windows
