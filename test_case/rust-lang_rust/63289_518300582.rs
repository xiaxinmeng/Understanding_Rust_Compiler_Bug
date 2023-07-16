plain
2019-08-05T15:08:53.5221799Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T15:08:53.5416870Z ##[command]git config gc.auto 0
2019-08-05T15:08:53.5490864Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T15:08:53.5537932Z ##[command]git config --get-all http.proxy
2019-08-05T15:08:53.5674528Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63289/merge:refs/remotes/pull/63289/merge
---
2019-08-05T15:09:28.7134456Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T15:09:28.7134480Z 
2019-08-05T15:09:28.7134684Z   git checkout -b <new-branch-name>
2019-08-05T15:09:28.7134710Z 
2019-08-05T15:09:28.7134749Z HEAD is now at 16dbed04f Merge 3629a9b57b1530257030b405a654bb389cfa9b30 into 4be067558962c004b638e4c6f162d50f7c0c98b6
2019-08-05T15:09:28.7302277Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T15:09:28.7304874Z ==============================================================================
2019-08-05T15:09:28.7304938Z Task         : Bash
2019-08-05T15:09:28.7304973Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T16:06:53.1475848Z .................................................................................................... 1400/8827
2019-08-05T16:06:58.8255638Z .................................................................................................... 1500/8827
2019-08-05T16:07:10.9319286Z ....................................................................i...............i............... 1600/8827
2019-08-05T16:07:17.7238368Z .................................................................................................... 1700/8827
2019-08-05T16:07:32.1339594Z ......................................................iiiii......................................... 1800/8827
2019-08-05T16:07:43.0710857Z .................................................................................................... 2000/8827
2019-08-05T16:07:45.3429538Z .................................................................................................... 2100/8827
2019-08-05T16:07:48.4145331Z .................................................................................................... 2200/8827
2019-08-05T16:07:55.8180453Z .................................................................................................... 2300/8827
---
2019-08-05T16:11:30.7406519Z .................................................................................................... 5200/8827
2019-08-05T16:11:38.4255763Z .....................................................................i.............................. 5300/8827
2019-08-05T16:11:45.2501881Z .................................................................................................... 5400/8827
2019-08-05T16:11:51.7410416Z .................................................................................................... 5500/8827
2019-08-05T16:12:02.2951362Z ...............................................................ii...i..ii...........i............... 5600/8827
2019-08-05T16:12:22.9081855Z .................................................................................................... 5800/8827
2019-08-05T16:12:27.6710715Z .................................................................................................... 5900/8827
2019-08-05T16:12:27.6710715Z .................................................................................................... 5900/8827
2019-08-05T16:12:33.3304884Z ................................................................i..ii............................... 6000/8827
2019-08-05T16:13:00.8043178Z .................................................................................................... 6200/8827
2019-08-05T16:13:02.9532087Z .......i............................................................................................ 6300/8827
2019-08-05T16:13:05.0341205Z ...............................................................................i.................... 6400/8827
2019-08-05T16:13:07.6454431Z .................................................................................................... 6500/8827
---
2019-08-05T16:16:56.4291576Z failures:
2019-08-05T16:16:56.4310214Z 
2019-08-05T16:16:56.4310876Z ---- [ui] ui/issues/issue-1697.rs stdout ----
2019-08-05T16:16:56.4311120Z 
2019-08-05T16:16:56.4311667Z error: /checkout/src/test/ui/issues/issue-1697.rs:3: expected message not found: maybe a missing `extern crate unresolved;`?
2019-08-05T16:16:56.4311904Z 
2019-08-05T16:16:56.4312108Z error: 0 unexpected errors found, 1 expected errors not found
2019-08-05T16:16:56.4312316Z status: exit code: 1
2019-08-05T16:16:56.4313385Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-1697.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1697" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1697/auxiliary" "-A" "unused"
2019-08-05T16:16:56.4313699Z not found errors (from test file): [
2019-08-05T16:16:56.4313896Z     Error {
2019-08-05T16:16:56.4314087Z         line_num: 3,
2019-08-05T16:16:56.4314290Z         kind: None,
2019-08-05T16:16:56.4314486Z         msg: "maybe a missing `extern crate unresolved;`?",
2019-08-05T16:16:56.4314882Z ]
2019-08-05T16:16:56.4315227Z 
2019-08-05T16:16:56.4316162Z thread '[ui] ui/issues/issue-1697.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1506:13
2019-08-05T16:16:56.4316475Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T16:16:56.4316475Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T16:16:56.4316686Z 
2019-08-05T16:16:56.4317162Z ---- [ui] ui/privacy/restricted/test.rs stdout ----
2019-08-05T16:16:56.4317406Z 
2019-08-05T16:16:56.4317957Z error: /checkout/src/test/ui/privacy/restricted/test.rs:50: unexpected error: '50:12: 50:15: failed to resolve: maybe a missing crate `bad`? [E0433]'
2019-08-05T16:16:56.4318228Z 
2019-08-05T16:16:56.4318464Z error: /checkout/src/test/ui/privacy/restricted/test.rs:50: expected error not found: failed to resolve: maybe a missing `extern crate bad;`?
2019-08-05T16:16:56.4318656Z 
2019-08-05T16:16:56.4319036Z error: 1 unexpected errors found, 1 expected errors not found
2019-08-05T16:16:56.4319226Z status: exit code: 1
2019-08-05T16:16:56.4320032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/restricted/test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test/auxiliary" "-A" "unused"
2019-08-05T16:16:56.4320338Z unexpected errors (from JSON output): [
2019-08-05T16:16:56.4320545Z     Error {
2019-08-05T16:16:56.4320726Z         line_num: 50,
2019-08-05T16:16:56.4320902Z         kind: Some(
2019-08-05T16:16:56.4321264Z         ),
2019-08-05T16:16:56.4321264Z         ),
2019-08-05T16:16:56.4321577Z         msg: "50:12: 50:15: failed to resolve: maybe a missing crate `bad`? [E0433]",
2019-08-05T16:16:56.4322009Z ]
2019-08-05T16:16:56.4322164Z 
2019-08-05T16:16:56.4322369Z not found errors (from test file): [
2019-08-05T16:16:56.4322558Z     Error {
2019-08-05T16:16:56.4322558Z     Error {
2019-08-05T16:16:56.4322733Z         line_num: 50,
2019-08-05T16:16:56.4322925Z         kind: Some(
2019-08-05T16:16:56.4323096Z             Error,
2019-08-05T16:16:56.4323268Z         ),
2019-08-05T16:16:56.4323463Z         msg: "failed to resolve: maybe a missing `extern crate bad;`?",
2019-08-05T16:16:56.4323809Z ]
2019-08-05T16:16:56.4323980Z 
2019-08-05T16:16:56.4324447Z thread '[ui] ui/privacy/restricted/test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1506:13
2019-08-05T16:16:56.4324671Z 
2019-08-05T16:16:56.4324671Z 
2019-08-05T16:16:56.4325504Z ---- [ui] ui/unresolved/unresolved-import.rs stdout ----
2019-08-05T16:16:56.4325795Z 
2019-08-05T16:16:56.4326350Z error: /checkout/src/test/ui/unresolved/unresolved-import.rs:1: expected message not found: maybe a missing `extern crate foo;`?
2019-08-05T16:16:56.4326626Z 
2019-08-05T16:16:56.4326850Z error: 0 unexpected errors found, 1 expected errors not found
2019-08-05T16:16:56.4327232Z status: exit code: 1
2019-08-05T16:16:56.4328269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unresolved/unresolved-import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unresolved/unresolved-import" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unresolved/unresolved-import/auxiliary" "-A" "unused"
2019-08-05T16:16:56.4328762Z not found errors (from test file): [
2019-08-05T16:16:56.4328963Z     Error {
2019-08-05T16:16:56.4329146Z         line_num: 1,
2019-08-05T16:16:56.4329355Z         kind: None,
2019-08-05T16:16:56.4329708Z         msg: "maybe a missing `extern crate foo;`?",
2019-08-05T16:16:56.4330086Z ]
2019-08-05T16:16:56.4330254Z 
2019-08-05T16:16:56.4330700Z thread '[ui] ui/unresolved/unresolved-import.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1506:13
2019-08-05T16:16:56.4330936Z 
---
2019-08-05T16:16:56.4333542Z 
2019-08-05T16:16:56.4347210Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-05T16:16:56.4364324Z 
2019-08-05T16:16:56.4365296Z 
2019-08-05T16:16:56.4367408Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-05T16:16:56.4368529Z 
2019-08-05T16:16:56.4368594Z 
2019-08-05T16:16:56.4374632Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-05T16:16:56.4374778Z Build completed unsuccessfully in 1:01:20
2019-08-05T16:16:56.4374778Z Build completed unsuccessfully in 1:01:20
2019-08-05T16:16:57.1744439Z ##[error]Bash exited with code '1'.
2019-08-05T16:16:57.1797074Z ##[section]Starting: Checkout
2019-08-05T16:16:57.1799095Z ==============================================================================
2019-08-05T16:16:57.1799162Z Task         : Get sources
2019-08-05T16:16:57.1799199Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
