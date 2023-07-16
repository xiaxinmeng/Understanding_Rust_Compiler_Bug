plain
2019-10-20T17:19:07.3849784Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T17:19:07.4077299Z ##[command]git config gc.auto 0
2019-10-20T17:19:07.4136684Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T17:19:07.4194244Z ##[command]git config --get-all http.proxy
2019-10-20T17:19:07.4331377Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65627/merge:refs/remotes/pull/65627/merge
---
2019-10-20T18:25:45.2588593Z .................................................................................................... 1600/9203
2019-10-20T18:25:50.9771703Z .................................................................................................... 1700/9203
2019-10-20T18:26:04.8149317Z ................................i...............i................................................... 1800/9203
2019-10-20T18:26:12.6096540Z .................................................................................................... 1900/9203
2019-10-20T18:26:27.6054214Z ......................iiiii......................................................................... 2000/9203
2019-10-20T18:26:38.7541233Z .................................................................................................... 2200/9203
2019-10-20T18:26:41.4939370Z .................................................................................................... 2300/9203
2019-10-20T18:26:46.8282332Z .................................................................................................... 2400/9203
2019-10-20T18:27:10.3865730Z .................................................................................................... 2500/9203
---
2019-10-20T18:30:17.2147210Z .........................i...............i.......................................................... 4800/9203
2019-10-20T18:30:29.9717875Z .................................................................................................... 4900/9203
2019-10-20T18:30:36.7075293Z .................................................................................................... 5000/9203
2019-10-20T18:30:46.8415110Z .................................................................................................... 5100/9203
2019-10-20T18:30:54.7760159Z .........................ii.ii...................................................................... 5200/9203
2019-10-20T18:31:05.4076613Z .................................................................................................... 5400/9203
2019-10-20T18:31:16.4326197Z ...........................................................................................i........ 5500/9203
2019-10-20T18:31:25.2028694Z .................................................................................................... 5600/9203
2019-10-20T18:31:30.2662242Z .................................................................................................... 5700/9203
2019-10-20T18:31:30.2662242Z .................................................................................................... 5700/9203
2019-10-20T18:31:41.4686989Z ........................................................................................ii...i..ii.. 5800/9203
2019-10-20T18:32:09.3326071Z .................................................................................................... 6000/9203
2019-10-20T18:32:19.0752626Z .................................................................................................... 6100/9203
2019-10-20T18:32:27.5147831Z .................................................................................................... 6200/9203
2019-10-20T18:32:27.5147831Z .................................................................................................... 6200/9203
2019-10-20T18:32:42.2748852Z ..........i..ii..................................................................................... 6300/9203
2019-10-20T18:33:03.1977926Z ......................................................................i............................. 6500/9203
2019-10-20T18:33:05.4610960Z .................................................................................................... 6600/9203
2019-10-20T18:33:07.9679706Z .............................................i...................................................... 6700/9203
2019-10-20T18:33:11.6522574Z .................................................................................................... 6800/9203
---
2019-10-20T18:37:57.9718383Z  finished in 5.974
2019-10-20T18:37:57.9923071Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:37:58.1753845Z 
2019-10-20T18:37:58.1754799Z running 153 tests
2019-10-20T18:38:01.5577572Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-20T18:38:03.7568231Z i..iiii..............i.........iii.i.........ii......
2019-10-20T18:38:03.7642528Z 
2019-10-20T18:38:03.7642586Z  finished in 5.679
2019-10-20T18:38:03.7642933Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:38:03.8623575Z 
---
2019-10-20T18:38:05.9839636Z  finished in 2.292
2019-10-20T18:38:06.0031110Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:38:06.7648713Z 
2019-10-20T18:38:06.7648848Z running 9 tests
2019-10-20T18:38:06.7649714Z iiiiiiiii
2019-10-20T18:38:06.7650683Z 
2019-10-20T18:38:06.7650906Z  finished in 0.173
2019-10-20T18:38:06.7651870Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:38:06.7651923Z 
---
2019-10-20T18:38:25.7646467Z  finished in 18.939
2019-10-20T18:38:25.7646874Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:38:25.7647054Z 
2019-10-20T18:38:25.7647184Z running 123 tests
2019-10-20T18:38:50.9209121Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-20T18:38:55.8663418Z i.i.i......iii.i.....ii
2019-10-20T18:38:55.8664565Z 
2019-10-20T18:38:55.8668922Z  finished in 30.707
2019-10-20T18:38:55.8679193Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T18:38:55.8679830Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-20T18:46:39.1879485Z ---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----
2019-10-20T18:46:39.1879690Z 
2019-10-20T18:46:39.1880211Z error: rustdoc failed!
2019-10-20T18:46:39.1880551Z status: exit code: 1
2019-10-20T18:46:39.1881528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
2019-10-20T18:46:39.1882434Z ------------------------------------------
2019-10-20T18:46:39.1882495Z 
2019-10-20T18:46:39.1882718Z ------------------------------------------
2019-10-20T18:46:39.1882764Z stderr:
---
2019-10-20T18:46:39.1883601Z 
2019-10-20T18:46:39.1883649Z error[E0740]: the types of const generic parameters must derive `PartialEq` and `Eq`
2019-10-20T18:46:39.1883915Z   --> /checkout/src/test/rustdoc/const-generics/const-impl.rs:15:33
2019-10-20T18:46:39.1883971Z    |
2019-10-20T18:46:39.1884015Z 15 | pub struct VSet<T, const ORDER: Order> {
2019-10-20T18:46:39.1884406Z    |                                 ^^^^^ `Order` doesn't derive both `PartialEq` and `Eq`
2019-10-20T18:46:39.1884491Z error: aborting due to previous error
2019-10-20T18:46:39.1884520Z 
2019-10-20T18:46:39.1884778Z For more information about this error, try `rustc --explain E0740`.
2019-10-20T18:46:39.1884813Z 
---
2019-10-20T18:46:39.1891463Z 
2019-10-20T18:46:39.1891541Z 
2019-10-20T18:46:39.1891856Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-20T18:46:39.1891921Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-20T18:46:39.1898554Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-20T18:46:39.1932593Z 
2019-10-20T18:46:39.1932895Z 
2019-10-20T18:46:39.1935113Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-20T18:46:39.1935372Z Build completed unsuccessfully in 1:20:25
2019-10-20T18:46:39.1935372Z Build completed unsuccessfully in 1:20:25
2019-10-20T18:46:39.1960459Z == clock drift check ==
2019-10-20T18:46:39.1991124Z   local time: Sun Oct 20 18:46:39 UTC 2019
2019-10-20T18:46:39.3477393Z   network time: Sun, 20 Oct 2019 18:46:39 GMT
2019-10-20T18:46:39.3483482Z == end clock drift check ==
2019-10-20T18:46:42.2969495Z 
2019-10-20T18:46:42.3100302Z ##[error]Bash exited with code '1'.
2019-10-20T18:46:42.3140984Z ##[section]Starting: Checkout
2019-10-20T18:46:42.3143039Z ==============================================================================
2019-10-20T18:46:42.3143112Z Task         : Get sources
2019-10-20T18:46:42.3143159Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
