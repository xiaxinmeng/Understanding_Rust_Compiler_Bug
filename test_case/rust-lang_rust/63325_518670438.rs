plain
2019-08-06T12:09:10.0769194Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T12:09:10.0769249Z 
2019-08-06T12:09:10.0769479Z   git checkout -b <new-branch-name>
2019-08-06T12:09:10.0769538Z 
2019-08-06T12:09:10.0769820Z HEAD is now at 9d9b930e3 Auto merge of #63325 - Centril:rollup-gaqv2nd, r=Centril
2019-08-06T12:09:10.0863405Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T12:09:10.0866460Z ==============================================================================
2019-08-06T12:09:10.0866563Z Task         : Bash
2019-08-06T12:09:10.0866629Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T13:28:06.5585515Z 
2019-08-06T13:28:06.5586262Z ---- [ui] ui/consts/issue-55878.rs stdout ----
2019-08-06T13:28:06.5586352Z diff of stderr:
2019-08-06T13:28:06.5586411Z 
2019-08-06T13:28:06.5586483Z 1 error[E0080]: the type `[u8; SIZE]` is too big for the current architecture
2019-08-06T13:28:06.5586816Z -   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-08-06T13:28:06.5587032Z -    |
2019-08-06T13:28:06.5587274Z - LL |     intrinsics::size_of::<T>()
2019-08-06T13:28:06.5587590Z 6    | 
2019-08-06T13:28:06.5587819Z 7   ::: $DIR/issue-55878.rs:6:26
2019-08-06T13:28:06.5587901Z 8    |
2019-08-06T13:28:06.5587936Z 
2019-08-06T13:28:06.5587936Z 
2019-08-06T13:28:06.5587969Z 
2019-08-06T13:28:06.5588052Z The actual stderr differed from the expected stderr.
2019-08-06T13:28:06.5588389Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878/issue-55878.stderr
2019-08-06T13:28:06.5588714Z To update references, rerun the tests and pass the `--bless` flag
2019-08-06T13:28:06.5613693Z To only update this specific test, also pass `--test-args consts/issue-55878.rs`
2019-08-06T13:28:06.5613849Z error: 1 errors occurred comparing output.
2019-08-06T13:28:06.5613918Z status: exit code: 1
2019-08-06T13:28:06.5613918Z status: exit code: 1
2019-08-06T13:28:06.5615145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-55878.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878/auxiliary" "-A" "unused"
2019-08-06T13:28:06.5615703Z ------------------------------------------
2019-08-06T13:28:06.5615759Z 
2019-08-06T13:28:06.5616010Z ------------------------------------------
2019-08-06T13:28:06.5616086Z stderr:
2019-08-06T13:28:06.5616086Z stderr:
2019-08-06T13:28:06.5616322Z ------------------------------------------
2019-08-06T13:28:06.5616412Z error[E0080]: the type `[u8; 4294967295]` is too big for the current architecture
2019-08-06T13:28:06.5616762Z   ::: /checkout/src/test/ui/consts/issue-55878.rs:6:26
2019-08-06T13:28:06.5616831Z    |
2019-08-06T13:28:06.5616831Z    |
2019-08-06T13:28:06.5616910Z LL |     println!("Size: {}", std::mem::size_of::<[u8; std::u64::MAX as usize]>());
2019-08-06T13:28:06.5617269Z 
2019-08-06T13:28:06.5617328Z error: aborting due to previous error
2019-08-06T13:28:06.5617369Z 
2019-08-06T13:28:06.5617642Z For more information about this error, try `rustc --explain E0080`.
---
2019-08-06T13:28:06.5624295Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-06T13:28:06.5624423Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-06T13:28:06.5639628Z 
2019-08-06T13:28:06.5639737Z 
2019-08-06T13:28:06.5641983Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-06T13:28:06.5642731Z 
2019-08-06T13:28:06.5642776Z 
2019-08-06T13:28:06.5650924Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-08-06T13:28:06.5651041Z Build completed unsuccessfully in 1:14:25
2019-08-06T13:28:06.5651041Z Build completed unsuccessfully in 1:14:25
2019-08-06T13:28:07.2734679Z ##[error]Bash exited with code '1'.
2019-08-06T13:28:07.2778691Z ##[section]Starting: Upload CPU usage statistics
2019-08-06T13:28:07.2786976Z ==============================================================================
2019-08-06T13:28:07.2787068Z Task         : Bash
2019-08-06T13:28:07.2787151Z Description  : Run a Bash script on macOS, Linux, or Windows
