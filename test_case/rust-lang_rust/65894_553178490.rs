plain
2019-11-12T22:58:32.8775936Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T22:58:33.4519065Z ##[command]git config gc.auto 0
2019-11-12T22:58:33.4523881Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T22:58:33.4525976Z ##[command]git config --get-all http.proxy
2019-11-12T22:58:33.4529860Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65894/merge:refs/remotes/pull/65894/merge
---
2019-11-12T23:58:35.9260721Z .................................................................................................... 1500/9234
2019-11-12T23:58:42.0691433Z .................................................................................................... 1600/9234
2019-11-12T23:58:51.4106505Z .................................................................................................... 1700/9234
2019-11-12T23:58:59.9697734Z ...i................................................................................................ 1800/9234
2019-11-12T23:59:06.8521577Z .......................................................................................iiiii........ 1900/9234
2019-11-12T23:59:28.1824793Z .................................................................................................... 2100/9234
2019-11-12T23:59:30.4896498Z .................................................................................................... 2200/9234
2019-11-12T23:59:32.9334067Z .................................................................................................... 2300/9234
2019-11-12T23:59:42.5629833Z .................................................................................................... 2400/9234
---
2019-11-13T00:02:36.5814877Z ....................................................................................i............... 4700/9234
2019-11-13T00:02:43.7099127Z i................................................................................................... 4800/9234
2019-11-13T00:02:53.1012457Z .................................................................................................... 4900/9234
2019-11-13T00:02:58.3663142Z .................................................................................................... 5000/9234
2019-11-13T00:03:10.0080681Z .......................................................................................ii.ii........ 5100/9234
2019-11-13T00:03:18.4042823Z ......................i............................................................................. 5300/9234
2019-11-13T00:03:26.7340730Z .................................................................................................... 5400/9234
2019-11-13T00:03:35.5785502Z .....................................................................i.............................. 5500/9234
2019-11-13T00:03:43.0529951Z .................................................................................................... 5600/9234
2019-11-13T00:03:43.0529951Z .................................................................................................... 5600/9234
2019-11-13T00:03:50.6366062Z .................................................................................................... 5700/9234
2019-11-13T00:04:00.0946821Z .......................................................ii...i..ii...........i....................... 5800/9234
2019-11-13T00:04:22.8089839Z .................................................................................................... 6000/9234
2019-11-13T00:04:31.1222699Z .................................................................................................... 6100/9234
2019-11-13T00:04:31.1222699Z .................................................................................................... 6100/9234
2019-11-13T00:04:36.9889456Z ..........................................................................i..ii..................... 6200/9234
2019-11-13T00:05:06.2304148Z .................................................................................................... 6400/9234
2019-11-13T00:05:08.4343890Z ..........................................i......................................................... 6500/9234
2019-11-13T00:05:10.5496292Z .................................................................................................... 6600/9234
2019-11-13T00:05:12.8209794Z ..........................i......................................................................... 6700/9234
---
2019-11-13T00:10:28.3936142Z  finished in 5.856
2019-11-13T00:10:28.4125392Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:10:28.5829313Z 
2019-11-13T00:10:28.5830053Z running 156 tests
2019-11-13T00:10:31.5015141Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-13T00:10:33.4640483Z .i.i..iiii..............i.........iii..i........ii......
2019-11-13T00:10:33.4641602Z 
2019-11-13T00:10:33.4645468Z  finished in 5.052
2019-11-13T00:10:33.4868853Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:10:33.6647043Z 
---
2019-11-13T00:10:35.6483674Z  finished in 2.160
2019-11-13T00:10:35.6683194Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:10:35.8365816Z 
2019-11-13T00:10:35.8366666Z running 9 tests
2019-11-13T00:10:35.8367557Z iiiiiiiii
2019-11-13T00:10:35.8368216Z 
2019-11-13T00:10:35.8368431Z  finished in 0.168
2019-11-13T00:10:35.8552864Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:10:36.0263871Z 
---
2019-11-13T00:10:55.1346168Z  finished in 19.269
2019-11-13T00:10:55.1433786Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:10:55.3127756Z 
2019-11-13T00:10:55.3128439Z running 123 tests
2019-11-13T00:11:18.3489746Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-13T00:11:23.6418884Z i.i.i......iii.i.....ii
2019-11-13T00:11:23.6420434Z 
2019-11-13T00:11:23.6421622Z  finished in 27.725
2019-11-13T00:11:23.6427951Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T00:11:23.6428439Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-13T00:18:06.9792566Z failures:
2019-11-13T00:18:06.9799809Z 
2019-11-13T00:18:06.9800348Z ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
2019-11-13T00:18:06.9800390Z 
2019-11-13T00:18:06.9801477Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok" in "/checkout/src/test/rustdoc/test_option_check/bar.rs":[6]
2019-11-13T00:18:06.9801551Z status: exit code: 0
2019-11-13T00:18:06.9802221Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
2019-11-13T00:18:06.9802530Z ------------------------------------------
2019-11-13T00:18:06.9802585Z 
2019-11-13T00:18:06.9802627Z running 1 test
2019-11-13T00:18:06.9802627Z running 1 test
2019-11-13T00:18:06.9802884Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok
2019-11-13T00:18:06.9803004Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T00:18:06.9803036Z 
2019-11-13T00:18:06.9803062Z 
2019-11-13T00:18:06.9803281Z ------------------------------------------
---
2019-11-13T00:18:06.9803856Z 
2019-11-13T00:18:06.9803882Z 
2019-11-13T00:18:06.9804110Z ---- [rustdoc] rustdoc/test_option_check/test.rs stdout ----
2019-11-13T00:18:06.9804144Z 
2019-11-13T00:18:06.9804638Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok" in "/checkout/src/test/rustdoc/test_option_check/test.rs":[8, 15]
2019-11-13T00:18:06.9805428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test" "/checkout/src/test/rustdoc/test_option_check/test.rs" "--test"
2019-11-13T00:18:06.9805520Z stdout:
2019-11-13T00:18:06.9805750Z ------------------------------------------
2019-11-13T00:18:06.9805802Z 
2019-11-13T00:18:06.9805802Z 
2019-11-13T00:18:06.9805841Z running 3 tests
2019-11-13T00:18:06.9806065Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok
2019-11-13T00:18:06.9806316Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 8) ... ok
2019-11-13T00:18:06.9806542Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 10) ... ok
2019-11-13T00:18:06.9806631Z test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T00:18:06.9806659Z 
2019-11-13T00:18:06.9806681Z 
2019-11-13T00:18:06.9806994Z ------------------------------------------
---
2019-11-13T00:18:06.9814464Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-13T00:18:06.9814692Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-13T00:18:06.9814721Z 
2019-11-13T00:18:06.9814765Z 
2019-11-13T00:18:06.9816206Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-13T00:18:06.9816447Z 
2019-11-13T00:18:06.9816471Z 
2019-11-13T00:18:06.9816510Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-13T00:18:06.9816570Z Build completed unsuccessfully in 1:13:01
2019-11-13T00:18:06.9816570Z Build completed unsuccessfully in 1:13:01
2019-11-13T00:18:06.9869906Z == clock drift check ==
2019-11-13T00:18:06.9886313Z   local time: Wed Nov 13 00:18:06 UTC 2019
2019-11-13T00:18:07.2686290Z   network time: Wed, 13 Nov 2019 00:18:07 GMT
2019-11-13T00:18:07.2692936Z == end clock drift check ==
2019-11-13T00:18:09.8080430Z 
2019-11-13T00:18:09.8145353Z ##[error]Bash exited with code '1'.
2019-11-13T00:18:09.8182475Z ##[section]Starting: Checkout
2019-11-13T00:18:09.8184359Z ==============================================================================
2019-11-13T00:18:09.8184590Z Task         : Get sources
2019-11-13T00:18:09.8184631Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
