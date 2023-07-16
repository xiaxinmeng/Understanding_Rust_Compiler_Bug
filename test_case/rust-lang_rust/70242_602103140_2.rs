
2020-03-21T20:51:34.3397473Z - 
2020-03-21T20:51:34.3397473Z - 
2020-03-21T20:51:34.3397753Z 77 This error occurs when the compiler was unable to infer the concrete type of a
2020-03-21T20:51:34.3398245Z 78 variable. It can happen in several cases, the most common being a mismatch
2020-03-21T20:51:34.3398649Z 79 between the type that the compiler inferred for a variable based on its
2020-03-21T20:51:34.3398887Z 
2020-03-21T20:51:34.3399170Z 80 initializing expression, on the one hand, and the type the author explicitly
2020-03-21T20:51:34.3420313Z - assigned to the variable, on the other hand."
2020-03-21T20:51:34.3420643Z + assigned to the variable, on the other hand.
2020-03-21T20:51:34.3424678Z 82 "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":801,"byte_end":809,"line_start":25,"line_end":26,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `std::string::String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":792,"byte_end":798,"line_start":25,"line_end":25,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:25:22: error[E0308]: mismatched types
2020-03-21T20:51:34.3427517Z 83 "}
2020-03-21T20:51:34.3428002Z 84 {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors
2020-03-21T20:51:34.3428575Z 
2020-03-21T20:51:34.3428795Z The actual stderr differed from the expected stderr.
2020-03-21T20:51:34.3429565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf-multifile/json-bom-plus-crlf-multifile.stderr
2020-03-21T20:51:34.3429565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf-multifile/json-bom-plus-crlf-multifile.stderr
2020-03-21T20:51:34.3430455Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T20:51:34.3431091Z To only update this specific test, also pass `--test-args json-bom-plus-crlf-multifile.rs`
2020-03-21T20:51:34.3431586Z error: 1 errors occurred comparing output.
2020-03-21T20:51:34.3431839Z status: exit code: 1
2020-03-21T20:51:34.3431839Z status: exit code: 1
2020-03-21T20:51:34.3433991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json-bom-plus-crlf-multifile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf-multifile" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--json=diagnostic-short" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf-multifile/auxiliary"
2020-03-21T20:51:34.3435764Z ------------------------------------------
2020-03-21T20:51:34.3435961Z 
2020-03-21T20:51:34.3436340Z ------------------------------------------
2020-03-21T20:51:34.3436553Z stderr:
---
2020-03-21T20:51:34.3454135Z 80 between the type that the compiler inferred for a variable based on its
2020-03-21T20:51:34.3454372Z 
2020-03-21T20:51:34.3454475Z 
2020-03-21T20:51:34.3454694Z The actual stderr differed from the expected stderr.
2020-03-21T20:51:34.3455416Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf/json-bom-plus-crlf.stderr
2020-03-21T20:51:34.3456079Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T20:51:34.3456678Z To only update this specific test, also pass `--test-args json-bom-plus-crlf.rs`
2020-03-21T20:51:34.3457186Z error: 1 errors occurred comparing output.
2020-03-21T20:51:34.3457437Z status: exit code: 1
2020-03-21T20:51:34.3457437Z status: exit code: 1
2020-03-21T20:51:34.3460340Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json-bom-plus-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--json=diagnostic-short" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json-bom-plus-crlf/auxiliary"
2020-03-21T20:51:34.3462868Z ------------------------------------------
2020-03-21T20:51:34.3463054Z 
2020-03-21T20:51:34.3463435Z ------------------------------------------
2020-03-21T20:51:34.3463663Z stderr:
---
2020-03-21T20:51:34.3471557Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-21T20:51:34.3472003Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-21T20:51:34.3479001Z 
2020-03-21T20:51:34.3479162Z 
2020-03-21T20:51:34.3483584Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-21T20:51:34.3486628Z 
2020-03-21T20:51:34.3486735Z 
2020-03-21T20:51:34.3487284Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-21T20:51:34.3487678Z Build completed unsuccessfully in 1:01:44
2020-03-21T20:51:34.3487678Z Build completed unsuccessfully in 1:01:44
2020-03-21T20:51:34.3516367Z == clock drift check ==
2020-03-21T20:51:34.3543505Z   local time: Sat Mar 21 20:51:34 UTC 2020
2020-03-21T20:51:34.5206013Z   network time: Sat, 21 Mar 2020 20:51:34 GMT
2020-03-21T20:51:34.5211293Z == end clock drift check ==
2020-03-21T20:51:34.8741712Z 
2020-03-21T20:51:34.8820694Z ##[error]Bash exited with code '1'.
2020-03-21T20:51:34.8888093Z ##[section]Finishing: Run build
2020-03-21T20:51:34.8940418Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70242/merge to s
2020-03-21T20:51:34.8945730Z Task         : Get sources
2020-03-21T20:51:34.8946090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T20:51:34.8946439Z Version      : 1.0.0
2020-03-21T20:51:34.8946680Z Author       : Microsoft
2020-03-21T20:51:34.8946680Z Author       : Microsoft
2020-03-21T20:51:34.8947055Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T20:51:34.8947498Z ==============================================================================
2020-03-21T20:51:35.2228525Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T20:51:35.2272432Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70242/merge to s
2020-03-21T20:51:35.2358231Z Cleaning up task key
2020-03-21T20:51:35.2359674Z Start cleaning up orphan processes.
2020-03-21T20:51:35.2537781Z Terminate orphan process: pid (3490) (python)
2020-03-21T20:51:35.2703632Z ##[section]Finishing: Finalize Job
