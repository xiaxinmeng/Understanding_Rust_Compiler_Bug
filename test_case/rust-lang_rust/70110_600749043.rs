plain
2020-03-18T16:59:20.4688260Z + 
2020-03-18T16:59:20.4688450Z + note: trace_macro
2020-03-18T16:59:20.4689040Z +   --> $DIR/trace_faulty_macros.rs:38:13
2020-03-18T16:59:20.4689310Z +    |
2020-03-18T16:59:20.4689520Z + LL |     let a = pat_macro!();
2020-03-18T16:59:20.4690000Z +    |
2020-03-18T16:59:20.4690000Z +    |
2020-03-18T16:59:20.4690250Z +    = note: expanding `pat_macro! {  }`
2020-03-18T16:59:20.4690620Z +    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T16:59:20.4691080Z +    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T16:59:20.4691500Z +    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:59:20.4692250Z + error: aborting due to 4 previous errors
2020-03-18T16:59:20.4692520Z 64 
2020-03-18T16:59:20.4692680Z 65 
2020-03-18T16:59:20.4692790Z 
2020-03-18T16:59:20.4692790Z 
2020-03-18T16:59:20.4692900Z 
2020-03-18T16:59:20.4693150Z The actual stderr differed from the expected stderr.
2020-03-18T16:59:20.4694150Z Actual stderr saved to /Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-03-18T16:59:20.4695080Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T16:59:20.4695890Z To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
2020-03-18T16:59:20.4696440Z error: 1 errors occurred comparing output.
2020-03-18T16:59:20.4696710Z status: exit code: 1
2020-03-18T16:59:20.4696710Z status: exit code: 1
2020-03-18T16:59:20.4699310Z command: "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.165.2/work/1/s/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-A" "unused" "-Z" "trace-macros" "-L" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/test/ui/macros/trace_faulty_macros/auxiliary"
2020-03-18T16:59:20.4701560Z ------------------------------------------
2020-03-18T16:59:20.4701790Z 
2020-03-18T16:59:20.4702320Z ------------------------------------------
2020-03-18T16:59:20.4702580Z stderr:
2020-03-18T16:59:20.4702580Z stderr:
2020-03-18T16:59:20.4703110Z ------------------------------------------
2020-03-18T16:59:20.4703440Z error: no rules expected the token `bcd`
2020-03-18T16:59:20.4704220Z   --> /Users/runner/runners/2.165.2/work/1/s/src/test/ui/macros/trace_faulty_macros.rs:7:26
2020-03-18T16:59:20.4704580Z    |
2020-03-18T16:59:20.4704810Z LL | macro_rules! my_faulty_macro {
2020-03-18T16:59:20.4705450Z    | ---------------------------- when calling this macro
2020-03-18T16:59:20.4705770Z LL |     () => {
2020-03-18T16:59:20.4706060Z LL |         my_faulty_macro!(bcd); //~ ERROR no rules
2020-03-18T16:59:20.4706760Z ...
2020-03-18T16:59:20.4706760Z ...
2020-03-18T16:59:20.4706960Z LL |     my_faulty_macro!();
2020-03-18T16:59:20.4708230Z    |
2020-03-18T16:59:20.4709050Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-18T16:59:20.4709400Z 
2020-03-18T16:59:20.4709590Z note: trace_macro
2020-03-18T16:59:20.4709590Z note: trace_macro
2020-03-18T16:59:20.4710450Z   --> /Users/runner/runners/2.165.2/work/1/s/src/test/ui/macros/trace_faulty_macros.rs:33:5
2020-03-18T16:59:20.4710830Z    |
2020-03-18T16:59:20.4711030Z LL |     my_faulty_macro!();
2020-03-18T16:59:20.4711510Z    |
2020-03-18T16:59:20.4711510Z    |
2020-03-18T16:59:20.4711750Z    = note: expanding `my_faulty_macro! {  }`
2020-03-18T16:59:20.4712080Z    = note: to `my_faulty_macro ! (bcd) ;`
2020-03-18T16:59:20.4712410Z    = note: expanding `my_faulty_macro! { bcd }`
2020-03-18T16:59:20.4720170Z error: recursion limit reached while expanding `my_recursive_macro!`
2020-03-18T16:59:20.4721250Z   --> /Users/runner/runners/2.165.2/work/1/s/src/test/ui/macros/trace_faulty_macros.rs:22:9
2020-03-18T16:59:20.4721650Z    |
2020-03-18T16:59:20.4721650Z    |
2020-03-18T16:59:20.4721930Z LL |         my_recursive_macro!(); //~ ERROR recursion limit
2020-03-18T16:59:20.4722500Z ...
2020-03-18T16:59:20.4722500Z ...
2020-03-18T16:59:20.4722710Z LL |     my_recursive_macro!();
2020-03-18T16:59:20.4723970Z    |
2020-03-18T16:59:20.4723970Z    |
2020-03-18T16:59:20.4724350Z    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
2020-03-18T16:59:20.4725740Z 
2020-03-18T16:59:20.4725920Z note: trace_macro
2020-03-18T16:59:20.4726680Z   --> /Users/runner/runners/2.165.2/work/1/s/src/test/ui/macros/trace_faulty_macros.rs:34:5
2020-03-18T16:59:20.4727040Z    |
2020-03-18T16:59:20.4727040Z    |
2020-03-18T16:59:20.4727260Z LL |     my_recursive_macro!();
2020-03-18T16:59:20.4727750Z    |
2020-03-18T16:59:20.4727750Z    |
2020-03-18T16:59:20.4728010Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:59:20.4728340Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:59:20.4728670Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:59:20.4729000Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:59:20.4729340Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:59:20.4729670Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:59:20.4730000Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:59:20.4730330Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:59:20.4730540Z 
2020-03-18T16:59:20.4730830Z error: expected expression, found `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:59:20.4732090Z    |
2020-03-18T16:59:20.4732090Z    |
2020-03-18T16:59:20.4732340Z LL |         $a //~ ERROR expected expression
2020-03-18T16:59:20.4732880Z ...
2020-03-18T16:59:20.4733090Z LL |     let a = pat_macro!();
2020-03-18T16:59:20.4733750Z    |             ------------ in this macro invocation
2020-03-18T16:59:20.4734040Z    |
---
2020-03-18T16:59:20.4738550Z    |
2020-03-18T16:59:20.4738770Z LL |     let a = pat_macro!();
2020-03-18T16:59:20.4739270Z    |             ^^^^^^^^^^^^
2020-03-18T16:59:20.4739490Z    |
2020-03-18T16:59:20.4739720Z    = note: expanding `pat_macro! {  }`
2020-03-18T16:59:20.4740100Z    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T16:59:20.4740560Z    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T16:59:20.4740970Z    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:59:20.4741420Z error: aborting due to 4 previous errors
2020-03-18T16:59:20.4741610Z 
2020-03-18T16:59:20.4741730Z 
2020-03-18T16:59:20.4742340Z ------------------------------------------
---
2020-03-18T16:59:20.4758470Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T16:59:20.4759030Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T16:59:20.4773750Z 
2020-03-18T16:59:20.4774040Z 
2020-03-18T16:59:20.4780440Z command did not execute successfully: "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.165.2/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1100.0.30.12\nApple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15)\n" "--lldb-python-dir" "/Applications/Xcode_11.3.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T16:59:20.4784640Z 
2020-03-18T16:59:20.4784770Z 
2020-03-18T16:59:20.4788070Z failed to run: /Users/runner/runners/2.165.2/work/1/s/build/bootstrap/debug/bootstrap test
2020-03-18T16:59:20.4788580Z Build completed unsuccessfully in 0:55:38
2020-03-18T16:59:20.4788580Z Build completed unsuccessfully in 0:55:38
2020-03-18T16:59:20.4838840Z == clock drift check ==
2020-03-18T16:59:20.4892670Z   local time: Wed Mar 18 16:59:20 UTC 2020
2020-03-18T16:59:20.5465330Z   network time: Wed, 18 Mar 2020 16:59:20 GMT
2020-03-18T16:59:20.5466860Z == end clock drift check ==
2020-03-18T16:59:20.5507210Z 
2020-03-18T16:59:20.5580880Z ##[error]Bash exited with code '1'.
2020-03-18T16:59:20.5668800Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-18T16:59:20.5675050Z ==============================================================================
2020-03-18T16:59:20.5675470Z Task         : Get sources
2020-03-18T16:59:20.5676170Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
