plain
2019-08-02T05:41:19.0221244Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T05:41:19.0223841Z 
2019-08-02T05:41:19.0227736Z   git checkout -b <new-branch-name>
2019-08-02T05:41:19.0229679Z 
2019-08-02T05:41:19.0232193Z HEAD is now at 83de5034c Auto merge of #63180 - varkor:trait-alias-impl-trait, r=Centril
2019-08-02T05:41:19.0364097Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T05:41:19.0367187Z ==============================================================================
2019-08-02T05:41:19.0367272Z Task         : Bash
2019-08-02T05:41:19.0367347Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T07:42:40.3616495Z 1 error: at least one trait must be specified
2019-08-02T07:42:40.3616884Z -   --> $DIR/generic_type_does_not_live_long_enough.rs:9:35
2019-08-02T07:42:40.3617154Z +   --> $DIR/generic_type_does_not_live_long_enough.rs:9:29
2019-08-02T07:42:40.3617222Z 3    |
2019-08-02T07:42:40.3617780Z - LL | existential type WrongGeneric<T>: 'static;
2019-08-02T07:42:40.3618079Z -    |                                   ^^^^^^^
2019-08-02T07:42:40.3618650Z + LL | type WrongGeneric<T> = impl 'static;
2019-08-02T07:42:40.3618820Z 6 
2019-08-02T07:42:40.3619054Z 7 error[E0308]: mismatched types
2019-08-02T07:42:40.3619413Z 8   --> $DIR/generic_type_does_not_live_long_enough.rs:6:18
2019-08-02T07:42:40.3619467Z 
2019-08-02T07:42:40.3619467Z 
2019-08-02T07:42:40.3619501Z 
2019-08-02T07:42:40.3619665Z The actual stderr differed from the expected stderr.
2019-08-02T07:42:40.3620116Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/generic_type_does_not_live_long_enough.nll.stderr
2019-08-02T07:42:40.3620457Z To update references, rerun the tests and pass the `--bless` flag
2019-08-02T07:42:40.3620812Z To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`
2019-08-02T07:42:40.3620961Z error: 1 errors occurred comparing output.
2019-08-02T07:42:40.3621040Z status: exit code: 1
2019-08-02T07:42:40.3621040Z status: exit code: 1
2019-08-02T07:42:40.3622074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/auxiliary" "-A" "unused"
2019-08-02T07:42:40.3622617Z ------------------------------------------
2019-08-02T07:42:40.3622666Z 
2019-08-02T07:42:40.3622923Z ------------------------------------------
2019-08-02T07:42:40.3622990Z stderr:
2019-08-02T07:42:40.3622990Z stderr:
2019-08-02T07:42:40.3623523Z ------------------------------------------
2019-08-02T07:42:40.3623598Z error: at least one trait must be specified
2019-08-02T07:42:40.3623909Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:29
2019-08-02T07:42:40.3623990Z    |
2019-08-02T07:42:40.3624232Z LL | type WrongGeneric<T> = impl 'static;
2019-08-02T07:42:40.3624354Z 
2019-08-02T07:42:40.3624408Z error[E0308]: mismatched types
2019-08-02T07:42:40.3624711Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:6:18
2019-08-02T07:42:40.3624788Z    |
2019-08-02T07:42:40.3624788Z    |
2019-08-02T07:42:40.3624864Z LL |     let z: i32 = x; //~ ERROR mismatched types
2019-08-02T07:42:40.3624934Z    |                  ^ expected i32, found opaque type
2019-08-02T07:42:40.3625062Z    = note: expected type `i32`
2019-08-02T07:42:40.3625062Z    = note: expected type `i32`
2019-08-02T07:42:40.3625142Z               found type `WrongGeneric::<&{integer}>`
2019-08-02T07:42:40.3625383Z error: aborting due to 2 previous errors
2019-08-02T07:42:40.3625422Z 
2019-08-02T07:42:40.3625727Z For more information about this error, try `rustc --explain E0308`.
2019-08-02T07:42:40.3625795Z 
---
2019-08-02T07:42:40.3627187Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-02T07:42:40.3627270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-02T07:42:40.3627315Z 
2019-08-02T07:42:40.3627358Z 
2019-08-02T07:42:40.3629539Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-02T07:42:40.3630166Z 
2019-08-02T07:42:40.3630219Z 
2019-08-02T07:42:40.3630286Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-02T07:42:40.3630376Z Build completed unsuccessfully in 1:56:31
2019-08-02T07:42:40.3630376Z Build completed unsuccessfully in 1:56:31
2019-08-02T07:42:41.1250320Z ##[error]Bash exited with code '1'.
2019-08-02T07:42:41.1284824Z ##[section]Starting: Upload CPU usage statistics
2019-08-02T07:42:41.1292280Z ==============================================================================
2019-08-02T07:42:41.1292375Z Task         : Bash
2019-08-02T07:42:41.1292456Z Description  : Run a Bash script on macOS, Linux, or Windows
