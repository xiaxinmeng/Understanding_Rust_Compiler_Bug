plain
2020-02-08T03:29:18.7598031Z 
2020-02-08T03:29:18.7598951Z ---- [ui] ui/feature-gates/feature-gate-no_sanitize.rs stdout ----
2020-02-08T03:29:18.7599332Z diff of stderr:
2020-02-08T03:29:18.7599489Z 
2020-02-08T03:29:18.7599628Z 4 LL | #[no_sanitize(address)]
2020-02-08T03:29:18.7599909Z 6    |
2020-02-08T03:29:18.7600369Z -    = note: for more information, see https://github.com/rust-lang/rust/issues/39699
2020-02-08T03:29:18.7600824Z +    = note: see issue #39699 <https://github.com/rust-lang/rust/issues/39699> for more information
2020-02-08T03:29:18.7600824Z +    = note: see issue #39699 <https://github.com/rust-lang/rust/issues/39699> for more information
2020-02-08T03:29:18.7601065Z 8    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-08T03:29:18.7601363Z 10 error: aborting due to previous error
2020-02-08T03:29:18.7601492Z 
2020-02-08T03:29:18.7601595Z 
2020-02-08T03:29:18.7601746Z The actual stderr differed from the expected stderr.
2020-02-08T03:29:18.7601746Z The actual stderr differed from the expected stderr.
2020-02-08T03:29:18.7602194Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/feature-gate-no_sanitize.stderr
2020-02-08T03:29:18.7602892Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T03:29:18.7603679Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-no_sanitize.rs`
2020-02-08T03:29:18.7604192Z error: 1 errors occurred comparing output.
2020-02-08T03:29:18.7604393Z status: exit code: 1
2020-02-08T03:29:18.7604393Z status: exit code: 1
2020-02-08T03:29:18.7605447Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/auxiliary" "-A" "unused"
2020-02-08T03:29:18.7606388Z ------------------------------------------
2020-02-08T03:29:18.7606578Z 
2020-02-08T03:29:18.7606911Z ------------------------------------------
2020-02-08T03:29:18.7607114Z stderr:
2020-02-08T03:29:18.7607114Z stderr:
2020-02-08T03:29:18.7607870Z ------------------------------------------
2020-02-08T03:29:18.7608458Z error[E0658]: the `#[no_sanitize]` attribute is an experimental feature
2020-02-08T03:29:18.7609055Z   --> /checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs:1:1
2020-02-08T03:29:18.7609298Z    |
2020-02-08T03:29:18.7609541Z LL | #[no_sanitize(address)]
2020-02-08T03:29:18.7609920Z    |
2020-02-08T03:29:18.7610432Z    = note: see issue #39699 <https://github.com/rust-lang/rust/issues/39699> for more information
2020-02-08T03:29:18.7610432Z    = note: see issue #39699 <https://github.com/rust-lang/rust/issues/39699> for more information
2020-02-08T03:29:18.7610716Z    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-08T03:29:18.7611045Z error: aborting due to previous error
2020-02-08T03:29:18.7611519Z 
2020-02-08T03:29:18.7612226Z For more information about this error, try `rustc --explain E0658`.
2020-02-08T03:29:18.7612392Z 
---
2020-02-08T03:29:18.7625143Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-08T03:29:18.7625504Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-08T03:29:18.7638614Z 
2020-02-08T03:29:18.7638782Z 
2020-02-08T03:29:18.7640906Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-08T03:29:18.7642124Z 
2020-02-08T03:29:18.7642239Z 
2020-02-08T03:29:18.7650537Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-08T03:29:18.7650833Z Build completed unsuccessfully in 0:58:29
2020-02-08T03:29:18.7650833Z Build completed unsuccessfully in 0:58:29
2020-02-08T03:29:18.7702297Z == clock drift check ==
2020-02-08T03:29:18.7732293Z   local time: Sat Feb  8 03:29:18 UTC 2020
2020-02-08T03:29:19.0743918Z   network time: Sat, 08 Feb 2020 03:29:19 GMT
2020-02-08T03:29:19.0744949Z == end clock drift check ==
2020-02-08T03:29:19.5637472Z 
2020-02-08T03:29:19.5718709Z ##[error]Bash exited with code '1'.
2020-02-08T03:29:19.5767873Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-08T03:29:19.5769686Z ==============================================================================
2020-02-08T03:29:19.5769759Z Task         : Get sources
2020-02-08T03:29:19.5769842Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
