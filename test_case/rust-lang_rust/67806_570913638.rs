plain
2020-01-05T12:32:00.3264475Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T12:32:00.3503896Z ##[command]git config gc.auto 0
2020-01-05T12:32:00.3586233Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T12:32:00.3645922Z ##[command]git config --get-all http.proxy
2020-01-05T12:32:00.3799419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67806/merge:refs/remotes/pull/67806/merge
---
2020-01-05T13:27:43.2855753Z .................................................................................................... 1500/9475
2020-01-05T13:27:49.5534362Z .................................................................................................... 1600/9475
2020-01-05T13:27:55.0030673Z .................................................................................................... 1700/9475
2020-01-05T13:28:05.0737872Z .................................................................................................... 1800/9475
2020-01-05T13:28:13.6188027Z i................................................................................................... 1900/9475
2020-01-05T13:28:20.5504738Z ........................................................................................iiiii....... 2000/9475
2020-01-05T13:28:43.3165410Z .................................................................................................... 2200/9475
2020-01-05T13:28:45.7970877Z .................................................................................................... 2300/9475
2020-01-05T13:28:48.3089728Z .................................................................................................... 2400/9475
2020-01-05T13:28:54.6149491Z .................................................................................................... 2500/9475
---
2020-01-05T13:32:03.9459667Z ....................i...............i............................................................... 4900/9475
2020-01-05T13:32:14.5415115Z .................................................................................................... 5000/9475
2020-01-05T13:32:20.5669914Z .................................................................i.................................. 5100/9475
2020-01-05T13:32:29.0296457Z .................................................................................................... 5200/9475
2020-01-05T13:32:36.9034539Z ................................ii.ii...........i................................................... 5300/9475
2020-01-05T13:32:46.7092773Z .................................................................................................... 5500/9475
2020-01-05T13:32:56.8425317Z .................................................................................................... 5600/9475
2020-01-05T13:33:04.4094161Z ................i................................................................................... 5700/9475
2020-01-05T13:33:10.8484073Z .................................................................................................... 5800/9475
2020-01-05T13:33:10.8484073Z .................................................................................................... 5800/9475
2020-01-05T13:33:22.7648227Z .................................................................................................... 5900/9475
2020-01-05T13:33:34.9005862Z .....ii...i..ii...........i......................................................................... 6000/9475
2020-01-05T13:33:53.3033939Z .................................................................................................... 6200/9475
2020-01-05T13:34:01.4216886Z .................................................................................................... 6300/9475
2020-01-05T13:34:01.4216886Z .................................................................................................... 6300/9475
2020-01-05T13:34:20.2917853Z ................................i..ii............................................................... 6400/9475
2020-01-05T13:34:41.3784173Z .................................................................................................... 6600/9475
2020-01-05T13:34:43.5729698Z .......i............................................................................................ 6700/9475
2020-01-05T13:34:45.9678218Z .................................................................................................... 6800/9475
2020-01-05T13:34:48.6633422Z .......i............................................................................................ 6900/9475
---
2020-01-05T13:36:30.9556566Z .................................................................................................... 7500/9475
2020-01-05T13:36:35.3272133Z .................................................................................................... 7600/9475
2020-01-05T13:36:40.9967583Z .................................................................................................... 7700/9475
2020-01-05T13:36:52.4176985Z .................................................................................................... 7800/9475
2020-01-05T13:37:00.8093236Z ...........................................iiii..................................................... 7900/9475
2020-01-05T13:37:16.3522915Z .................................................................................................... 8100/9475
2020-01-05T13:37:24.9382160Z .................................................................................................... 8200/9475
2020-01-05T13:37:39.4364662Z .................................................................................................... 8300/9475
2020-01-05T13:37:47.4693278Z .................................................................................................... 8400/9475
---
2020-01-05T13:40:16.3720733Z  finished in 7.025
2020-01-05T13:40:16.3923430Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:40:16.5880856Z 
2020-01-05T13:40:16.5881846Z running 166 tests
2020-01-05T13:40:19.7047103Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-05T13:40:22.0651173Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-05T13:40:22.0658092Z 
2020-01-05T13:40:22.0665428Z  finished in 5.674
2020-01-05T13:40:22.0872359Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:40:22.2690124Z 
---
2020-01-05T13:40:24.3453914Z  finished in 2.257
2020-01-05T13:40:24.3667955Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:40:24.5328694Z 
2020-01-05T13:40:24.5328819Z running 9 tests
2020-01-05T13:40:24.5336720Z iiiiiiiii
2020-01-05T13:40:24.5351514Z 
2020-01-05T13:40:24.5402583Z  finished in 0.169
2020-01-05T13:40:24.5538058Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:40:24.7498746Z 
---
2020-01-05T13:40:45.9350229Z  finished in 21.381
2020-01-05T13:40:45.9599791Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:40:46.1660894Z 
2020-01-05T13:40:46.1661115Z running 124 tests
2020-01-05T13:41:11.6024527Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-05T13:41:15.8673766Z .i.iii.....iiiiii.....ii
2020-01-05T13:41:15.8675374Z 
2020-01-05T13:41:15.8675631Z  finished in 29.907
2020-01-05T13:41:15.8682169Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T13:41:15.8682758Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-05T13:41:57.6312314Z + 
2020-01-05T13:41:57.6312613Z + error: cannot determine resolution for the macro `impl_lint_pass`
2020-01-05T13:41:57.6313210Z +   --> $DIR/lint_pass_impl_without_macro.rs:43:1
2020-01-05T13:41:57.6313367Z +    |
2020-01-05T13:41:57.6313489Z + LL | impl_lint_pass!(Bar => [TEST_LINT]);
2020-01-05T13:41:57.6313924Z +    |
2020-01-05T13:41:57.6314065Z +    = note: import resolution is stuck, try simplifying macro imports
2020-01-05T13:41:57.6314204Z + 
2020-01-05T13:41:57.6314327Z + error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-05T13:41:57.6314327Z + error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-05T13:41:57.6314668Z +   --> $DIR/lint_pass_impl_without_macro.rs:45:1
2020-01-05T13:41:57.6314833Z +    |
2020-01-05T13:41:57.6314955Z + LL | declare_lint_pass!(Baz => [TEST_LINT]);
2020-01-05T13:41:57.6315292Z +    |
2020-01-05T13:41:57.6315412Z +    = note: import resolution is stuck, try simplifying macro imports
2020-01-05T13:41:57.6315544Z + 
2020-01-05T13:41:57.6315663Z 1 error: implementing `LintPass` by hand
2020-01-05T13:41:57.6315663Z 1 error: implementing `LintPass` by hand
2020-01-05T13:41:57.6315985Z 2   --> $DIR/lint_pass_impl_without_macro.rs:21:6
2020-01-05T13:41:57.6316149Z 3    |
2020-01-05T13:41:57.6316252Z 
2020-01-05T13:41:57.6316372Z 22    |
2020-01-05T13:41:57.6316517Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-05T13:41:57.6316947Z - error: aborting due to 2 previous errors
2020-01-05T13:41:57.6317111Z + error: aborting due to 5 previous errors
2020-01-05T13:41:57.6317231Z 26 
2020-01-05T13:41:57.6317552Z + For more information about this error, try `rustc --explain E0432`.
2020-01-05T13:41:57.6317552Z + For more information about this error, try `rustc --explain E0432`.
2020-01-05T13:41:57.6317718Z 27 
2020-01-05T13:41:57.6317823Z 
2020-01-05T13:41:57.6317926Z 
2020-01-05T13:41:57.6318048Z The actual stderr differed from the expected stderr.
2020-01-05T13:41:57.6319002Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2020-01-05T13:41:57.6319481Z To update references, rerun the tests and pass the `--bless` flag
2020-01-05T13:41:57.6320432Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2020-01-05T13:41:57.6320825Z error: 1 errors occurred comparing output.
2020-01-05T13:41:57.6320999Z status: exit code: 1
2020-01-05T13:41:57.6320999Z status: exit code: 1
2020-01-05T13:41:57.6322168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2020-01-05T13:41:57.6323363Z ------------------------------------------
2020-01-05T13:41:57.6323509Z 
2020-01-05T13:41:57.6323811Z ------------------------------------------
2020-01-05T13:41:57.6324313Z stderr:
---
2020-01-05T13:41:57.6326201Z 
2020-01-05T13:41:57.6326496Z error: cannot determine resolution for the macro `impl_lint_pass`
2020-01-05T13:41:57.6326827Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:43:1
2020-01-05T13:41:57.6326999Z    |
2020-01-05T13:41:57.6327242Z LL | impl_lint_pass!(Bar => [TEST_LINT]);
2020-01-05T13:41:57.6327452Z    |
2020-01-05T13:41:57.6327591Z    = note: import resolution is stuck, try simplifying macro imports
2020-01-05T13:41:57.6327619Z 
2020-01-05T13:41:57.6327697Z error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-05T13:41:57.6327697Z error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-05T13:41:57.6328002Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:45:1
2020-01-05T13:41:57.6328042Z    |
2020-01-05T13:41:57.6328216Z LL | declare_lint_pass!(Baz => [TEST_LINT]);
2020-01-05T13:41:57.6328434Z    |
2020-01-05T13:41:57.6328682Z    = note: import resolution is stuck, try simplifying macro imports
2020-01-05T13:41:57.6328937Z 
2020-01-05T13:41:57.6328982Z error: implementing `LintPass` by hand
2020-01-05T13:41:57.6328982Z error: implementing `LintPass` by hand
2020-01-05T13:41:57.6329482Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:21:6
2020-01-05T13:41:57.6329685Z    |
2020-01-05T13:41:57.6329763Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2020-01-05T13:41:57.6329895Z    |
2020-01-05T13:41:57.6329938Z note: lint level defined here
2020-01-05T13:41:57.6330285Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2020-01-05T13:41:57.6330453Z    |
2020-01-05T13:41:57.6330453Z    |
2020-01-05T13:41:57.6330706Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2020-01-05T13:41:57.6330796Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-05T13:41:57.6330868Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-05T13:41:57.6330999Z error: implementing `LintPass` by hand
2020-01-05T13:41:57.6331347Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:31:14
2020-01-05T13:41:57.6331397Z    |
2020-01-05T13:41:57.6331397Z    |
2020-01-05T13:41:57.6331596Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2020-01-05T13:41:57.6331824Z ...
2020-01-05T13:41:57.6331824Z ...
2020-01-05T13:41:57.6331886Z LL | custom_lint_pass_macro!();
2020-01-05T13:41:57.6332398Z    |
2020-01-05T13:41:57.6332398Z    |
2020-01-05T13:41:57.6332574Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-05T13:41:57.6332767Z error: aborting due to 5 previous errors
2020-01-05T13:41:57.6332808Z 
2020-01-05T13:41:57.6333067Z For more information about this error, try `rustc --explain E0432`.
2020-01-05T13:41:57.6333198Z 
---
2020-01-05T13:41:57.6334768Z test result: FAILED. 63 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-05T13:41:57.6334817Z 
2020-01-05T13:41:57.6338455Z 
2020-01-05T13:41:57.6339044Z 
2020-01-05T13:41:57.6345899Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-05T13:41:57.6346520Z 
2020-01-05T13:41:57.6346584Z 
2020-01-05T13:41:57.6357559Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-05T13:41:57.6357930Z Build completed unsuccessfully in 1:03:21
2020-01-05T13:41:57.6357930Z Build completed unsuccessfully in 1:03:21
2020-01-05T13:41:57.6415020Z == clock drift check ==
2020-01-05T13:41:57.6437888Z   local time: Sun Jan  5 13:41:57 UTC 2020
2020-01-05T13:41:58.2127695Z   network time: Sun, 05 Jan 2020 13:41:58 GMT
2020-01-05T13:41:58.2149396Z == end clock drift check ==
2020-01-05T13:41:59.5681182Z 
2020-01-05T13:41:59.5796309Z ##[error]Bash exited with code '1'.
2020-01-05T13:41:59.5840095Z ##[section]Starting: Checkout
2020-01-05T13:41:59.5841815Z ==============================================================================
2020-01-05T13:41:59.5841872Z Task         : Get sources
2020-01-05T13:41:59.5841936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
