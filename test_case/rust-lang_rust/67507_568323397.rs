plain
2019-12-23T00:16:31.8502936Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T00:16:31.8519373Z ##[command]git config gc.auto 0
2019-12-23T00:16:31.8521993Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T00:16:31.8523883Z ##[command]git config --get-all http.proxy
2019-12-23T00:16:31.8526868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67507/merge:refs/remotes/pull/67507/merge
---
2019-12-23T01:13:13.9199897Z .................................................................................................... 1600/9427
2019-12-23T01:13:18.3898379Z .................................................................................................... 1700/9427
2019-12-23T01:13:28.4297830Z .....................................................................................i.............. 1800/9427
2019-12-23T01:13:35.6128548Z .................................................................................................... 1900/9427
2019-12-23T01:13:42.1935179Z ......................................................................iiiii......................... 2000/9427
2019-12-23T01:14:01.5776232Z .................................................................................................... 2200/9427
2019-12-23T01:14:03.8522864Z .................................................................................................... 2300/9427
2019-12-23T01:14:06.4447942Z .................................................................................................... 2400/9427
2019-12-23T01:14:18.8139193Z .................................................................................................... 2500/9427
---
2019-12-23T01:17:06.2998991Z .i...............i.................................................................................. 4900/9427
2019-12-23T01:17:16.1227793Z .................................................................................................... 5000/9427
2019-12-23T01:17:20.8208322Z .............................................i...................................................... 5100/9427
2019-12-23T01:17:30.1064741Z .................................................................................................... 5200/9427
2019-12-23T01:17:35.7482295Z ............ii.ii...........i....................................................................... 5300/9427
2019-12-23T01:17:44.5307318Z .................................................................................................... 5500/9427
2019-12-23T01:17:55.2174499Z ..............................................................................................i..... 5600/9427
2019-12-23T01:18:03.0362637Z .................................................................................................... 5700/9427
2019-12-23T01:18:08.0379005Z .................................................................................................... 5800/9427
2019-12-23T01:18:08.0379005Z .................................................................................................... 5800/9427
2019-12-23T01:18:17.5271823Z ..................................................................................ii...i...ii....... 5900/9427
2019-12-23T01:18:39.4272627Z .................................................................................................... 6100/9427
2019-12-23T01:18:46.8627491Z .................................................................................................... 6200/9427
2019-12-23T01:18:54.6000628Z .................................................................................................... 6300/9427
2019-12-23T01:18:54.6000628Z .................................................................................................... 6300/9427
2019-12-23T01:19:09.9886624Z .........i..ii...................................................................................... 6400/9427
2019-12-23T01:19:28.8417875Z .....................................................................................i.............. 6600/9427
2019-12-23T01:19:31.0351837Z .................................................................................................... 6700/9427
2019-12-23T01:19:33.3372571Z .....................................................................................i.............. 6800/9427
2019-12-23T01:19:36.1336188Z .................................................................................................... 6900/9427
---
2019-12-23T01:21:06.5780446Z .................................................................................................... 7400/9427
2019-12-23T01:21:11.3240464Z .................................................................................................... 7500/9427
2019-12-23T01:21:16.4095020Z .................................................................................................... 7600/9427
2019-12-23T01:21:23.2540474Z .................................................................................................... 7700/9427
2019-12-23T01:21:33.5174575Z ...................................................................................................i 7800/9427
2019-12-23T01:21:40.0005167Z iii................................................................................................. 7900/9427
2019-12-23T01:21:53.5928138Z .................................................................................................... 8100/9427
2019-12-23T01:22:04.9106297Z .................................................................................................... 8200/9427
2019-12-23T01:22:16.4218888Z .................................................................................................... 8300/9427
2019-12-23T01:22:22.1104955Z .................................................................................................... 8400/9427
---
2019-12-23T01:24:13.0245166Z ---- [ui] ui/abi/stack-probes-lto.rs stdout ----
2019-12-23T01:24:13.0245207Z 
2019-12-23T01:24:13.0245991Z error: test compilation failed although it shouldn't!
2019-12-23T01:24:13.0246225Z status: exit code: 1
2019-12-23T01:24:13.0247120Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/auxiliary"
2019-12-23T01:24:13.0247744Z ------------------------------------------
2019-12-23T01:24:13.0247928Z 
2019-12-23T01:24:13.0248244Z ------------------------------------------
2019-12-23T01:24:13.0248294Z stderr:
2019-12-23T01:24:13.0248294Z stderr:
2019-12-23T01:24:13.0248665Z ------------------------------------------
2019-12-23T01:24:13.0249097Z error[E0433]: failed to resolve: use of undeclared type or module `MaybeUninit`
2019-12-23T01:24:13.0249426Z   --> /checkout/src/test/ui/abi/stack-probes.rs:55:43
2019-12-23T01:24:13.0249502Z    |
2019-12-23T01:24:13.0249549Z LL |     let local: MaybeUninit<[u64; 1024]> = MaybeUninit::uninit();
2019-12-23T01:24:13.0249834Z 
2019-12-23T01:24:13.0250089Z error[E0412]: cannot find type `MaybeUninit` in this scope
2019-12-23T01:24:13.0250394Z   --> /checkout/src/test/ui/abi/stack-probes.rs:51:20
2019-12-23T01:24:13.0250610Z    |
2019-12-23T01:24:13.0250610Z    |
2019-12-23T01:24:13.0250683Z LL | fn recurse(array: &MaybeUninit<[u64; 1024]>) {
2019-12-23T01:24:13.0250834Z    |
2019-12-23T01:24:13.0250902Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T01:24:13.0250966Z    |
2019-12-23T01:24:13.0251049Z LL | use std::mem::MaybeUninit;
2019-12-23T01:24:13.0251049Z LL | use std::mem::MaybeUninit;
2019-12-23T01:24:13.0251090Z    |
2019-12-23T01:24:13.0251138Z 
2019-12-23T01:24:13.0251224Z error[E0412]: cannot find type `MaybeUninit` in this scope
2019-12-23T01:24:13.0251642Z   --> /checkout/src/test/ui/abi/stack-probes.rs:55:16
2019-12-23T01:24:13.0251830Z    |
2019-12-23T01:24:13.0251926Z LL |     let local: MaybeUninit<[u64; 1024]> = MaybeUninit::uninit();
2019-12-23T01:24:13.0252052Z    |
2019-12-23T01:24:13.0252140Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T01:24:13.0252205Z    |
2019-12-23T01:24:13.0252265Z LL | use std::mem::MaybeUninit;
---
2019-12-23T01:24:13.0253932Z ---- [ui] ui/abi/stack-probes.rs stdout ----
2019-12-23T01:24:13.0253967Z 
2019-12-23T01:24:13.0254176Z error: test compilation failed although it shouldn't!
2019-12-23T01:24:13.0254222Z status: exit code: 1
2019-12-23T01:24:13.0255161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/auxiliary"
2019-12-23T01:24:13.0255485Z ------------------------------------------
2019-12-23T01:24:13.0255848Z 
2019-12-23T01:24:13.0256204Z ------------------------------------------
2019-12-23T01:24:13.0256267Z stderr:
2019-12-23T01:24:13.0256267Z stderr:
2019-12-23T01:24:13.0256488Z ------------------------------------------
2019-12-23T01:24:13.0256700Z error[E0433]: failed to resolve: use of undeclared type or module `MaybeUninit`
2019-12-23T01:24:13.0257270Z   --> /checkout/src/test/ui/abi/stack-probes.rs:55:43
2019-12-23T01:24:13.0257348Z    |
2019-12-23T01:24:13.0297376Z LL |     let local: MaybeUninit<[u64; 1024]> = MaybeUninit::uninit();
2019-12-23T01:24:13.0302626Z 
2019-12-23T01:24:13.0302821Z error[E0412]: cannot find type `MaybeUninit` in this scope
2019-12-23T01:24:13.0303388Z   --> /checkout/src/test/ui/abi/stack-probes.rs:51:20
2019-12-23T01:24:13.0303503Z    |
2019-12-23T01:24:13.0303503Z    |
2019-12-23T01:24:13.0303553Z LL | fn recurse(array: &MaybeUninit<[u64; 1024]>) {
2019-12-23T01:24:13.0303666Z    |
2019-12-23T01:24:13.0303734Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T01:24:13.0303781Z    |
2019-12-23T01:24:13.0303824Z LL | use std::mem::MaybeUninit;
2019-12-23T01:24:13.0303824Z LL | use std::mem::MaybeUninit;
2019-12-23T01:24:13.0303884Z    |
2019-12-23T01:24:13.0304057Z 
2019-12-23T01:24:13.0304115Z error[E0412]: cannot find type `MaybeUninit` in this scope
2019-12-23T01:24:13.0304412Z   --> /checkout/src/test/ui/abi/stack-probes.rs:55:16
2019-12-23T01:24:13.0304481Z    |
2019-12-23T01:24:13.0304528Z LL |     let local: MaybeUninit<[u64; 1024]> = MaybeUninit::uninit();
2019-12-23T01:24:13.0304641Z    |
2019-12-23T01:24:13.0304689Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T01:24:13.0304734Z    |
2019-12-23T01:24:13.0304793Z LL | use std::mem::MaybeUninit;
---
2019-12-23T01:24:13.0307032Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T01:24:13.0307092Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T01:24:13.0307175Z 
2019-12-23T01:24:13.0307203Z 
2019-12-23T01:24:13.0309256Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T01:24:13.0309519Z 
2019-12-23T01:24:13.0309550Z 
2019-12-23T01:24:13.0314095Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T01:24:13.0314341Z Build completed unsuccessfully in 1:01:49
2019-12-23T01:24:13.0314341Z Build completed unsuccessfully in 1:01:49
2019-12-23T01:24:13.0365892Z == clock drift check ==
2019-12-23T01:24:13.0387705Z   local time: Mon Dec 23 01:24:13 UTC 2019
2019-12-23T01:24:13.3096898Z   network time: Mon, 23 Dec 2019 01:24:13 GMT
2019-12-23T01:24:13.3100595Z == end clock drift check ==
2019-12-23T01:24:14.5684697Z 
2019-12-23T01:24:14.5752647Z ##[error]Bash exited with code '1'.
2019-12-23T01:24:14.5796689Z ##[section]Starting: Checkout
2019-12-23T01:24:14.5798587Z ==============================================================================
2019-12-23T01:24:14.5798632Z Task         : Get sources
2019-12-23T01:24:14.5798670Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
