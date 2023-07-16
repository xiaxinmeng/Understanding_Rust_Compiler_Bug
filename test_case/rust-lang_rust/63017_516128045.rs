plain
2019-07-29T18:11:02.0155323Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T18:11:02.0326737Z ##[command]git config gc.auto 0
2019-07-29T18:11:02.0389517Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T18:11:02.0442078Z ##[command]git config --get-all http.proxy
2019-07-29T18:11:02.0563355Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63017/merge:refs/remotes/pull/63017/merge
---
2019-07-29T18:11:38.1266617Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T18:11:38.1267570Z 
2019-07-29T18:11:38.1269436Z   git checkout -b <new-branch-name>
2019-07-29T18:11:38.1270799Z 
2019-07-29T18:11:38.1271586Z HEAD is now at 76a98f320 Merge a0ca45d10c513f5da4f2bbf0e99b688cebdbe7b4 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T18:11:38.1390257Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T18:11:38.1392864Z ==============================================================================
2019-07-29T18:11:38.1392926Z Task         : Bash
2019-07-29T18:11:38.1392959Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T19:06:50.3197566Z .................................................................................................... 1400/8804
2019-07-29T19:06:55.8917797Z .................................................................................................... 1500/8804
2019-07-29T19:07:08.2373641Z ................................................................i...............i................... 1600/8804
2019-07-29T19:07:15.8204417Z .................................................................................................... 1700/8804
2019-07-29T19:07:29.6259466Z ..................................................iiiii............................................. 1800/8804
2019-07-29T19:07:39.7470313Z .................................................................................................... 2000/8804
2019-07-29T19:07:42.4925033Z .................................................................................................... 2100/8804
2019-07-29T19:07:45.6209852Z .................................................................................................... 2200/8804
2019-07-29T19:07:51.9281965Z .................................................................................................... 2300/8804
---
2019-07-29T19:11:13.9860773Z .................................................................................................... 5200/8804
2019-07-29T19:11:24.5232170Z .................................................................................................... 5300/8804
2019-07-29T19:11:31.6764425Z ..i................................................................................................. 5400/8804
2019-07-29T19:11:36.4801017Z .................................................................................................... 5500/8804
2019-07-29T19:11:47.4359754Z ................................................................................................ii.. 5600/8804
2019-07-29T19:12:01.2861768Z .i..ii...........i.................................................................................. 5700/8804
2019-07-29T19:12:14.3704576Z .................................................................................................... 5900/8804
2019-07-29T19:12:18.8934431Z ................................................................................................i..i 6000/8804
2019-07-29T19:12:32.2722885Z i................................................................................................... 6100/8804
2019-07-29T19:12:47.7012955Z .................................................................................................... 6200/8804
---
2019-07-29T19:17:04.3445432Z  finished in 18.339
2019-07-29T19:17:04.3603156Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:05.1260085Z 
2019-07-29T19:17:05.1327753Z running 146 tests
2019-07-29T19:17:07.3336160Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-29T19:17:09.1269648Z iii..............i.........iii.i......ii......
2019-07-29T19:17:09.1270951Z 
2019-07-29T19:17:09.1307624Z  finished in 4.564
2019-07-29T19:17:09.1319163Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:09.1338375Z 
---
2019-07-29T19:17:10.9036924Z  finished in 1.963
2019-07-29T19:17:10.9189805Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:11.0590963Z 
2019-07-29T19:17:11.0591563Z running 9 tests
2019-07-29T19:17:11.0592321Z iiiiiiiii
2019-07-29T19:17:11.0593824Z 
2019-07-29T19:17:11.0593986Z  finished in 0.140
2019-07-29T19:17:11.0748076Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:11.2169465Z 
---
2019-07-29T19:17:27.2832930Z  finished in 16.208
2019-07-29T19:17:27.2986710Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:27.4475726Z 
2019-07-29T19:17:27.4476134Z running 122 tests
2019-07-29T19:17:48.7537124Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-07-29T19:17:52.7181016Z .i.i......iii.i.....ii
2019-07-29T19:17:52.7181913Z 
2019-07-29T19:17:52.7186875Z  finished in 25.420
2019-07-29T19:17:52.7192945Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T19:17:52.7193244Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-07-29T19:24:20.5410430Z failures:
2019-07-29T19:24:20.5414232Z 
2019-07-29T19:24:20.5416406Z ---- [rustdoc] rustdoc/bad-codeblock-syntax.rs stdout ----
2019-07-29T19:24:20.5416460Z 
2019-07-29T19:24:20.5416648Z error: htmldocck failed!
2019-07-29T19:24:20.5417121Z status: exit code: 1
2019-07-29T19:24:20.5417682Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax" "/checkout/src/test/rustdoc/bad-codeblock-syntax.rs"
2019-07-29T19:24:20.5417976Z ------------------------------------------
2019-07-29T19:24:20.5418001Z 
2019-07-29T19:24:20.5418184Z ------------------------------------------
2019-07-29T19:24:20.5418378Z stderr:
2019-07-29T19:24:20.5418378Z stderr:
2019-07-29T19:24:20.5418584Z ------------------------------------------
2019-07-29T19:24:20.5418620Z 2: @has check failed
2019-07-29T19:24:20.5418668Z  `XPATH PATTERN` did not match
2019-07-29T19:24:20.5418853Z  // @has - '//*[@class="docblock"]/pre/code' '\_'
2019-07-29T19:24:20.5418889Z 9: @has check failed
2019-07-29T19:24:20.5418936Z  `XPATH PATTERN` did not match
2019-07-29T19:24:20.5419129Z  // @has - '//*[@class="docblock"]/pre/code' '`baz::foobar`'
2019-07-29T19:24:20.5419166Z 16: @has check failed
2019-07-29T19:24:20.5419206Z  `XPATH PATTERN` did not match
2019-07-29T19:24:20.5419407Z  // @has - '//*[@class="docblock"]/pre/code' '\_'
2019-07-29T19:24:20.5419468Z Encountered 3 errors
2019-07-29T19:24:20.5419490Z 
2019-07-29T19:24:20.5419711Z ------------------------------------------
2019-07-29T19:24:20.5419736Z 
---
2019-07-29T19:24:20.5420545Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T19:24:20.5420588Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T19:24:20.5420613Z 
2019-07-29T19:24:20.5420648Z 
2019-07-29T19:24:20.5422832Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T19:24:20.5423148Z 
2019-07-29T19:24:20.5423184Z 
2019-07-29T19:24:20.5429881Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T19:24:20.5458454Z Build completed unsuccessfully in 1:06:16
2019-07-29T19:24:20.5458454Z Build completed unsuccessfully in 1:06:16
2019-07-29T19:24:22.8561653Z ##[error]Bash exited with code '1'.
2019-07-29T19:24:22.8598146Z ##[section]Starting: Checkout
2019-07-29T19:24:22.8599662Z ==============================================================================
2019-07-29T19:24:22.8599707Z Task         : Get sources
2019-07-29T19:24:22.8599744Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
