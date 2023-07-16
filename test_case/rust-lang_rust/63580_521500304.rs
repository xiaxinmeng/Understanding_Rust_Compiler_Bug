plain
2019-08-15T02:13:13.2305623Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T02:13:13.2471459Z ##[command]git config gc.auto 0
2019-08-15T02:13:13.2541500Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T02:13:13.2599855Z ##[command]git config --get-all http.proxy
2019-08-15T02:13:13.2734660Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63580/merge:refs/remotes/pull/63580/merge
---
2019-08-15T02:13:48.3611651Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T02:13:48.3611680Z 
2019-08-15T02:13:48.3611888Z   git checkout -b <new-branch-name>
2019-08-15T02:13:48.3612030Z 
2019-08-15T02:13:48.3612106Z HEAD is now at 4e118acbb Merge 7676920f0f61bdde2287bd6e1854ab61686db7a8 into 082cf2f9d136166cd1d552d3fb5abb1c46c99a14
2019-08-15T02:13:48.3778126Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T02:13:48.3781098Z ==============================================================================
2019-08-15T02:13:48.3781164Z Task         : Bash
2019-08-15T02:13:48.3781232Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T02:16:09.6617271Z Looks like docker image is the same as before, not uploading
2019-08-15T02:16:19.1377948Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-15T02:16:19.3052328Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-15T02:16:19.3082884Z == clock drift check ==
2019-08-15T02:16:19.3093774Z   local time: Thu Aug 15 02:16:19 UTC 2019
2019-08-15T02:16:19.4603599Z   network time: Thu, 15 Aug 2019 02:16:19 GMT
2019-08-15T02:16:19.4630437Z Starting sccache server...
2019-08-15T02:16:19.5603784Z configure: processing command line
2019-08-15T02:16:19.5603922Z configure: 
2019-08-15T02:16:19.5608902Z configure: rust.dist-src        := False
---
2019-08-15T03:16:52.1087780Z .................................................................................................... 1400/8906
2019-08-15T03:16:58.7813281Z .................................................................................................... 1500/8906
2019-08-15T03:17:08.5458375Z ..............................F.............................................................i....... 1600/8906
2019-08-15T03:17:17.1574257Z ........i........................................................................................... 1700/8906
2019-08-15T03:17:24.5488463Z ...................................................................................iiiii............ 1800/8906
2019-08-15T03:17:47.6010276Z .................................................................................................... 2000/8906
2019-08-15T03:17:50.2267919Z .................................................................................................... 2100/8906
2019-08-15T03:17:52.9992932Z .................................................................................................... 2200/8906
2019-08-15T03:18:00.7730300Z .................................................................................................... 2300/8906
---
2019-08-15T03:22:05.6133568Z .................................................................................................... 5300/8906
2019-08-15T03:22:13.5691508Z .................i.................................................................................. 5400/8906
2019-08-15T03:22:19.5539623Z .................................................................................................... 5500/8906
2019-08-15T03:22:32.0011571Z .................................................................................................... 5600/8906
2019-08-15T03:22:45.0946476Z ............ii...i..ii...........i.................................................................. 5700/8906
2019-08-15T03:22:59.6154772Z .................................................................................................... 5900/8906
2019-08-15T03:23:04.5656553Z .................................................................................................... 6000/8906
2019-08-15T03:23:18.6545163Z .............i...ii................................................................................. 6100/8906
2019-08-15T03:23:30.0927508Z .................................................................................................... 6200/8906
---
2019-08-15T03:27:52.5717188Z failures:
2019-08-15T03:27:52.5751127Z 
2019-08-15T03:27:52.5751964Z ---- [ui] ui/consts/promotion.rs stdout ----
2019-08-15T03:27:52.5752303Z 
2019-08-15T03:27:52.5752875Z error: test compilation failed although it shouldn't!
2019-08-15T03:27:52.5753259Z status: exit code: 1
2019-08-15T03:27:52.5754286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promotion/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promotion/auxiliary" "-A" "unused"
2019-08-15T03:27:52.5755179Z ------------------------------------------
2019-08-15T03:27:52.5755479Z 
2019-08-15T03:27:52.5755989Z ------------------------------------------
2019-08-15T03:27:52.5756318Z stderr:
2019-08-15T03:27:52.5756318Z stderr:
2019-08-15T03:27:52.5756813Z ------------------------------------------
2019-08-15T03:27:52.5757127Z error: this expression will panic at runtime
2019-08-15T03:27:52.5758208Z   --> /checkout/src/test/ui/consts/promotion.rs:16:14
2019-08-15T03:27:52.5758635Z    |
2019-08-15T03:27:52.5759167Z LL |     baz_i32(&-std::i32::MIN);
2019-08-15T03:27:52.5759529Z    |              ^^^^^^^^^^^^^^ attempt to negate with overflow
2019-08-15T03:27:52.5760044Z    = note: `#[deny(const_err)]` on by default
2019-08-15T03:27:52.5760293Z 
2019-08-15T03:27:52.5760543Z error: this expression will panic at runtime
2019-08-15T03:27:52.5761054Z   --> /checkout/src/test/ui/consts/promotion.rs:15:14
2019-08-15T03:27:52.5761054Z   --> /checkout/src/test/ui/consts/promotion.rs:15:14
2019-08-15T03:27:52.5761392Z    |
2019-08-15T03:27:52.5761882Z LL |     baz_u32(&(0-1));
2019-08-15T03:27:52.5762204Z    |              ^^^^^ attempt to subtract with overflow
2019-08-15T03:27:52.5762729Z error: aborting due to 2 previous errors
2019-08-15T03:27:52.5762952Z 
2019-08-15T03:27:52.5763202Z 
2019-08-15T03:27:52.5763701Z ------------------------------------------
---
2019-08-15T03:27:52.5789445Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-15T03:27:52.5789744Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-15T03:27:52.5803671Z 
2019-08-15T03:27:52.5803823Z 
2019-08-15T03:27:52.5809831Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-15T03:27:52.5811001Z 
2019-08-15T03:27:52.5811204Z 
2019-08-15T03:27:52.5819931Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-15T03:27:52.5820212Z Build completed unsuccessfully in 1:07:33
2019-08-15T03:27:52.5820212Z Build completed unsuccessfully in 1:07:33
2019-08-15T03:27:52.5878985Z == clock drift check ==
2019-08-15T03:27:52.5895236Z   local time: Thu Aug 15 03:27:52 UTC 2019
2019-08-15T03:27:52.7405248Z   network time: Thu, 15 Aug 2019 03:27:52 GMT
2019-08-15T03:27:52.7406144Z == end clock drift check ==
2019-08-15T03:27:53.5058240Z ##[error]Bash exited with code '1'.
2019-08-15T03:27:53.5102274Z ##[section]Starting: Checkout
2019-08-15T03:27:53.5104322Z ==============================================================================
2019-08-15T03:27:53.5104397Z Task         : Get sources
2019-08-15T03:27:53.5104444Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
