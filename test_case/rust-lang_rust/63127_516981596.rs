plain
2019-07-31T17:56:05.3212651Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T17:56:05.3394760Z ##[command]git config gc.auto 0
2019-07-31T17:56:05.3469838Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T17:56:05.3536649Z ##[command]git config --get-all http.proxy
2019-07-31T17:56:05.3685684Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-31T17:56:43.1931324Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T17:56:43.1931356Z 
2019-07-31T17:56:43.1931573Z   git checkout -b <new-branch-name>
2019-07-31T17:56:43.1931603Z 
2019-07-31T17:56:43.1931670Z HEAD is now at 9fefbfe9b Merge 28e2a9113395af1560121285740ceeed3f257598 into 9152fe4ea053a29469691349f4b63aa94c9aac56
2019-07-31T17:56:43.2086855Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T17:56:43.2089724Z ==============================================================================
2019-07-31T17:56:43.2089784Z Task         : Bash
2019-07-31T17:56:43.2089834Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T18:58:35.8411596Z .................................................................................................... 1400/8819
2019-07-31T18:58:41.7063920Z .................................................................................................... 1500/8819
2019-07-31T18:58:54.2163069Z .................................................................i...............i.................. 1600/8819
2019-07-31T18:59:01.6589120Z .................................................................................................... 1700/8819
2019-07-31T18:59:16.6934815Z ...................................................iiiii............................................ 1800/8819
2019-07-31T18:59:27.9572885Z .................................................................................................... 2000/8819
2019-07-31T18:59:30.4992332Z .................................................................................................... 2100/8819
2019-07-31T18:59:34.1690238Z .................................................................................................... 2200/8819
2019-07-31T18:59:40.7497208Z .................................................................................................... 2300/8819
---
2019-07-31T19:03:36.5342195Z .................................................................................................... 5300/8819
2019-07-31T19:03:43.9600736Z ..............i..................................................................................... 5400/8819
2019-07-31T19:03:49.5574252Z .................................................................................................... 5500/8819
2019-07-31T19:04:01.9896798Z .................................................................................................... 5600/8819
2019-07-31T19:04:15.6917052Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-31T19:04:33.0889149Z .................................................................................................... 5900/8819
2019-07-31T19:04:37.9315737Z .................................................................................................... 6000/8819
2019-07-31T19:04:37.9315737Z .................................................................................................... 6000/8819
2019-07-31T19:04:51.9117741Z ........i..ii....................................................................................... 6100/8819
2019-07-31T19:05:11.0048683Z ...................................................i................................................ 6300/8819
2019-07-31T19:05:13.1674010Z .................................................................................................... 6400/8819
2019-07-31T19:05:15.6107753Z .....................i.............................................................................. 6500/8819
2019-07-31T19:05:20.1357186Z .................................................................................................... 6600/8819
---
2019-07-31T19:09:16.4020115Z 
2019-07-31T19:09:16.4020627Z ---- [ui] ui/async-await/no-args-non-move-async-closure.rs stdout ----
2019-07-31T19:09:16.4020684Z diff of stderr:
2019-07-31T19:09:16.4020710Z 
2019-07-31T19:09:16.4020986Z - error[E0708]: `async` non-`move` closures with paramaters are not currently supported
2019-07-31T19:09:16.4021219Z + error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T19:09:16.4021489Z 3    |
2019-07-31T19:09:16.4021489Z 3    |
2019-07-31T19:09:16.4021529Z 4 LL |     let _ = async |x: u8| {};
2019-07-31T19:09:16.4021577Z 
2019-07-31T19:09:16.4021797Z The actual stderr differed from the expected stderr.
2019-07-31T19:09:16.4022357Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T19:09:16.4022357Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T19:09:16.4022604Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T19:09:16.4022927Z To only update this specific test, also pass `--test-args async-await/no-args-non-move-async-closure.rs`
2019-07-31T19:09:16.4023014Z error: 1 errors occurred comparing output.
2019-07-31T19:09:16.4023080Z status: exit code: 1
2019-07-31T19:09:16.4023080Z status: exit code: 1
2019-07-31T19:09:16.4023980Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-args-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/auxiliary" "-A" "unused"
2019-07-31T19:09:16.4024387Z ------------------------------------------
2019-07-31T19:09:16.4024422Z 
2019-07-31T19:09:16.4024762Z ------------------------------------------
2019-07-31T19:09:16.4024830Z stderr:
2019-07-31T19:09:16.4024830Z stderr:
2019-07-31T19:09:16.4025658Z ------------------------------------------
2019-07-31T19:09:16.4025933Z error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T19:09:16.4026273Z    |
2019-07-31T19:09:16.4026273Z    |
2019-07-31T19:09:16.4026319Z LL |     let _ = async |x: u8| {};
2019-07-31T19:09:16.4026427Z    |
2019-07-31T19:09:16.4026427Z    |
2019-07-31T19:09:16.4026495Z    = help: consider using `let` statements to manually capture variables by reference before entering an `async move` closure
2019-07-31T19:09:16.4026604Z error: aborting due to previous error
2019-07-31T19:09:16.4026633Z 
2019-07-31T19:09:16.4026659Z 
2019-07-31T19:09:16.4026902Z ------------------------------------------
---
2019-07-31T19:09:16.4071328Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-31T19:09:16.4071407Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-31T19:09:16.4088458Z 
2019-07-31T19:09:16.4088665Z 
2019-07-31T19:09:16.4090127Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-31T19:09:16.4090538Z 
2019-07-31T19:09:16.4090567Z 
2019-07-31T19:09:16.4098081Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T19:09:16.4098195Z Build completed unsuccessfully in 1:05:52
2019-07-31T19:09:16.4098195Z Build completed unsuccessfully in 1:05:52
2019-07-31T19:09:17.2384620Z ##[error]Bash exited with code '1'.
2019-07-31T19:09:17.2439798Z ##[section]Starting: Checkout
2019-07-31T19:09:17.2441692Z ==============================================================================
2019-07-31T19:09:17.2441751Z Task         : Get sources
2019-07-31T19:09:17.2441818Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
