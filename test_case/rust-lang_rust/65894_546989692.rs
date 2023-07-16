plain
2019-10-28T13:42:01.2524942Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T13:42:01.2758675Z ##[command]git config gc.auto 0
2019-10-28T13:42:01.2841253Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T13:42:01.2904151Z ##[command]git config --get-all http.proxy
2019-10-28T13:42:01.3048082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65894/merge:refs/remotes/pull/65894/merge
---
2019-10-28T14:44:38.0753697Z .................................................................................................... 1600/9259
2019-10-28T14:44:44.1039738Z .................................................................................................... 1700/9259
2019-10-28T14:44:57.1307317Z ..........................................................i...............i......................... 1800/9259
2019-10-28T14:45:05.1954786Z .................................................................................................... 1900/9259
2019-10-28T14:45:20.7373984Z ................................................iiiii............................................... 2000/9259
2019-10-28T14:45:31.9802374Z .................................................................................................... 2200/9259
2019-10-28T14:45:34.6436421Z .................................................................................................... 2300/9259
2019-10-28T14:45:38.6444855Z .................................................................................................... 2400/9259
2019-10-28T14:46:03.5094906Z .................................................................................................... 2500/9259
---
2019-10-28T14:49:05.0834534Z .................................................i...............i.................................. 4800/9259
2019-10-28T14:49:14.4294400Z .................................................................................................... 4900/9259
2019-10-28T14:49:23.5338115Z .................................................................................................... 5000/9259
2019-10-28T14:49:30.0625317Z .................................................................................................... 5100/9259
2019-10-28T14:49:40.9620546Z ..................................................ii.ii...........i................................. 5200/9259
2019-10-28T14:49:50.9395661Z .................................................................................................... 5400/9259
2019-10-28T14:50:00.7635404Z .................................................................................................... 5500/9259
2019-10-28T14:50:08.9977626Z ....................i............................................................................... 5600/9259
2019-10-28T14:50:15.1960783Z .................................................................................................... 5700/9259
2019-10-28T14:50:15.1960783Z .................................................................................................... 5700/9259
2019-10-28T14:50:27.5846790Z .................................................................................................... 5800/9259
2019-10-28T14:50:40.3921782Z .....ii...i..ii...........i......................................................................... 5900/9259
2019-10-28T14:51:02.9144754Z .................................................................................................... 6100/9259
2019-10-28T14:51:07.8363733Z .................................................................................................... 6200/9259
2019-10-28T14:51:07.8363733Z .................................................................................................... 6200/9259
2019-10-28T14:51:22.5582532Z ........................i..ii....................................................................... 6300/9259
2019-10-28T14:51:43.5553196Z ..........................................................................................i......... 6500/9259
2019-10-28T14:51:45.9041082Z .................................................................................................... 6600/9259
2019-10-28T14:51:48.2599712Z .................................................................i.................................. 6700/9259
2019-10-28T14:51:51.3051209Z .................................................................................................... 6800/9259
---
2019-10-28T14:56:36.5917808Z  finished in 5.868
2019-10-28T14:56:36.6121408Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:56:36.7865229Z 
2019-10-28T14:56:36.7866197Z running 153 tests
2019-10-28T14:56:40.1293210Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-28T14:56:42.1985135Z i..iiii..............i.........iii.i.........ii......
2019-10-28T14:56:42.1989052Z 
2019-10-28T14:56:42.1989136Z  finished in 5.586
2019-10-28T14:56:42.2288030Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:56:42.4044310Z 
---
2019-10-28T14:56:44.5173754Z  finished in 2.288
2019-10-28T14:56:44.5407024Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:56:44.7190795Z 
2019-10-28T14:56:44.7192022Z running 9 tests
2019-10-28T14:56:44.7193338Z iiiiiiiii
2019-10-28T14:56:44.7194284Z 
2019-10-28T14:56:44.7197374Z  finished in 0.178
2019-10-28T14:56:44.7414257Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:56:44.9337860Z 
---
2019-10-28T14:57:04.4726282Z  finished in 19.731
2019-10-28T14:57:04.4925930Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:57:04.6638794Z 
2019-10-28T14:57:04.6639126Z running 123 tests
2019-10-28T14:57:30.2948490Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-28T14:57:35.2318660Z i.i.i......iii.i.....ii
2019-10-28T14:57:35.2319223Z 
2019-10-28T14:57:35.2319337Z  finished in 30.739
2019-10-28T14:57:35.2330073Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T14:57:35.2335221Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-28T14:59:14.4264303Z    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-10-28T14:59:18.2292272Z warning: unused variable: `offset`
2019-10-28T14:59:18.2294333Z    --> src/librustdoc/html/markdown.rs:572:49
2019-10-28T14:59:18.2295192Z     |
2019-10-28T14:59:18.2295839Z 572 |                 while let Some((Event::Text(s), offset)) = parser.next() {
2019-10-28T14:59:18.2297511Z     |                                                 ^^^^^^ help: consider prefixing with an underscore: `_offset`
2019-10-28T14:59:18.2298618Z     = note: `#[warn(unused_variables)]` on by default
2019-10-28T14:59:18.2299023Z 
2019-10-28T14:59:18.2299438Z warning: unused variable: `fence_idx`
2019-10-28T14:59:18.2299862Z    --> src/librustdoc/html/markdown.rs:946:30
2019-10-28T14:59:18.2299862Z    --> src/librustdoc/html/markdown.rs:946:30
2019-10-28T14:59:18.2300249Z     |
2019-10-28T14:59:18.2300704Z 946 |                         Some(fence_idx) => {
2019-10-28T14:59:18.2301233Z     |                              ^^^^^^^^^ help: consider prefixing with an underscore: `_fence_idx`
2019-10-28T15:01:33.1582198Z    Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2019-10-28T15:01:33.8396759Z     Finished release [optimized] target(s) in 2m 51s
2019-10-28T15:01:33.8716507Z Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-28T15:01:34.0649573Z 
---
2019-10-28T15:05:09.3519219Z failures:
2019-10-28T15:05:09.3519463Z 
2019-10-28T15:05:09.3520032Z ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
2019-10-28T15:05:09.3520435Z 
2019-10-28T15:05:09.3521018Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok" in "/checkout/src/test/rustdoc/test_option_check/bar.rs":[6]
2019-10-28T15:05:09.3521312Z status: exit code: 0
2019-10-28T15:05:09.3522088Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
2019-10-28T15:05:09.3522838Z ------------------------------------------
2019-10-28T15:05:09.3523080Z 
2019-10-28T15:05:09.3523295Z running 1 test
2019-10-28T15:05:09.3523295Z running 1 test
2019-10-28T15:05:09.3523746Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok
2019-10-28T15:05:09.3524210Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-28T15:05:09.3524383Z 
2019-10-28T15:05:09.3524550Z 
2019-10-28T15:05:09.3525718Z ------------------------------------------
---
2019-10-28T15:05:09.3527455Z 
2019-10-28T15:05:09.3527668Z 
2019-10-28T15:05:09.3528153Z ---- [rustdoc] rustdoc/test_option_check/test.rs stdout ----
2019-10-28T15:05:09.3528547Z 
2019-10-28T15:05:09.3529107Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok" in "/checkout/src/test/rustdoc/test_option_check/test.rs":[8, 15]
2019-10-28T15:05:09.3530913Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test" "/checkout/src/test/rustdoc/test_option_check/test.rs" "--test"
2019-10-28T15:05:09.3531290Z stdout:
2019-10-28T15:05:09.3531759Z ------------------------------------------
2019-10-28T15:05:09.3532023Z 
2019-10-28T15:05:09.3532023Z 
2019-10-28T15:05:09.3532260Z running 3 tests
2019-10-28T15:05:09.3532742Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok
2019-10-28T15:05:09.3533646Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 8) ... ok
2019-10-28T15:05:09.3535948Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 10) ... ok
2019-10-28T15:05:09.3536932Z test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-28T15:05:09.3537132Z 
2019-10-28T15:05:09.3537316Z 
2019-10-28T15:05:09.3537807Z ------------------------------------------
---
2019-10-28T15:05:09.3541637Z test result: FAILED. 318 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out
2019-10-28T15:05:09.3541877Z 
2019-10-28T15:05:09.3546297Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537
2019-10-28T15:05:09.3547502Z 
2019-10-28T15:05:09.3549561Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T15:05:09.3550104Z 
2019-10-28T15:05:09.3550226Z 
2019-10-28T15:05:09.3550384Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T15:05:09.3550535Z Build completed unsuccessfully in 1:16:17
2019-10-28T15:05:09.3550535Z Build completed unsuccessfully in 1:16:17
2019-10-28T15:05:09.3554178Z :22
2019-10-28T15:05:09.3554376Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T15:05:09.3599249Z == clock drift check ==
2019-10-28T15:05:09.3612207Z   local time: Mon Oct 28 15:05:09 UTC 2019
2019-10-28T15:05:09.5045860Z   network time: Mon, 28 Oct 2019 15:05:09 GMT
2019-10-28T15:05:09.5049554Z == end clock drift check ==
2019-10-28T15:05:12.2801263Z 
2019-10-28T15:05:12.2944110Z ##[error]Bash exited with code '1'.
2019-10-28T15:05:12.2982625Z ##[section]Starting: Checkout
2019-10-28T15:05:12.2984310Z ==============================================================================
2019-10-28T15:05:12.2984380Z Task         : Get sources
2019-10-28T15:05:12.2984426Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
