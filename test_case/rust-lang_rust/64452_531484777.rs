plain
2019-09-14T14:36:43.3822091Z 
2019-09-14T14:36:43.3822925Z ---- [ui (nll)] ui/continue-after-missing-main.rs stdout ----
2019-09-14T14:36:43.3823289Z diff of stderr:
2019-09-14T14:36:43.3823506Z 
2019-09-14T14:36:43.3823763Z 1 error[E0601]: `main` function not found in crate `continue_after_missing_main`
2019-09-14T14:36:43.3824253Z +   --> $DIR/continue-after-missing-main.rs:1:1
2019-09-14T14:36:43.3824566Z 2    |
2019-09-14T14:36:43.3825044Z -    = note: consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
2019-09-14T14:36:43.3831876Z + LL | / #![allow(dead_code)]
2019-09-14T14:36:43.3832141Z + LL | |
2019-09-14T14:36:43.3832681Z + LL | | // error-pattern:`main` function not found in crate
2019-09-14T14:36:43.3832903Z + LL | |
2019-09-14T14:36:43.3833084Z + ...  |
2019-09-14T14:36:43.3833278Z + LL | |
2019-09-14T14:36:43.3833437Z + LL | | }
2019-09-14T14:36:43.3833860Z +    | |_^ consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
2019-09-14T14:36:43.3834268Z 5 error: aborting due to previous error
2019-09-14T14:36:43.3834432Z 6 
2019-09-14T14:36:43.3834582Z 
2019-09-14T14:36:43.3834715Z 
2019-09-14T14:36:43.3834715Z 
2019-09-14T14:36:43.3834895Z The actual stderr differed from the expected stderr.
2019-09-14T14:36:43.3835372Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/continue-after-missing-main.nll.stderr
2019-09-14T14:36:43.3835835Z To update references, rerun the tests and pass the `--bless` flag
2019-09-14T14:36:43.3836277Z To only update this specific test, also pass `--test-args continue-after-missing-main.rs`
2019-09-14T14:36:43.3836648Z error: 1 errors occurred comparing output.
2019-09-14T14:36:43.3836830Z status: exit code: 1
2019-09-14T14:36:43.3836830Z status: exit code: 1
2019-09-14T14:36:43.3838689Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/continue-after-missing-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/auxiliary" "-A" "unused"
2019-09-14T14:36:43.3839744Z ------------------------------------------
2019-09-14T14:36:43.3839965Z 
2019-09-14T14:36:43.3840430Z ------------------------------------------
2019-09-14T14:36:43.3840681Z stderr:
2019-09-14T14:36:43.3840681Z stderr:
2019-09-14T14:36:43.3841098Z ------------------------------------------
2019-09-14T14:36:43.3841491Z error[E0601]: `main` function not found in crate `continue_after_missing_main`
2019-09-14T14:36:43.3842354Z    |
2019-09-14T14:36:43.3842354Z    |
2019-09-14T14:36:43.3842522Z LL | / #![allow(dead_code)]
2019-09-14T14:36:43.3842707Z LL | |
2019-09-14T14:36:43.3843090Z LL | | // error-pattern:`main` function not found in crate
2019-09-14T14:36:43.3843460Z ...  |
2019-09-14T14:36:43.3843460Z ...  |
2019-09-14T14:36:43.3843616Z LL | |     //~^ ERROR lifetime mismatch
2019-09-14T14:36:43.3843793Z LL | | }
2019-09-14T14:36:43.3844197Z    | |_^ consider adding a `main` function to `/checkout/src/test/ui/continue-after-missing-main.rs`
2019-09-14T14:36:43.3844571Z error: aborting due to previous error
2019-09-14T14:36:43.3844904Z 
2019-09-14T14:36:43.3845452Z For more information about this error, try `rustc --explain E0601`.
2019-09-14T14:36:43.3845627Z 
---
2019-09-14T14:36:43.3870542Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-14T14:36:43.3911939Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T14:36:43.3912767Z 
2019-09-14T14:36:43.3913175Z 
2019-09-14T14:36:43.3915317Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-09-14T14:36:43.3916506Z 
2019-09-14T14:36:43.3916704Z 
2019-09-14T14:36:43.3916952Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-14T14:36:43.3917394Z Build completed unsuccessfully in 1:58:53
2019-09-14T14:36:43.3917394Z Build completed unsuccessfully in 1:58:53
2019-09-14T14:36:43.3960735Z == clock drift check ==
2019-09-14T14:36:43.3979566Z   local time: Sat Sep 14 14:36:43 UTC 2019
2019-09-14T14:36:43.5530033Z   network time: Sat, 14 Sep 2019 14:36:43 GMT
2019-09-14T14:36:43.5530379Z == end clock drift check ==
2019-09-14T14:36:44.6283260Z ##[error]Bash exited with code '1'.
2019-09-14T14:36:44.6323886Z ##[section]Starting: Upload CPU usage statistics
2019-09-14T14:36:44.6330921Z ==============================================================================
2019-09-14T14:36:44.6331016Z Task         : Bash
2019-09-14T14:36:44.6331262Z Description  : Run a Bash script on macOS, Linux, or Windows
