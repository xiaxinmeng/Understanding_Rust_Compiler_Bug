plain
2020-03-24T11:41:48.9095378Z ========================== Starting Command Output ===========================
2020-03-24T11:41:48.9097877Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/83970d5d-f04c-45e6-90eb-bd56aea3e075.sh
2020-03-24T11:41:48.9098170Z 
2020-03-24T11:41:48.9101858Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T11:41:48.9152003Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T11:41:48.9155572Z Task         : Get sources
2020-03-24T11:41:48.9155870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T11:41:48.9156146Z Version      : 1.0.0
2020-03-24T11:41:48.9156330Z Author       : Microsoft
---
2020-03-24T11:41:49.9067000Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T11:41:49.9072854Z ##[command]git config gc.auto 0
2020-03-24T11:41:49.9077861Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T11:41:49.9082510Z ##[command]git config --get-all http.proxy
2020-03-24T11:41:49.9089748Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70352/merge:refs/remotes/pull/70352/merge
---
2020-03-24T11:49:57.5035150Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T11:49:58.0995172Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T11:50:07.3188763Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T11:50:15.6951181Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T11:50:19.6231855Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T11:50:21.9542435Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T11:50:54.2141013Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T11:51:02.1454377Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T11:51:49.0927395Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T12:11:32.8345061Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T12:11:33.6901414Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T12:11:44.6621543Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T12:11:55.9255741Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T12:12:01.3716986Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T12:12:02.8620787Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T12:12:45.7690077Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T12:12:55.5610064Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T12:13:59.3799823Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T12:36:45.5750842Z .................................................................................................... 1700/9832
2020-03-24T12:36:49.4366622Z .................................................................................................... 1800/9832
2020-03-24T12:36:59.0129750Z .......................................................................................i............ 1900/9832
2020-03-24T12:37:05.7398126Z .................................................................................................... 2000/9832
2020-03-24T12:37:11.6346822Z .............................................................................iiiii.................. 2100/9832
2020-03-24T12:37:32.3526874Z .................................................................................................... 2300/9832
2020-03-24T12:37:34.5386933Z .................................................................................................... 2400/9832
2020-03-24T12:37:37.0007224Z .................................................................................................... 2500/9832
2020-03-24T12:37:46.2174511Z .................................................................................................... 2600/9832
---
2020-03-24T12:40:35.2282762Z ....................................................i...............i............................... 5000/9832
2020-03-24T12:40:42.7744688Z .................................................................................................... 5100/9832
2020-03-24T12:40:50.3339600Z .................................................................................................i.. 5200/9832
2020-03-24T12:40:55.4352984Z .................................................................................................... 5300/9832
2020-03-24T12:41:05.9611278Z .................................................................................iiii........i...i.. 5400/9832
2020-03-24T12:41:13.3603587Z ....................i............................................................................... 5600/9832
2020-03-24T12:41:20.5280092Z .........................i.......................................................................... 5700/9832
2020-03-24T12:41:28.2638327Z ..........................................ii....................................i................... 5800/9832
2020-03-24T12:41:35.3428155Z .................................................................................................... 5900/9832
2020-03-24T12:41:35.3428155Z .................................................................................................... 5900/9832
2020-03-24T12:41:40.7509185Z .................................................................................................... 6000/9832
2020-03-24T12:41:50.0687627Z ..........................................................................ii...i..ii...........i.... 6100/9832
2020-03-24T12:42:10.1394195Z .................................................................................................... 6300/9832
2020-03-24T12:42:16.8836776Z .................................................................................................... 6400/9832
2020-03-24T12:42:21.0601216Z .................................................................................................... 6500/9832
2020-03-24T12:42:21.0601216Z .................................................................................................... 6500/9832
2020-03-24T12:42:33.0344779Z ....i..ii........................................................................................... 6600/9832
2020-03-24T12:42:52.3942492Z .................................................................................................... 6800/9832
2020-03-24T12:42:54.4551633Z ...i................................................................................................ 6900/9832
2020-03-24T12:42:56.4791043Z .................................................................................................... 7000/9832
2020-03-24T12:42:58.8388909Z ......................................i............................................................. 7100/9832
---
2020-03-24T12:44:36.1677400Z .................................................................................................... 7800/9832
2020-03-24T12:44:40.8685605Z .................................................................................................... 7900/9832
2020-03-24T12:44:47.5499171Z ..............................................................................................i..... 8000/9832
2020-03-24T12:44:55.5986369Z .................................................................................................... 8100/9832
2020-03-24T12:45:02.9721620Z ...........................................iiiiiiiiii.i............................................. 8200/9832
2020-03-24T12:45:16.9684791Z .................................................................................................... 8400/9832
2020-03-24T12:45:22.1187448Z .................................................................................................... 8500/9832
2020-03-24T12:45:35.5185325Z .................................................................................................... 8600/9832
2020-03-24T12:45:44.1402657Z .................................................................................................... 8700/9832
---
2020-03-24T12:47:37.1657948Z 
2020-03-24T12:47:37.1658046Z 
2020-03-24T12:47:37.1658253Z The actual stderr differed from the expected stderr.
2020-03-24T12:47:37.1659108Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/tool_lints.stderr
2020-03-24T12:47:37.1659739Z To update references, rerun the tests and pass the `--bless` flag
2020-03-24T12:47:37.1660400Z To only update this specific test, also pass `--test-args tool_lints.rs`
2020-03-24T12:47:37.1660836Z error: 1 errors occurred comparing output.
2020-03-24T12:47:37.1661072Z status: exit code: 1
2020-03-24T12:47:37.1661072Z status: exit code: 1
2020-03-24T12:47:37.1662894Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary"
2020-03-24T12:47:37.1664425Z ------------------------------------------
2020-03-24T12:47:37.1664598Z 
2020-03-24T12:47:37.1664960Z ------------------------------------------
2020-03-24T12:47:37.1665166Z stderr:
2020-03-24T12:47:37.1665166Z stderr:
2020-03-24T12:47:37.1665556Z ------------------------------------------
2020-03-24T12:47:37.1665889Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T12:47:37.1666419Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T12:47:37.1666646Z    |
2020-03-24T12:47:37.1666816Z LL | #[warn(foo::bar)]
2020-03-24T12:47:37.1667141Z 
2020-03-24T12:47:37.1667403Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T12:47:37.1667927Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T12:47:37.1668135Z    |
2020-03-24T12:47:37.1668135Z    |
2020-03-24T12:47:37.1668321Z LL | #[warn(foo::bar)]
2020-03-24T12:47:37.1668633Z 
2020-03-24T12:47:37.1668907Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T12:47:37.1669434Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T12:47:37.1669642Z    |
2020-03-24T12:47:37.1669642Z    |
2020-03-24T12:47:37.1669813Z LL | #[warn(foo::bar)]
2020-03-24T12:47:37.1670138Z 
2020-03-24T12:47:37.1670327Z error: aborting due to 3 previous errors
2020-03-24T12:47:37.1670495Z 
2020-03-24T12:47:37.1670946Z For more information about this error, try `rustc --explain E0710`.
---
2020-03-24T12:47:37.1687119Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T12:47:37.1687573Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-24T12:47:37.1707072Z 
2020-03-24T12:47:37.1707262Z 
2020-03-24T12:47:37.1711494Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T12:47:37.1714279Z 
2020-03-24T12:47:37.1714381Z 
2020-03-24T12:47:37.1723304Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T12:47:37.1723716Z Build completed unsuccessfully in 1:01:00
2020-03-24T12:47:37.1723716Z Build completed unsuccessfully in 1:01:00
2020-03-24T12:47:37.1781616Z == clock drift check ==
2020-03-24T12:47:37.1805429Z   local time: Tue Mar 24 12:47:37 UTC 2020
2020-03-24T12:47:37.4750489Z   network time: Tue, 24 Mar 2020 12:47:37 GMT
2020-03-24T12:47:37.4750810Z == end clock drift check ==
2020-03-24T12:47:37.9103587Z 
2020-03-24T12:47:37.9141513Z ##[error]Bash exited with code '1'.
2020-03-24T12:47:37.9153974Z ##[section]Finishing: Run build
2020-03-24T12:47:37.9207380Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T12:47:37.9212320Z Task         : Get sources
2020-03-24T12:47:37.9212678Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T12:47:37.9212992Z Version      : 1.0.0
2020-03-24T12:47:37.9213216Z Author       : Microsoft
2020-03-24T12:47:37.9213216Z Author       : Microsoft
2020-03-24T12:47:37.9213590Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T12:47:37.9213994Z ==============================================================================
2020-03-24T12:47:38.2478724Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T12:47:38.2523712Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T12:47:38.2632268Z Cleaning up task key
2020-03-24T12:47:38.2633524Z Start cleaning up orphan processes.
2020-03-24T12:47:38.2847793Z Terminate orphan process: pid (3733) (python)
2020-03-24T12:47:38.3006478Z ##[section]Finishing: Finalize Job
