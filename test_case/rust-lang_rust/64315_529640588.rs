plain
2019-09-09T18:47:22.9198161Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T18:47:22.9391257Z ##[command]git config gc.auto 0
2019-09-09T18:47:22.9467951Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T18:47:22.9541161Z ##[command]git config --get-all http.proxy
2019-09-09T18:47:22.9667267Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64315/merge:refs/remotes/pull/64315/merge
---
2019-09-09T19:46:16.9515799Z .................................................................................................... 1500/9003
2019-09-09T19:46:22.0497635Z .................................................................................................... 1600/9003
2019-09-09T19:46:33.5801170Z .....................................................i...............i.............................. 1700/9003
2019-09-09T19:46:40.6948372Z .................................................................................................... 1800/9003
2019-09-09T19:46:53.6342572Z ............................................iiiii................................................... 1900/9003
2019-09-09T19:47:03.5188473Z .................................................................................................... 2100/9003
2019-09-09T19:47:05.8066888Z .................................................................................................... 2200/9003
2019-09-09T19:47:09.3202906Z .................................................................................................... 2300/9003
2019-09-09T19:47:16.3816055Z .................................................................................................... 2400/9003
---
2019-09-09T19:50:00.4214053Z ...............................i...............i.................................................... 4700/9003
2019-09-09T19:50:11.6926798Z .................................................................................................... 4800/9003
2019-09-09T19:50:17.4860555Z .................................................................................................... 4900/9003
2019-09-09T19:50:27.5244986Z .................................................................................................... 5000/9003
2019-09-09T19:50:33.2131440Z .............ii.ii.................................................................................. 5100/9003
2019-09-09T19:50:43.2000178Z .................................................................................................... 5300/9003
2019-09-09T19:50:52.7817267Z ............................................................................i....................... 5400/9003
2019-09-09T19:50:59.8572350Z .................................................................................................... 5500/9003
2019-09-09T19:51:05.7284285Z .................................................................................................... 5600/9003
2019-09-09T19:51:05.7284285Z .................................................................................................... 5600/9003
2019-09-09T19:51:15.7278606Z ......................................................................ii...i..ii...........i........ 5700/9003
2019-09-09T19:51:39.1532940Z .................................................................................................... 5900/9003
2019-09-09T19:51:47.9389324Z .................................................................................................... 6000/9003
2019-09-09T19:51:47.9389324Z .................................................................................................... 6000/9003
2019-09-09T19:51:53.8300438Z ........................................................................i..ii....................... 6100/9003
2019-09-09T19:52:21.2329386Z .................................................................................................... 6300/9003
2019-09-09T19:52:23.1184392Z ...............................i.................................................................... 6400/9003
2019-09-09T19:52:25.1705295Z .................................................................................................... 6500/9003
2019-09-09T19:52:27.6380488Z ...i................................................................................................ 6600/9003
---
2019-09-09T19:56:55.2154257Z  finished in 18.640
2019-09-09T19:56:55.2154704Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T19:56:55.2154736Z 
2019-09-09T19:56:55.2154787Z running 150 tests
2019-09-09T19:56:58.1244810Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T19:56:59.8930764Z ..iiii..............i.........iii.i.......ii......
2019-09-09T19:56:59.8931579Z 
2019-09-09T19:56:59.8931637Z  finished in 4.887
2019-09-09T19:56:59.9097259Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T19:57:00.0515157Z 
---
2019-09-09T19:57:01.9322835Z  finished in 2.022
2019-09-09T19:57:01.9481583Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T19:57:02.0902594Z 
2019-09-09T19:57:02.0902775Z running 9 tests
2019-09-09T19:57:02.0903795Z iiiiiiiii
2019-09-09T19:57:02.0904067Z 
2019-09-09T19:57:02.0904101Z  finished in 0.142
2019-09-09T19:57:02.1053364Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T19:57:02.2481642Z 
2019-09-09T19:57:02.2481642Z 
2019-09-09T19:57:02.2481818Z running 104 tests
2019-09-09T19:57:18.1026023Z ............................F...F................................................................... 100/104
2019-09-09T19:57:18.6948633Z ....
2019-09-09T19:57:18.6950422Z failures:
2019-09-09T19:57:18.6950816Z 
2019-09-09T19:57:18.6953572Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-09-09T19:57:18.6955197Z 
2019-09-09T19:57:18.6955867Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-09T19:57:18.6956073Z status: exit code: 1
2019-09-09T19:57:18.6957072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-09-09T19:57:18.6957893Z ------------------------------------------
2019-09-09T19:57:18.6958031Z 
2019-09-09T19:57:18.6958343Z ------------------------------------------
2019-09-09T19:57:18.6958495Z stderr:
2019-09-09T19:57:18.6958495Z stderr:
2019-09-09T19:57:18.6958768Z ------------------------------------------
2019-09-09T19:57:18.6958953Z error: `mir_built(make_extern)` should be clean but is not
2019-09-09T19:57:18.6960131Z    |
2019-09-09T19:57:18.6960131Z    |
2019-09-09T19:57:18.6960312Z LL | pub extern "C" fn make_extern() {}
2019-09-09T19:57:18.6960613Z 
2019-09-09T19:57:18.6962098Z error: aborting due to previous error
2019-09-09T19:57:18.6962262Z 
2019-09-09T19:57:18.6962394Z 
2019-09-09T19:57:18.6962394Z 
2019-09-09T19:57:18.6963418Z ------------------------------------------
2019-09-09T19:57:18.6964064Z 
2019-09-09T19:57:18.6964249Z 
2019-09-09T19:57:18.6964907Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2019-09-09T19:57:18.6965161Z 
2019-09-09T19:57:18.6965503Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-09T19:57:18.6965659Z status: exit code: 1
2019-09-09T19:57:18.6966730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2019-09-09T19:57:18.6967330Z ------------------------------------------
2019-09-09T19:57:18.6967470Z 
2019-09-09T19:57:18.6967778Z ------------------------------------------
2019-09-09T19:57:18.6967925Z stderr:
2019-09-09T19:57:18.6967925Z stderr:
2019-09-09T19:57:18.6968209Z ------------------------------------------
2019-09-09T19:57:18.6968385Z error: `mir_built(Foo::make_method_extern)` should be clean but is not
2019-09-09T19:57:18.6968864Z    |
2019-09-09T19:57:18.6968864Z    |
2019-09-09T19:57:18.6969016Z LL |     pub extern fn make_method_extern(&self) { }
2019-09-09T19:57:18.6969434Z 
2019-09-09T19:57:18.6970111Z error: aborting due to previous error
2019-09-09T19:57:18.6970244Z 
2019-09-09T19:57:18.6970373Z 
---
2019-09-09T19:57:18.6972892Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T19:57:18.6973542Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T19:57:18.6973700Z 
2019-09-09T19:57:18.6973830Z 
2019-09-09T19:57:18.6975138Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T19:57:18.6975553Z 
2019-09-09T19:57:18.6975658Z 
2019-09-09T19:57:18.6975775Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T19:57:18.6976000Z Build completed unsuccessfully in 1:02:45
2019-09-09T19:57:18.6976000Z Build completed unsuccessfully in 1:02:45
2019-09-09T19:57:18.7014381Z == clock drift check ==
2019-09-09T19:57:18.7027009Z   local time: Mon Sep  9 19:57:18 UTC 2019
2019-09-09T19:57:18.8869035Z   network time: Mon, 09 Sep 2019 19:57:18 GMT
2019-09-09T19:57:18.8875802Z == end clock drift check ==
2019-09-09T19:57:23.0235533Z ##[error]Bash exited with code '1'.
2019-09-09T19:57:23.0273068Z ##[section]Starting: Checkout
2019-09-09T19:57:23.0274954Z ==============================================================================
2019-09-09T19:57:23.0274998Z Task         : Get sources
2019-09-09T19:57:23.0275053Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
