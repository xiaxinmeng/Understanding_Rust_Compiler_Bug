plain
2020-03-16T13:20:38.5925791Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-16T13:20:38.5927874Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-16T13:20:38.5938708Z 
2020-03-16T13:20:38.5938949Z 
2020-03-16T13:20:38.5947596Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-16T13:20:38.5950748Z 
2020-03-16T13:20:38.5951040Z 
2020-03-16T13:20:38.5958754Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-16T13:20:38.5959079Z Build completed unsuccessfully in 1:32:00
2020-03-16T13:20:38.5959079Z Build completed unsuccessfully in 1:32:00
2020-03-16T13:20:38.6022898Z == clock drift check ==
2020-03-16T13:20:38.6046394Z   local time: Mon Mar 16 13:20:38 UTC 2020
2020-03-16T13:20:38.8733954Z   network time: Mon, 16 Mar 2020 13:20:38 GMT
2020-03-16T13:20:38.8740928Z == end clock drift check ==
2020-03-16T13:20:39.3346594Z 
2020-03-16T13:20:39.3401413Z ##[error]Bash exited with code '1'.
2020-03-16T13:20:39.3464197Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-16T13:20:39.3469654Z ==============================================================================
2020-03-16T13:20:39.3469964Z Task         : Get sources
2020-03-16T13:20:39.3470726Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
