plain
2019-08-05T16:19:33.6312378Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T16:19:33.6505965Z ##[command]git config gc.auto 0
2019-08-05T16:19:33.6589564Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T16:19:33.6644934Z ##[command]git config --get-all http.proxy
2019-08-05T16:19:33.6787047Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62330/merge:refs/remotes/pull/62330/merge
---
2019-08-05T16:20:10.3163046Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T16:20:10.3163076Z 
2019-08-05T16:20:10.3163283Z   git checkout -b <new-branch-name>
2019-08-05T16:20:10.3163496Z 
2019-08-05T16:20:10.3163563Z HEAD is now at a91151e62 Merge d2a4d5732f55204b21d0fc0b6ca31375e26f74df into 4be067558962c004b638e4c6f162d50f7c0c98b6
2019-08-05T16:20:10.3322060Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T16:20:10.3324878Z ==============================================================================
2019-08-05T16:20:10.3324935Z Task         : Bash
2019-08-05T16:20:10.3324981Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T17:23:21.8166646Z .................................................................................................... 1400/8828
2019-08-05T17:23:28.0171526Z .................................................................................................... 1500/8828
2019-08-05T17:23:40.9382506Z ....................................................................i...............i............... 1600/8828
2019-08-05T17:23:48.3033728Z .................................................................................................... 1700/8828
2019-08-05T17:24:04.2113395Z ......................................................iiiii......................................... 1800/8828
2019-08-05T17:24:16.0511268Z .................................................................................................... 2000/8828
2019-08-05T17:24:18.6973967Z .................................................................................................... 2100/8828
2019-08-05T17:24:22.1406633Z .................................................................................................... 2200/8828
2019-08-05T17:24:30.3514382Z .................................................................................................... 2300/8828
---
2019-08-05T17:28:25.9335711Z .................................................................................................... 5200/8828
2019-08-05T17:28:34.4877314Z .....................................................................i.............................. 5300/8828
2019-08-05T17:28:42.0481869Z .................................................................................................... 5400/8828
2019-08-05T17:28:49.2727584Z .................................................................................................... 5500/8828
2019-08-05T17:29:00.7763942Z ...............................................................ii...i..ii...........i............... 5600/8828
2019-08-05T17:29:25.6092377Z .................................................................................................... 5800/8828
2019-08-05T17:29:30.7832532Z .................................................................................................... 5900/8828
2019-08-05T17:29:30.7832532Z .................................................................................................... 5900/8828
2019-08-05T17:29:37.1194686Z ................................................................i..ii............................... 6000/8828
2019-08-05T17:30:06.9953633Z .................................................................................................... 6200/8828
2019-08-05T17:30:09.2799401Z .......i............................................................................................ 6300/8828
2019-08-05T17:30:11.5014800Z ...............................................................................i.................... 6400/8828
2019-08-05T17:30:14.2790581Z .................................................................................................... 6500/8828
---
2019-08-05T17:34:25.9845824Z failures:
2019-08-05T17:34:25.9873366Z 
2019-08-05T17:34:25.9874167Z ---- [ui] ui/union/union-with-drop-fields-lint-rpass.rs stdout ----
2019-08-05T17:34:25.9874347Z 
2019-08-05T17:34:25.9874870Z error: test compilation failed although it shouldn't!
2019-08-05T17:34:25.9875048Z status: exit code: 1
2019-08-05T17:34:25.9875973Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-with-drop-fields-lint-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-with-drop-fields-lint-rpass/auxiliary" "-A" "unused"
2019-08-05T17:34:25.9876593Z ------------------------------------------
2019-08-05T17:34:25.9876741Z 
2019-08-05T17:34:25.9877079Z ------------------------------------------
2019-08-05T17:34:25.9877238Z stderr:
2019-08-05T17:34:25.9877238Z stderr:
2019-08-05T17:34:25.9877589Z ------------------------------------------
2019-08-05T17:34:25.9877751Z warning: unknown lint: `unions_with_drop_fields`
2019-08-05T17:34:25.9878339Z    |
2019-08-05T17:34:25.9878339Z    |
2019-08-05T17:34:25.9878470Z LL | #![allow(unions_with_drop_fields)]
2019-08-05T17:34:25.9878751Z    |
2019-08-05T17:34:25.9878873Z    = note: `#[warn(unknown_lints)]` on by default
2019-08-05T17:34:25.9879000Z 
2019-08-05T17:34:25.9879124Z error[E0740]: unions may not contain fields that need dropping
2019-08-05T17:34:25.9879124Z error[E0740]: unions may not contain fields that need dropping
2019-08-05T17:34:25.9879503Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:12:5
2019-08-05T17:34:25.9879687Z    |
2019-08-05T17:34:25.9879808Z LL |     a: String, // OK
2019-08-05T17:34:25.9880064Z    |
2019-08-05T17:34:25.9880186Z note: `std::mem::ManuallyDrop` can be used to wrap the type
2019-08-05T17:34:25.9880554Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:12:5
2019-08-05T17:34:25.9880997Z    |
2019-08-05T17:34:25.9880997Z    |
2019-08-05T17:34:25.9881199Z LL |     a: String, // OK
2019-08-05T17:34:25.9881677Z 
2019-08-05T17:34:25.9881895Z error[E0740]: unions may not contain fields that need dropping
2019-08-05T17:34:25.9882380Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:20:5
2019-08-05T17:34:25.9882578Z    |
2019-08-05T17:34:25.9882578Z    |
2019-08-05T17:34:25.9882698Z LL |     a: S, // OK
2019-08-05T17:34:25.9882953Z    |
2019-08-05T17:34:25.9883077Z note: `std::mem::ManuallyDrop` can be used to wrap the type
2019-08-05T17:34:25.9883466Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:20:5
2019-08-05T17:34:25.9883632Z    |
2019-08-05T17:34:25.9883632Z    |
2019-08-05T17:34:25.9883751Z LL |     a: S, // OK
2019-08-05T17:34:25.9883988Z 
2019-08-05T17:34:25.9884351Z error[E0740]: unions may not contain fields that need dropping
2019-08-05T17:34:25.9884848Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:25:5
2019-08-05T17:34:25.9885030Z    |
2019-08-05T17:34:25.9885030Z    |
2019-08-05T17:34:25.9885150Z LL |     a: T, // OK
2019-08-05T17:34:25.9885412Z    |
2019-08-05T17:34:25.9885533Z note: `std::mem::ManuallyDrop` can be used to wrap the type
2019-08-05T17:34:25.9885922Z   --> /checkout/src/test/ui/union/union-with-drop-fields-lint-rpass.rs:25:5
2019-08-05T17:34:25.9886088Z    |
2019-08-05T17:34:25.9886088Z    |
2019-08-05T17:34:25.9886207Z LL |     a: T, // OK
2019-08-05T17:34:25.9886445Z 
2019-08-05T17:34:25.9886566Z error: aborting due to 3 previous errors
2019-08-05T17:34:25.9886667Z 
2019-08-05T17:34:25.9887046Z For more information about this error, try `rustc --explain E0740`.
---
2019-08-05T17:34:25.9915626Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-05T17:34:25.9915876Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T17:34:25.9942831Z 
2019-08-05T17:34:26.8257846Z 
2019-08-05T17:34:26.8298815Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-05T17:34:26.8299091Z 
2019-08-05T17:34:26.8299136Z 
2019-08-05T17:34:26.8299186Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-05T17:34:26.8299239Z Build completed unsuccessfully in 1:07:58
2019-08-05T17:34:26.8299239Z Build completed unsuccessfully in 1:07:58
2019-08-05T17:34:26.8357601Z ##[error]Bash exited with code '1'.
2019-08-05T17:34:26.8411585Z ##[section]Starting: Checkout
2019-08-05T17:34:26.8413222Z ==============================================================================
2019-08-05T17:34:26.8413283Z Task         : Get sources
2019-08-05T17:34:26.8413350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
