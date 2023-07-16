plain
2020-02-23T23:33:53.0038584Z 10 
2020-02-23T23:33:53.0038756Z 
2020-02-23T23:33:53.0038910Z 
2020-02-23T23:33:53.0039172Z The actual stderr differed from the expected stderr.
2020-02-23T23:33:53.0039815Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158/async-generator-issue-67158.stderr
2020-02-23T23:33:53.0040443Z To update references, rerun the tests and pass the `--bless` flag
2020-02-23T23:33:53.0041028Z To only update this specific test, also pass `--test-args generator/async-generator-issue-67158.rs`
2020-02-23T23:33:53.0041567Z error: 1 errors occurred comparing output.
2020-02-23T23:33:53.0041852Z status: exit code: 1
2020-02-23T23:33:53.0041852Z status: exit code: 1
2020-02-23T23:33:53.0043673Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/async-generator-issue-67158.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158/auxiliary" "-A" "unused"
2020-02-23T23:33:53.0045157Z ------------------------------------------
2020-02-23T23:33:53.0045375Z 
2020-02-23T23:33:53.0045744Z ------------------------------------------
2020-02-23T23:33:53.0045990Z stderr:
2020-02-23T23:33:53.0045990Z stderr:
2020-02-23T23:33:53.0046362Z ------------------------------------------
2020-02-23T23:33:53.0046681Z error[E0727]: `async` generators are not yet supported
2020-02-23T23:33:53.0047203Z   --> /checkout/src/test/ui/generator/async-generator-issue-67158.rs:5:13
2020-02-23T23:33:53.0047517Z    |
2020-02-23T23:33:53.0047901Z LL |     async { yield print!(":C") }; //~ ERROR `async` generators are not yet supported
2020-02-23T23:33:53.0048488Z 
2020-02-23T23:33:53.0048722Z error: aborting due to previous error
2020-02-23T23:33:53.0048925Z 
2020-02-23T23:33:53.0049081Z 
---
2020-02-23T23:33:53.0062886Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-02-23T23:33:53.0063602Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-23T23:33:53.0077307Z 
2020-02-23T23:33:53.0077532Z 
2020-02-23T23:33:53.0081011Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-23T23:33:53.0083043Z 
2020-02-23T23:33:53.0083126Z 
2020-02-23T23:33:53.0085255Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-23T23:33:53.0085680Z Build completed unsuccessfully in 0:43:01
2020-02-23T23:33:53.0085680Z Build completed unsuccessfully in 0:43:01
2020-02-23T23:33:53.0129612Z == clock drift check ==
2020-02-23T23:33:53.0146530Z   local time: Sun Feb 23 23:33:53 UTC 2020
2020-02-23T23:33:53.0499498Z   network time: Sun, 23 Feb 2020 23:33:53 GMT
2020-02-23T23:33:53.0500570Z == end clock drift check ==
2020-02-23T23:33:53.5763971Z 
2020-02-23T23:33:53.5826658Z ##[error]Bash exited with code '1'.
2020-02-23T23:33:53.5887401Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-23T23:33:53.5892041Z ==============================================================================
2020-02-23T23:33:53.5892409Z Task         : Get sources
2020-02-23T23:33:53.5892808Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
