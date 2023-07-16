plain
2020-02-05T05:11:13.0928388Z ========================== Starting Command Output ===========================
2020-02-05T05:11:13.0943095Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/049cddd3-0d3e-4cbb-8770-2201514de04c.sh
2020-02-05T05:11:13.1153701Z 
2020-02-05T05:11:13.1206979Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T05:11:13.1213010Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-05T05:11:13.1214661Z Task         : Get sources
2020-02-05T05:11:13.1214697Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T05:11:13.1214733Z Version      : 1.0.0
2020-02-05T05:11:13.1214809Z Author       : Microsoft
---
2020-02-05T05:11:14.5078835Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T05:11:14.5149068Z ##[command]git config gc.auto 0
2020-02-05T05:11:14.5226809Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T05:11:14.5270449Z ##[command]git config --get-all http.proxy
2020-02-05T05:11:14.5462494Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68847/merge:refs/remotes/pull/68847/merge
---
2020-02-05T06:02:42.1678848Z .................................................................................................... 1700/9585
2020-02-05T06:02:47.0024423Z .................................................................................................... 1800/9585
2020-02-05T06:02:58.4114338Z ............................i....................................................................... 1900/9585
2020-02-05T06:03:05.3063357Z .................................................................................................... 2000/9585
2020-02-05T06:03:18.1497287Z ..................iiiii............................................................................. 2100/9585
2020-02-05T06:03:27.2537367Z .................................................................................................... 2300/9585
2020-02-05T06:03:29.4994671Z .................................................................................................... 2400/9585
2020-02-05T06:03:34.0170673Z .................................................................................................... 2500/9585
2020-02-05T06:03:52.6979321Z .................................................................................................... 2600/9585
---
2020-02-05T06:06:14.1491758Z .............................................................i...............i...................... 4900/9585
2020-02-05T06:06:21.1737439Z .................................................................................................... 5000/9585
2020-02-05T06:06:28.2664216Z .................................................................................................... 5100/9585
2020-02-05T06:06:32.4288865Z ....i............................................................................................... 5200/9585
2020-02-05T06:06:42.1591283Z ..............................................................................ii.ii........i...i.... 5300/9585
2020-02-05T06:06:49.7184622Z ................i................................................................................... 5500/9585
2020-02-05T06:06:57.9339295Z .................................................................................................... 5600/9585
2020-02-05T06:07:04.0031924Z .................................................................i.................................. 5700/9585
2020-02-05T06:07:10.5144881Z .................................................................................................... 5800/9585
2020-02-05T06:07:10.5144881Z .................................................................................................... 5800/9585
2020-02-05T06:07:17.4280330Z .................................................................................................... 5900/9585
2020-02-05T06:07:25.9109301Z ........................................................ii...i..ii...........i...................... 6000/9585
2020-02-05T06:07:45.2208745Z .................................................................................................... 6200/9585
2020-02-05T06:07:48.7283923Z .................................................................................................... 6300/9585
2020-02-05T06:07:48.7283923Z .................................................................................................... 6300/9585
2020-02-05T06:07:52.4950700Z ....................................................................................i..ii........... 6400/9585
2020-02-05T06:08:12.8237706Z .................................................................................................... 6600/9585
2020-02-05T06:08:21.2924569Z ......................................................................i............................. 6700/9585
2020-02-05T06:08:23.5188662Z .................................................................................................... 6800/9585
2020-02-05T06:08:25.8083456Z ........................................................................i........................... 6900/9585
---
2020-02-05T06:09:54.5185117Z .................................................................................................... 7500/9585
2020-02-05T06:09:59.3875992Z .................................................................................................... 7600/9585
2020-02-05T06:10:03.8540253Z .................................................................................................... 7700/9585
2020-02-05T06:10:10.3268147Z ..............................................................................................F..... 7800/9585
2020-02-05T06:10:18.2296211Z .....FF..F.......................................................................................... 7900/9585
2020-02-05T06:10:25.1748676Z ...................................iiiiiii.i........................................................ 8000/9585
2020-02-05T06:10:38.9589850Z .................................................................................................... 8200/9585
2020-02-05T06:10:46.6030923Z .................................................................................................... 8300/9585
2020-02-05T06:10:59.4915522Z .................................................................................................... 8400/9585
2020-02-05T06:11:06.4488483Z .................................................................................................... 8500/9585
---
2020-02-05T06:12:52.7696949Z -   --> $DIR/call-const-trait-method.rs:16:5
2020-02-05T06:12:52.7697147Z + error[E0601]: `main` function not found in crate `call_const_trait_method`
2020-02-05T06:12:52.7697482Z +   --> $DIR/call-const-trait-method.rs:1:1
2020-02-05T06:12:52.7697659Z 3    |
2020-02-05T06:12:52.7697983Z - LL |     a.add(b)
2020-02-05T06:12:52.7698304Z -    |     ^^^^^^^^
2020-02-05T06:12:52.7698471Z + LL | / #![allow(incomplete_features)]
2020-02-05T06:12:52.7698636Z + LL | | #![feature(const_trait_impl)]
2020-02-05T06:12:52.7698788Z + LL | | #![feature(const_fn)]
2020-02-05T06:12:52.7698935Z + LL | |
2020-02-05T06:12:52.7699102Z + ...  |
2020-02-05T06:12:52.7699249Z + LL | |
2020-02-05T06:12:52.7699394Z + LL | | }
2020-02-05T06:12:52.7699776Z +    | |_^ consider adding a `main` function to `$DIR/call-const-trait-method.rs`
2020-02-05T06:12:52.7700090Z 7 error: aborting due to previous error
2020-02-05T06:12:52.7700253Z 8 
2020-02-05T06:12:52.7700383Z 
2020-02-05T06:12:52.7700737Z - For more information about this error, try `rustc --explain E0015`.
2020-02-05T06:12:52.7700737Z - For more information about this error, try `rustc --explain E0015`.
2020-02-05T06:12:52.7701118Z + For more information about this error, try `rustc --explain E0601`.
2020-02-05T06:12:52.7701302Z 10 
2020-02-05T06:12:52.7701452Z 
2020-02-05T06:12:52.7701583Z 
2020-02-05T06:12:52.7701747Z The actual stderr differed from the expected stderr.
2020-02-05T06:12:52.7702193Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method/call-const-trait-method.stderr
2020-02-05T06:12:52.7702766Z To update references, rerun the tests and pass the `--bless` flag
2020-02-05T06:12:52.7703227Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/call-const-trait-method.rs`
2020-02-05T06:12:52.7703562Z error: 1 errors occurred comparing output.
2020-02-05T06:12:52.7703713Z status: exit code: 1
2020-02-05T06:12:52.7703713Z status: exit code: 1
2020-02-05T06:12:52.7704697Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method/auxiliary" "-A" "unused"
2020-02-05T06:12:52.7705393Z ------------------------------------------
2020-02-05T06:12:52.7705539Z 
2020-02-05T06:12:52.7705877Z ------------------------------------------
2020-02-05T06:12:52.7706041Z stderr:
2020-02-05T06:12:52.7706041Z stderr:
2020-02-05T06:12:52.7706363Z ------------------------------------------
2020-02-05T06:12:52.7706532Z error[E0601]: `main` function not found in crate `call_const_trait_method`
2020-02-05T06:12:52.7706915Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method.rs:1:1
2020-02-05T06:12:52.7707117Z    |
2020-02-05T06:12:52.7707269Z LL | / #![allow(incomplete_features)]
2020-02-05T06:12:52.7707420Z LL | | #![feature(const_trait_impl)]
2020-02-05T06:12:52.7707595Z LL | | #![feature(const_fn)]
2020-02-05T06:12:52.7707889Z ...  |
2020-02-05T06:12:52.7707889Z ...  |
2020-02-05T06:12:52.7708035Z LL | |     //~^ ERROR
2020-02-05T06:12:52.7708196Z LL | | }
2020-02-05T06:12:52.7708613Z    | |_^ consider adding a `main` function to `/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method.rs`
2020-02-05T06:12:52.7708936Z error: aborting due to previous error
2020-02-05T06:12:52.7709068Z 
2020-02-05T06:12:52.7709440Z For more information about this error, try `rustc --explain E0601`.
2020-02-05T06:12:52.7709588Z 
---
2020-02-05T06:12:52.7711560Z -   --> $DIR/feature-gate.rs:9:1
2020-02-05T06:12:52.7711727Z + error: fatal error triggered by #[rustc_error]
2020-02-05T06:12:52.7712062Z +   --> $DIR/feature-gate.rs:14:1
2020-02-05T06:12:52.7712223Z 3    |
2020-02-05T06:12:52.7712539Z - LL | impl const T for S {}
2020-02-05T06:12:52.7713071Z + LL | fn main() {}
2020-02-05T06:12:52.7713222Z +    | ^^^^^^^^^^^^
2020-02-05T06:12:52.7713367Z 6 
2020-02-05T06:12:52.7713516Z 7 error: aborting due to previous error
2020-02-05T06:12:52.7713516Z 7 error: aborting due to previous error
2020-02-05T06:12:52.7713660Z 8 
2020-02-05T06:12:52.7713807Z 
2020-02-05T06:12:52.7713936Z 
2020-02-05T06:12:52.7714103Z The actual stderr differed from the expected stderr.
2020-02-05T06:12:52.7714540Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated/feature-gate.gated.stderr
2020-02-05T06:12:52.7714913Z To update references, rerun the tests and pass the `--bless` flag
2020-02-05T06:12:52.7715326Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/feature-gate.rs`
2020-02-05T06:12:52.7715494Z 
2020-02-05T06:12:52.7715664Z error in revision `gated`: 1 errors occurred comparing output.
2020-02-05T06:12:52.7715816Z status: exit code: 1
2020-02-05T06:12:52.7716945Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gated" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated/auxiliary" "-A" "unused"
2020-02-05T06:12:52.7717550Z ------------------------------------------
2020-02-05T06:12:52.7717698Z 
2020-02-05T06:12:52.7718037Z ------------------------------------------
2020-02-05T06:12:52.7718199Z stderr:
2020-02-05T06:12:52.7718199Z stderr:
2020-02-05T06:12:52.7718521Z ------------------------------------------
2020-02-05T06:12:52.7718725Z error: fatal error triggered by #[rustc_error]
2020-02-05T06:12:52.7719093Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs:14:1
2020-02-05T06:12:52.7719264Z    |
2020-02-05T06:12:52.7719418Z LL | fn main() {} //[gated]~ ERROR fatal error triggered by #[rustc_error]
2020-02-05T06:12:52.7719719Z 
2020-02-05T06:12:52.7719869Z error: aborting due to previous error
2020-02-05T06:12:52.7720137Z 
2020-02-05T06:12:52.7720858Z 
---
2020-02-05T06:12:52.7722608Z 1 error[E0658]: const trait impls are experimental
2020-02-05T06:12:52.7722933Z -   --> $DIR/feature-gate.rs:9:6
2020-02-05T06:12:52.7723289Z +   --> $DIR/feature-gate.rs:10:6
2020-02-05T06:12:52.7723462Z 3    |
2020-02-05T06:12:52.7723612Z 4 LL | impl const T for S {}
2020-02-05T06:12:52.7723908Z 
2020-02-05T06:12:52.7723908Z 
2020-02-05T06:12:52.7724405Z 7    = note: for more information, see ***/issues/67792
2020-02-05T06:12:52.7724755Z 9 
2020-02-05T06:12:52.7725099Z - error: const trait impls are not yet implemented
2020-02-05T06:12:52.7725446Z -   --> $DIR/feature-gate.rs:9:1
2020-02-05T06:12:52.7725788Z -    |
2020-02-05T06:12:52.7725788Z -    |
2020-02-05T06:12:52.7726113Z - LL | impl const T for S {}
2020-02-05T06:12:52.7726748Z - 
2020-02-05T06:12:52.7727081Z - error: aborting due to 2 previous errors
2020-02-05T06:12:52.7727249Z + error: aborting due to previous error
2020-02-05T06:12:52.7727428Z 17 
2020-02-05T06:12:52.7727428Z 17 
2020-02-05T06:12:52.7727782Z 18 For more information about this error, try `rustc --explain E0658`.
2020-02-05T06:12:52.7728711Z 19 
2020-02-05T06:12:52.7728862Z 
2020-02-05T06:12:52.7728995Z 
2020-02-05T06:12:52.7729164Z The actual stderr differed from the expected stderr.
2020-02-05T06:12:52.7729606Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/feature-gate.stock.stderr
2020-02-05T06:12:52.7729980Z To update references, rerun the tests and pass the `--bless` flag
2020-02-05T06:12:52.7730383Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/feature-gate.rs`
2020-02-05T06:12:52.7730569Z 
2020-02-05T06:12:52.7730722Z error in revision `stock`: 1 errors occurred comparing output.
2020-02-05T06:12:52.7730886Z status: exit code: 1
2020-02-05T06:12:52.7733336Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/auxiliary" "-A" "unused"
2020-02-05T06:12:52.7734041Z ------------------------------------------
2020-02-05T06:12:52.7734201Z 
2020-02-05T06:12:52.7734551Z ------------------------------------------
2020-02-05T06:12:52.7734731Z stderr:
2020-02-05T06:12:52.7734731Z stderr:
2020-02-05T06:12:52.7735198Z ------------------------------------------
2020-02-05T06:12:52.7735377Z error[E0658]: const trait impls are experimental
2020-02-05T06:12:52.7735784Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs:10:6
2020-02-05T06:12:52.7735963Z    |
2020-02-05T06:12:52.7736122Z LL | impl const T for S {}
2020-02-05T06:12:52.7736469Z    |
2020-02-05T06:12:52.7736469Z    |
2020-02-05T06:12:52.7736919Z    = note: for more information, see ***/issues/67792
2020-02-05T06:12:52.7737299Z 
2020-02-05T06:12:52.7737463Z error: aborting due to previous error
2020-02-05T06:12:52.7737607Z 
2020-02-05T06:12:52.7737995Z For more information about this error, try `rustc --explain E0658`.
---
2020-02-05T06:12:52.7746551Z 
2020-02-05T06:12:52.7746680Z 
2020-02-05T06:12:52.7746848Z The actual stderr differed from the expected stderr.
2020-02-05T06:12:52.7747287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/inherent-impl/inherent-impl.stderr
2020-02-05T06:12:52.7747736Z To update references, rerun the tests and pass the `--bless` flag
2020-02-05T06:12:52.7748416Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/inherent-impl.rs`
2020-02-05T06:12:52.7748670Z error: 1 errors occurred comparing output.
2020-02-05T06:12:52.7748820Z status: exit code: 1
2020-02-05T06:12:52.7748820Z status: exit code: 1
2020-02-05T06:12:52.7749851Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/inherent-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/inherent-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/inherent-impl/auxiliary" "-A" "unused"
2020-02-05T06:12:52.7750418Z ------------------------------------------
2020-02-05T06:12:52.7750564Z 
2020-02-05T06:12:52.7750909Z ------------------------------------------
2020-02-05T06:12:52.7751074Z stderr:
---
2020-02-05T06:12:52.7759362Z test result: FAILED. 9529 passed; 4 failed; 52 ignored; 0 measured; 0 filtered out
2020-02-05T06:12:52.7759732Z 
2020-02-05T06:12:52.7759885Z 
2020-02-05T06:12:52.7760041Z 
2020-02-05T06:12:52.7761694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-05T06:12:52.7762670Z 
2020-02-05T06:12:52.7762991Z 
2020-02-05T06:12:52.7763347Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-05T06:12:52.7763518Z Build completed unsuccessfully in 0:55:39
2020-02-05T06:12:52.7763518Z Build completed unsuccessfully in 0:55:39
2020-02-05T06:12:52.7763698Z == clock drift check ==
2020-02-05T06:12:52.7763912Z   local time: Wed Feb  5 06:12:52 UTC 2020
2020-02-05T06:12:52.9309079Z   network time: Wed, 05 Feb 2020 06:12:52 GMT
2020-02-05T06:12:52.9314103Z == end clock drift check ==
2020-02-05T06:12:53.6225975Z 
2020-02-05T06:12:53.6321921Z ##[error]Bash exited with code '1'.
2020-02-05T06:12:53.6334816Z ##[section]Finishing: Run build
2020-02-05T06:12:53.6355957Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-05T06:12:53.6357753Z Task         : Get sources
2020-02-05T06:12:53.6357800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T06:12:53.6357847Z Version      : 1.0.0
2020-02-05T06:12:53.6358022Z Author       : Microsoft
2020-02-05T06:12:53.6358022Z Author       : Microsoft
2020-02-05T06:12:53.6358069Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-05T06:12:53.6358119Z ==============================================================================
2020-02-05T06:12:54.0306274Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-05T06:12:54.0344648Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-05T06:12:54.0442519Z Cleaning up task key
2020-02-05T06:12:54.0443258Z Start cleaning up orphan processes.
2020-02-05T06:12:54.0539540Z Terminate orphan process: pid (4885) (python)
2020-02-05T06:12:54.0799460Z ##[section]Finishing: Finalize Job
