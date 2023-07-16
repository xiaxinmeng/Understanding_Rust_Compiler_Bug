plain
2019-08-14T16:10:10.5158123Z test [ui] ui/async-await/issues/issue-63388-1.rs ... ok
2019-08-14T16:10:10.5345026Z test [ui] ui/async-await/issues/issue-63388-2.rs ... ok
2019-08-14T16:10:10.5735600Z test [ui] ui/async-await/issues/issue-63388-3.rs ... ok
2019-08-14T16:10:10.5809883Z test [ui] ui/async-await/issues/issue-63388-4.rs ... ok
2019-08-14T16:10:10.6314002Z test [ui] ui/async-await/issues/non-async-enclosing-span.rs ... ok
2019-08-14T16:10:10.7206339Z test [ui] ui/async-await/move-part-await-return-rest-tuple.rs ... ok
2019-08-14T16:10:10.7971231Z test [ui] ui/async-await/multiple-lifetimes/elided.rs ... ok
2019-08-14T16:10:10.8606788Z test [ui] ui/async-await/multiple-lifetimes/fn-ptr.rs ... ok
2019-08-14T16:10:10.9209424Z test [ui] ui/async-await/multiple-lifetimes/hrtb.rs ... ok
---
2019-08-14T16:11:11.1584513Z test [ui] ui/coherence/re-rebalance-coherence.rs ... ok
2019-08-14T16:11:11.2097769Z test [ui] ui/command-line-diagnostics.rs ... ok
2019-08-14T16:11:12.7803923Z test [ui] ui/command-pre-exec.rs ... ok
2019-08-14T16:11:13.1068176Z test [ui] ui/command-exec.rs ... ok
2019-08-14T16:11:13.1172435Z test [ui] ui/commandline-argfile-badutf8.rs ... FAILED
2019-08-14T16:11:13.1240044Z test [ui] ui/commandline-argfile-missing.rs ... FAILED
2019-08-14T16:11:13.3002756Z test [ui] ui/commandline-argfile.rs ... ok
2019-08-14T16:11:13.3976774Z test [ui] ui/compare-method/region-extra-2.rs ... ok
2019-08-14T16:11:13.4031144Z test [ui] ui/command-uid-gid.rs ... ok
2019-08-14T16:11:13.4437677Z test [ui] ui/compare-method/region-unrelated.rs ... ok
2019-08-14T16:11:13.4438241Z test [ui] ui/compare-method/region-extra.rs ... ok
---
2019-08-14T16:22:14.3043798Z test [ui] ui/zero-sized/zero-sized-vec-deque-push.rs ... ok
2019-08-14T16:22:14.3043901Z 
2019-08-14T16:22:14.3043965Z failures:
2019-08-14T16:22:14.3084539Z 
2019-08-14T16:22:14.3085074Z ---- [ui] ui/commandline-argfile-badutf8.rs stdout ----
2019-08-14T16:22:14.3085259Z 
2019-08-14T16:22:14.3085259Z 
2019-08-14T16:22:14.3085592Z - error: Argument 18 is not valid: Utf8 error in $DIR/commandline-argfile-badutf8.args
2019-08-14T16:22:14.3085903Z + error: Argument 19 is not valid: Utf8 error in $DIR/commandline-argfile-badutf8.args
2019-08-14T16:22:14.3086062Z 3 
2019-08-14T16:22:14.3086117Z 
2019-08-14T16:22:14.3086151Z 
2019-08-14T16:22:14.3086215Z The actual stderr differed from the expected stderr.
2019-08-14T16:22:14.3086215Z The actual stderr differed from the expected stderr.
2019-08-14T16:22:14.3086616Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-badutf8/commandline-argfile-badutf8.stderr
2019-08-14T16:22:14.3086938Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T16:22:14.3087244Z To only update this specific test, also pass `--test-args commandline-argfile-badutf8.rs`
2019-08-14T16:22:14.3087388Z error: 1 errors occurred comparing output.
2019-08-14T16:22:14.3087454Z status: exit code: 1
2019-08-14T16:22:14.3087454Z status: exit code: 1
2019-08-14T16:22:14.3088461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/commandline-argfile-badutf8.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-badutf8" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--cfg" "cmdline_set" "@/checkout/src/test/ui/commandline-argfile-badutf8.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-badutf8/auxiliary" "-A" "unused"
2019-08-14T16:22:14.3089019Z ------------------------------------------
2019-08-14T16:22:14.3089065Z 
2019-08-14T16:22:14.3089299Z ------------------------------------------
2019-08-14T16:22:14.3089387Z stderr:
2019-08-14T16:22:14.3089387Z stderr:
2019-08-14T16:22:14.3089616Z ------------------------------------------
2019-08-14T16:22:14.3090122Z error: Argument 19 is not valid: Utf8 error in /checkout/src/test/ui/commandline-argfile-badutf8.args
2019-08-14T16:22:14.3090266Z 
2019-08-14T16:22:14.3090550Z ------------------------------------------
2019-08-14T16:22:14.3090595Z 
2019-08-14T16:22:14.3090650Z 
2019-08-14T16:22:14.3090650Z 
2019-08-14T16:22:14.3090898Z ---- [ui] ui/commandline-argfile-missing.rs stdout ----
2019-08-14T16:22:14.3090991Z diff of stderr:
2019-08-14T16:22:14.3091029Z 
2019-08-14T16:22:14.3091530Z - error: Argument 18 is not valid: IO Error: $DIR/commandline-argfile-missing.args: No such file or directory (os error 2)
2019-08-14T16:22:14.3091887Z + error: Argument 19 is not valid: IO Error: $DIR/commandline-argfile-missing.args: No such file or directory (os error 2)
2019-08-14T16:22:14.3092050Z 3 
2019-08-14T16:22:14.3092084Z 
2019-08-14T16:22:14.3092137Z 
2019-08-14T16:22:14.3092201Z The actual stderr differed from the expected stderr.
2019-08-14T16:22:14.3092201Z The actual stderr differed from the expected stderr.
2019-08-14T16:22:14.3092589Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-missing/commandline-argfile-missing.stderr
2019-08-14T16:22:14.3092890Z To update references, rerun the tests and pass the `--bless` flag
2019-08-14T16:22:14.3093492Z To only update this specific test, also pass `--test-args commandline-argfile-missing.rs`
2019-08-14T16:22:14.3094002Z error: 1 errors occurred comparing output.
2019-08-14T16:22:14.3094072Z status: exit code: 1
2019-08-14T16:22:14.3094072Z status: exit code: 1
2019-08-14T16:22:14.3095122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/commandline-argfile-missing.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-missing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--cfg" "cmdline_set" "@/checkout/src/test/ui/commandline-argfile-missing.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/commandline-argfile-missing/auxiliary" "-A" "unused"
2019-08-14T16:22:14.3095639Z ------------------------------------------
2019-08-14T16:22:14.3095710Z 
2019-08-14T16:22:14.3095947Z ------------------------------------------
2019-08-14T16:22:14.3096035Z stderr:
2019-08-14T16:22:14.3096035Z stderr:
2019-08-14T16:22:14.3096265Z ------------------------------------------
2019-08-14T16:22:14.3096647Z error: Argument 19 is not valid: IO Error: /checkout/src/test/ui/commandline-argfile-missing.args: No such file or directory (os error 2)
2019-08-14T16:22:14.3096757Z 
2019-08-14T16:22:14.3097010Z ------------------------------------------
2019-08-14T16:22:14.3097055Z 
2019-08-14T16:22:14.3097087Z 
2019-08-14T16:22:14.3097087Z 
2019-08-14T16:22:14.3097140Z 
2019-08-14T16:22:14.3097196Z failures:
2019-08-14T16:22:14.3097449Z     [ui] ui/commandline-argfile-badutf8.rs
2019-08-14T16:22:14.3097688Z     [ui] ui/commandline-argfile-missing.rs
2019-08-14T16:22:14.3098060Z test result: FAILED. 8863 passed; 2 failed; 45 ignored; 0 measured; 0 filtered out
2019-08-14T16:22:14.3098120Z 
2019-08-14T16:22:14.3125855Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-14T16:22:14.3125969Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T16:22:14.3125969Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-14T16:22:14.3142300Z 
2019-08-14T16:22:14.3142587Z 
2019-08-14T16:22:14.3145385Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-14T16:22:14.3146422Z 
2019-08-14T16:22:14.3146695Z 
2019-08-14T16:22:14.3155173Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-08-14T16:22:14.3155315Z Build completed unsuccessfully in 1:15:57
2019-08-14T16:22:14.3155315Z Build completed unsuccessfully in 1:15:57
2019-08-14T16:22:14.3220270Z == clock drift check ==
2019-08-14T16:22:14.3235370Z   local time: Wed Aug 14 16:22:14 UTC 2019
2019-08-14T16:22:14.4326318Z   network time: Wed, 14 Aug 2019 16:22:14 GMT
2019-08-14T16:22:14.4326631Z == end clock drift check ==
2019-08-14T16:22:15.1277542Z ##[error]Bash exited with code '1'.
2019-08-14T16:22:15.1322526Z ##[section]Starting: Upload CPU usage statistics
2019-08-14T16:22:15.1331270Z ==============================================================================
2019-08-14T16:22:15.1331399Z Task         : Bash
2019-08-14T16:22:15.1331477Z Description  : Run a Bash script on macOS, Linux, or Windows
