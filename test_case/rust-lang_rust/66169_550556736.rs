plain
2019-11-06T23:58:17.2348202Z test time::tests::since_epoch ... ok
2019-11-06T23:58:17.2348269Z 
2019-11-06T23:58:17.2348616Z failures:
2019-11-06T23:58:17.2348665Z 
2019-11-06T23:58:17.2349458Z ---- process::tests::test_process_output_error stdout ----
2019-11-06T23:58:17.2349835Z thread '<unnamed>' panicked at 'assertion failed: status.code() == Some(1)', src/libstd/process.rs:1820:9
2019-11-06T23:58:17.2349977Z 
2019-11-06T23:58:17.2350059Z failures:
2019-11-06T23:58:17.2350129Z     process::tests::test_process_output_error
2019-11-06T23:58:17.2350197Z 
2019-11-06T23:58:17.2350197Z 
2019-11-06T23:58:17.2350278Z test result: FAILED. 748 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
2019-11-06T23:58:17.2350340Z 
2019-11-06T23:58:17.2350659Z error: test failed, to rerun pass '-p std --lib'
2019-11-06T23:58:17.2363093Z 
2019-11-06T23:58:17.2363939Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "arm-linux-androideabi" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--"
2019-11-06T23:58:17.2364169Z expected success, got: exit code: 101
2019-11-06T23:58:17.2364221Z 
---
2019-11-06T23:58:17.2448816Z   local time: Wed Nov  6 23:58:17 UTC 2019
2019-11-06T23:58:17.5251493Z   network time: Wed, 06 Nov 2019 23:58:17 GMT
2019-11-06T23:58:17.5257566Z == end clock drift check ==
2019-11-06T23:58:18.3765471Z 
2019-11-06T23:58:18.3876135Z ##[error]Bash exited with code '1'.
2019-11-06T23:58:18.3917811Z ##[section]Starting: Checkout
2019-11-06T23:58:18.3920142Z ==============================================================================
2019-11-06T23:58:18.3920258Z Task         : Get sources
2019-11-06T23:58:18.3920345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
