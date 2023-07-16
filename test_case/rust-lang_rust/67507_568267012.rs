plain
2019-12-22T13:10:58.5095431Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T13:10:59.1332505Z ##[command]git config gc.auto 0
2019-12-22T13:10:59.1338786Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T13:10:59.1344259Z ##[command]git config --get-all http.proxy
2019-12-22T13:10:59.1348368Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67507/merge:refs/remotes/pull/67507/merge
---
2019-12-22T14:05:07.6377841Z .................................................................................................... 1600/9427
2019-12-22T14:05:11.8293329Z .................................................................................................... 1700/9427
2019-12-22T14:05:21.2224361Z .....................................................................................i.............. 1800/9427
2019-12-22T14:05:28.0807060Z .................................................................................................... 1900/9427
2019-12-22T14:05:34.4542036Z ......................................................................iiiii......................... 2000/9427
2019-12-22T14:05:53.1772398Z .................................................................................................... 2200/9427
2019-12-22T14:05:55.3279458Z .................................................................................................... 2300/9427
2019-12-22T14:05:57.6987817Z .................................................................................................... 2400/9427
2019-12-22T14:06:09.6616360Z .................................................................................................... 2500/9427
---
2019-12-22T14:08:42.7506178Z .i...............i.................................................................................. 4900/9427
2019-12-22T14:08:51.6959511Z .................................................................................................... 5000/9427
2019-12-22T14:08:56.0263559Z .............................................i...................................................... 5100/9427
2019-12-22T14:09:04.6994738Z .................................................................................................... 5200/9427
2019-12-22T14:09:09.9370063Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-22T14:09:17.9795418Z .................................................................................................... 5500/9427
2019-12-22T14:09:28.4849282Z ..............................................................................................i..... 5600/9427
2019-12-22T14:09:35.6756303Z .................................................................................................... 5700/9427
2019-12-22T14:09:40.2471499Z .................................................................................................... 5800/9427
2019-12-22T14:09:40.2471499Z .................................................................................................... 5800/9427
2019-12-22T14:09:49.0780015Z ..................................................................................ii...i..ii........ 5900/9427
2019-12-22T14:10:09.6859328Z .................................................................................................... 6100/9427
2019-12-22T14:10:16.4583581Z .................................................................................................... 6200/9427
2019-12-22T14:10:23.5218369Z .................................................................................................... 6300/9427
2019-12-22T14:10:23.5218369Z .................................................................................................... 6300/9427
2019-12-22T14:10:38.7214290Z .........i..ii...................................................................................... 6400/9427
2019-12-22T14:10:56.1255439Z .....................................................................................i.............. 6600/9427
2019-12-22T14:10:57.9834345Z .................................................................................................... 6700/9427
2019-12-22T14:10:59.9150189Z .....................................................................................i.............. 6800/9427
2019-12-22T14:11:02.3660994Z .................................................................................................... 6900/9427
---
2019-12-22T14:15:18.0678707Z diff of stderr:
2019-12-22T14:15:18.0679097Z 
2019-12-22T14:15:18.0679345Z 7    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0679577Z 8 
2019-12-22T14:15:18.0680096Z 9 error[E0004]: non-exhaustive patterns: type `&Void` is non-empty
2019-12-22T14:15:18.0681503Z +   --> $DIR/uninhabited-matches-feature-gated.rs:16:19
2019-12-22T14:15:18.0681793Z 11    |
2019-12-22T14:15:18.0682705Z 12 LL | enum Void {}
2019-12-22T14:15:18.0682705Z 12 LL | enum Void {}
2019-12-22T14:15:18.0683262Z 13    | ------------ `Void` defined here
2019-12-22T14:15:18.0683794Z 18    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0684053Z 19 
2019-12-22T14:15:18.0684053Z 19 
2019-12-22T14:15:18.0684580Z 20 error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty
2019-12-22T14:15:18.0685734Z +   --> $DIR/uninhabited-matches-feature-gated.rs:19:19
2019-12-22T14:15:18.0686026Z 22    |
2019-12-22T14:15:18.0686263Z 23 LL |     let _ = match x {};
2019-12-22T14:15:18.0686663Z 24    |                   ^
2019-12-22T14:15:18.0686663Z 24    |                   ^
2019-12-22T14:15:18.0686891Z 
2019-12-22T14:15:18.0687273Z 26    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0687687Z 27 
2019-12-22T14:15:18.0688502Z 28 error[E0004]: non-exhaustive patterns: type `[Void; 1]` is non-empty
2019-12-22T14:15:18.0689566Z +   --> $DIR/uninhabited-matches-feature-gated.rs:22:19
2019-12-22T14:15:18.0689837Z 30    |
2019-12-22T14:15:18.0690057Z 31 LL |     let _ = match x {};
2019-12-22T14:15:18.0690258Z 32    |                   ^
2019-12-22T14:15:18.0690258Z 32    |                   ^
2019-12-22T14:15:18.0690432Z 
2019-12-22T14:15:18.0690659Z 34    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0690861Z 35 
2019-12-22T14:15:18.0691489Z 36 error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
2019-12-22T14:15:18.0694732Z +   --> $DIR/uninhabited-matches-feature-gated.rs:25:19
2019-12-22T14:15:18.0695098Z 38    |
2019-12-22T14:15:18.0695360Z 39 LL |     let _ = match x {
2019-12-22T14:15:18.0695360Z 39 LL |     let _ = match x {
2019-12-22T14:15:18.0695741Z 40    |                   ^ pattern `&[_, ..]` not covered
2019-12-22T14:15:18.0697500Z 42    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0697693Z 43 
2019-12-22T14:15:18.0698190Z 44 error[E0004]: non-exhaustive patterns: `Err(_)` not covered
2019-12-22T14:15:18.0698692Z -   --> $DIR/uninhabited-matches-feature-gated.rs:32:19
---
2019-12-22T14:15:18.0703720Z 56    |         ^^^^^ pattern `Err(_)` not covered
2019-12-22T14:15:18.0703864Z 
2019-12-22T14:15:18.0703991Z 
2019-12-22T14:15:18.0704164Z The actual stderr differed from the expected stderr.
2019-12-22T14:15:18.0704766Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/uninhabited-matches-feature-gated.stderr
2019-12-22T14:15:18.0705278Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T14:15:18.0705906Z To only update this specific test, also pass `--test-args uninhabited/uninhabited-matches-feature-gated.rs`
2019-12-22T14:15:18.0706392Z error: 1 errors occurred comparing output.
2019-12-22T14:15:18.0706513Z status: exit code: 1
2019-12-22T14:15:18.0706513Z status: exit code: 1
2019-12-22T14:15:18.0707670Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/auxiliary" "-A" "unused"
2019-12-22T14:15:18.0708252Z ------------------------------------------
2019-12-22T14:15:18.0708414Z 
2019-12-22T14:15:18.0708748Z ------------------------------------------
2019-12-22T14:15:18.0709086Z stderr:
---
2019-12-22T14:15:18.0710884Z    |                   ^ pattern `Err(_)` not covered
2019-12-22T14:15:18.0711006Z    |
2019-12-22T14:15:18.0711309Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0711574Z 
2019-12-22T14:15:18.0713980Z error[E0004]: non-exhaustive patterns: type `&Void` is non-empty
2019-12-22T14:15:18.0715158Z    |
2019-12-22T14:15:18.0715309Z LL | enum Void {}
2019-12-22T14:15:18.0715309Z LL | enum Void {}
2019-12-22T14:15:18.0715873Z    | ------------ `Void` defined here
2019-12-22T14:15:18.0716029Z ...
2019-12-22T14:15:18.0716361Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:15:18.0716631Z    |
2019-12-22T14:15:18.0716772Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0716878Z 
2019-12-22T14:15:18.0716878Z 
2019-12-22T14:15:18.0717202Z error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty
2019-12-22T14:15:18.0717750Z    |
2019-12-22T14:15:18.0717750Z    |
2019-12-22T14:15:18.0718057Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:15:18.0718519Z    |
2019-12-22T14:15:18.0718641Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0718774Z 
2019-12-22T14:15:18.0718774Z 
2019-12-22T14:15:18.0719779Z error[E0004]: non-exhaustive patterns: type `[Void; 1]` is non-empty
2019-12-22T14:15:18.0720417Z    |
2019-12-22T14:15:18.0720417Z    |
2019-12-22T14:15:18.0720730Z LL |     let _ = match x {}; //~ ERROR non-exhaustive
2019-12-22T14:15:18.0721014Z    |
2019-12-22T14:15:18.0721134Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0721238Z 
2019-12-22T14:15:18.0721238Z 
2019-12-22T14:15:18.0721576Z error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
2019-12-22T14:15:18.0722570Z    |
2019-12-22T14:15:18.0723056Z LL |     let _ = match x {   //~ ERROR non-exhaustive
2019-12-22T14:15:18.0723056Z LL |     let _ = match x {   //~ ERROR non-exhaustive
2019-12-22T14:15:18.0723425Z    |                   ^ pattern `&[_, ..]` not covered
2019-12-22T14:15:18.0723750Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-22T14:15:18.0723883Z 
2019-12-22T14:15:18.0724330Z error[E0004]: non-exhaustive patterns: `Err(_)` not covered
2019-12-22T14:15:18.0724809Z   --> /checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs:33:19
---
2019-12-22T14:15:18.0727022Z    |
2019-12-22T14:15:18.0727142Z LL |     let Ok(x) = x;
2019-12-22T14:15:18.0727263Z    |         ^^^^^ pattern `Err(_)` not covered
2019-12-22T14:15:18.0727381Z    |
2019-12-22T14:15:18.0727528Z    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
2019-12-22T14:15:18.0728328Z help: you might want to use `if let` to ignore the variant that isn't matched
2019-12-22T14:15:18.0728487Z    |
2019-12-22T14:15:18.0728607Z LL |     if let Ok(x) = x { /* */ }
2019-12-22T14:15:18.0728742Z    |
---
2019-12-22T14:15:18.0732434Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-22T14:15:18.0732702Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T14:15:18.0732925Z 
2019-12-22T14:15:18.0733058Z 
2019-12-22T14:15:18.0735037Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-22T14:15:18.0736079Z 
2019-12-22T14:15:18.0736172Z 
2019-12-22T14:15:18.0736307Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-22T14:15:18.0736419Z Build completed unsuccessfully in 0:58:49
2019-12-22T14:15:18.0736419Z Build completed unsuccessfully in 0:58:49
2019-12-22T14:15:18.0768181Z == clock drift check ==
2019-12-22T14:15:18.0781477Z   local time: Sun Dec 22 14:15:18 UTC 2019
2019-12-22T14:15:18.6187949Z   network time: Sun, 22 Dec 2019 14:15:18 GMT
2019-12-22T14:15:18.6192395Z == end clock drift check ==
2019-12-22T14:15:19.5538142Z 
2019-12-22T14:15:19.5638706Z ##[error]Bash exited with code '1'.
2019-12-22T14:15:19.5714938Z ##[section]Starting: Checkout
2019-12-22T14:15:19.5717068Z ==============================================================================
2019-12-22T14:15:19.5717142Z Task         : Get sources
2019-12-22T14:15:19.5717191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
