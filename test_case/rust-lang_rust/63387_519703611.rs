plain
2019-08-08T20:53:46.9444660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T20:53:46.9651328Z ##[command]git config gc.auto 0
2019-08-08T20:53:46.9720204Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T20:53:46.9793253Z ##[command]git config --get-all http.proxy
2019-08-08T20:53:46.9949234Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63387/merge:refs/remotes/pull/63387/merge
---
2019-08-08T20:54:23.7696560Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T20:54:23.7696585Z 
2019-08-08T20:54:23.7696751Z   git checkout -b <new-branch-name>
2019-08-08T20:54:23.7696774Z 
2019-08-08T20:54:23.7696833Z HEAD is now at f8b0ce1d8 Merge 21a6c214f0bcfb2881ae80d28141185e17713f97 into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T20:54:23.7872194Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T20:54:23.7874862Z ==============================================================================
2019-08-08T20:54:23.7874906Z Task         : Bash
2019-08-08T20:54:23.7874942Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T21:51:14.4056868Z .................................................................................................... 1300/8851
2019-08-08T21:51:20.7294755Z .................................................................................................... 1400/8851
2019-08-08T21:51:26.8105003Z .................................................................................................... 1500/8851
2019-08-08T21:51:37.4215917Z ....................................................................................i............... 1600/8851
2019-08-08T21:51:44.9154648Z i................................................................................................... 1700/8851
2019-08-08T21:51:51.7157252Z ......................................................................iiiii......................... 1800/8851
2019-08-08T21:52:12.7369679Z .................................................................................................... 2000/8851
2019-08-08T21:52:15.1372447Z .................................................................................................... 2100/8851
2019-08-08T21:52:17.8887783Z .................................................................................................... 2200/8851
2019-08-08T21:52:25.5555472Z .................................................................................................... 2300/8851
---
2019-08-08T21:56:02.3323596Z .................................................................................................... 5200/8851
2019-08-08T21:56:12.9771346Z .............................................................................................i...... 5300/8851
2019-08-08T21:56:20.7658866Z .................................................................................................... 5400/8851
2019-08-08T21:56:25.3256185Z .................................................................................................... 5500/8851
2019-08-08T21:56:36.2527623Z .......................................................................................ii...i..ii... 5600/8851
2019-08-08T21:56:58.4957027Z .................................................................................................... 5800/8851
2019-08-08T21:57:03.5328651Z .................................................................................................... 5900/8851
2019-08-08T21:57:03.5328651Z .................................................................................................... 5900/8851
2019-08-08T21:57:07.8935887Z ........................................................................................i..ii....... 6000/8851
2019-08-08T21:57:36.8532731Z .................................................................................................... 6200/8851
2019-08-08T21:57:38.7968011Z ...............................i.................................................................... 6300/8851
2019-08-08T21:57:40.7350175Z .................................................................................................... 6400/8851
2019-08-08T21:57:43.2323500Z ...i................................................................................................ 6500/8851
---
2019-08-08T22:01:33.9436286Z 
2019-08-08T22:01:33.9437746Z ---- [ui] ui/async-await/async-block-control-flow-static-semantics.rs stdout ----
2019-08-08T22:01:33.9438188Z diff of stderr:
2019-08-08T22:01:33.9438494Z 
2019-08-08T22:01:33.9439081Z 3    = note: consider adding a `main` function to `$DIR/async-block-control-flow-static-semantics.rs`
2019-08-08T22:01:33.9439405Z 4 
2019-08-08T22:01:33.9439692Z 5 error[E0267]: `break` inside of a closure
2019-08-08T22:01:33.9440206Z -   --> $DIR/async-block-control-flow-static-semantics.rs:32:9
2019-08-08T22:01:33.9440904Z +   --> $DIR/async-block-control-flow-static-semantics.rs:33:9
2019-08-08T22:01:33.9441354Z 8 LL |         break 0u8;
2019-08-08T22:01:33.9441354Z 8 LL |         break 0u8;
2019-08-08T22:01:33.9441564Z 9    |         ^^^^^^^^^ cannot break inside of a closure
2019-08-08T22:01:33.9442312Z 10 
2019-08-08T22:01:33.9442312Z 10 
2019-08-08T22:01:33.9442838Z 11 error[E0267]: `break` inside of a closure
2019-08-08T22:01:33.9443324Z -   --> $DIR/async-block-control-flow-static-semantics.rs:40:13
2019-08-08T22:01:33.9443781Z +   --> $DIR/async-block-control-flow-static-semantics.rs:41:13
2019-08-08T22:01:33.9444457Z 14 LL |             break 0u8;
2019-08-08T22:01:33.9444457Z 14 LL |             break 0u8;
2019-08-08T22:01:33.9444650Z 15    |             ^^^^^^^^^ cannot break inside of a closure
2019-08-08T22:01:33.9445034Z 16 
2019-08-08T22:01:33.9445221Z 17 error[E0308]: mismatched types
2019-08-08T22:01:33.9445655Z -   --> $DIR/async-block-control-flow-static-semantics.rs:12:43
2019-08-08T22:01:33.9446118Z +   --> $DIR/async-block-control-flow-static-semantics.rs:13:43
2019-08-08T22:01:33.9446118Z +   --> $DIR/async-block-control-flow-static-semantics.rs:13:43
2019-08-08T22:01:33.9446366Z 19    |
2019-08-08T22:01:33.9447244Z 20 LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-08T22:01:33.9448629Z 21    |    ---------------------------------      ^^ expected u8, found ()
2019-08-08T22:01:33.9449136Z 26               found type `()`
2019-08-08T22:01:33.9449315Z 27 
2019-08-08T22:01:33.9449315Z 27 
2019-08-08T22:01:33.9449496Z 28 error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-08T22:01:33.9450046Z -   --> $DIR/async-block-control-flow-static-semantics.rs:17:39
2019-08-08T22:01:33.9450695Z +   --> $DIR/async-block-control-flow-static-semantics.rs:18:39
2019-08-08T22:01:33.9450879Z 30    |
2019-08-08T22:01:33.9451014Z 31 LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-08T22:01:33.9451149Z 32    |                                       ^^^^^^ expected u8, found ()
2019-08-08T22:01:33.9451279Z 
2019-08-08T22:01:33.9451418Z 36    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-08T22:01:33.9451550Z 37 
2019-08-08T22:01:33.9451699Z 38 error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-08T22:01:33.9452034Z -   --> $DIR/async-block-control-flow-static-semantics.rs:26:39
2019-08-08T22:01:33.9452579Z +   --> $DIR/async-block-control-flow-static-semantics.rs:27:39
2019-08-08T22:01:33.9452771Z 40    |
2019-08-08T22:01:33.9452903Z 41 LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-08T22:01:33.9453048Z 42    |                                       ^^^^^^ expected u8, found ()
2019-08-08T22:01:33.9453186Z 
2019-08-08T22:01:33.9453320Z 46    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-08T22:01:33.9453449Z 47 
2019-08-08T22:01:33.9453600Z 48 error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == u8`
2019-08-08T22:01:33.9453975Z -   --> $DIR/async-block-control-flow-static-semantics.rs:21:55
2019-08-08T22:01:33.9454378Z +   --> $DIR/async-block-control-flow-static-semantics.rs:22:55
2019-08-08T22:01:33.9454554Z 50    |
2019-08-08T22:01:33.9454890Z 51 LL | async fn return_targets_async_block_not_async_fn() -> u8 {
2019-08-08T22:01:33.9455103Z 52    |                                                       ^^ expected (), found u8
2019-08-08T22:01:33.9455230Z 
2019-08-08T22:01:33.9455392Z 56    = note: the return type of a function must have a statically known size
2019-08-08T22:01:33.9455687Z 58 error[E0308]: mismatched types
2019-08-08T22:01:33.9456030Z -   --> $DIR/async-block-control-flow-static-semantics.rs:48:44
2019-08-08T22:01:33.9456423Z +   --> $DIR/async-block-control-flow-static-semantics.rs:49:44
2019-08-08T22:01:33.9456594Z 60    |
2019-08-08T22:01:33.9456594Z 60    |
2019-08-08T22:01:33.9457847Z 61 LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-08T22:01:33.9458441Z 62    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-08T22:01:33.9458820Z 67               found type `()`
2019-08-08T22:01:33.9459010Z 68 
2019-08-08T22:01:33.9459179Z 69 error[E0308]: mismatched types
2019-08-08T22:01:33.9459585Z -   --> $DIR/async-block-control-flow-static-semantics.rs:57:50
2019-08-08T22:01:33.9459585Z -   --> $DIR/async-block-control-flow-static-semantics.rs:57:50
2019-08-08T22:01:33.9460091Z +   --> $DIR/async-block-control-flow-static-semantics.rs:58:50
2019-08-08T22:01:33.9460300Z 71    |
2019-08-08T22:01:33.9461170Z 72 LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-08T22:01:33.9461621Z 73    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-08T22:01:33.9462072Z 
2019-08-08T22:01:33.9462231Z The actual stderr differed from the expected stderr.
2019-08-08T22:01:33.9462231Z The actual stderr differed from the expected stderr.
2019-08-08T22:01:33.9462644Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-08-08T22:01:33.9463029Z To update references, rerun the tests and pass the `--bless` flag
2019-08-08T22:01:33.9463429Z To only update this specific test, also pass `--test-args async-await/async-block-control-flow-static-semantics.rs`
2019-08-08T22:01:33.9463757Z error: 1 errors occurred comparing output.
2019-08-08T22:01:33.9463891Z status: exit code: 1
2019-08-08T22:01:33.9463891Z status: exit code: 1
2019-08-08T22:01:33.9464821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/auxiliary" "-A" "unused"
2019-08-08T22:01:33.9468029Z ------------------------------------------
2019-08-08T22:01:33.9468273Z 
2019-08-08T22:01:33.9468912Z ------------------------------------------
2019-08-08T22:01:33.9469162Z stderr:
2019-08-08T22:01:33.9469162Z stderr:
2019-08-08T22:01:33.9469549Z ------------------------------------------
2019-08-08T22:01:33.9469790Z error[E0601]: `main` function not found in crate `async_block_control_flow_static_semantics`
2019-08-08T22:01:33.9469989Z    |
2019-08-08T22:01:33.9470479Z    = note: consider adding a `main` function to `/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs`
2019-08-08T22:01:33.9470837Z 
2019-08-08T22:01:33.9470977Z error[E0267]: `break` inside of a closure
2019-08-08T22:01:33.9471308Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:33:9
2019-08-08T22:01:33.9471493Z    |
2019-08-08T22:01:33.9471624Z LL |         break 0u8; //~ ERROR `break` inside of a closure
2019-08-08T22:01:33.9471756Z    |         ^^^^^^^^^ cannot break inside of a closure
2019-08-08T22:01:33.9471886Z 
2019-08-08T22:01:33.9472014Z error[E0267]: `break` inside of a closure
2019-08-08T22:01:33.9472355Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:41:13
2019-08-08T22:01:33.9472538Z    |
2019-08-08T22:01:33.9472672Z LL |             break 0u8; //~ ERROR `break` inside of a closure
2019-08-08T22:01:33.9472837Z    |             ^^^^^^^^^ cannot break inside of a closure
2019-08-08T22:01:33.9473098Z error[E0308]: mismatched types
2019-08-08T22:01:33.9473457Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:13:43
2019-08-08T22:01:33.9473645Z    |
2019-08-08T22:01:33.9473645Z    |
2019-08-08T22:01:33.9473965Z LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-08T22:01:33.9474345Z    |    ---------------------------------      ^^ expected u8, found ()
2019-08-08T22:01:33.9474516Z    |    |
2019-08-08T22:01:33.9474825Z    |    this function's body doesn't return
2019-08-08T22:01:33.9475159Z    = note: expected type `u8`
2019-08-08T22:01:33.9475294Z               found type `()`
2019-08-08T22:01:33.9475446Z 
2019-08-08T22:01:33.9475446Z 
2019-08-08T22:01:33.9475776Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-08T22:01:33.9476176Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:18:39
2019-08-08T22:01:33.9476448Z    |
2019-08-08T22:01:33.9476621Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-08T22:01:33.9477260Z    |                                       ^^^^^^ expected u8, found ()
2019-08-08T22:01:33.9477714Z    = note: expected type `u8`
2019-08-08T22:01:33.9477885Z               found type `()`
2019-08-08T22:01:33.9477885Z               found type `()`
2019-08-08T22:01:33.9478089Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-08T22:01:33.9478271Z 
2019-08-08T22:01:33.9478456Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-08T22:01:33.9478968Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:27:39
2019-08-08T22:01:33.9479209Z    |
2019-08-08T22:01:33.9479413Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-08T22:01:33.9479590Z    |                                       ^^^^^^ expected u8, found ()
2019-08-08T22:01:33.9482754Z    = note: expected type `u8`
2019-08-08T22:01:33.9482936Z               found type `()`
2019-08-08T22:01:33.9482936Z               found type `()`
2019-08-08T22:01:33.9483100Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-08T22:01:33.9483226Z 
2019-08-08T22:01:33.9483363Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == u8`
2019-08-08T22:01:33.9483799Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:22:55
2019-08-08T22:01:33.9484007Z    |
2019-08-08T22:01:33.9484355Z LL | async fn return_targets_async_block_not_async_fn() -> u8 {
2019-08-08T22:01:33.9484532Z    |                                                       ^^ expected (), found u8
2019-08-08T22:01:33.9484959Z    = note: expected type `()`
2019-08-08T22:01:33.9485106Z               found type `u8`
2019-08-08T22:01:33.9485106Z               found type `u8`
2019-08-08T22:01:33.9485241Z    = note: the return type of a function must have a statically known size
2019-08-08T22:01:33.9485514Z error[E0308]: mismatched types
2019-08-08T22:01:33.9485880Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:49:44
2019-08-08T22:01:33.9486070Z    |
2019-08-08T22:01:33.9486070Z    |
2019-08-08T22:01:33.9486394Z LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-08T22:01:33.9487230Z    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-08T22:01:33.9487518Z    |    |
2019-08-08T22:01:33.9488011Z    |    this function's body doesn't return
2019-08-08T22:01:33.9488246Z    |
2019-08-08T22:01:33.9488457Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-08T22:01:33.9490956Z 
2019-08-08T22:01:33.9491090Z error[E0308]: mismatched types
2019-08-08T22:01:33.9491533Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:58:50
2019-08-08T22:01:33.9491919Z    |
2019-08-08T22:01:33.9491919Z    |
2019-08-08T22:01:33.9492281Z LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-08T22:01:33.9492690Z    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-08T22:01:33.9492869Z    |    |
2019-08-08T22:01:33.9493203Z    |    this function's body doesn't return
2019-08-08T22:01:33.9493402Z    |
2019-08-08T22:01:33.9493573Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-08T22:01:33.9493841Z 
2019-08-08T22:01:33.9493986Z error: aborting due to 9 previous errors
2019-08-08T22:01:33.9494131Z 
2019-08-08T22:01:33.9494278Z Some errors have detailed explanations: E0267, E0271, E0308, E0601.
---
2019-08-08T22:01:33.9497223Z test result: FAILED. 8817 passed; 1 failed; 33 ignored; 0 measured; 0 filtered out
2019-08-08T22:01:33.9497582Z 
2019-08-08T22:01:33.9502146Z 
2019-08-08T22:01:33.9502544Z 
2019-08-08T22:01:33.9504359Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-08T22:01:33.9505140Z 
2019-08-08T22:01:33.9505251Z 
2019-08-08T22:01:33.9505403Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-08T22:01:33.9505536Z Build completed unsuccessfully in 1:01:22
2019-08-08T22:01:33.9505536Z Build completed unsuccessfully in 1:01:22
2019-08-08T22:01:33.9507890Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-08T22:01:33.9508111Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-08T22:01:34.6233570Z ##[error]Bash exited with code '1'.
2019-08-08T22:01:34.6274000Z ##[section]Starting: Checkout
2019-08-08T22:01:34.6275457Z ==============================================================================
2019-08-08T22:01:34.6275499Z Task         : Get sources
2019-08-08T22:01:34.6275535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
