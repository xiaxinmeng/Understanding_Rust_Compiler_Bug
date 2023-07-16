plain
2020-03-11T09:12:14.7843190Z test [ui] ui/allocator/xcrate-use.rs ... ok
2020-03-11T09:12:14.8160375Z test [ui] ui/annotate-snippet/missing-type.rs ... ok
2020-03-11T09:12:14.8466465Z test [ui] ui/anon-params/anon-params-denied-2018.rs ... ok
2020-03-11T09:12:14.9261073Z test [ui] ui/allocator/xcrate-use2.rs ... ok
2020-03-11T09:12:15.0000888Z test [ui] ui/anon-params/anon-params-edition-hygiene.rs ... ok
2020-03-11T09:12:15.0056492Z test [ui] ui/anon-params/anon-params-deprecated.rs ... ok
2020-03-11T09:12:15.0409847Z test [ui] ui/anonymous-higher-ranked-lifetime.rs ... ok
2020-03-11T09:12:15.0753610Z test [ui] ui/arg-type-mismatch.rs ... ok
2020-03-11T09:12:15.1591550Z test [ui] ui/array-break-length.rs ... ok
2020-03-11T09:12:15.2331006Z test [ui] ui/array-not-vector.rs ... ok
---
2020-03-11T09:22:47.0035874Z 
2020-03-11T09:22:47.0036815Z ---- [ui] ui/lint/issue-69485-var-size-diffs-too-large.rs stdout ----
2020-03-11T09:22:47.0037334Z diff of stderr:
2020-03-11T09:22:47.0037608Z 
2020-03-11T09:22:47.0038293Z - error: the type `[u8; 18446744073709551615]` is too big for the current architecture
2020-03-11T09:22:47.0038866Z + error: the type `[u8; 4294967295]` is too big for the current architecture
2020-03-11T09:22:47.0039588Z 2   --> $DIR/issue-69485-var-size-diffs-too-large.rs:4:12
2020-03-11T09:22:47.0040037Z 3    |
2020-03-11T09:22:47.0040381Z 4 LL |     Bug::V([0; !0]);
2020-03-11T09:22:47.0040940Z 
2020-03-11T09:22:47.0041299Z The actual stderr differed from the expected stderr.
2020-03-11T09:22:47.0041299Z The actual stderr differed from the expected stderr.
2020-03-11T09:22:47.0042165Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-69485-var-size-diffs-too-large/issue-69485-var-size-diffs-too-large.stderr
2020-03-11T09:22:47.0043041Z To update references, rerun the tests and pass the `--bless` flag
2020-03-11T09:22:47.0044158Z To only update this specific test, also pass `--test-args lint/issue-69485-var-size-diffs-too-large.rs`
2020-03-11T09:22:47.0045547Z error: 1 errors occurred comparing output.
2020-03-11T09:22:47.0046204Z status: exit code: 1
2020-03-11T09:22:47.0046204Z status: exit code: 1
2020-03-11T09:22:47.0048355Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-69485-var-size-diffs-too-large.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-69485-var-size-diffs-too-large" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-69485-var-size-diffs-too-large/auxiliary"
2020-03-11T09:22:47.0050767Z ------------------------------------------
2020-03-11T09:22:47.0051416Z 
2020-03-11T09:22:47.0052233Z ------------------------------------------
2020-03-11T09:22:47.0052894Z stderr:
2020-03-11T09:22:47.0052894Z stderr:
2020-03-11T09:22:47.0054063Z ------------------------------------------
2020-03-11T09:22:47.0055025Z error: the type `[u8; 4294967295]` is too big for the current architecture
2020-03-11T09:22:47.0056152Z   --> /checkout/src/test/ui/lint/issue-69485-var-size-diffs-too-large.rs:4:12
2020-03-11T09:22:47.0056986Z    |
2020-03-11T09:22:47.0059289Z LL |     Bug::V([0; !0]); //~ ERROR is too big for the current
2020-03-11T09:22:47.0060989Z 
2020-03-11T09:22:47.0061832Z error: aborting due to previous error
2020-03-11T09:22:47.0062514Z 
2020-03-11T09:22:47.0063046Z 
---
2020-03-11T09:22:47.0160669Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-11T09:22:47.0161103Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-11T09:22:47.0161325Z 
2020-03-11T09:22:47.0161415Z 
2020-03-11T09:22:47.0165301Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-11T09:22:47.0170773Z 
2020-03-11T09:22:47.0170870Z 
2020-03-11T09:22:47.0171717Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-11T09:22:47.0182979Z Build completed unsuccessfully in 1:18:09
2020-03-11T09:22:47.0182979Z Build completed unsuccessfully in 1:18:09
2020-03-11T09:22:47.0183275Z == clock drift check ==
2020-03-11T09:22:47.0183552Z   local time: Wed Mar 11 09:22:47 UTC 2020
2020-03-11T09:22:47.2915556Z   network time: Wed, 11 Mar 2020 09:22:47 GMT
2020-03-11T09:22:47.2916600Z == end clock drift check ==
2020-03-11T09:22:48.5912328Z 
2020-03-11T09:22:48.5955553Z ##[error]Bash exited with code '1'.
2020-03-11T09:22:48.6047817Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-11T09:22:48.6051755Z ==============================================================================
2020-03-11T09:22:48.6052040Z Task         : Get sources
2020-03-11T09:22:48.6052349Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
