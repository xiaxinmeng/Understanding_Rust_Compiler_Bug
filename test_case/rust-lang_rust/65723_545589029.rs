plain
2019-10-23T17:26:22.7521197Z ##[section]Starting: Linux test-various
2019-10-23T17:26:23.3854033Z ##[section]Starting: Initialize job
2019-10-23T17:26:23.3854282Z Agent name: 'Azure Pipelines 56'
2019-10-23T17:26:23.3854404Z Agent machine name: 'fv-az619'
2019-10-23T17:26:23.3854475Z Current agent version: '2.159.2'
2019-10-23T17:26:23.3877528Z Agent running as: 'vsts'
2019-10-23T17:26:23.4288542Z Set build variables.
2019-10-23T17:26:23.4333751Z Download all required tasks.
2019-10-23T17:26:23.4490879Z Downloading task: Bash (3.159.3)
2019-10-23T17:26:24.5410276Z Downloading task: CmdLine (2.151.2)
---
2019-10-23T18:58:56.0475200Z ---- [ui] ui/proc-macro/proc-macro-deprecated-attr.rs stdout ----
2019-10-23T18:58:56.0476092Z 
2019-10-23T18:58:56.0476444Z error: test compilation failed although it shouldn't!
2019-10-23T18:58:56.0476547Z status: exit code: 1
2019-10-23T18:58:56.0478226Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/proc-macro-deprecated-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-deprecated-attr/auxiliary" "-A" "unused"
2019-10-23T18:58:56.0478940Z ------------------------------------------
2019-10-23T18:58:56.0478991Z 
2019-10-23T18:58:56.0479248Z ------------------------------------------
2019-10-23T18:58:56.0479333Z stderr:
2019-10-23T18:58:56.0479333Z stderr:
2019-10-23T18:58:56.0480429Z ------------------------------------------
2019-10-23T18:58:56.0481063Z warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
2019-10-23T18:58:56.0481155Z 
2019-10-23T18:58:56.0481541Z error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
2019-10-23T18:58:56.0481959Z    |
2019-10-23T18:58:56.0482023Z LL | #[proc_macro]
2019-10-23T18:58:56.0482106Z    | ^^^^^^^^^^^^^
2019-10-23T18:58:56.0482149Z 
---
2019-10-23T18:58:56.0545196Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-23T18:58:56.0546429Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-23T18:58:56.0560905Z 
2019-10-23T18:58:56.0561405Z 
2019-10-23T18:58:56.0566118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-23T18:58:56.0567789Z 
2019-10-23T18:58:56.0568016Z 
2019-10-23T18:58:56.0627297Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-10-23T18:58:56.0627764Z Build completed unsuccessfully in 1:23:19
2019-10-23T18:58:56.0627764Z Build completed unsuccessfully in 1:23:19
2019-10-23T18:58:56.0631266Z == clock drift check ==
2019-10-23T18:58:56.0659221Z   local time: Wed Oct 23 18:58:56 UTC 2019
2019-10-23T18:58:56.0928023Z   network time: Wed, 23 Oct 2019 18:58:56 GMT
2019-10-23T18:58:56.0933703Z == end clock drift check ==
2019-10-23T18:58:57.2645265Z 
2019-10-23T18:58:57.2755098Z ##[error]Bash exited with code '1'.
2019-10-23T18:58:57.2798907Z ##[section]Starting: Upload CPU usage statistics
2019-10-23T18:58:57.2823254Z ==============================================================================
2019-10-23T18:58:57.2823368Z Task         : Bash
2019-10-23T18:58:57.2823560Z Description  : Run a Bash script on macOS, Linux, or Windows
