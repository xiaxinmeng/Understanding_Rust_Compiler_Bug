plain
2019-11-18T05:47:26.6871374Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T05:47:27.6538045Z ##[command]git config gc.auto 0
2019-11-18T05:47:27.6543728Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T05:47:27.6548617Z ##[command]git config --get-all http.proxy
2019-11-18T05:47:27.6553947Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66507/merge:refs/remotes/pull/66507/merge
---
2019-11-18T06:43:45.2183360Z .................................................................................................... 1600/9264
2019-11-18T06:43:51.0550431Z .................................................................................................... 1700/9264
2019-11-18T06:44:02.7897276Z .....................i.............................................................................. 1800/9264
2019-11-18T06:44:09.5391576Z .................................................................................................... 1900/9264
2019-11-18T06:44:24.0259743Z ......iiiii......................................................................................... 2000/9264
2019-11-18T06:44:33.1164149Z .................................................................................................... 2200/9264
2019-11-18T06:44:35.6999536Z .................................................................................................... 2300/9264
2019-11-18T06:44:41.4417312Z .................................................................................................... 2400/9264
2019-11-18T06:45:02.1587562Z .................................................................................................... 2500/9264
---
2019-11-18T06:47:40.8641421Z ......i...............i............................................................................. 4800/9264
2019-11-18T06:47:50.5436251Z .................................................................................................... 4900/9264
2019-11-18T06:47:55.7457985Z .................................................................................................... 5000/9264
2019-11-18T06:48:05.1570413Z .................................................................................................... 5100/9264
2019-11-18T06:48:10.6135206Z ..........ii.ii...........i......................................................................... 5200/9264
2019-11-18T06:48:20.0019737Z .................................................................................................... 5400/9264
2019-11-18T06:48:30.3299288Z ............................................................................................i....... 5500/9264
2019-11-18T06:48:38.2566171Z .................................................................................................... 5600/9264
2019-11-18T06:48:43.2933844Z .................................................................................................... 5700/9264
2019-11-18T06:48:43.2933844Z .................................................................................................... 5700/9264
2019-11-18T06:48:53.0268149Z ..............................................................................ii...i..ii...........i 5800/9264
2019-11-18T06:49:15.0265544Z .................................................................................................... 6000/9264
2019-11-18T06:49:22.8468137Z .................................................................................................... 6100/9264
2019-11-18T06:49:28.6764077Z .................................................................................................i.. 6200/9264
2019-11-18T06:49:41.6467919Z ii.................................................................................................. 6300/9264
---
2019-11-18T06:55:06.7888974Z  finished in 5.541
2019-11-18T06:55:06.8065976Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-18T06:55:06.9790109Z 
2019-11-18T06:55:06.9790915Z running 156 tests
2019-11-18T06:55:09.8159756Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-18T06:55:11.6503670Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-18T06:55:11.6506302Z 
2019-11-18T06:55:11.6507210Z  finished in 4.843
2019-11-18T06:55:11.6684502Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-18T06:55:11.8243847Z 
---
2019-11-18T06:55:13.7298317Z  finished in 2.061
2019-11-18T06:55:13.7469987Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-18T06:55:13.9014254Z 
2019-11-18T06:55:13.9014497Z running 9 tests
2019-11-18T06:55:13.9015222Z iiiiiiiii
2019-11-18T06:55:13.9015578Z 
2019-11-18T06:55:13.9015622Z  finished in 0.153
2019-11-18T06:55:13.9208308Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-18T06:55:14.0943048Z 
2019-11-18T06:55:14.0943048Z 
2019-11-18T06:55:14.0943276Z running 109 tests
2019-11-18T06:55:31.2939135Z .................................................................................................... 100/109
2019-11-18T06:55:32.5675452Z .......F.
2019-11-18T06:55:32.5677175Z failures:
2019-11-18T06:55:32.5678194Z 
2019-11-18T06:55:32.5679767Z ---- [incremental] incremental/warnings-reemitted.rs stdout ----
2019-11-18T06:55:32.5681153Z thread '[incremental] incremental/warnings-reemitted.rs' panicked at 'Error annotations must look like `//[X]~` in incremental tests', src/libcore/option.rs:1185:5
2019-11-18T06:55:32.5682614Z 
2019-11-18T06:55:32.5682887Z 
2019-11-18T06:55:32.5683674Z failures:
2019-11-18T06:55:32.5685025Z     [incremental] incremental/warnings-reemitted.rs
2019-11-18T06:55:32.5685025Z     [incremental] incremental/warnings-reemitted.rs
2019-11-18T06:55:32.5685995Z 
2019-11-18T06:55:32.5687433Z test result: FAILED. 108 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-18T06:55:32.5688546Z 
2019-11-18T06:55:32.5693876Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-18T06:55:32.5697807Z 
2019-11-18T06:55:32.5698890Z 
2019-11-18T06:55:32.5701323Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-18T06:55:32.5701620Z 
2019-11-18T06:55:32.5701649Z 
2019-11-18T06:55:32.5710414Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-18T06:55:32.5710505Z Build completed unsuccessfully in 1:01:56
2019-11-18T06:55:32.5710505Z Build completed unsuccessfully in 1:01:56
2019-11-18T06:55:32.5765259Z == clock drift check ==
2019-11-18T06:55:32.5786155Z   local time: Mon Nov 18 06:55:32 UTC 2019
2019-11-18T06:55:33.1003373Z   network time: Mon, 18 Nov 2019 06:55:33 GMT
2019-11-18T06:55:33.1004237Z == end clock drift check ==
2019-11-18T06:55:38.1292835Z 
2019-11-18T06:55:38.1386704Z ##[error]Bash exited with code '1'.
2019-11-18T06:55:38.1423978Z ##[section]Starting: Checkout
2019-11-18T06:55:38.1425643Z ==============================================================================
2019-11-18T06:55:38.1425715Z Task         : Get sources
2019-11-18T06:55:38.1425800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
