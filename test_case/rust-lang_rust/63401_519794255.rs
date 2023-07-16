plain
2019-08-09T03:28:41.1586156Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T03:28:41.1771240Z ##[command]git config gc.auto 0
2019-08-09T03:28:41.1850460Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T03:28:41.1908709Z ##[command]git config --get-all http.proxy
2019-08-09T03:28:41.2037586Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63401/merge:refs/remotes/pull/63401/merge
---
2019-08-09T03:29:17.5653432Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T03:29:17.5653466Z 
2019-08-09T03:29:17.5653702Z   git checkout -b <new-branch-name>
2019-08-09T03:29:17.5653735Z 
2019-08-09T03:29:17.5653817Z HEAD is now at e6e234f71 Merge 71648fb04d3b65eb4b9126e7c8ad4bdfc7303b1e into 5aa3d9a7b5d3a46a7f158e8881146331a6bc9243
2019-08-09T03:29:17.5813038Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T03:29:17.5816385Z ==============================================================================
2019-08-09T03:29:17.5816473Z Task         : Bash
2019-08-09T03:29:17.5816520Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T04:31:13.6563529Z .................................................................................................... 1300/8858
2019-08-09T04:31:20.2190650Z .................................................................................................... 1400/8858
2019-08-09T04:31:26.4777516Z .................................................................................................... 1500/8858
2019-08-09T04:31:37.1601640Z ....................................................................................i............... 1600/8858
2019-08-09T04:31:44.6906327Z i................................................................................................... 1700/8858
2019-08-09T04:31:51.7059382Z ......................................................................iiiii......................... 1800/8858
2019-08-09T04:32:13.6927734Z .................................................................................................... 2000/8858
2019-08-09T04:32:16.2358563Z .................................................................................................... 2100/8858
2019-08-09T04:32:19.1420659Z .................................................................................................... 2200/8858
2019-08-09T04:32:27.1281082Z .................................................................................................... 2300/8858
---
2019-08-09T04:36:14.4395768Z .................................................................................................... 5200/8858
2019-08-09T04:36:25.3289702Z ................................................................................................i... 5300/8858
2019-08-09T04:36:33.8408310Z .................................................................................................... 5400/8858
2019-08-09T04:36:38.6910560Z .................................................................................................... 5500/8858
2019-08-09T04:36:50.5107401Z ...........................................................................................ii...i..i 5600/8858
2019-08-09T04:37:05.9311410Z i...........i....................................................................................... 5700/8858
2019-08-09T04:37:19.6789711Z .................................................................................................... 5900/8858
2019-08-09T04:37:19.6789711Z .................................................................................................... 5900/8858
2019-08-09T04:37:24.4121603Z ............................................................................................i..ii... 6000/8858
2019-08-09T04:37:55.3036184Z .................................................................................................... 6200/8858
2019-08-09T04:37:57.5309944Z ...................................i................................................................ 6300/8858
2019-08-09T04:37:59.6734981Z .................................................................................................... 6400/8858
2019-08-09T04:38:02.2857667Z .......i............................................................................................ 6500/8858
---
2019-08-09T04:42:08.5255724Z 
2019-08-09T04:42:08.5256531Z ---- [ui] ui/async-await/async-block-control-flow-static-semantics.rs stdout ----
2019-08-09T04:42:08.5256933Z diff of stderr:
2019-08-09T04:42:08.5257090Z 
2019-08-09T04:42:08.5257647Z 16 LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-09T04:42:08.5258214Z 17    |    ---------------------------------      ^^ expected u8, found ()
2019-08-09T04:42:08.5258410Z 18    |    |
2019-08-09T04:42:08.5259310Z -    |    this function's body doesn't return
2019-08-09T04:42:08.5259973Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5260257Z 21    = note: expected type `u8`
2019-08-09T04:42:08.5260411Z 22               found type `()`
2019-08-09T04:42:08.5260545Z 
2019-08-09T04:42:08.5260545Z 
2019-08-09T04:42:08.5261042Z 57 LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-09T04:42:08.5261553Z 58    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:42:08.5261743Z 59    |    |
2019-08-09T04:42:08.5262132Z -    |    this function's body doesn't return
2019-08-09T04:42:08.5262321Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5262472Z 61    |
2019-08-09T04:42:08.5262634Z 62    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:42:08.5262908Z 
2019-08-09T04:42:08.5262908Z 
2019-08-09T04:42:08.5263366Z 68 LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-09T04:42:08.5263875Z 69    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:42:08.5264060Z 70    |    |
2019-08-09T04:42:08.5264724Z -    |    this function's body doesn't return
2019-08-09T04:42:08.5264926Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5265069Z 72    |
2019-08-09T04:42:08.5265235Z 73    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:42:08.5265496Z 
2019-08-09T04:42:08.5265631Z 
2019-08-09T04:42:08.5265772Z The actual stderr differed from the expected stderr.
2019-08-09T04:42:08.5266308Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-08-09T04:42:08.5266308Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-08-09T04:42:08.5266802Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T04:42:08.5267294Z To only update this specific test, also pass `--test-args async-await/async-block-control-flow-static-semantics.rs`
2019-08-09T04:42:08.5267625Z error: 1 errors occurred comparing output.
2019-08-09T04:42:08.5267777Z status: exit code: 1
2019-08-09T04:42:08.5267777Z status: exit code: 1
2019-08-09T04:42:08.5268785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/auxiliary" "-A" "unused"
2019-08-09T04:42:08.5269859Z ------------------------------------------
2019-08-09T04:42:08.5270027Z 
2019-08-09T04:42:08.5270411Z ------------------------------------------
2019-08-09T04:42:08.5270587Z stderr:
2019-08-09T04:42:08.5270587Z stderr:
2019-08-09T04:42:08.5270959Z ------------------------------------------
2019-08-09T04:42:08.5272110Z error[E0267]: `break` inside of a closure
2019-08-09T04:42:08.5272567Z    |
2019-08-09T04:42:08.5272567Z    |
2019-08-09T04:42:08.5272619Z LL |         break 0u8; //~ ERROR `break` inside of a closure
2019-08-09T04:42:08.5272670Z    |         ^^^^^^^^^ cannot break inside of a closure
2019-08-09T04:42:08.5272701Z 
2019-08-09T04:42:08.5272760Z error[E0267]: `break` inside of a closure
2019-08-09T04:42:08.5273212Z    |
2019-08-09T04:42:08.5273212Z    |
2019-08-09T04:42:08.5273289Z LL |             break 0u8; //~ ERROR `break` inside of a closure
2019-08-09T04:42:08.5273340Z    |             ^^^^^^^^^ cannot break inside of a closure
2019-08-09T04:42:08.5273412Z error[E0308]: mismatched types
2019-08-09T04:42:08.5273741Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:15:43
2019-08-09T04:42:08.5273790Z    |
2019-08-09T04:42:08.5273790Z    |
2019-08-09T04:42:08.5274018Z LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-09T04:42:08.5274292Z    |    ---------------------------------      ^^ expected u8, found ()
2019-08-09T04:42:08.5274339Z    |    |
2019-08-09T04:42:08.5274386Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5274488Z    = note: expected type `u8`
2019-08-09T04:42:08.5274530Z               found type `()`
2019-08-09T04:42:08.5274558Z 
2019-08-09T04:42:08.5274558Z 
2019-08-09T04:42:08.5274634Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-09T04:42:08.5274959Z    |
2019-08-09T04:42:08.5274959Z    |
2019-08-09T04:42:08.5275019Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-09T04:42:08.5275159Z    |                                       ^^^^^^ expected u8, found ()
2019-08-09T04:42:08.5275257Z    = note: expected type `u8`
2019-08-09T04:42:08.5275300Z               found type `()`
2019-08-09T04:42:08.5275300Z               found type `()`
2019-08-09T04:42:08.5275349Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-09T04:42:08.5275383Z 
2019-08-09T04:42:08.5275449Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-09T04:42:08.5275800Z    |
2019-08-09T04:42:08.5275800Z    |
2019-08-09T04:42:08.5275869Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-09T04:42:08.5275920Z    |                                       ^^^^^^ expected u8, found ()
2019-08-09T04:42:08.5276017Z    = note: expected type `u8`
2019-08-09T04:42:08.5276059Z               found type `()`
2019-08-09T04:42:08.5276059Z               found type `()`
2019-08-09T04:42:08.5276116Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-09T04:42:08.5276163Z 
2019-08-09T04:42:08.5276213Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == u8`
2019-08-09T04:42:08.5277099Z    |
2019-08-09T04:42:08.5277099Z    |
2019-08-09T04:42:08.5277398Z LL | async fn return_targets_async_block_not_async_fn() -> u8 {
2019-08-09T04:42:08.5277454Z    |                                                       ^^ expected (), found u8
2019-08-09T04:42:08.5277566Z    = note: expected type `()`
2019-08-09T04:42:08.5277609Z               found type `u8`
2019-08-09T04:42:08.5277609Z               found type `u8`
2019-08-09T04:42:08.5277656Z    = note: the return type of a function must have a statically known size
2019-08-09T04:42:08.5277744Z error[E0308]: mismatched types
2019-08-09T04:42:08.5278019Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:51:44
2019-08-09T04:42:08.5278076Z    |
2019-08-09T04:42:08.5278076Z    |
2019-08-09T04:42:08.5278347Z LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-09T04:42:08.5278634Z    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:42:08.5278685Z    |    |
2019-08-09T04:42:08.5278747Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5278791Z    |
2019-08-09T04:42:08.5278835Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:42:08.5279250Z 
2019-08-09T04:42:08.5279422Z error[E0308]: mismatched types
2019-08-09T04:42:08.5279789Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:60:50
2019-08-09T04:42:08.5279859Z    |
2019-08-09T04:42:08.5279859Z    |
2019-08-09T04:42:08.5280112Z LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-09T04:42:08.5280418Z    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:42:08.5280492Z    |    |
2019-08-09T04:42:08.5280543Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:42:08.5280587Z    |
2019-08-09T04:42:08.5280648Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:42:08.5280720Z 
2019-08-09T04:42:08.5280777Z error: aborting due to 8 previous errors
2019-08-09T04:42:08.5280807Z 
2019-08-09T04:42:08.5280851Z Some errors have detailed explanations: E0267, E0271, E0308.
---
2019-08-09T04:42:08.5299402Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T04:42:08.5299481Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T04:42:08.5317355Z 
2019-08-09T04:42:08.5317456Z 
2019-08-09T04:42:08.5319716Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T04:42:08.5320028Z 
2019-08-09T04:42:08.5320061Z 
2019-08-09T04:42:08.5324964Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-09T04:42:08.5325076Z Build completed unsuccessfully in 1:06:19
2019-08-09T04:42:08.5325076Z Build completed unsuccessfully in 1:06:19
2019-08-09T04:42:09.2867804Z ##[error]Bash exited with code '1'.
2019-08-09T04:42:09.2911916Z ##[section]Starting: Checkout
2019-08-09T04:42:09.2913643Z ==============================================================================
2019-08-09T04:42:09.2913706Z Task         : Get sources
2019-08-09T04:42:09.2913752Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
