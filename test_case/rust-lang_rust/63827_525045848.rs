plain
2019-08-26T20:26:50.1377294Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T20:26:50.1605793Z ##[command]git config gc.auto 0
2019-08-26T20:26:50.1664445Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T20:26:50.1717065Z ##[command]git config --get-all http.proxy
2019-08-26T20:26:50.1831549Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63827/merge:refs/remotes/pull/63827/merge
---
2019-08-26T20:27:25.0228405Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T20:27:25.0228605Z 
2019-08-26T20:27:25.0228931Z   git checkout -b <new-branch-name>
2019-08-26T20:27:25.0229112Z 
2019-08-26T20:27:25.0229296Z HEAD is now at bf96088cb Merge ad6b322fd9bf5988bf0cee6c470c693e24601cca into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T20:27:25.0380778Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T20:27:25.0383261Z ==============================================================================
2019-08-26T20:27:25.0383305Z Task         : Bash
2019-08-26T20:27:25.0383339Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T21:26:55.1456583Z .................................................................................................... 1500/8953
2019-08-26T21:27:00.5729243Z .................................................................................................... 1600/8953
2019-08-26T21:27:12.7328848Z .............................................i...............i...................................... 1700/8953
2019-08-26T21:27:20.4891173Z .................................................................................................... 1800/8953
2019-08-26T21:27:34.2666267Z .....................................iiiii.......................................................... 1900/8953
2019-08-26T21:27:44.2812324Z .................................................................................................... 2100/8953
2019-08-26T21:27:46.6792699Z .................................................................................................... 2200/8953
2019-08-26T21:27:50.7318860Z .................................................................................................... 2300/8953
2019-08-26T21:27:57.6935505Z .................................................................................................... 2400/8953
---
2019-08-26T21:30:47.7973214Z .........................i...............i.......................................................... 4700/8953
2019-08-26T21:30:58.8477403Z .................................................................................................... 4800/8953
2019-08-26T21:31:04.7316654Z .................................................................................................... 4900/8953
2019-08-26T21:31:15.0594047Z .................................................................................................... 5000/8953
2019-08-26T21:31:20.2057335Z ......ii.ii......................................................................................... 5100/8953
2019-08-26T21:31:33.6193559Z .................................................................................................... 5300/8953
2019-08-26T21:31:40.2664773Z ..............................................................i..................................... 5400/8953
2019-08-26T21:31:47.0175618Z .................................................................................................... 5500/8953
2019-08-26T21:31:54.3332999Z .................................................................................................... 5600/8953
2019-08-26T21:31:54.3332999Z .................................................................................................... 5600/8953
2019-08-26T21:32:03.4379302Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-26T21:32:23.6244962Z .................................................................................................... 5900/8953
2019-08-26T21:32:28.2278269Z .................................................................................................... 6000/8953
2019-08-26T21:32:28.2278269Z .................................................................................................... 6000/8953
2019-08-26T21:32:35.0536380Z .........................................................i..ii...................................... 6100/8953
2019-08-26T21:33:01.1529261Z .................................................................................................... 6300/8953
2019-08-26T21:33:03.0910341Z ...i................................................................................................ 6400/8953
2019-08-26T21:33:05.0659011Z ...........................................................................i........................ 6500/8953
2019-08-26T21:33:07.5158387Z .................................................................................................... 6600/8953
---
2019-08-26T21:37:32.5293092Z  finished in 18.624
2019-08-26T21:37:32.5452290Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:37:32.7001336Z 
2019-08-26T21:37:32.7001497Z running 149 tests
2019-08-26T21:37:35.6726221Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-08-26T21:37:37.5282444Z ..iiii..............i.........iii.i......ii......
2019-08-26T21:37:37.5283699Z 
2019-08-26T21:37:37.5287791Z  finished in 4.983
2019-08-26T21:37:37.5449222Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:37:37.7001432Z 
---
2019-08-26T21:37:39.6910132Z  finished in 2.146
2019-08-26T21:37:39.7072828Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:37:39.8606668Z 
2019-08-26T21:37:39.8607108Z running 9 tests
2019-08-26T21:37:39.8608106Z iiiiiiiii
2019-08-26T21:37:39.8608522Z 
2019-08-26T21:37:39.8608561Z  finished in 0.153
2019-08-26T21:37:39.8763845Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:37:40.0303230Z 
---
2019-08-26T21:37:57.0984613Z  finished in 17.220
2019-08-26T21:37:57.1146206Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:37:57.2671537Z 
2019-08-26T21:38:19.4467650Z running 122 tests
2019-08-26T21:38:19.4469294Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-26T21:38:23.7242071Z .i.i......iii.i.....ii
2019-08-26T21:38:23.7243625Z 
2019-08-26T21:38:23.7243749Z  finished in 26.609
2019-08-26T21:38:23.7253338Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T21:38:23.7253963Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-26T21:45:25.0821222Z ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
2019-08-26T21:45:25.0821282Z 
2019-08-26T21:45:25.0821334Z error: rustdoc failed!
2019-08-26T21:45:25.0821372Z status: exit code: 101
2019-08-26T21:45:25.0821859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
2019-08-26T21:45:25.0822123Z ------------------------------------------
2019-08-26T21:45:25.0822378Z 
2019-08-26T21:45:25.0823499Z running 1 test
2019-08-26T21:45:25.0823947Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-26T21:45:25.0823947Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-26T21:45:25.0823989Z 
2019-08-26T21:45:25.0824061Z failures:
2019-08-26T21:45:25.0824088Z 
2019-08-26T21:45:25.0824346Z ---- /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
2019-08-26T21:45:25.0827293Z "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--cfg" "rustdoc" "--cfg" "doctest" "--edition" "2015" "-o" "/tmp/rustdoctestmtxQ8P/rust_out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-"
2019-08-26T21:45:25.0827823Z Output { status: ExitStatus(ExitStatus(256)), stdout: "", stderr: "error: couldn\'t read \"/checkout/src/test/rustdoc/auxiliary/empty.rs: No such file or directory (os error 2)\n --> \"/checkout/src/test/rustdoc/issue-43153.rs\":7:1\n  |\n2 | include!(\"auxiliary/empty.rs\");\n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\nerror: aborting due to previous error\n\n" }
2019-08-26T21:45:25.0828225Z 
2019-08-26T21:45:25.0828262Z failures:
2019-08-26T21:45:25.0828470Z     /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6)
2019-08-26T21:45:25.0828506Z 
---
2019-08-26T21:45:25.0830424Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-26T21:45:25.0830473Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T21:45:25.0830885Z 
2019-08-26T21:45:25.0831573Z 
2019-08-26T21:45:25.0834418Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-26T21:45:25.0835034Z 
2019-08-26T21:45:25.0835069Z 
2019-08-26T21:45:25.0838056Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T21:45:25.0838530Z Build completed unsuccessfully in 1:11:23
2019-08-26T21:45:25.0838530Z Build completed unsuccessfully in 1:11:23
2019-08-26T21:45:25.0888363Z == clock drift check ==
2019-08-26T21:45:25.0906671Z   local time: Mon Aug 26 21:45:25 UTC 2019
2019-08-26T21:45:25.1858040Z   network time: Mon, 26 Aug 2019 21:45:25 GMT
2019-08-26T21:45:25.1862572Z == end clock drift check ==
2019-08-26T21:45:27.5151705Z ##[error]Bash exited with code '1'.
2019-08-26T21:45:27.5189157Z ##[section]Starting: Checkout
2019-08-26T21:45:27.5190977Z ==============================================================================
2019-08-26T21:45:27.5191020Z Task         : Get sources
2019-08-26T21:45:27.5191055Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
