plain
2019-11-03T05:17:03.8943252Z ---- [ui (nll)] ui/continue-after-missing-main.rs stdout ----
2019-11-03T05:17:03.8943361Z diff of stderr:
2019-11-03T05:17:03.8943403Z 
2019-11-03T05:17:03.8943476Z 3    |
2019-11-03T05:17:03.8943530Z 4 LL | / #![allow(dead_code)]
2019-11-03T05:17:03.8943609Z 5 LL | |
2019-11-03T05:17:03.8943851Z - LL | | // error-pattern:`main` function not found in crate
2019-11-03T05:17:03.8944070Z - LL | |
2019-11-03T05:17:03.8944279Z + LL | | struct Tableau<'a, MP> {
2019-11-03T05:17:03.8944479Z + LL | |     provider: &'a MP,
2019-11-03T05:17:03.8944610Z 9 LL | |
2019-11-03T05:17:03.8944679Z 10 LL | | }
2019-11-03T05:17:03.8944743Z 
2019-11-03T05:17:03.8944773Z 
2019-11-03T05:17:03.8944773Z 
2019-11-03T05:17:03.8944861Z The actual stderr differed from the expected stderr.
2019-11-03T05:17:03.8945197Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/continue-after-missing-main.nll.stderr
2019-11-03T05:17:03.8945495Z To update references, rerun the tests and pass the `--bless` flag
2019-11-03T05:17:03.8945776Z To only update this specific test, also pass `--test-args continue-after-missing-main.rs`
2019-11-03T05:17:03.8945903Z error: 1 errors occurred comparing output.
2019-11-03T05:17:03.8945966Z status: exit code: 1
2019-11-03T05:17:03.8945966Z status: exit code: 1
2019-11-03T05:17:03.8946771Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/continue-after-missing-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/auxiliary" "-A" "unused"
2019-11-03T05:17:03.8947256Z ------------------------------------------
2019-11-03T05:17:03.8947320Z 
2019-11-03T05:17:03.8947545Z ------------------------------------------
2019-11-03T05:17:03.8947628Z stderr:
2019-11-03T05:17:03.8947628Z stderr:
2019-11-03T05:17:03.8947847Z ------------------------------------------
2019-11-03T05:17:03.8947945Z error[E0601]: `main` function not found in crate `continue_after_missing_main`
2019-11-03T05:17:03.8948297Z    |
2019-11-03T05:17:03.8948297Z    |
2019-11-03T05:17:03.8948383Z LL | / #![allow(dead_code)] //~ ERROR `main` function not found in crate
2019-11-03T05:17:03.8948455Z LL | |
2019-11-03T05:17:03.8949566Z LL | | struct Tableau<'a, MP> {
2019-11-03T05:17:03.8949893Z LL | |     provider: &'a MP,
2019-11-03T05:17:03.8951045Z ...  |
2019-11-03T05:17:03.8951119Z LL | |     //~^ ERROR lifetime mismatch
2019-11-03T05:17:03.8951213Z LL | | }
2019-11-03T05:17:03.8951648Z    | |_^ consider adding a `main` function to `/checkout/src/test/ui/continue-after-missing-main.rs`
2019-11-03T05:17:03.8951792Z error: aborting due to previous error
2019-11-03T05:17:03.8951859Z 
2019-11-03T05:17:03.8952141Z For more information about this error, try `rustc --explain E0601`.
2019-11-03T05:17:03.8952220Z 
---
2019-11-03T05:17:03.8977419Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-03T05:17:03.8977801Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-03T05:17:03.8995595Z 
2019-11-03T05:17:03.9000940Z 
2019-11-03T05:17:03.9003361Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-03T05:17:03.9003961Z 
2019-11-03T05:17:03.9003995Z 
2019-11-03T05:17:03.9006741Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-03T05:17:03.9007362Z Build completed unsuccessfully in 1:45:06
2019-11-03T05:17:03.9007362Z Build completed unsuccessfully in 1:45:06
2019-11-03T05:17:03.9061430Z == clock drift check ==
2019-11-03T05:17:03.9076086Z   local time: Sun Nov  3 05:17:03 UTC 2019
2019-11-03T05:17:04.1912082Z   network time: Sun, 03 Nov 2019 05:17:04 GMT
2019-11-03T05:17:04.1912453Z == end clock drift check ==
2019-11-03T05:17:05.5441498Z 
2019-11-03T05:17:05.5531574Z ##[error]Bash exited with code '1'.
2019-11-03T05:17:05.5577019Z ##[section]Starting: Checkout
2019-11-03T05:17:05.5579579Z ==============================================================================
2019-11-03T05:17:05.5579680Z Task         : Get sources
2019-11-03T05:17:05.5579787Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
