plain
2020-02-06T15:25:13.8766249Z test [ui] ui/lub-match.rs ... ok
2020-02-06T15:25:14.1607401Z test [ui] ui/lto-still-runs-thread-dtors.rs ... ok
2020-02-06T15:25:14.8176814Z test [ui] ui/macro-quote-cond.rs ... ok
2020-02-06T15:25:14.9048446Z test [ui] ui/macro-quote-test.rs ... ok
2020-02-06T15:25:14.9604302Z test [ui] ui/macro_backtrace/main.rs#-Zmacro-backtrace ... ok
2020-02-06T15:25:15.0311145Z test [ui] ui/macro_backtrace/main.rs#default ... ok
2020-02-06T15:25:15.2966505Z test [ui] ui/macros/assert-eq-macro-unsized.rs ... ok
2020-02-06T15:25:15.3221390Z test [ui] ui/macros/assert-eq-macro-success.rs ... ok
2020-02-06T15:25:15.5772498Z test [ui] ui/macros/assert-ne-macro-unsized.rs ... ok
---
2020-02-06T15:36:42.9017803Z test [ui (nll)] ui/lub-match.rs ... ok
2020-02-06T15:36:43.0715964Z test [ui (nll)] ui/lto-still-runs-thread-dtors.rs ... ok
2020-02-06T15:36:43.8310443Z test [ui (nll)] ui/macro-quote-test.rs ... ok
2020-02-06T15:36:43.8364223Z test [ui (nll)] ui/macro-quote-cond.rs ... ok
2020-02-06T15:36:43.9530512Z test [ui (nll)] ui/macro_backtrace/main.rs#default ... ok
2020-02-06T15:36:43.9537479Z test [ui (nll)] ui/macro_backtrace/main.rs#-Zmacro-backtrace ... ok
2020-02-06T15:36:44.2464914Z test [ui (nll)] ui/macros/assert-eq-macro-unsized.rs ... ok
2020-02-06T15:36:44.2686603Z test [ui (nll)] ui/macros/assert-eq-macro-success.rs ... ok
2020-02-06T15:36:44.5241267Z test [ui (nll)] ui/macros/assert-ne-macro-unsized.rs ... ok
2020-02-06T15:36:44.5430865Z test [ui (nll)] ui/macros/assert-ne-macro-success.rs ... ok
---
2020-02-06T15:42:05.2727096Z diff of stderr:
2020-02-06T15:42:05.2727142Z 
2020-02-06T15:42:05.2727478Z 13    | |__________________________________________________- in this macro invocation
2020-02-06T15:42:05.2727569Z 14    |
2020-02-06T15:42:05.2727860Z 15    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2728420Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2728630Z 17 error: lifetime may not live long enough
2020-02-06T15:42:05.2728894Z 18   --> $DIR/hr-subtype.rs:39:13
2020-02-06T15:42:05.2728960Z 
2020-02-06T15:42:05.2729254Z 29    | |__________________________________________________- in this macro invocation
2020-02-06T15:42:05.2729254Z 29    | |__________________________________________________- in this macro invocation
2020-02-06T15:42:05.2729356Z 30    |
2020-02-06T15:42:05.2729619Z 31    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2729968Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2730147Z 33 error: aborting due to 2 previous errors
2020-02-06T15:42:05.2730219Z 34 
2020-02-06T15:42:05.2730270Z 
2020-02-06T15:42:05.2730309Z 
2020-02-06T15:42:05.2730309Z 
2020-02-06T15:42:05.2730395Z The actual stderr differed from the expected stderr.
2020-02-06T15:42:05.2730828Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll/hr-subtype.free_inv_x_vs_free_inv_y.nll.stderr
2020-02-06T15:42:05.2731175Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T15:42:05.2731486Z To only update this specific test, also pass `--test-args hr-subtype/hr-subtype.rs`
2020-02-06T15:42:05.2731565Z 
2020-02-06T15:42:05.2731646Z error in revision `free_inv_x_vs_free_inv_y`: 1 errors occurred comparing output.
2020-02-06T15:42:05.2731749Z status: exit code: 1
2020-02-06T15:42:05.2732890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "free_inv_x_vs_free_inv_y" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll/auxiliary" "-A" "unused"
2020-02-06T15:42:05.2733559Z ------------------------------------------
2020-02-06T15:42:05.2733612Z 
2020-02-06T15:42:05.2733868Z ------------------------------------------
2020-02-06T15:42:05.2733944Z stderr:
2020-02-06T15:42:05.2733944Z stderr:
2020-02-06T15:42:05.2734197Z ------------------------------------------
2020-02-06T15:42:05.2734291Z error: lifetime may not live long enough
2020-02-06T15:42:05.2734566Z   --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:33:13
2020-02-06T15:42:05.2734663Z    |
2020-02-06T15:42:05.2734911Z LL |           fn subtype<'x,'y:'x,'z:'y>() {
2020-02-06T15:42:05.2735198Z    |                      -- -- lifetime `'y` defined here
2020-02-06T15:42:05.2735556Z    |                      lifetime `'x` defined here
2020-02-06T15:42:05.2735556Z    |                      lifetime `'x` defined here
2020-02-06T15:42:05.2735656Z LL |               gimme::<$t2>(None::<$t1>);
2020-02-06T15:42:05.2735983Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'x` must outlive `'y`
2020-02-06T15:42:05.2736088Z ...
2020-02-06T15:42:05.2736348Z LL | / check! { free_inv_x_vs_free_inv_y: (fn(Inv<'x>),
2020-02-06T15:42:05.2736637Z LL | |                                     fn(Inv<'y>)) }
2020-02-06T15:42:05.2737044Z    |
2020-02-06T15:42:05.2737044Z    |
2020-02-06T15:42:05.2737303Z    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2737652Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2737803Z error: lifetime may not live long enough
2020-02-06T15:42:05.2738078Z   --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:39:13
2020-02-06T15:42:05.2738170Z    |
2020-02-06T15:42:05.2738170Z    |
2020-02-06T15:42:05.2738478Z LL |           fn supertype<'x,'y:'x,'z:'y>() {
2020-02-06T15:42:05.2738791Z    |                        -- -- lifetime `'y` defined here
2020-02-06T15:42:05.2739151Z    |                        lifetime `'x` defined here
2020-02-06T15:42:05.2739151Z    |                        lifetime `'x` defined here
2020-02-06T15:42:05.2739252Z LL |               gimme::<$t1>(None::<$t2>);
2020-02-06T15:42:05.2739559Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'x` must outlive `'y`
2020-02-06T15:42:05.2739663Z ...
2020-02-06T15:42:05.2739919Z LL | / check! { free_inv_x_vs_free_inv_y: (fn(Inv<'x>),
2020-02-06T15:42:05.2740206Z LL | |                                     fn(Inv<'y>)) }
2020-02-06T15:42:05.2740607Z    |
2020-02-06T15:42:05.2740607Z    |
2020-02-06T15:42:05.2740864Z    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2741210Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2741378Z error: aborting due to 2 previous errors
2020-02-06T15:42:05.2741424Z 
2020-02-06T15:42:05.2741462Z 
2020-02-06T15:42:05.2741718Z ------------------------------------------
2020-02-06T15:42:05.2741718Z ------------------------------------------
2020-02-06T15:42:05.2741767Z 
2020-02-06T15:42:05.2741817Z 
2020-02-06T15:42:05.2742095Z ---- [ui (nll)] ui/hr-subtype/hr-subtype.rs#free_x_vs_free_y stdout ----
2020-02-06T15:42:05.2742199Z diff of stderr:
2020-02-06T15:42:05.2742242Z 
2020-02-06T15:42:05.2742524Z 13    | |__________________________________________- in this macro invocation
2020-02-06T15:42:05.2742626Z 14    |
2020-02-06T15:42:05.2742903Z 15    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2743239Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2743416Z 17 error: aborting due to previous error
2020-02-06T15:42:05.2743504Z 18 
2020-02-06T15:42:05.2743542Z 
2020-02-06T15:42:05.2743643Z 
2020-02-06T15:42:05.2743643Z 
2020-02-06T15:42:05.2743731Z The actual stderr differed from the expected stderr.
2020-02-06T15:42:05.2744143Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_x_vs_free_y.nll/hr-subtype.free_x_vs_free_y.nll.stderr
2020-02-06T15:42:05.2744476Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T15:42:05.2744799Z To only update this specific test, also pass `--test-args hr-subtype/hr-subtype.rs`
2020-02-06T15:42:05.2744862Z 
2020-02-06T15:42:05.2744954Z error in revision `free_x_vs_free_y`: 1 errors occurred comparing output.
2020-02-06T15:42:05.2745039Z status: exit code: 1
2020-02-06T15:42:05.2746140Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "free_x_vs_free_y" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_x_vs_free_y.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_x_vs_free_y.nll/auxiliary" "-A" "unused"
2020-02-06T15:42:05.2746694Z ------------------------------------------
2020-02-06T15:42:05.2746761Z 
2020-02-06T15:42:05.2747001Z ------------------------------------------
2020-02-06T15:42:05.2747094Z stderr:
2020-02-06T15:42:05.2747094Z stderr:
2020-02-06T15:42:05.2747333Z ------------------------------------------
2020-02-06T15:42:05.2747428Z error: lifetime may not live long enough
2020-02-06T15:42:05.2747701Z   --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:39:13
2020-02-06T15:42:05.2747795Z    |
2020-02-06T15:42:05.2749222Z LL |           fn supertype<'x,'y:'x,'z:'y>() {
2020-02-06T15:42:05.2749643Z    |                        -- -- lifetime `'y` defined here
2020-02-06T15:42:05.2750029Z    |                        lifetime `'x` defined here
2020-02-06T15:42:05.2750029Z    |                        lifetime `'x` defined here
2020-02-06T15:42:05.2750131Z LL |               gimme::<$t1>(None::<$t2>);
2020-02-06T15:42:05.2750441Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'x` must outlive `'y`
2020-02-06T15:42:05.2750550Z ...
2020-02-06T15:42:05.2750803Z LL | / check! { free_x_vs_free_y: (fn(&'x u32),
2020-02-06T15:42:05.2751081Z LL | |                             fn(&'y u32)) }
2020-02-06T15:42:05.2751472Z    |
2020-02-06T15:42:05.2751472Z    |
2020-02-06T15:42:05.2751741Z    = help: consider adding the following bound: `'x: 'y`
2020-02-06T15:42:05.2752072Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-06T15:42:05.2752228Z error: aborting due to previous error
2020-02-06T15:42:05.2752290Z 
2020-02-06T15:42:05.2752343Z 
2020-02-06T15:42:05.2752583Z ------------------------------------------
---
2020-02-06T15:42:05.2766500Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-06T15:42:05.2766698Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-06T15:42:05.2785702Z 
2020-02-06T15:42:05.2785809Z 
2020-02-06T15:42:05.2787970Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-02-06T15:42:05.2789250Z 
2020-02-06T15:42:05.2789291Z 
2020-02-06T15:42:05.2794763Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-06T15:42:05.2794871Z Build completed unsuccessfully in 1:43:14
2020-02-06T15:42:05.2794871Z Build completed unsuccessfully in 1:43:14
2020-02-06T15:42:05.2847255Z == clock drift check ==
2020-02-06T15:42:05.2862671Z   local time: Thu Feb  6 15:42:05 UTC 2020
2020-02-06T15:42:05.4505342Z   network time: Thu, 06 Feb 2020 15:42:05 GMT
2020-02-06T15:42:05.4509278Z == end clock drift check ==
2020-02-06T15:42:05.9168577Z 
2020-02-06T15:42:05.9273974Z ##[error]Bash exited with code '1'.
2020-02-06T15:42:05.9321437Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-06T15:42:05.9323761Z ==============================================================================
2020-02-06T15:42:05.9323859Z Task         : Get sources
2020-02-06T15:42:05.9323969Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
