
2020-02-13T18:58:36.0629460Z + 
2020-02-13T18:58:36.0629460Z + 
2020-02-13T18:58:36.0629633Z + This error occurs when the compiler was unable to infer the concrete type of a
2020-02-13T18:58:36.0629685Z + variable. It can occur for several cases, the most common of which is a
2020-02-13T18:58:36.0629972Z + mismatch in the expected type that the compiler inferred for a variable's
2020-02-13T18:58:36.0630043Z + initializing expression, and the actual type explicitly assigned to the
2020-02-13T18:58:36.0630087Z + variable.
2020-02-13T18:58:36.0631255Z 75 "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":786,"byte_end":794,"line_start":24,"line_end":25,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `std::string::String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":777,"byte_end":783,"line_start":24,"line_end":24,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"$DIR/json-bom-plus-crlf.rs:24:22: error[E0308]: mismatched types
2020-02-13T18:58:36.0631401Z 76 "}
2020-02-13T18:58:36.0631475Z 77 {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors
2020-02-13T18:58:36.0631664Z 
2020-02-13T18:58:36.0631724Z The actual stderr differed from the expected stderr.
2020-02-13T18:58:36.0631724Z The actual stderr differed from the expected stderr.
2020-02-13T18:58:36.0632083Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf/json-bom-plus-crlf.stderr
2020-02-13T18:58:36.0632336Z To update references, rerun the tests and pass the `--bless` flag
2020-02-13T18:58:36.0632610Z To only update this specific test, also pass `--test-args json-bom-plus-crlf.rs`
2020-02-13T18:58:36.0632698Z error: 1 errors occurred comparing output.
2020-02-13T18:58:36.0632741Z status: exit code: 1
2020-02-13T18:58:36.0632741Z status: exit code: 1
2020-02-13T18:58:36.0633568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json-bom-plus-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--json=diagnostic-short" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf/auxiliary" "-A" "unused"
2020-02-13T18:58:36.0633898Z ------------------------------------------
2020-02-13T18:58:36.0633938Z 
2020-02-13T18:58:36.0634154Z ------------------------------------------
2020-02-13T18:58:36.0634471Z stderr:
---
2020-02-13T18:58:36.0637663Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-13T18:58:36.0637721Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-13T18:58:36.0637752Z 
2020-02-13T18:58:36.0637817Z 
2020-02-13T18:58:36.0639339Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-13T18:58:36.0639575Z 
2020-02-13T18:58:36.0639619Z 
2020-02-13T18:58:36.0639663Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-13T18:58:36.0639717Z Build completed unsuccessfully in 1:09:16
2020-02-13T18:58:36.0639717Z Build completed unsuccessfully in 1:09:16
2020-02-13T18:58:36.0656272Z == clock drift check ==
2020-02-13T18:58:36.0676999Z   local time: Thu Feb 13 18:58:36 UTC 2020
2020-02-13T18:58:36.6211523Z   network time: Thu, 13 Feb 2020 18:58:36 GMT
2020-02-13T18:58:36.6211638Z == end clock drift check ==
2020-02-13T18:58:37.1037274Z 
2020-02-13T18:58:37.1157065Z ##[error]Bash exited with code '1'.
2020-02-13T18:58:37.1174095Z ##[section]Finishing: Run build
2020-02-13T18:58:37.1197097Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69139/merge to s
2020-02-13T18:58:37.1199078Z Task         : Get sources
2020-02-13T18:58:37.1199125Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T18:58:37.1199185Z Version      : 1.0.0
2020-02-13T18:58:37.1199226Z Author       : Microsoft
2020-02-13T18:58:37.1199226Z Author       : Microsoft
2020-02-13T18:58:37.1199274Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T18:58:37.1199353Z ==============================================================================
2020-02-13T18:58:37.5753495Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T18:58:37.5796716Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69139/merge to s
2020-02-13T18:58:37.5918713Z Cleaning up task key
2020-02-13T18:58:37.5919832Z Start cleaning up orphan processes.
2020-02-13T18:58:37.6041790Z Terminate orphan process: pid (4184) (python)
2020-02-13T18:58:37.6318805Z ##[section]Finishing: Finalize Job
