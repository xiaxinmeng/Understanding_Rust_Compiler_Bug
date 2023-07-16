
2019-07-17T20:35:50.6648259Z    |
2019-07-17T20:35:50.6648314Z note: lint level defined here
2019-07-17T20:35:50.6648606Z   --> /checkout/src/test/rustdoc-ui/lint-group.rs:7:9
2019-07-17T20:35:50.6648658Z    |
2019-07-17T20:35:50.6648658Z    |
2019-07-17T20:35:50.6648701Z LL | #![deny(rustdoc)]
2019-07-17T20:35:50.6648762Z    |         ^^^^^^^
2019-07-17T20:35:50.6649622Z    = note: `#[deny(private_doc_tests)]` implied by `#[deny(rustdoc)]`
2019-07-17T20:35:50.6649665Z 
2019-07-17T20:35:50.6649737Z error: `[error]` cannot be resolved, ignoring it...
2019-07-17T20:35:50.6650382Z    |
2019-07-17T20:35:50.6650382Z    |
2019-07-17T20:35:50.6650658Z LL | /// what up, let's make an [error]
2019-07-17T20:35:50.6650760Z    |
2019-07-17T20:35:50.6650803Z note: lint level defined here
2019-07-17T20:35:50.6651094Z   --> /checkout/src/test/rustdoc-ui/lint-group.rs:7:9
2019-07-17T20:35:50.6651145Z    |
2019-07-17T20:35:50.6651145Z    |
2019-07-17T20:35:50.6651187Z LL | #![deny(rustdoc)]
2019-07-17T20:35:50.6651242Z    |         ^^^^^^^
2019-07-17T20:35:50.6651303Z    = note: `#[deny(intra_doc_link_resolution_failure)]` implied by `#[deny(rustdoc)]`
2019-07-17T20:35:50.6651652Z 
2019-07-17T20:35:50.6651652Z 
2019-07-17T20:35:50.6651713Z error: Missing code example in this documentation
2019-07-17T20:35:50.6652040Z    |
2019-07-17T20:35:50.6652040Z    |
2019-07-17T20:35:50.6652630Z LL | /// wait, this doesn't have a doctest?
2019-07-17T20:35:50.6652739Z    |
2019-07-17T20:35:50.6652795Z note: lint level defined here
2019-07-17T20:35:50.6653059Z   --> /checkout/src/test/rustdoc-ui/lint-group.rs:7:9
2019-07-17T20:35:50.6653107Z    |
2019-07-17T20:35:50.6653107Z    |
2019-07-17T20:35:50.6653147Z LL | #![deny(rustdoc)]
2019-07-17T20:35:50.6653200Z    |         ^^^^^^^
2019-07-17T20:35:50.6653250Z    = note: `#[deny(missing_doc_code_examples)]` implied by `#[deny(rustdoc)]`
2019-07-17T20:35:50.6653486Z error: aborting due to 3 previous errors
2019-07-17T20:35:50.6653516Z 
2019-07-17T20:35:50.6653540Z 
2019-07-17T20:35:50.6653824Z ------------------------------------------
---
2019-07-17T20:35:50.6655018Z test result: FAILED. 27 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-07-17T20:35:50.6655066Z 
2019-07-17T20:35:50.6655096Z 
2019-07-17T20:35:50.6655120Z 
2019-07-17T20:35:50.6656700Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-17T20:35:50.6657093Z 
2019-07-17T20:35:50.6657121Z 
2019-07-17T20:35:50.6657514Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-17T20:35:50.6657578Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-17T20:35:50.6657578Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-17T20:35:50.6657659Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-17T20:35:50.6657707Z Build completed unsuccessfully in 1:51:46
2019-07-17T20:35:51.9138782Z ##[error]Bash exited with code '1'.
2019-07-17T20:35:51.9177403Z ##[section]Starting: Checkout
2019-07-17T20:35:51.9179213Z ==============================================================================
2019-07-17T20:35:51.9179275Z Task         : Get sources
2019-07-17T20:35:51.9179345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
