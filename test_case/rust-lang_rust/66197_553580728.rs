plain
2019-11-13T20:08:09.4648637Z 1 error: at least one trait must be specified
2019-11-13T20:08:09.4648936Z -   --> $DIR/generic_type_does_not_live_long_enough.rs:9:29
2019-11-13T20:08:09.4649187Z +   --> $DIR/generic_type_does_not_live_long_enough.rs:9:24
2019-11-13T20:08:09.4649250Z 3    |
2019-11-13T20:08:09.4649465Z 4 LL | type WrongGeneric<T> = impl 'static;
2019-11-13T20:08:09.4649743Z +    |                        ^^^^^^^^^^^^
2019-11-13T20:08:09.4649816Z 6 
2019-11-13T20:08:09.4649998Z 7 error[E0308]: mismatched types
2019-11-13T20:08:09.4650300Z 8   --> $DIR/generic_type_does_not_live_long_enough.rs:6:18
2019-11-13T20:08:09.4650300Z 8   --> $DIR/generic_type_does_not_live_long_enough.rs:6:18
2019-11-13T20:08:09.4650346Z 
2019-11-13T20:08:09.4650649Z 
2019-11-13T20:08:09.4650753Z The actual stderr differed from the expected stderr.
2019-11-13T20:08:09.4651183Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/generic_type_does_not_live_long_enough.nll.stderr
2019-11-13T20:08:09.4651949Z To update references, rerun the tests and pass the `--bless` flag
2019-11-13T20:08:09.4652505Z To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`
2019-11-13T20:08:09.4652780Z error: 1 errors occurred comparing output.
2019-11-13T20:08:09.4652872Z status: exit code: 1
2019-11-13T20:08:09.4652872Z status: exit code: 1
2019-11-13T20:08:09.4653969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/auxiliary" "-A" "unused"
2019-11-13T20:08:09.4654934Z ------------------------------------------
2019-11-13T20:08:09.4655096Z 
2019-11-13T20:08:09.4655338Z ------------------------------------------
2019-11-13T20:08:09.4655524Z stderr:
2019-11-13T20:08:09.4655524Z stderr:
2019-11-13T20:08:09.4655756Z ------------------------------------------
2019-11-13T20:08:09.4655938Z error: at least one trait must be specified
2019-11-13T20:08:09.4656251Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:24
2019-11-13T20:08:09.4656505Z    |
2019-11-13T20:08:09.4656864Z LL | type WrongGeneric<T> = impl 'static;
2019-11-13T20:08:09.4657137Z 
2019-11-13T20:08:09.4657186Z error[E0308]: mismatched types
2019-11-13T20:08:09.4657600Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:6:18
2019-11-13T20:08:09.4657804Z    |
2019-11-13T20:08:09.4657804Z    |
2019-11-13T20:08:09.4657891Z LL |     let z: i32 = x; //~ ERROR mismatched types
2019-11-13T20:08:09.4657998Z    |                  ^ expected i32, found opaque type
2019-11-13T20:08:09.4658122Z    = note: expected type `i32`
2019-11-13T20:08:09.4658122Z    = note: expected type `i32`
2019-11-13T20:08:09.4658290Z               found type `WrongGeneric::<&{integer}>`
2019-11-13T20:08:09.4658584Z error: aborting due to 2 previous errors
2019-11-13T20:08:09.4658724Z 
2019-11-13T20:08:09.4662572Z For more information about this error, try `rustc --explain E0308`.
2019-11-13T20:08:09.4662689Z 
---
2019-11-13T20:08:09.4738772Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-13T20:08:09.4738941Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-13T20:08:09.4753119Z 
2019-11-13T20:08:09.4753757Z 
2019-11-13T20:08:09.4765406Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-13T20:08:09.4766543Z 
2019-11-13T20:08:09.4766596Z 
2019-11-13T20:08:09.4770278Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-13T20:08:09.4770581Z Build completed unsuccessfully in 1:30:34
2019-11-13T20:08:09.4770581Z Build completed unsuccessfully in 1:30:34
2019-11-13T20:08:09.4827204Z == clock drift check ==
2019-11-13T20:08:09.4850113Z   local time: Wed Nov 13 20:08:09 UTC 2019
2019-11-13T20:08:09.6237849Z   network time: Wed, 13 Nov 2019 20:08:09 GMT
2019-11-13T20:08:09.6241620Z == end clock drift check ==
2019-11-13T20:08:10.7903743Z 
2019-11-13T20:08:10.8012724Z ##[error]Bash exited with code '1'.
2019-11-13T20:08:10.8048060Z ##[section]Starting: Checkout
2019-11-13T20:08:10.8050056Z ==============================================================================
2019-11-13T20:08:10.8050133Z Task         : Get sources
2019-11-13T20:08:10.8050218Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
