plain
2019-07-27T11:25:59.1015637Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T11:25:59.1232957Z ##[command]git config gc.auto 0
2019-07-27T11:25:59.1339917Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T11:25:59.1424095Z ##[command]git config --get-all http.proxy
2019-07-27T11:25:59.1523961Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63038/merge:refs/remotes/pull/63038/merge
---
2019-07-27T11:26:32.9097159Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T11:26:32.9098087Z 
2019-07-27T11:26:32.9099247Z   git checkout -b <new-branch-name>
2019-07-27T11:26:32.9100002Z 
2019-07-27T11:26:32.9111130Z HEAD is now at 9db522257 Merge 2787cb23f0f3178f5eae11d571ee5e026b79d7fb into 09e39897587dca70f0b15093d425a682c392349c
2019-07-27T11:26:32.9242621Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T11:26:32.9245331Z ==============================================================================
2019-07-27T11:26:32.9245379Z Task         : Bash
2019-07-27T11:26:32.9245417Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T12:24:18.9881327Z .................................................................................................... 700/5870
2019-07-27T12:24:22.8093459Z .................................................................................................... 800/5870
2019-07-27T12:24:28.1427324Z .................................................................................................... 900/5870
2019-07-27T12:24:32.8741536Z .................................................................................................... 1000/5870
2019-07-27T12:24:38.2456693Z i...........i....................................................................................... 1100/5870
2019-07-27T12:24:41.8308405Z ..............................iiiii................................................................. 1200/5870
2019-07-27T12:24:47.5141646Z .................................................................................................... 1400/5870
2019-07-27T12:24:49.9632365Z .................................................................................................... 1500/5870
2019-07-27T12:24:53.6933495Z .................................................................................................... 1600/5870
2019-07-27T12:24:56.8736508Z .................................................................................................... 1700/5870
---
2019-07-27T12:26:05.1886343Z .................................................................................................... 3400/5870
2019-07-27T12:26:10.1706542Z .................................................................................................... 3500/5870
2019-07-27T12:26:14.0395887Z ..........................i......................................................................... 3600/5870
2019-07-27T12:26:18.0518119Z .................................................................................................... 3700/5870
2019-07-27T12:26:21.2248280Z ....ii...i..ii...................................................................................... 3800/5870
2019-07-27T12:26:29.5124883Z .................................................................................................... 4000/5870
2019-07-27T12:26:33.3021855Z .......................ii........................................................................... 4100/5870
2019-07-27T12:26:35.6819707Z ......................F.....................i....................................................... 4200/5870
2019-07-27T12:26:37.7759510Z .................................................................................................... 4300/5870
---
2019-07-27T12:27:54.0407793Z 1 error: an inner attribute is not permitted in this context
2019-07-27T12:27:54.0408409Z -   --> $DIR/attr.rs:5:3
2019-07-27T12:27:54.0408690Z +   --> $DIR/attr.rs:5:1
2019-07-27T12:27:54.0408733Z 3    |
2019-07-27T12:27:54.0408770Z 4 LL | #![lang = "foo"]
2019-07-27T12:27:54.0408975Z -    |   ^
2019-07-27T12:27:54.0409059Z 6    |
2019-07-27T12:27:54.0409059Z 6    |
2019-07-27T12:27:54.0409130Z 7    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T12:27:54.0409213Z 
2019-07-27T12:27:54.0409235Z 
2019-07-27T12:27:54.0409292Z The actual stderr differed from the expected stderr.
2019-07-27T12:27:54.0409572Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/attr.stderr
2019-07-27T12:27:54.0409572Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/attr.stderr
2019-07-27T12:27:54.0409978Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T12:27:54.0410675Z To only update this specific test, also pass `--test-args parser/attr.rs`
2019-07-27T12:27:54.0410993Z error: 1 errors occurred comparing output.
2019-07-27T12:27:54.0411055Z status: exit code: 1
2019-07-27T12:27:54.0411055Z status: exit code: 1
2019-07-27T12:27:54.0411892Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/auxiliary" "-A" "unused"
2019-07-27T12:27:54.0412434Z ------------------------------------------
2019-07-27T12:27:54.0412475Z 
2019-07-27T12:27:54.0412728Z ------------------------------------------
2019-07-27T12:27:54.0412804Z stderr:
2019-07-27T12:27:54.0412804Z stderr:
2019-07-27T12:27:54.0413057Z ------------------------------------------
2019-07-27T12:27:54.0413131Z error: an inner attribute is not permitted in this context
2019-07-27T12:27:54.0413413Z   --> /checkout/src/test/ui/parser/attr.rs:5:1
2019-07-27T12:27:54.0413472Z    |
2019-07-27T12:27:54.0413591Z LL | #![lang = "foo"] //~ ERROR an inner attribute is not permitted in this context
2019-07-27T12:27:54.0413716Z    |
2019-07-27T12:27:54.0413716Z    |
2019-07-27T12:27:54.0414053Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T12:27:54.0414301Z 
2019-07-27T12:27:54.0414356Z error[E0522]: definition of an unknown language item: `foo`
2019-07-27T12:27:54.0415008Z    |
2019-07-27T12:27:54.0415008Z    |
2019-07-27T12:27:54.0415054Z LL | #![lang = "foo"] //~ ERROR an inner attribute is not permitted in this context
2019-07-27T12:27:54.0415111Z    | ^^^^^^^^^^^^^^^^ definition of unknown language item `foo`
2019-07-27T12:27:54.0415199Z error: aborting due to 2 previous errors
2019-07-27T12:27:54.0415225Z 
2019-07-27T12:27:54.0415671Z For more information about this error, try `rustc --explain E0522`.
2019-07-27T12:27:54.0415727Z 
---
2019-07-27T12:27:54.0422497Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T12:27:54.0422854Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T12:27:54.0429280Z 
2019-07-27T12:27:54.0429400Z 
2019-07-27T12:27:54.0435512Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T12:27:54.0436123Z 
2019-07-27T12:27:54.0436174Z 
2019-07-27T12:27:54.0447499Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T12:27:54.0447789Z Build completed unsuccessfully in 0:53:53
2019-07-27T12:27:54.0447789Z Build completed unsuccessfully in 0:53:53
2019-07-27T12:27:55.1960922Z ##[error]Bash exited with code '1'.
2019-07-27T12:27:55.2034179Z ##[section]Starting: Checkout
2019-07-27T12:27:55.2035791Z ==============================================================================
2019-07-27T12:27:55.2035844Z Task         : Get sources
2019-07-27T12:27:55.2035915Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
