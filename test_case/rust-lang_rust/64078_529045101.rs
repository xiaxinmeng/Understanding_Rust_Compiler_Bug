plain
2019-09-06T22:14:57.9917073Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T22:14:58.0118538Z ##[command]git config gc.auto 0
2019-09-06T22:14:58.0189882Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T22:14:58.0250582Z ##[command]git config --get-all http.proxy
2019-09-06T22:14:58.0409300Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64078/merge:refs/remotes/pull/64078/merge
---
2019-09-06T23:15:21.8113942Z .................................................................................................... 1500/9004
2019-09-06T23:15:27.4766846Z .................................................................................................... 1600/9004
2019-09-06T23:15:39.7775280Z ......................................................i...............i............................. 1700/9004
2019-09-06T23:15:47.2776475Z .................................................................................................... 1800/9004
2019-09-06T23:16:01.0881228Z .............................................iiiii.................................................. 1900/9004
2019-09-06T23:16:11.6212322Z .................................................................................................... 2100/9004
2019-09-06T23:16:14.0719972Z .................................................................................................... 2200/9004
2019-09-06T23:16:17.6020548Z .................................................................................................... 2300/9004
2019-09-06T23:16:25.3207941Z .................................................................................................... 2400/9004
---
2019-09-06T23:19:16.5960711Z .................................i...............i.................................................. 4700/9004
2019-09-06T23:19:27.8495268Z .................................................................................................... 4800/9004
2019-09-06T23:19:33.9290823Z .................................................................................................... 4900/9004
2019-09-06T23:19:44.1960948Z .................................................................................................... 5000/9004
2019-09-06T23:19:49.8757671Z ...............ii.ii................................................................................ 5100/9004
2019-09-06T23:20:00.1519921Z .................................................................................................... 5300/9004
2019-09-06T23:20:09.8367602Z ..............................................................................i..................... 5400/9004
2019-09-06T23:20:17.1892257Z .................................................................................................... 5500/9004
2019-09-06T23:20:23.0998366Z .................................................................................................... 5600/9004
2019-09-06T23:20:23.0998366Z .................................................................................................... 5600/9004
2019-09-06T23:20:33.3294616Z ........................................................................ii...i..ii...........i...... 5700/9004
2019-09-06T23:20:57.5090425Z .................................................................................................... 5900/9004
2019-09-06T23:21:06.9133603Z .................................................................................................... 6000/9004
2019-09-06T23:21:06.9133603Z .................................................................................................... 6000/9004
2019-09-06T23:21:13.1452869Z ..........................................................................i..ii..................... 6100/9004
2019-09-06T23:21:41.4509402Z .................................................................................................... 6300/9004
2019-09-06T23:21:43.4355802Z .................................i.................................................................. 6400/9004
2019-09-06T23:21:45.4692737Z .................................................................................................... 6500/9004
2019-09-06T23:21:47.9053277Z .....i.............................................................................................. 6600/9004
---
2019-09-06T23:25:41.7951490Z failures:
2019-09-06T23:25:41.7981765Z 
2019-09-06T23:25:41.7982564Z ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition stdout ----
2019-09-06T23:25:41.7982754Z 
2019-09-06T23:25:41.7983121Z error in revision `edition`: test compilation failed although it shouldn't!
2019-09-06T23:25:41.7983304Z status: exit code: 1
2019-09-06T23:25:41.7984149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/auxiliary" "-A" "unused"
2019-09-06T23:25:41.7985662Z ------------------------------------------
2019-09-06T23:25:41.7985852Z 
2019-09-06T23:25:41.7986213Z ------------------------------------------
2019-09-06T23:25:41.7986388Z stderr:
2019-09-06T23:25:41.7986388Z stderr:
2019-09-06T23:25:41.7986763Z ------------------------------------------
2019-09-06T23:25:41.7986955Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-06T23:25:41.7987550Z    |
2019-09-06T23:25:41.7987696Z LL |     let x = &mut block;
2019-09-06T23:25:41.7988686Z    |             ---------- mutable borrow occurs here
2019-09-06T23:25:41.7988686Z    |             ---------- mutable borrow occurs here
2019-09-06T23:25:41.7989107Z LL |     let p: &'a u8 = &*block.current;
2019-09-06T23:25:41.7989272Z    |                     ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-09-06T23:25:41.7989541Z LL |     drop(x);
2019-09-06T23:25:41.7989541Z LL |     drop(x);
2019-09-06T23:25:41.7989840Z    |          - mutable borrow later used here
2019-09-06T23:25:41.7990142Z error: aborting due to previous error
2019-09-06T23:25:41.7990246Z 
2019-09-06T23:25:41.7990572Z For more information about this error, try `rustc --explain E0502`.
2019-09-06T23:25:41.7990708Z 
---
2019-09-06T23:25:41.8019031Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-06T23:25:41.8019328Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-06T23:25:41.8036657Z 
2019-09-06T23:25:42.6558177Z 
2019-09-06T23:25:42.6566600Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-06T23:25:42.6567358Z 
2019-09-06T23:25:42.6567388Z 
2019-09-06T23:25:42.6567455Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-06T23:25:42.6567499Z Build completed unsuccessfully in 1:03:33
2019-09-06T23:25:42.6567499Z Build completed unsuccessfully in 1:03:33
2019-09-06T23:25:42.6568031Z == clock drift check ==
2019-09-06T23:25:42.6568100Z   local time: Fri Sep  6 23:25:41 UTC 2019
2019-09-06T23:25:42.6568154Z   network time: Fri, 06 Sep 2019 23:25:42 GMT
2019-09-06T23:25:42.6568275Z == end clock drift check ==
2019-09-06T23:25:42.8758891Z ##[error]Bash exited with code '1'.
2019-09-06T23:25:42.8797669Z ##[section]Starting: Checkout
2019-09-06T23:25:42.8799624Z ==============================================================================
2019-09-06T23:25:42.8799666Z Task         : Get sources
2019-09-06T23:25:42.8799702Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
