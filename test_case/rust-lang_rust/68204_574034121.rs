plain
2020-01-14T05:54:21.4389751Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T05:54:21.4493036Z ##[command]git config gc.auto 0
2020-01-14T05:54:21.4579987Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T05:54:21.4637046Z ##[command]git config --get-all http.proxy
2020-01-14T05:54:21.4799017Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68204/merge:refs/remotes/pull/68204/merge
---
2020-01-14T06:56:32.8836763Z ........................................i...............i........................................... 4900/9518
2020-01-14T06:56:42.4230536Z .................................................................................................... 5000/9518
2020-01-14T06:56:49.1492669Z ...................................................................................i................ 5100/9518
2020-01-14T06:56:54.8150985Z .................................................................................................... 5200/9518
2020-01-14T06:57:05.6256054Z ......................................................ii.ii...........i............................. 5300/9518
2020-01-14T06:57:15.0942867Z .................................................................................................... 5500/9518
2020-01-14T06:57:25.5587173Z .................................................................................................... 5600/9518
2020-01-14T06:57:32.5033550Z .......................................i............................................................ 5700/9518
2020-01-14T06:57:39.2699669Z .................................................................................................... 5800/9518
2020-01-14T06:57:39.2699669Z .................................................................................................... 5800/9518
2020-01-14T06:57:50.6601535Z .................................................................................................... 5900/9518
2020-01-14T06:58:00.9753060Z ..............................ii...i..ii...........i................................................ 6000/9518
2020-01-14T06:58:20.3360162Z .................................................................................................... 6200/9518
2020-01-14T06:58:28.7009630Z .................................................................................................... 6300/9518
2020-01-14T06:58:28.7009630Z .................................................................................................... 6300/9518
2020-01-14T06:58:41.2569903Z ..........................................................i..ii..................................... 6400/9518
2020-01-14T06:59:10.2379943Z .................................................................................................... 6600/9518
2020-01-14T06:59:12.5355760Z ..................................i................................................................. 6700/9518
2020-01-14T06:59:14.8320693Z .................................................................................................... 6800/9518
2020-01-14T06:59:17.4795154Z ..................................i................................................................. 6900/9518
---
2020-01-14T07:00:53.5152852Z .................................................................................................... 7400/9518
2020-01-14T07:00:58.8072096Z .................................................................................................... 7500/9518
2020-01-14T07:01:03.2588495Z .................................................................................................... 7600/9518
2020-01-14T07:01:09.5196863Z .................................................................................................... 7700/9518
2020-01-14T07:01:17.0281254Z .....................................................FF............................................. 7800/9518
2020-01-14T07:01:27.2587948Z ....................................................................................iiii............ 7900/9518
2020-01-14T07:01:44.4946088Z .................i......i........................................................................... 8100/9518
2020-01-14T07:01:49.9100097Z .................................................................................................... 8200/9518
2020-01-14T07:02:04.0548925Z .................................................................................................... 8300/9518
2020-01-14T07:02:14.4945888Z .................................................................................................... 8400/9518
---
2020-01-14T07:04:20.7255296Z failures:
2020-01-14T07:04:20.7275270Z 
2020-01-14T07:04:20.7276396Z ---- [ui] ui/rfc-2632-const-trait-impl/feature-gate.rs#gated stdout ----
2020-01-14T07:04:20.7276704Z 
2020-01-14T07:04:20.7276930Z error in revision `gated`: ui test compiled successfully!
2020-01-14T07:04:20.7277251Z status: exit code: 0
2020-01-14T07:04:20.7278453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gated" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated/auxiliary" "-A" "unused"
2020-01-14T07:04:20.7279283Z ------------------------------------------
2020-01-14T07:04:20.7279550Z 
2020-01-14T07:04:20.7280010Z ------------------------------------------
2020-01-14T07:04:20.7280268Z stderr:
---
2020-01-14T07:04:20.7281844Z 
2020-01-14T07:04:20.7282316Z ---- [ui] ui/rfc-2632-const-trait-impl/feature-gate.rs#stock stdout ----
2020-01-14T07:04:20.7282579Z diff of stderr:
2020-01-14T07:04:20.7282796Z 
2020-01-14T07:04:20.7283431Z 7    = note: for more information, see ***/issues/67792
2020-01-14T07:04:20.7283972Z 9 
2020-01-14T07:04:20.7284428Z - error: const trait impls are not yet implemented
2020-01-14T07:04:20.7284932Z -   --> $DIR/feature-gate.rs:9:1
2020-01-14T07:04:20.7285541Z -    |
2020-01-14T07:04:20.7285541Z -    |
2020-01-14T07:04:20.7288534Z - LL | impl const T for S {}
2020-01-14T07:04:20.7290945Z - 
2020-01-14T07:04:20.7291190Z - error: aborting due to 2 previous errors
2020-01-14T07:04:20.7296486Z + error: aborting due to previous error
2020-01-14T07:04:20.7297018Z 17 
2020-01-14T07:04:20.7297018Z 17 
2020-01-14T07:04:20.7298296Z 18 For more information about this error, try `rustc --explain E0658`.
2020-01-14T07:04:20.7299461Z 19 
2020-01-14T07:04:20.7299857Z 
2020-01-14T07:04:20.7299981Z 
2020-01-14T07:04:20.7300051Z The actual stderr differed from the expected stderr.
2020-01-14T07:04:20.7300681Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/feature-gate.stock.stderr
2020-01-14T07:04:20.7301164Z To update references, rerun the tests and pass the `--bless` flag
2020-01-14T07:04:20.7301518Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/feature-gate.rs`
2020-01-14T07:04:20.7301580Z 
2020-01-14T07:04:20.7301749Z error in revision `stock`: 1 errors occurred comparing output.
2020-01-14T07:04:20.7301797Z status: exit code: 1
2020-01-14T07:04:20.7302770Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/auxiliary" "-A" "unused"
2020-01-14T07:04:20.7303131Z ------------------------------------------
2020-01-14T07:04:20.7303166Z 
2020-01-14T07:04:20.7303388Z ------------------------------------------
2020-01-14T07:04:20.7303618Z stderr:
2020-01-14T07:04:20.7303618Z stderr:
2020-01-14T07:04:20.7303904Z ------------------------------------------
2020-01-14T07:04:20.7303955Z error[E0658]: const trait impls are experimental
2020-01-14T07:04:20.7304221Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs:9:6
2020-01-14T07:04:20.7304269Z    |
2020-01-14T07:04:20.7304314Z LL | impl const T for S {}
2020-01-14T07:04:20.7304468Z    |
2020-01-14T07:04:20.7304468Z    |
2020-01-14T07:04:20.7304793Z    = note: for more information, see ***/issues/67792
2020-01-14T07:04:20.7304901Z 
2020-01-14T07:04:20.7304943Z error: aborting due to previous error
2020-01-14T07:04:20.7304971Z 
2020-01-14T07:04:20.7305404Z For more information about this error, try `rustc --explain E0658`.
---
2020-01-14T07:04:20.7307453Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-14T07:04:20.7307518Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-14T07:04:20.7324133Z 
2020-01-14T07:04:20.7324219Z 
2020-01-14T07:04:20.7326268Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-14T07:04:20.7326639Z 
2020-01-14T07:04:20.7326693Z 
2020-01-14T07:04:20.7335038Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-14T07:04:20.7335115Z Build completed unsuccessfully in 1:04:17
2020-01-14T07:04:20.7335115Z Build completed unsuccessfully in 1:04:17
2020-01-14T07:04:20.7397982Z == clock drift check ==
2020-01-14T07:04:20.7417755Z   local time: Tue Jan 14 07:04:20 UTC 2020
2020-01-14T07:04:21.0579853Z   network time: Tue, 14 Jan 2020 07:04:21 GMT
2020-01-14T07:04:21.0584416Z == end clock drift check ==
2020-01-14T07:04:21.4782996Z 
2020-01-14T07:04:21.4901824Z ##[error]Bash exited with code '1'.
2020-01-14T07:04:21.4941703Z ##[section]Starting: Checkout
2020-01-14T07:04:21.4943346Z ==============================================================================
2020-01-14T07:04:21.4943396Z Task         : Get sources
2020-01-14T07:04:21.4943468Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
