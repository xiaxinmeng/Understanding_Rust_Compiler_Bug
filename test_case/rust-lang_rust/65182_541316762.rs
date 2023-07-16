plain
2019-10-12T11:36:30.2168801Z 
2019-10-12T11:36:30.2169495Z 6    |
2019-10-12T11:36:30.2171143Z 7    = note: `#[warn(incomplete_features)]` on by default
2019-10-12T11:36:30.2172768Z 8 
2019-10-12T11:36:30.2173540Z - thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', $SRC_DIR/libcore/slice/mod.rs:LL:COL
2019-10-12T11:36:30.2174525Z + thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/e33249ae394ae5ab8e081e2a22038e92ba211583/src/libcore/slice/mod.rs:2716:10
2019-10-12T11:36:30.2175167Z 11 
2019-10-12T11:36:30.2175382Z 12 error: internal compiler error: unexpected panic
2019-10-12T11:36:30.2175585Z 
2019-10-12T11:36:30.2175752Z 
2019-10-12T11:36:30.2175752Z 
2019-10-12T11:36:30.2175974Z The actual stderr differed from the expected stderr.
2019-10-12T11:36:30.2176497Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer/taking-fn-pointer.stderr
2019-10-12T11:36:30.2177042Z To update references, rerun the tests and pass the `--bless` flag
2019-10-12T11:36:30.2177592Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/taking-fn-pointer.rs`
2019-10-12T11:36:30.2178792Z error: 1 errors occurred comparing output.
2019-10-12T11:36:30.2179070Z status: exit code: 101
2019-10-12T11:36:30.2179070Z status: exit code: 101
2019-10-12T11:36:30.2180551Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/taking-fn-pointer/auxiliary" "-A" "unused"
2019-10-12T11:36:30.2181852Z ------------------------------------------
2019-10-12T11:36:30.2182275Z 
2019-10-12T11:36:30.2182710Z ------------------------------------------
2019-10-12T11:36:30.2182985Z stderr:
2019-10-12T11:36:30.2182985Z stderr:
2019-10-12T11:36:30.2183427Z ------------------------------------------
2019-10-12T11:36:30.2183716Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-12T11:36:30.2184402Z   --> /checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs:5:12
2019-10-12T11:36:30.2184715Z    |
2019-10-12T11:36:30.2185172Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-12T11:36:30.2185870Z    |
2019-10-12T11:36:30.2186304Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-12T11:36:30.2186509Z 
2019-10-12T11:36:30.2186509Z 
2019-10-12T11:36:30.2187127Z thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/e33249ae394ae5ab8e081e2a22038e92ba211583/src/libcore/slice/mod.rs:2716:10
2019-10-12T11:36:30.2187895Z 
2019-10-12T11:36:30.2188657Z error: internal compiler error: unexpected panic
2019-10-12T11:36:30.2188910Z 
2019-10-12T11:36:30.2189215Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-12T11:36:30.2189215Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-12T11:36:30.2189440Z 
2019-10-12T11:36:30.2190303Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-12T11:36:30.2190664Z 
2019-10-12T11:36:30.2191263Z note: rustc 1.40.0-nightly (e33249ae3 2019-10-12) running on x86_64-unknown-linux-gnu
2019-10-12T11:36:30.2191565Z 
2019-10-12T11:36:30.2192264Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=cc
2019-10-12T11:36:30.2192716Z 
2019-10-12T11:36:30.2193118Z ------------------------------------------
2019-10-12T11:36:30.2193354Z 
2019-10-12T11:36:30.2193553Z 
---
2019-10-12T11:36:30.2237099Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-12T11:36:30.2237255Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-12T11:36:30.2267436Z 
2019-10-12T11:36:30.2267561Z 
2019-10-12T11:36:30.2272189Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-12T11:36:30.2272861Z 
2019-10-12T11:36:30.2272917Z 
2019-10-12T11:36:30.2273355Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-12T11:36:30.2273483Z Build completed unsuccessfully in 1:14:28
2019-10-12T11:36:30.2273483Z Build completed unsuccessfully in 1:14:28
2019-10-12T11:36:30.2334940Z == clock drift check ==
2019-10-12T11:36:30.2347413Z   local time: Sat Oct 12 11:36:30 UTC 2019
2019-10-12T11:36:30.3970713Z   network time: Sat, 12 Oct 2019 11:36:30 GMT
2019-10-12T11:36:30.3970850Z == end clock drift check ==
2019-10-12T11:36:30.8366471Z ##[error]Bash exited with code '1'.
2019-10-12T11:36:30.8414196Z ##[section]Starting: Upload CPU usage statistics
2019-10-12T11:36:30.8431562Z ==============================================================================
2019-10-12T11:36:30.8431668Z Task         : Bash
2019-10-12T11:36:30.8431767Z Description  : Run a Bash script on macOS, Linux, or Windows
