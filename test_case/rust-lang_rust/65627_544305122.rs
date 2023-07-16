plain
2019-10-20T22:22:43.5058923Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T22:22:43.5252556Z ##[command]git config gc.auto 0
2019-10-20T22:22:43.5322628Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T22:22:43.5372119Z ##[command]git config --get-all http.proxy
2019-10-20T22:22:43.5536950Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65627/merge:refs/remotes/pull/65627/merge
---
2019-10-20T23:26:14.3280364Z .................................................................................................... 1600/9204
2019-10-20T23:26:19.7165935Z .................................................................................................... 1700/9204
2019-10-20T23:26:32.7474491Z .................................i...............i.................................................. 1800/9204
2019-10-20T23:26:40.4051466Z .................................................................................................... 1900/9204
2019-10-20T23:26:54.7079251Z .......................iiiii........................................................................ 2000/9204
2019-10-20T23:27:05.2954380Z .................................................................................................... 2200/9204
2019-10-20T23:27:07.9333111Z .................................................................................................... 2300/9204
2019-10-20T23:27:12.9076736Z .................................................................................................... 2400/9204
2019-10-20T23:27:35.0496472Z .................................................................................................... 2500/9204
---
2019-10-20T23:30:30.5356391Z ..........................i...............i......................................................... 4800/9204
2019-10-20T23:30:42.5459218Z .................................................................................................... 4900/9204
2019-10-20T23:30:49.0672006Z .................................................................................................... 5000/9204
2019-10-20T23:30:58.2943980Z .................................................................................................... 5100/9204
2019-10-20T23:31:06.0634951Z ..........................ii.ii..................................................................... 5200/9204
2019-10-20T23:31:16.2261065Z .................................................................................................... 5400/9204
2019-10-20T23:31:26.6648475Z ............................................................................................i....... 5500/9204
2019-10-20T23:31:34.7733725Z .................................................................................................... 5600/9204
2019-10-20T23:31:39.5787023Z .................................................................................................... 5700/9204
2019-10-20T23:31:39.5787023Z .................................................................................................... 5700/9204
2019-10-20T23:31:50.3011217Z .........................................................................................ii...i..ii. 5800/9204
2019-10-20T23:32:16.5079467Z .................................................................................................... 6000/9204
2019-10-20T23:32:25.9269661Z .................................................................................................... 6100/9204
2019-10-20T23:32:32.2600809Z .................................................................................................... 6200/9204
2019-10-20T23:32:32.2600809Z .................................................................................................... 6200/9204
2019-10-20T23:32:46.3754735Z ...........i..ii.................................................................................... 6300/9204
2019-10-20T23:33:06.1556854Z .......................................................................i............................ 6500/9204
2019-10-20T23:33:08.3352012Z .................................................................................................... 6600/9204
2019-10-20T23:33:10.7497358Z ..............................................i..................................................... 6700/9204
2019-10-20T23:33:14.2013020Z .................................................................................................... 6800/9204
---
2019-10-20T23:37:47.7353234Z  finished in 5.611
2019-10-20T23:37:47.7545481Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:37:47.9182296Z 
2019-10-20T23:37:47.9182554Z running 153 tests
2019-10-20T23:37:51.2006947Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-20T23:37:53.2105271Z i..iiii..............i..........iiii.........ii......
2019-10-20T23:37:53.2107085Z 
2019-10-20T23:37:53.2111368Z  finished in 5.456
2019-10-20T23:37:53.2308913Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:37:53.3882331Z 
---
2019-10-20T23:37:55.4910683Z  finished in 2.259
2019-10-20T23:37:55.5112977Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:37:55.6893773Z 
2019-10-20T23:37:55.6894614Z running 9 tests
2019-10-20T23:37:55.6895680Z iiiiiiiii
2019-10-20T23:37:55.6896604Z 
2019-10-20T23:37:55.6900072Z  finished in 0.178
2019-10-20T23:37:55.7119761Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:37:55.8974307Z 
---
2019-10-20T23:38:13.9466851Z  finished in 18.236
2019-10-20T23:38:13.9663721Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:38:14.1315293Z 
2019-10-20T23:38:14.1315583Z running 123 tests
2019-10-20T23:38:38.3974296Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-20T23:38:43.3264117Z i.i.i......iii.i.....ii
2019-10-20T23:38:43.3265982Z 
2019-10-20T23:38:43.3266323Z  finished in 29.360
2019-10-20T23:38:43.3277290Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-20T23:38:43.3279812Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-20T23:46:07.8755280Z ---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----
2019-10-20T23:46:07.8755321Z 
2019-10-20T23:46:07.8755366Z error: rustdoc failed!
2019-10-20T23:46:07.8755434Z status: exit code: 1
2019-10-20T23:46:07.8756044Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
2019-10-20T23:46:07.8756625Z ------------------------------------------
2019-10-20T23:46:07.8756677Z 
2019-10-20T23:46:07.8756982Z ------------------------------------------
2019-10-20T23:46:07.8757033Z stderr:
2019-10-20T23:46:07.8757033Z stderr:
2019-10-20T23:46:07.8757266Z ------------------------------------------
2019-10-20T23:46:07.8757319Z error: `derive` may only be applied to structs, enums and unions
2019-10-20T23:46:07.8757652Z   |
2019-10-20T23:46:07.8757697Z 7 | #![derive(PartialEq, Eq)]
2019-10-20T23:46:07.8757767Z   | ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try an outer attribute: `#[derive(PartialEq, Eq)]`
2019-10-20T23:46:07.8757802Z 
---
2019-10-20T23:46:07.8765848Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-20T23:46:07.8765980Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-20T23:46:07.8771951Z 
2019-10-20T23:46:07.8772064Z 
2019-10-20T23:46:07.8777265Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-20T23:46:07.8777626Z 
2019-10-20T23:46:07.8777675Z 
2019-10-20T23:46:07.8789179Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-20T23:46:07.8789265Z Build completed unsuccessfully in 1:16:28
2019-10-20T23:46:07.8789265Z Build completed unsuccessfully in 1:16:28
2019-10-20T23:46:07.8843437Z == clock drift check ==
2019-10-20T23:46:07.8858815Z   local time: Sun Oct 20 23:46:07 UTC 2019
2019-10-20T23:46:08.0220074Z   network time: Sun, 20 Oct 2019 23:46:08 GMT
2019-10-20T23:46:08.0230096Z == end clock drift check ==
2019-10-20T23:46:10.4023767Z 
2019-10-20T23:46:10.4154006Z ##[error]Bash exited with code '1'.
2019-10-20T23:46:10.4194844Z ##[section]Starting: Checkout
2019-10-20T23:46:10.4197266Z ==============================================================================
2019-10-20T23:46:10.4197347Z Task         : Get sources
2019-10-20T23:46:10.4197399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
