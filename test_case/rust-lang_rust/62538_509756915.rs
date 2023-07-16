plain
2019-07-09T16:55:43.7546801Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-09T16:55:43.7546854Z 
2019-07-09T16:55:43.7547081Z   git checkout -b <new-branch-name>
2019-07-09T16:55:43.7547151Z 
2019-07-09T16:55:43.7547434Z HEAD is now at 043bb8c1b Auto merge of #62538 - Centril:rollup-qdenlwf, r=Centril
2019-07-09T16:55:43.7609872Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-09T16:55:43.7613106Z ==============================================================================
2019-07-09T16:55:43.7613197Z Task         : Bash
2019-07-09T16:55:43.7613307Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-09T18:28:08.0909711Z test [ui] ui/type-alias-enum-variants/issue-57866.rs ... ok
2019-07-09T18:28:08.1398354Z test [ui] ui/type-alias-enum-variants/no-type-application-on-aliased-enum-variant.rs ... ok
2019-07-09T18:28:08.1579383Z test [ui] ui/type-alias-enum-variants/issue-61801-path-pattern-can-infer.rs ... ok
2019-07-09T18:28:08.1822814Z test [ui] ui/type-alias-enum-variants/resolve-to-enum-variant-in-type-namespace-and-error.rs ... ok
2019-07-09T18:28:08.2257946Z test [ui] ui/type-alias/issue-62263-self-in-atb.rs ... ok
2019-07-09T18:28:08.2680860Z test [ui] ui/type-alias/issue-62305-self-assoc-ty.rs ... ok
2019-07-09T18:28:08.3134776Z test [ui] ui/type-alias/issue-62364-self-ty-arg.rs ... ok
2019-07-09T18:28:08.4745575Z test [ui] ui/type-alias-enum-variants/type-alias-enum-variants-pass.rs ... ok
2019-07-09T18:28:08.5264493Z test [ui] ui/type/type-arg-out-of-scope.rs ... ok
2019-07-09T18:28:08.5264989Z test [ui] ui/type/type-annotation-needed.rs ... ok
2019-07-09T18:28:08.5731150Z test [ui] ui/type/type-ascription-instead-of-statement-end.rs ... ok
---
2019-07-09T18:32:14.4844694Z test [ui (nll)] ui/type-alias-enum-variants/issue-57866.rs ... ok
2019-07-09T18:32:14.5306084Z test [ui (nll)] ui/type-alias-enum-variants/no-type-application-on-aliased-enum-variant.rs ... ok
2019-07-09T18:32:14.5522268Z test [ui (nll)] ui/type-alias-enum-variants/issue-61801-path-pattern-can-infer.rs ... ok
2019-07-09T18:32:14.5732528Z test [ui (nll)] ui/type-alias-enum-variants/resolve-to-enum-variant-in-type-namespace-and-error.rs ... ok
2019-07-09T18:32:14.6138555Z test [ui (nll)] ui/type-alias/issue-62263-self-in-atb.rs ... ok
2019-07-09T18:32:14.6557612Z test [ui (nll)] ui/type-alias/issue-62305-self-assoc-ty.rs ... ok
2019-07-09T18:32:14.7011102Z test [ui (nll)] ui/type-alias/issue-62364-self-ty-arg.rs ... ok
2019-07-09T18:32:14.8712131Z test [ui (nll)] ui/type-alias-enum-variants/type-alias-enum-variants-pass.rs ... ok
2019-07-09T18:32:14.9086532Z test [ui (nll)] ui/type/type-annotation-needed.rs ... ok
2019-07-09T18:32:14.9300003Z test [ui (nll)] ui/type/type-arg-out-of-scope.rs ... ok
2019-07-09T18:32:14.9555062Z test [ui (nll)] ui/type/type-ascription-instead-of-initializer.rs ... ok
---
2019-07-09T18:32:26.9002212Z 
2019-07-09T18:32:26.9002829Z ---- [ui (nll)] ui/hrtb/issue-30786.rs stdout ----
2019-07-09T18:32:26.9002931Z diff of stderr:
2019-07-09T18:32:26.9002972Z 
2019-07-09T18:32:26.9003275Z - error: implementation of `Stream` is not general enough
2019-07-09T18:32:26.9003526Z -   --> $DIR/issue-30786.rs:98:22
2019-07-09T18:32:26.9003774Z + error: higher-ranked subtype error
2019-07-09T18:32:26.9004006Z +   --> $DIR/issue-30786.rs:103:18
2019-07-09T18:32:26.9004089Z 3    |
2019-07-09T18:32:26.9004328Z - LL |     let map = source.map(|x: &_| x);
2019-07-09T18:32:26.9004578Z -    |                      ^^^
2019-07-09T18:32:26.9004650Z + LL |     let filter = map.filter(|x: &_| true);
2019-07-09T18:32:26.9004806Z + 
2019-07-09T18:32:26.9004806Z + 
2019-07-09T18:32:26.9005044Z + error: higher-ranked subtype error
2019-07-09T18:32:26.9005272Z +   --> $DIR/issue-30786.rs:104:17
2019-07-09T18:32:26.9005356Z 6    |
2019-07-09T18:32:26.9005714Z -    = note: `Stream` would have to be implemented for the type `&'0 mut Map<Repeat, [closure@$DIR/issue-30786.rs:98:26: 98:35]>`, for any lifetime `'0`
2019-07-09T18:32:26.9006154Z -    = note: but `Stream` is actually implemented for the type `&'1 mut Map<Repeat, [closure@$DIR/issue-30786.rs:98:26: 98:35]>`, for some specific lifetime `'1`
2019-07-09T18:32:26.9006289Z + LL |     let count = filter.count(); // Assert that we still have a valid stream.
2019-07-09T18:32:26.9006457Z 9 
2019-07-09T18:32:26.9006680Z - error: aborting due to previous error
2019-07-09T18:32:26.9006766Z + error: aborting due to 2 previous errors
2019-07-09T18:32:26.9006829Z 11 
2019-07-09T18:32:26.9006829Z 11 
2019-07-09T18:32:26.9006903Z 12 
2019-07-09T18:32:26.9006937Z 
2019-07-09T18:32:26.9007147Z 
2019-07-09T18:32:26.9007231Z The actual stderr differed from the expected stderr.
2019-07-09T18:32:26.9007861Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll/issue-30786.nll.stderr
2019-07-09T18:32:26.9008196Z To update references, rerun the tests and pass the `--bless` flag
2019-07-09T18:32:26.9008491Z To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`
2019-07-09T18:32:26.9008629Z error: 1 errors occurred comparing output.
2019-07-09T18:32:26.9008715Z status: exit code: 1
2019-07-09T18:32:26.9008715Z status: exit code: 1
2019-07-09T18:32:26.9009778Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll/auxiliary" "-A" "unused"
2019-07-09T18:32:26.9010349Z ------------------------------------------
2019-07-09T18:32:26.9010518Z 
2019-07-09T18:32:26.9010942Z ------------------------------------------
2019-07-09T18:32:26.9011031Z stderr:
2019-07-09T18:32:26.9011031Z stderr:
2019-07-09T18:32:26.9011251Z ------------------------------------------
2019-07-09T18:32:26.9011492Z error: higher-ranked subtype error
2019-07-09T18:32:26.9011737Z   --> /checkout/src/test/ui/hrtb/issue-30786.rs:103:18
2019-07-09T18:32:26.9011825Z    |
2019-07-09T18:32:26.9011888Z LL |     let filter = map.filter(|x: &_| true);
2019-07-09T18:32:26.9012028Z 
2019-07-09T18:32:26.9012028Z 
2019-07-09T18:32:26.9012263Z error: higher-ranked subtype error
2019-07-09T18:32:26.9012510Z   --> /checkout/src/test/ui/hrtb/issue-30786.rs:104:17
2019-07-09T18:32:26.9012595Z    |
2019-07-09T18:32:26.9012665Z LL |     let count = filter.count(); // Assert that we still have a valid stream.
2019-07-09T18:32:26.9012808Z 
2019-07-09T18:32:26.9012883Z error: aborting due to 2 previous errors
2019-07-09T18:32:26.9012935Z 
2019-07-09T18:32:26.9012967Z 
---
2019-07-09T18:32:26.9021006Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-09T18:32:26.9021146Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-09T18:32:26.9036684Z 
2019-07-09T18:32:26.9036813Z 
2019-07-09T18:32:26.9039580Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-07-09T18:32:26.9040467Z 
2019-07-09T18:32:26.9040526Z 
2019-07-09T18:32:26.9043769Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-09T18:32:26.9043886Z Build completed unsuccessfully in 1:32:15
2019-07-09T18:32:26.9043886Z Build completed unsuccessfully in 1:32:15
2019-07-09T18:32:28.0288596Z ##[error]Bash exited with code '1'.
2019-07-09T18:32:28.0386644Z ##[section]Starting: Upload CPU usage statistics
2019-07-09T18:32:28.0393520Z ==============================================================================
2019-07-09T18:32:28.0393630Z Task         : Bash
2019-07-09T18:32:28.0393697Z Description  : Run a Bash script on macOS, Linux, or Windows
