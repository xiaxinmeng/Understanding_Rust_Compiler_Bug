plain
2019-09-03T02:50:35.8023347Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T02:50:35.8257897Z ##[command]git config gc.auto 0
2019-09-03T02:50:35.8356073Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T02:50:35.8448177Z ##[command]git config --get-all http.proxy
2019-09-03T02:50:35.8607209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64111/merge:refs/remotes/pull/64111/merge
---
2019-09-03T03:56:42.7716889Z .................................................................................................... 1500/8991
2019-09-03T03:56:48.7139387Z .................................................................................................... 1600/8991
2019-09-03T03:57:02.1165103Z .................................................i...............i.................................. 1700/8991
2019-09-03T03:57:10.8809925Z .................................................................................................... 1800/8991
2019-09-03T03:57:25.9832408Z ........................................iiiii....................................................... 1900/8991
2019-09-03T03:57:37.4422484Z .................................................................................................... 2100/8991
2019-09-03T03:57:40.2133086Z .................................................................................................... 2200/8991
2019-09-03T03:57:44.4614689Z .................................................................................................... 2300/8991
2019-09-03T03:57:52.4723036Z .................................................................................................... 2400/8991
---
2019-09-03T04:01:00.0997792Z ...........................i...............i........................................................ 4700/8991
2019-09-03T04:01:12.9505217Z .................................................................................................... 4800/8991
2019-09-03T04:01:19.3568574Z .................................................................................................... 4900/8991
2019-09-03T04:01:30.5777330Z .................................................................................................... 5000/8991
2019-09-03T04:01:36.5072312Z ........ii.ii....................................................................................... 5100/8991
2019-09-03T04:01:50.3452365Z .................................................................................................... 5300/8991
2019-09-03T04:01:59.0805934Z .......................................................................i............................ 5400/8991
2019-09-03T04:02:06.7517968Z .................................................................................................... 5500/8991
2019-09-03T04:02:13.9580713Z .................................................................................................... 5600/8991
2019-09-03T04:02:13.9580713Z .................................................................................................... 5600/8991
2019-09-03T04:02:25.2449135Z .................................................................ii...i..ii...........i............. 5700/8991
2019-09-03T04:02:51.9101120Z .................................................................................................... 5900/8991
2019-09-03T04:03:02.2984730Z .................................................................................................... 6000/8991
2019-09-03T04:03:02.2984730Z .................................................................................................... 6000/8991
2019-09-03T04:03:12.4414408Z ...................................................................i..ii............................ 6100/8991
2019-09-03T04:03:43.1225824Z .................................................................................................... 6300/8991
2019-09-03T04:03:45.3644750Z ..........................i......................................................................... 6400/8991
2019-09-03T04:03:47.6516342Z ..................................................................................................i. 6500/8991
2019-09-03T04:03:50.5368163Z .................................................................................................... 6600/8991
---
2019-09-03T04:08:53.7624182Z  finished in 21.979
2019-09-03T04:08:53.7841151Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:08:53.9692871Z 
2019-09-03T04:08:53.9694062Z running 149 tests
2019-09-03T04:08:57.4917213Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-09-03T04:08:59.6004681Z ..iiii..............i.........iii.i......ii......
2019-09-03T04:08:59.6005699Z 
2019-09-03T04:08:59.6010785Z  finished in 5.817
2019-09-03T04:08:59.6207267Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:08:59.7908287Z 
---
2019-09-03T04:09:01.9815053Z  finished in 2.360
2019-09-03T04:09:02.0061654Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:09:02.1946178Z 
2019-09-03T04:09:02.1946364Z running 9 tests
2019-09-03T04:09:02.1947218Z iiiiiiiii
2019-09-03T04:09:02.1951864Z 
2019-09-03T04:09:02.1951913Z  finished in 0.189
2019-09-03T04:09:02.2197870Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:09:03.1041150Z 
---
2019-09-03T04:09:21.5657274Z  finished in 19.346
2019-09-03T04:09:21.5908558Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:09:21.7886524Z 
2019-09-03T04:09:21.7888058Z running 123 tests
2019-09-03T04:09:47.8595033Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-03T04:09:53.0016855Z i.i.i......iii.i.....ii
2019-09-03T04:09:53.0018749Z 
2019-09-03T04:09:53.0022242Z  finished in 31.411
2019-09-03T04:09:53.0031660Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T04:09:53.0035623Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-03T04:10:46.5077791Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-09-03T04:10:46.5078003Z 
2019-09-03T04:10:46.5078421Z error: test compilation failed although it shouldn't!
2019-09-03T04:10:46.5078608Z status: exit code: 1
2019-09-03T04:10:46.5079460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary" "-A" "unused"
2019-09-03T04:10:46.5080340Z ------------------------------------------
2019-09-03T04:10:46.5080560Z 
2019-09-03T04:10:46.5080934Z ------------------------------------------
2019-09-03T04:10:46.5081112Z stderr:
2019-09-03T04:10:46.5081112Z stderr:
2019-09-03T04:10:46.5081483Z ------------------------------------------
2019-09-03T04:10:46.5081663Z error[E0308]: mismatched types
2019-09-03T04:10:46.5082039Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:158:64
2019-09-03T04:10:46.5082249Z    |
2019-09-03T04:10:46.5082660Z LL |                 iter_exprs(depth - 1, &mut |e| g(ExprKind::Let(ps.clone(), e)))
2019-09-03T04:10:46.5082900Z    |                                                                ^^^^^^^^^^ expected struct `syntax::ptr::P`, found struct `std::vec::Vec`
2019-09-03T04:10:46.5083195Z    = note: expected type `syntax::ptr::P<_>`
2019-09-03T04:10:46.5083195Z    = note: expected type `syntax::ptr::P<_>`
2019-09-03T04:10:46.5084292Z               found type `std::vec::Vec<syntax::ptr::P<_>>`
2019-09-03T04:10:46.5084615Z error: aborting due to previous error
2019-09-03T04:10:46.5085100Z 
2019-09-03T04:10:46.5085671Z For more information about this error, try `rustc --explain E0308`.
2019-09-03T04:10:46.5085969Z 
---
2019-09-03T04:10:46.5089965Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-03T04:10:46.5090038Z 
2019-09-03T04:10:46.5095148Z 
2019-09-03T04:10:46.5104281Z 
2019-09-03T04:10:46.5106990Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-03T04:10:46.5107697Z 
2019-09-03T04:10:46.5107850Z 
2019-09-03T04:10:46.5108086Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-03T04:10:46.5108281Z Build completed unsuccessfully in 1:12:42
2019-09-03T04:10:46.5108281Z Build completed unsuccessfully in 1:12:42
2019-09-03T04:10:46.5166967Z == clock drift check ==
2019-09-03T04:10:46.5199545Z   local time: Tue Sep  3 04:10:46 UTC 2019
2019-09-03T04:10:46.6682621Z   network time: Tue, 03 Sep 2019 04:10:46 GMT
2019-09-03T04:10:46.6682755Z == end clock drift check ==
2019-09-03T04:10:47.5768521Z ##[error]Bash exited with code '1'.
2019-09-03T04:10:47.5811934Z ##[section]Starting: Checkout
2019-09-03T04:10:47.5814306Z ==============================================================================
2019-09-03T04:10:47.5814368Z Task         : Get sources
2019-09-03T04:10:47.5814419Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
