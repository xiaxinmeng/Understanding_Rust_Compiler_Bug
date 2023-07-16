plain
2019-12-23T01:37:53.4512783Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T01:37:53.4742811Z ##[command]git config gc.auto 0
2019-12-23T01:37:53.4819907Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T01:37:53.4876320Z ##[command]git config --get-all http.proxy
2019-12-23T01:37:53.5047889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67507/merge:refs/remotes/pull/67507/merge
---
2019-12-23T02:37:38.1822748Z .................................................................................................... 1600/9427
2019-12-23T02:37:43.1451121Z .................................................................................................... 1700/9427
2019-12-23T02:37:54.5800466Z .....................................................................................i.............. 1800/9427
2019-12-23T02:38:02.6097654Z .................................................................................................... 1900/9427
2019-12-23T02:38:10.2988000Z .......................................................................iiiii........................ 2000/9427
2019-12-23T02:38:31.6674168Z .................................................................................................... 2200/9427
2019-12-23T02:38:34.2537730Z .................................................................................................... 2300/9427
2019-12-23T02:38:37.2050263Z .................................................................................................... 2400/9427
2019-12-23T02:38:50.9881185Z .................................................................................................... 2500/9427
---
2019-12-23T02:41:44.3018887Z .i...............i.................................................................................. 4900/9427
2019-12-23T02:41:54.5081129Z .................................................................................................... 5000/9427
2019-12-23T02:41:59.5578432Z .............................................i...................................................... 5100/9427
2019-12-23T02:42:09.7381531Z .................................................................................................... 5200/9427
2019-12-23T02:42:15.7650714Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-23T02:42:25.6827879Z .................................................................................................... 5500/9427
2019-12-23T02:42:37.3812198Z ..............................................................................................i..... 5600/9427
2019-12-23T02:42:46.0319811Z .................................................................................................... 5700/9427
2019-12-23T02:42:51.5242802Z .................................................................................................... 5800/9427
2019-12-23T02:42:51.5242802Z .................................................................................................... 5800/9427
2019-12-23T02:43:01.4071115Z ..................................................................................ii...i..ii........ 5900/9427
2019-12-23T02:43:24.2077429Z .................................................................................................... 6100/9427
2019-12-23T02:43:31.9974737Z .................................................................................................... 6200/9427
2019-12-23T02:43:40.0896999Z .................................................................................................... 6300/9427
2019-12-23T02:43:40.0896999Z .................................................................................................... 6300/9427
2019-12-23T02:43:57.3560855Z .........i..ii...................................................................................... 6400/9427
2019-12-23T02:44:17.8503854Z .....................................................................................i.............. 6600/9427
2019-12-23T02:44:20.0572087Z .................................................................................................... 6700/9427
2019-12-23T02:44:22.2737333Z .....................................................................................i.............. 6800/9427
2019-12-23T02:44:25.0262773Z .................................................................................................... 6900/9427
---
2019-12-23T02:49:10.2203578Z ---- [ui] ui/abi/stack-probes-lto.rs stdout ----
2019-12-23T02:49:10.2203881Z 
2019-12-23T02:49:10.2204418Z error: test compilation failed although it shouldn't!
2019-12-23T02:49:10.2204721Z status: exit code: 1
2019-12-23T02:49:10.2205792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/auxiliary"
2019-12-23T02:49:10.2206657Z ------------------------------------------
2019-12-23T02:49:10.2206928Z 
2019-12-23T02:49:10.2207409Z ------------------------------------------
2019-12-23T02:49:10.2207724Z stderr:
2019-12-23T02:49:10.2207724Z stderr:
2019-12-23T02:49:10.2208185Z ------------------------------------------
2019-12-23T02:49:10.2208472Z error[E0308]: mismatched types
2019-12-23T02:49:10.2208962Z   --> /checkout/src/test/ui/abi/stack-probes.rs:31:38
2019-12-23T02:49:10.2209247Z    |
2019-12-23T02:49:10.2209721Z LL |             "main-thread" => recurse(&[]),
2019-12-23T02:49:10.2210043Z    |                                      ^^^ expected union `std::mem::MaybeUninit`, found array of 0 elements 
2019-12-23T02:49:10.2210575Z    = note: expected reference `&std::mem::MaybeUninit<[u64; 1024]>`
2019-12-23T02:49:10.2210804Z               found reference `&[_; 0]`
2019-12-23T02:49:10.2210997Z 
2019-12-23T02:49:10.2211211Z error[E0308]: mismatched types
2019-12-23T02:49:10.2211211Z error[E0308]: mismatched types
2019-12-23T02:49:10.2211711Z   --> /checkout/src/test/ui/abi/stack-probes.rs:32:56
2019-12-23T02:49:10.2211998Z    |
2019-12-23T02:49:10.2212526Z LL |             "child-thread" => thread::spawn(|| recurse(&[])).join().unwrap(),
2019-12-23T02:49:10.2212834Z    |                                                        ^^^ expected union `std::mem::MaybeUninit`, found array of 0 elements 
2019-12-23T02:49:10.2213751Z    = note: expected reference `&std::mem::MaybeUninit<[u64; 1024]>`
2019-12-23T02:49:10.2213977Z               found reference `&[_; 0]`
2019-12-23T02:49:10.2214167Z 
2019-12-23T02:49:10.2214402Z error: aborting due to 2 previous errors
---
2019-12-23T02:49:10.2217168Z ---- [ui] ui/abi/stack-probes.rs stdout ----
2019-12-23T02:49:10.2217431Z 
2019-12-23T02:49:10.2217900Z error: test compilation failed although it shouldn't!
2019-12-23T02:49:10.2218211Z status: exit code: 1
2019-12-23T02:49:10.2219221Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/auxiliary"
2019-12-23T02:49:10.2220252Z ------------------------------------------
2019-12-23T02:49:10.2220516Z 
2019-12-23T02:49:10.2221013Z ------------------------------------------
2019-12-23T02:49:10.2221293Z stderr:
2019-12-23T02:49:10.2221293Z stderr:
2019-12-23T02:49:10.2221740Z ------------------------------------------
2019-12-23T02:49:10.2222647Z error[E0308]: mismatched types
2019-12-23T02:49:10.2223209Z   --> /checkout/src/test/ui/abi/stack-probes.rs:31:38
2019-12-23T02:49:10.2223421Z    |
2019-12-23T02:49:10.2223790Z LL |             "main-thread" => recurse(&[]),
2019-12-23T02:49:10.2224010Z    |                                      ^^^ expected union `std::mem::MaybeUninit`, found array of 0 elements 
2019-12-23T02:49:10.2224319Z    = note: expected reference `&std::mem::MaybeUninit<[u64; 1024]>`
2019-12-23T02:49:10.2224462Z               found reference `&[_; 0]`
2019-12-23T02:49:10.2224594Z 
2019-12-23T02:49:10.2224748Z error[E0308]: mismatched types
2019-12-23T02:49:10.2224748Z error[E0308]: mismatched types
2019-12-23T02:49:10.2225141Z   --> /checkout/src/test/ui/abi/stack-probes.rs:32:56
2019-12-23T02:49:10.2225325Z    |
2019-12-23T02:49:10.2226134Z LL |             "child-thread" => thread::spawn(|| recurse(&[])).join().unwrap(),
2019-12-23T02:49:10.2226396Z    |                                                        ^^^ expected union `std::mem::MaybeUninit`, found array of 0 elements 
2019-12-23T02:49:10.2226699Z    = note: expected reference `&std::mem::MaybeUninit<[u64; 1024]>`
2019-12-23T02:49:10.2226839Z               found reference `&[_; 0]`
2019-12-23T02:49:10.2226955Z 
2019-12-23T02:49:10.2227190Z error: aborting due to 2 previous errors
---
2019-12-23T02:49:10.2258622Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T02:49:10.2259021Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T02:49:10.2277553Z 
2019-12-23T02:49:10.2277939Z 
2019-12-23T02:49:10.2280378Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T02:49:10.2281786Z 
2019-12-23T02:49:10.2281981Z 
2019-12-23T02:49:10.2342176Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T02:49:10.2342312Z Build completed unsuccessfully in 1:04:49
2019-12-23T02:49:10.2342312Z Build completed unsuccessfully in 1:04:49
2019-12-23T02:49:10.2349864Z == clock drift check ==
2019-12-23T02:49:10.2376269Z   local time: Mon Dec 23 02:49:10 UTC 2019
2019-12-23T02:49:10.7747278Z   network time: Mon, 23 Dec 2019 02:49:10 GMT
2019-12-23T02:49:10.7753330Z == end clock drift check ==
2019-12-23T02:49:11.8271186Z 
2019-12-23T02:49:11.8462766Z ##[error]Bash exited with code '1'.
2019-12-23T02:49:11.8549692Z ##[section]Starting: Checkout
2019-12-23T02:49:11.8551784Z ==============================================================================
2019-12-23T02:49:11.8551841Z Task         : Get sources
2019-12-23T02:49:11.8551927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
