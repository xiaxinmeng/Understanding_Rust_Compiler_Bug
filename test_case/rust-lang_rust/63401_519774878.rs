plain
2019-08-09T04:39:31.2188410Z 
2019-08-09T04:39:31.2189520Z ---- [ui] ui/async-await/async-block-control-flow-static-semantics.rs stdout ----
2019-08-09T04:39:31.2189690Z diff of stderr:
2019-08-09T04:39:31.2189750Z 
2019-08-09T04:39:31.2190460Z 16 LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-09T04:39:31.2191200Z 17    |    ---------------------------------      ^^ expected u8, found ()
2019-08-09T04:39:31.2191340Z 18    |    |
2019-08-09T04:39:31.2192000Z -    |    this function's body doesn't return
2019-08-09T04:39:31.2192180Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2192370Z 21    = note: expected type `u8`
2019-08-09T04:39:31.2192450Z 22               found type `()`
2019-08-09T04:39:31.2192520Z 
2019-08-09T04:39:31.2192520Z 
2019-08-09T04:39:31.2193490Z 57 LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-09T04:39:31.2194280Z 58    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:39:31.2194410Z 59    |    |
2019-08-09T04:39:31.2195070Z -    |    this function's body doesn't return
2019-08-09T04:39:31.2195200Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2195320Z 61    |
2019-08-09T04:39:31.2195400Z 62    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:39:31.2195560Z 
2019-08-09T04:39:31.2195560Z 
2019-08-09T04:39:31.2196280Z 68 LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-09T04:39:31.2197040Z 69    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:39:31.2197190Z 70    |    |
2019-08-09T04:39:31.2198080Z -    |    this function's body doesn't return
2019-08-09T04:39:31.2198280Z +    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2198370Z 72    |
2019-08-09T04:39:31.2198470Z 73    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:39:31.2198630Z 
2019-08-09T04:39:31.2198680Z 
2019-08-09T04:39:31.2198770Z The actual stderr differed from the expected stderr.
2019-08-09T04:39:31.2199670Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-08-09T04:39:31.2199670Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-08-09T04:39:31.2200870Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T04:39:31.2201670Z To only update this specific test, also pass `--test-args async-await/async-block-control-flow-static-semantics.rs`
2019-08-09T04:39:31.2201890Z error: 1 errors occurred comparing output.
2019-08-09T04:39:31.2201990Z status: exit code: 1
2019-08-09T04:39:31.2201990Z status: exit code: 1
2019-08-09T04:39:31.2203740Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/async-await/async-block-control-flow-static-semantics.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/async-await/async-block-control-flow-static-semantics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/async-await/async-block-control-flow-static-semantics/auxiliary" "-A" "unused"
2019-08-09T04:39:31.2204790Z ------------------------------------------
2019-08-09T04:39:31.2204860Z 
2019-08-09T04:39:31.2205520Z ------------------------------------------
2019-08-09T04:39:31.2205630Z stderr:
2019-08-09T04:39:31.2205630Z stderr:
2019-08-09T04:39:31.2206430Z ------------------------------------------
2019-08-09T04:39:31.2206550Z error[E0267]: `break` inside of a closure
2019-08-09T04:39:31.2207460Z    |
2019-08-09T04:39:31.2207460Z    |
2019-08-09T04:39:31.2207570Z LL |         break 0u8; //~ ERROR `break` inside of a closure
2019-08-09T04:39:31.2207670Z    |         ^^^^^^^^^ cannot break inside of a closure
2019-08-09T04:39:31.2207750Z 
2019-08-09T04:39:31.2207820Z error[E0267]: `break` inside of a closure
2019-08-09T04:39:31.2208770Z    |
2019-08-09T04:39:31.2208770Z    |
2019-08-09T04:39:31.2208880Z LL |             break 0u8; //~ ERROR `break` inside of a closure
2019-08-09T04:39:31.2209000Z    |             ^^^^^^^^^ cannot break inside of a closure
2019-08-09T04:39:31.2209300Z error[E0308]: mismatched types
2019-08-09T04:39:31.2210060Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:15:43
2019-08-09T04:39:31.2210190Z    |
2019-08-09T04:39:31.2210190Z    |
2019-08-09T04:39:31.2210910Z LL | fn return_targets_async_block_not_fn() -> u8 {
2019-08-09T04:39:31.2211610Z    |    ---------------------------------      ^^ expected u8, found ()
2019-08-09T04:39:31.2211740Z    |    |
2019-08-09T04:39:31.2211830Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2212000Z    = note: expected type `u8`
2019-08-09T04:39:31.2212100Z               found type `()`
2019-08-09T04:39:31.2212160Z 
2019-08-09T04:39:31.2212160Z 
2019-08-09T04:39:31.2212250Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-09T04:39:31.2213400Z    |
2019-08-09T04:39:31.2213400Z    |
2019-08-09T04:39:31.2213480Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-09T04:39:31.2213590Z    |                                       ^^^^^^ expected u8, found ()
2019-08-09T04:39:31.2213750Z    = note: expected type `u8`
2019-08-09T04:39:31.2213830Z               found type `()`
2019-08-09T04:39:31.2213830Z               found type `()`
2019-08-09T04:39:31.2213940Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-09T04:39:31.2214020Z 
2019-08-09T04:39:31.2214110Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-08-09T04:39:31.2215260Z    |
2019-08-09T04:39:31.2215260Z    |
2019-08-09T04:39:31.2215340Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-08-09T04:39:31.2215470Z    |                                       ^^^^^^ expected u8, found ()
2019-08-09T04:39:31.2215630Z    = note: expected type `u8`
2019-08-09T04:39:31.2215710Z               found type `()`
2019-08-09T04:39:31.2215710Z               found type `()`
2019-08-09T04:39:31.2215820Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-08-09T04:39:31.2215890Z 
2019-08-09T04:39:31.2215980Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == u8`
2019-08-09T04:39:31.2216910Z    |
2019-08-09T04:39:31.2216910Z    |
2019-08-09T04:39:31.2217540Z LL | async fn return_targets_async_block_not_async_fn() -> u8 {
2019-08-09T04:39:31.2217710Z    |                                                       ^^ expected (), found u8
2019-08-09T04:39:31.2217880Z    = note: expected type `()`
2019-08-09T04:39:31.2217960Z               found type `u8`
2019-08-09T04:39:31.2217960Z               found type `u8`
2019-08-09T04:39:31.2218070Z    = note: the return type of a function must have a statically known size
2019-08-09T04:39:31.2218210Z error[E0308]: mismatched types
2019-08-09T04:39:31.2218950Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:51:44
2019-08-09T04:39:31.2219080Z    |
2019-08-09T04:39:31.2219080Z    |
2019-08-09T04:39:31.2219740Z LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-08-09T04:39:31.2220650Z    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:39:31.2220780Z    |    |
2019-08-09T04:39:31.2220890Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2220990Z    |
2019-08-09T04:39:31.2221090Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:39:31.2221230Z 
2019-08-09T04:39:31.2221310Z error[E0308]: mismatched types
2019-08-09T04:39:31.2222080Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:60:50
2019-08-09T04:39:31.2222230Z    |
2019-08-09T04:39:31.2222230Z    |
2019-08-09T04:39:31.2223050Z LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-08-09T04:39:31.2223810Z    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-09T04:39:31.2223950Z    |    |
2019-08-09T04:39:31.2224040Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-08-09T04:39:31.2224140Z    |
2019-08-09T04:39:31.2224220Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-08-09T04:39:31.2224390Z 
2019-08-09T04:39:31.2224450Z error: aborting due to 8 previous errors
2019-08-09T04:39:31.2224520Z 
2019-08-09T04:39:31.2224600Z Some errors have detailed explanations: E0267, E0271, E0308.
---
2019-08-09T04:39:31.2269320Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T04:39:31.2269550Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T04:39:31.2284660Z 
2019-08-09T04:39:31.2285090Z 
2019-08-09T04:39:31.2288270Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T04:39:31.2289180Z 
2019-08-09T04:39:31.2289220Z 
2019-08-09T04:39:31.2299490Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-09T04:39:31.2299640Z Build completed unsuccessfully in 1:04:53
2019-08-09T04:39:31.2299640Z Build completed unsuccessfully in 1:04:53
2019-08-09T04:39:31.2558140Z ##[error]Bash exited with code '1'.
2019-08-09T04:39:31.2609250Z ##[section]Starting: Upload CPU usage statistics
2019-08-09T04:39:31.2613980Z ==============================================================================
2019-08-09T04:39:31.2614110Z Task         : Bash
2019-08-09T04:39:31.2614200Z Description  : Run a Bash script on macOS, Linux, or Windows
