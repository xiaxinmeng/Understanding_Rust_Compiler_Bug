plain
2020-02-19T15:52:37.1933577Z 
2020-02-19T15:52:37.1934169Z ---- [ui] ui/huge-array-simple-32.rs stdout ----
2020-02-19T15:52:37.1934246Z diff of stderr:
2020-02-19T15:52:37.1934456Z 
2020-02-19T15:52:37.1934651Z + warning: lint `exceeding_bitshifts` has been renamed to `arithmetic_overflow`
2020-02-19T15:52:37.1935110Z +   --> $DIR/huge-array-simple-32.rs:7:10
2020-02-19T15:52:37.1935322Z +    |
2020-02-19T15:52:37.1935427Z + LL | #![allow(exceeding_bitshifts)]
2020-02-19T15:52:37.1935522Z +    |          ^^^^^^^^^^^^^^^^^^^ help: use the new name: `arithmetic_overflow`
2020-02-19T15:52:37.1935822Z +    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-02-19T15:52:37.1935911Z + 
2020-02-19T15:52:37.1935911Z + 
2020-02-19T15:52:37.1936029Z 1 error: the type `[u8; 2147516416]` is too big for the current architecture
2020-02-19T15:52:37.1936328Z 2   --> $DIR/huge-array-simple-32.rs:10:9
2020-02-19T15:52:37.1936583Z 
2020-02-19T15:52:37.1936855Z 
2020-02-19T15:52:37.1936949Z The actual stderr differed from the expected stderr.
2020-02-19T15:52:37.1936949Z The actual stderr differed from the expected stderr.
2020-02-19T15:52:37.1937337Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array-simple-32/huge-array-simple-32.stderr
2020-02-19T15:52:37.1938042Z To update references, rerun the tests and pass the `--bless` flag
2020-02-19T15:52:37.1938499Z To only update this specific test, also pass `--test-args huge-array-simple-32.rs`
2020-02-19T15:52:37.1939046Z error: 1 errors occurred comparing output.
2020-02-19T15:52:37.1939422Z status: exit code: 1
2020-02-19T15:52:37.1939422Z status: exit code: 1
2020-02-19T15:52:37.1941050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/huge-array-simple-32.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array-simple-32" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array-simple-32/auxiliary"
2020-02-19T15:52:37.1941599Z ------------------------------------------
2020-02-19T15:52:37.1941651Z 
2020-02-19T15:52:37.1942096Z ------------------------------------------
2020-02-19T15:52:37.1942319Z stderr:
2020-02-19T15:52:37.1942319Z stderr:
2020-02-19T15:52:37.1942642Z ------------------------------------------
2020-02-19T15:52:37.1942752Z warning: lint `exceeding_bitshifts` has been renamed to `arithmetic_overflow`
2020-02-19T15:52:37.1943208Z   --> /checkout/src/test/ui/huge-array-simple-32.rs:7:10
2020-02-19T15:52:37.1943314Z    |
2020-02-19T15:52:37.1943527Z LL | #![allow(exceeding_bitshifts)]
2020-02-19T15:52:37.1943662Z    |          ^^^^^^^^^^^^^^^^^^^ help: use the new name: `arithmetic_overflow`
2020-02-19T15:52:37.1944020Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-02-19T15:52:37.1944094Z 
2020-02-19T15:52:37.1944094Z 
2020-02-19T15:52:37.1944508Z error: the type `[u8; 2147516416]` is too big for the current architecture
2020-02-19T15:52:37.1944960Z    |
2020-02-19T15:52:37.1944960Z    |
2020-02-19T15:52:37.1945183Z LL |     let _fat: [u8; (1<<31)+(1<<15)] = //~ ERROR too big for the current architecture
2020-02-19T15:52:37.1965645Z 
2020-02-19T15:52:37.1969815Z error: aborting due to previous error
2020-02-19T15:52:37.1970068Z 
2020-02-19T15:52:37.1970105Z 
---
2020-02-19T15:52:37.2004085Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-19T15:52:37.2004372Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T15:52:37.2023208Z 
2020-02-19T15:52:37.2023295Z 
2020-02-19T15:52:37.2029118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-19T15:52:37.2029874Z 
2020-02-19T15:52:37.2029913Z 
2020-02-19T15:52:37.2037977Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-02-19T15:52:37.2038285Z Build completed unsuccessfully in 1:16:03
2020-02-19T15:52:37.2038285Z Build completed unsuccessfully in 1:16:03
2020-02-19T15:52:37.2097663Z == clock drift check ==
2020-02-19T15:52:37.2114340Z   local time: Wed Feb 19 15:52:37 UTC 2020
2020-02-19T15:52:37.5075162Z   network time: Wed, 19 Feb 2020 15:52:37 GMT
2020-02-19T15:52:37.5075301Z == end clock drift check ==
2020-02-19T15:52:38.1034007Z 
2020-02-19T15:52:38.1121150Z ##[error]Bash exited with code '1'.
2020-02-19T15:52:38.1168344Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-19T15:52:38.1170485Z ==============================================================================
2020-02-19T15:52:38.1170569Z Task         : Get sources
2020-02-19T15:52:38.1170661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
