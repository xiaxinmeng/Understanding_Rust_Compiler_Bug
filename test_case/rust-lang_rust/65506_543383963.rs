plain
2019-10-17T21:06:08.4007621Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-17T21:06:08.4206081Z ##[command]git config gc.auto 0
2019-10-17T21:06:08.4275560Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-17T21:06:08.4325893Z ##[command]git config --get-all http.proxy
2019-10-17T21:06:08.4479130Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65506/merge:refs/remotes/pull/65506/merge
---
2019-10-17T22:04:12.3874230Z .................................................................................................... 1600/9248
2019-10-17T22:04:17.9551304Z .................................................................................................... 1700/9248
2019-10-17T22:04:30.1995159Z ..................................i...............i................................................. 1800/9248
2019-10-17T22:04:37.3688505Z .................................................................................................... 1900/9248
2019-10-17T22:04:51.2055237Z ........................iiiii....................................................................... 2000/9248
2019-10-17T22:05:01.9230200Z .................................................................................................... 2200/9248
2019-10-17T22:05:04.4815865Z .................................................................................................... 2300/9248
2019-10-17T22:05:09.3900204Z .................................................................................................... 2400/9248
2019-10-17T22:05:31.3681651Z .................................................................................................... 2500/9248
---
2019-10-17T22:08:17.4225578Z ...........................i...............i........................................................ 4800/9248
2019-10-17T22:08:28.1826344Z .................................................................................................... 4900/9248
2019-10-17T22:08:34.1291326Z .................................................................................................... 5000/9248
2019-10-17T22:08:42.7912575Z .................................................................................................... 5100/9248
2019-10-17T22:08:50.0114430Z ...........................ii.ii.................................................................... 5200/9248
2019-10-17T22:08:58.8887581Z .................................................................................................... 5400/9248
2019-10-17T22:09:09.3352062Z .................................................................................................... 5500/9248
2019-10-17T22:09:16.3580048Z i................................................................................................... 5600/9248
2019-10-17T22:09:21.7860863Z .................................................................................................... 5700/9248
2019-10-17T22:09:21.7860863Z .................................................................................................... 5700/9248
2019-10-17T22:09:32.7796363Z .................................................................................................... 5800/9248
2019-10-17T22:09:44.8671201Z ..ii...i..ii...........i............................................................................ 5900/9248
2019-10-17T22:10:05.5592887Z .................................................................................................... 6100/9248
2019-10-17T22:10:12.7139492Z .................................................................................................... 6200/9248
2019-10-17T22:10:12.7139492Z .................................................................................................... 6200/9248
2019-10-17T22:10:26.0074261Z ........................i..ii....................................................................... 6300/9248
2019-10-17T22:10:48.5412795Z ............................................................F....................................... 6500/9248
2019-10-17T22:10:50.4472464Z ...................i................................................................................ 6600/9248
2019-10-17T22:10:52.5222267Z .............................................................................................i...... 6700/9248
2019-10-17T22:10:55.0199732Z .................................................................................................... 6800/9248
---
2019-10-17T22:14:52.8079141Z failures:
2019-10-17T22:14:52.8109032Z 
2019-10-17T22:14:52.8109788Z ---- [ui] ui/binop/binop-fail-3.rs stdout ----
2019-10-17T22:14:52.8110037Z normalized stderr:
2019-10-17T22:14:52.8110727Z warning: lint `resolve_trait_on_defaulted_unit` has been removed: `converted into hard error, see ***/issues/48950`
2019-10-17T22:14:52.8111306Z   --> $DIR/binop-fail-3.rs:7:9
2019-10-17T22:14:52.8111494Z    |
2019-10-17T22:14:52.8111620Z LL | #[allow(resolve_trait_on_defaulted_unit)]
2019-10-17T22:14:52.8111903Z    |
2019-10-17T22:14:52.8112030Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2019-10-17T22:14:52.8112147Z 
2019-10-17T22:14:52.8112271Z 
2019-10-17T22:14:52.8112271Z 
2019-10-17T22:14:52.8112370Z 
2019-10-17T22:14:52.8112472Z 
2019-10-17T22:14:52.8112594Z The actual stderr differed from the expected stderr.
2019-10-17T22:14:52.8113198Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/binop-fail-3.stderr
2019-10-17T22:14:52.8113565Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T22:14:52.8113959Z To only update this specific test, also pass `--test-args binop/binop-fail-3.rs`
2019-10-17T22:14:52.8114235Z error: 1 errors occurred comparing output.
2019-10-17T22:14:52.8114378Z status: exit code: 0
2019-10-17T22:14:52.8114378Z status: exit code: 0
2019-10-17T22:14:52.8115124Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-fail-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/auxiliary" "-A" "unused"
2019-10-17T22:14:52.8115943Z ------------------------------------------
2019-10-17T22:14:52.8116095Z 
2019-10-17T22:14:52.8116428Z ------------------------------------------
2019-10-17T22:14:52.8116806Z stderr:
2019-10-17T22:14:52.8116806Z stderr:
2019-10-17T22:14:52.8117096Z ------------------------------------------
2019-10-17T22:14:52.8117568Z warning: lint `resolve_trait_on_defaulted_unit` has been removed: `converted into hard error, see ***/issues/48950`
2019-10-17T22:14:52.8118653Z   --> /checkout/src/test/ui/binop/binop-fail-3.rs:7:9
2019-10-17T22:14:52.8118888Z    |
2019-10-17T22:14:52.8119050Z LL | #[allow(resolve_trait_on_defaulted_unit)]
2019-10-17T22:14:52.8119364Z    |
2019-10-17T22:14:52.8119518Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2019-10-17T22:14:52.8119644Z 
2019-10-17T22:14:52.8119789Z 
---
2019-10-17T22:14:52.8120995Z normalized stderr:
2019-10-17T22:14:52.8121328Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-17T22:14:52.8121791Z   --> $DIR/panic-macro-any.rs:7:27
2019-10-17T22:14:52.8121941Z    |
2019-10-17T22:14:52.8122081Z LL |     panic!(box 413 as Box<::std::any::Any + Send>);
2019-10-17T22:14:52.8122211Z    |                           ^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn ::std::any::Any + Send`
2019-10-17T22:14:52.8122589Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-17T22:14:52.8122743Z 
2019-10-17T22:14:52.8122846Z 
2019-10-17T22:14:52.8122943Z 
2019-10-17T22:14:52.8122943Z 
2019-10-17T22:14:52.8123072Z 
2019-10-17T22:14:52.8123192Z The actual stderr differed from the expected stderr.
2019-10-17T22:14:52.8123767Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/panic-macro-any.stderr
2019-10-17T22:14:52.8124145Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T22:14:52.8124570Z To only update this specific test, also pass `--test-args panics/panic-macro-any.rs`
2019-10-17T22:14:52.8124873Z error: 1 errors occurred comparing output.
2019-10-17T22:14:52.8124999Z status: exit code: 0
2019-10-17T22:14:52.8124999Z status: exit code: 0
2019-10-17T22:14:52.8125774Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/panic-macro-any.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/auxiliary" "-A" "unused"
2019-10-17T22:14:52.8126331Z ------------------------------------------
2019-10-17T22:14:52.8126481Z 
2019-10-17T22:14:52.8126965Z ------------------------------------------
2019-10-17T22:14:52.8127137Z stderr:
2019-10-17T22:14:52.8127137Z stderr:
2019-10-17T22:14:52.8127847Z ------------------------------------------
2019-10-17T22:14:52.8128550Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-17T22:14:52.8129154Z   --> /checkout/src/test/ui/panics/panic-macro-any.rs:7:27
2019-10-17T22:14:52.8129779Z    |
2019-10-17T22:14:52.8129832Z LL |     panic!(box 413 as Box<::std::any::Any + Send>);
2019-10-17T22:14:52.8129923Z    |                           ^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn ::std::any::Any + Send`
2019-10-17T22:14:52.8130040Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-17T22:14:52.8130202Z 
2019-10-17T22:14:52.8130228Z 
2019-10-17T22:14:52.8130535Z ------------------------------------------
---
2019-10-17T22:14:52.8151828Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-17T22:14:52.8151903Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-17T22:14:52.8165570Z 
2019-10-17T22:14:52.8165646Z 
2019-10-17T22:14:52.8172054Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-17T22:14:52.8172328Z 
2019-10-17T22:14:52.8172354Z 
2019-10-17T22:14:52.8175694Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-17T22:14:52.8175755Z Build completed unsuccessfully in 1:02:16
2019-10-17T22:14:52.8175755Z Build completed unsuccessfully in 1:02:16
2019-10-17T22:14:52.8225743Z == clock drift check ==
2019-10-17T22:14:52.8245658Z   local time: Thu Oct 17 22:14:52 UTC 2019
2019-10-17T22:14:52.8633128Z   network time: Thu, 17 Oct 2019 22:14:52 GMT
2019-10-17T22:14:52.8636149Z == end clock drift check ==
2019-10-17T22:14:53.9130603Z ##[error]Bash exited with code '1'.
2019-10-17T22:14:53.9183882Z ##[section]Starting: Checkout
2019-10-17T22:14:53.9185603Z ==============================================================================
2019-10-17T22:14:53.9185650Z Task         : Get sources
2019-10-17T22:14:53.9185724Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
