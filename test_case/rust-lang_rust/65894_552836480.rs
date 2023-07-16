plain
2019-11-12T09:21:37.3757040Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T09:21:37.3943804Z ##[command]git config gc.auto 0
2019-11-12T09:21:37.4012423Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T09:21:37.4065684Z ##[command]git config --get-all http.proxy
2019-11-12T09:21:37.4202128Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65894/merge:refs/remotes/pull/65894/merge
---
2019-11-12T10:19:23.8344643Z .................................................................................................... 1400/9231
2019-11-12T10:19:30.1151513Z .................................................................................................... 1500/9231
2019-11-12T10:19:36.1463594Z .................................................................................................... 1600/9231
2019-11-12T10:19:45.0279406Z .................................................................................................... 1700/9231
2019-11-12T10:19:53.0595900Z ..i................................................................................................. 1800/9231
2019-11-12T10:19:59.5384797Z .......................................................................................iiiii........ 1900/9231
2019-11-12T10:20:19.8428842Z .................................................................................................... 2100/9231
2019-11-12T10:20:22.1328075Z .................................................................................................... 2200/9231
2019-11-12T10:20:24.5640208Z .................................................................................................... 2300/9231
2019-11-12T10:20:33.6395849Z .................................................................................................... 2400/9231
---
2019-11-12T10:23:22.8155789Z ..................................................................................i...............i. 4700/9231
2019-11-12T10:23:29.9060025Z .................................................................................................... 4800/9231
2019-11-12T10:23:38.9181233Z .................................................................................................... 4900/9231
2019-11-12T10:23:44.1911092Z .................................................................................................... 5000/9231
2019-11-12T10:23:55.1586531Z .....................................................................................ii.ii.......... 5100/9231
2019-11-12T10:23:58.8991904Z .i.................................................................................................. 5200/9231
2019-11-12T10:24:12.8443651Z .................................................................................................... 5400/9231
2019-11-12T10:24:19.6564521Z ...................................................................i................................ 5500/9231
2019-11-12T10:24:26.9054273Z .................................................................................................... 5600/9231
2019-11-12T10:24:34.5738955Z .................................................................................................... 5700/9231
2019-11-12T10:24:34.5738955Z .................................................................................................... 5700/9231
2019-11-12T10:24:43.3913720Z ....................................................ii...i..ii...........i.......................... 5800/9231
2019-11-12T10:25:05.1636618Z .................................................................................................... 6000/9231
2019-11-12T10:25:12.3528569Z .................................................................................................... 6100/9231
2019-11-12T10:25:12.3528569Z .................................................................................................... 6100/9231
2019-11-12T10:25:17.1487016Z .......................................................................i..ii........................ 6200/9231
2019-11-12T10:25:44.7346862Z .................................................................................................... 6400/9231
2019-11-12T10:25:46.6962113Z .......................................i............................................................ 6500/9231
2019-11-12T10:25:48.7840959Z .................................................................................................... 6600/9231
2019-11-12T10:25:50.9404029Z .......................i............................................................................ 6700/9231
---
2019-11-12T10:30:56.3775569Z  finished in 5.600
2019-11-12T10:30:56.3950199Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:30:57.0543243Z 
2019-11-12T10:30:57.0578299Z running 156 tests
2019-11-12T10:30:59.4069774Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-12T10:31:01.5507639Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-12T10:31:01.5509289Z 
2019-11-12T10:31:01.5589361Z  finished in 4.846
2019-11-12T10:31:01.5599421Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:31:01.5618323Z 
---
2019-11-12T10:31:03.3325564Z  finished in 2.072
2019-11-12T10:31:03.3508806Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:31:03.5050664Z 
2019-11-12T10:31:03.5051509Z running 9 tests
2019-11-12T10:31:03.5052560Z iiiiiiiii
2019-11-12T10:31:03.5054745Z 
2019-11-12T10:31:03.5055026Z  finished in 0.154
2019-11-12T10:31:03.5231151Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:31:04.0567697Z 
---
2019-11-12T10:31:22.4120125Z  finished in 18.888
2019-11-12T10:31:22.4320784Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:31:22.6014540Z 
2019-11-12T10:31:22.6015563Z running 123 tests
2019-11-12T10:31:45.9286020Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-12T10:31:50.4352124Z i.i.i......iii.i.....ii
2019-11-12T10:31:50.4353429Z 
2019-11-12T10:31:50.4357537Z  finished in 28.003
2019-11-12T10:31:50.4366684Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:31:50.4367366Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-12T10:33:17.6010311Z    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-11-12T10:33:20.9899430Z warning: unused variable: `offset`
2019-11-12T10:33:20.9900683Z    --> src/librustdoc/html/markdown.rs:572:49
2019-11-12T10:33:20.9901183Z     |
2019-11-12T10:33:20.9901773Z 572 |                 while let Some((Event::Text(s), offset)) = parser.next() {
2019-11-12T10:33:20.9902670Z     |                                                 ^^^^^^ help: consider prefixing with an underscore: `_offset`
2019-11-12T10:33:20.9903980Z     = note: `#[warn(unused_variables)]` on by default
2019-11-12T10:33:20.9904264Z 
2019-11-12T10:33:20.9904868Z warning: unused variable: `fence_idx`
2019-11-12T10:33:20.9905427Z    --> src/librustdoc/html/markdown.rs:946:30
2019-11-12T10:33:20.9905427Z    --> src/librustdoc/html/markdown.rs:946:30
2019-11-12T10:33:20.9906082Z     |
2019-11-12T10:33:20.9906611Z 946 |                         Some(fence_idx) => {
2019-11-12T10:33:20.9907191Z     |                              ^^^^^^^^^ help: consider prefixing with an underscore: `_fence_idx`
2019-11-12T10:35:18.4425334Z    Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2019-11-12T10:35:19.0477815Z     Finished release [optimized] target(s) in 2m 29s
2019-11-12T10:35:19.0721200Z Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T10:35:19.2339218Z 
---
2019-11-12T10:38:06.9268679Z failures:
2019-11-12T10:38:06.9268969Z 
2019-11-12T10:38:06.9270045Z ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
2019-11-12T10:38:06.9270125Z 
2019-11-12T10:38:06.9270531Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok" in "/checkout/src/test/rustdoc/test_option_check/bar.rs":[6]
2019-11-12T10:38:06.9270619Z status: exit code: 0
2019-11-12T10:38:06.9271329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
2019-11-12T10:38:06.9271606Z ------------------------------------------
2019-11-12T10:38:06.9271642Z 
2019-11-12T10:38:06.9271698Z running 1 test
2019-11-12T10:38:06.9271698Z running 1 test
2019-11-12T10:38:06.9271922Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 8) ... ok
2019-11-12T10:38:06.9272011Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-12T10:38:06.9272037Z 
2019-11-12T10:38:06.9272056Z 
2019-11-12T10:38:06.9272295Z ------------------------------------------
---
2019-11-12T10:38:06.9272769Z 
2019-11-12T10:38:06.9272789Z 
2019-11-12T10:38:06.9272987Z ---- [rustdoc] rustdoc/test_option_check/test.rs stdout ----
2019-11-12T10:38:06.9273021Z 
2019-11-12T10:38:06.9273336Z error: Not found doc test: "test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok" in "/checkout/src/test/rustdoc/test_option_check/test.rs":[8, 15]
2019-11-12T10:38:06.9273912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test" "/checkout/src/test/rustdoc/test_option_check/test.rs" "--test"
2019-11-12T10:38:06.9273973Z stdout:
2019-11-12T10:38:06.9274153Z ------------------------------------------
2019-11-12T10:38:06.9274199Z 
2019-11-12T10:38:06.9274199Z 
2019-11-12T10:38:06.9274233Z running 3 tests
2019-11-12T10:38:06.9274444Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 17) ... ok
2019-11-12T10:38:06.9274686Z test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 8) ... ok
2019-11-12T10:38:06.9274905Z test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 10) ... ok
2019-11-12T10:38:06.9274979Z test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-12T10:38:06.9275021Z 
2019-11-12T10:38:06.9275041Z 
2019-11-12T10:38:06.9275223Z ------------------------------------------
---
2019-11-12T10:38:06.9284990Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T10:38:06.9285344Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T10:38:06.9299560Z 
2019-11-12T10:38:06.9299707Z 
2019-11-12T10:38:06.9301579Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T10:38:06.9301816Z 
2019-11-12T10:38:06.9301840Z 
2019-11-12T10:38:06.9305854Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T10:38:06.9305926Z Build completed unsuccessfully in 1:10:03
2019-11-12T10:38:06.9305926Z Build completed unsuccessfully in 1:10:03
2019-11-12T10:38:06.9352893Z == clock drift check ==
2019-11-12T10:38:06.9369710Z   local time: Tue Nov 12 10:38:06 UTC 2019
2019-11-12T10:38:07.2140069Z   network time: Tue, 12 Nov 2019 10:38:07 GMT
2019-11-12T10:38:07.2146334Z == end clock drift check ==
2019-11-12T10:38:10.0575698Z 
2019-11-12T10:38:10.0668282Z ##[error]Bash exited with code '1'.
2019-11-12T10:38:10.0705536Z ##[section]Starting: Checkout
2019-11-12T10:38:10.0707570Z ==============================================================================
2019-11-12T10:38:10.0707648Z Task         : Get sources
2019-11-12T10:38:10.0707696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
