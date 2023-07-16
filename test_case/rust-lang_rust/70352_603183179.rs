plain
2020-03-24T10:12:27.1418291Z ========================== Starting Command Output ===========================
2020-03-24T10:12:27.1420723Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/95864800-1c81-4e38-9b60-db2d1a741179.sh
2020-03-24T10:12:27.1420971Z 
2020-03-24T10:12:27.1424926Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T10:12:27.1444710Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T10:12:27.1447973Z Task         : Get sources
2020-03-24T10:12:27.1448287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T10:12:27.1448570Z Version      : 1.0.0
2020-03-24T10:12:27.1448761Z Author       : Microsoft
---
2020-03-24T10:12:28.1593078Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T10:12:28.1602023Z ##[command]git config gc.auto 0
2020-03-24T10:12:28.1608925Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T10:12:28.1617170Z ##[command]git config --get-all http.proxy
2020-03-24T10:12:28.1627071Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70352/merge:refs/remotes/pull/70352/merge
---
2020-03-24T10:20:58.8707578Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T10:21:00.6076315Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T10:21:09.7124026Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T10:21:20.3702100Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T10:21:24.5705516Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T10:21:26.0253674Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T10:22:01.2101715Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T10:22:10.2391908Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T10:23:03.2906920Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T10:44:40.8453744Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T10:44:54.1041593Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T10:45:02.8401210Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T10:45:19.1056361Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T10:45:23.6750166Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T10:45:25.4586974Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T10:46:12.9442890Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T10:46:24.1693856Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T10:47:33.5397072Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T11:12:16.1024066Z .................................................................................................... 1700/9832
2020-03-24T11:12:20.1979455Z .................................................................................................... 1800/9832
2020-03-24T11:12:30.7512223Z .......................................................................................i............ 1900/9832
2020-03-24T11:12:38.0458021Z .................................................................................................... 2000/9832
2020-03-24T11:12:44.5123029Z .............................................................................iiiii.................. 2100/9832
2020-03-24T11:13:06.8419504Z .................................................................................................... 2300/9832
2020-03-24T11:13:09.1945789Z .................................................................................................... 2400/9832
2020-03-24T11:13:11.8200410Z .................................................................................................... 2500/9832
2020-03-24T11:13:21.5303023Z .................................................................................................... 2600/9832
---
2020-03-24T11:16:20.0326795Z ....................................................i...............i............................... 5000/9832
2020-03-24T11:16:27.8307131Z .................................................................................................... 5100/9832
2020-03-24T11:16:35.4599535Z .................................................................................................i.. 5200/9832
2020-03-24T11:16:40.6039854Z .................................................................................................... 5300/9832
2020-03-24T11:16:51.4539442Z ................................................................................ii.ii........i...i.. 5400/9832
2020-03-24T11:16:58.9762448Z ....................i............................................................................... 5600/9832
2020-03-24T11:17:06.4803230Z .........................i.......................................................................... 5700/9832
2020-03-24T11:17:14.4870596Z ..........................................ii....................................i................... 5800/9832
2020-03-24T11:17:21.7097717Z .................................................................................................... 5900/9832
2020-03-24T11:17:21.7097717Z .................................................................................................... 5900/9832
2020-03-24T11:17:27.3327178Z .................................................................................................... 6000/9832
2020-03-24T11:17:36.5773258Z ..........................................................................ii...i..ii...........i.... 6100/9832
2020-03-24T11:17:56.8098522Z .................................................................................................... 6300/9832
2020-03-24T11:18:04.0170367Z .................................................................................................... 6400/9832
2020-03-24T11:18:11.2636093Z .................................................................................................... 6500/9832
2020-03-24T11:18:11.2636093Z .................................................................................................... 6500/9832
2020-03-24T11:18:34.6983692Z ....i..ii........................................................................................... 6600/9832
2020-03-24T11:18:54.7630808Z .................................................................................................... 6800/9832
2020-03-24T11:18:56.8762455Z ...i................................................................................................ 6900/9832
2020-03-24T11:18:59.0075054Z .................................................................................................... 7000/9832
2020-03-24T11:19:01.4183234Z ......................................i............................................................. 7100/9832
---
2020-03-24T11:20:44.4808647Z .................................................................................................... 7800/9832
2020-03-24T11:20:49.3905415Z .................................................................................................... 7900/9832
2020-03-24T11:20:56.2012966Z ..............................................................................................i..... 8000/9832
2020-03-24T11:21:04.3872970Z .................................................................................................... 8100/9832
2020-03-24T11:21:11.9506799Z ...........................................iiiiiiiiii.i............................................. 8200/9832
2020-03-24T11:21:26.5897917Z .................................................................................................... 8400/9832
2020-03-24T11:21:31.9046896Z .................................................................................................... 8500/9832
2020-03-24T11:21:45.8527174Z .................................................................................................... 8600/9832
2020-03-24T11:21:54.8797607Z .................................................................................................... 8700/9832
---
2020-03-24T11:23:51.8219496Z 
2020-03-24T11:23:51.8219706Z 
2020-03-24T11:23:51.8220029Z The actual stderr differed from the expected stderr.
2020-03-24T11:23:51.8220846Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/tool_lints.stderr
2020-03-24T11:23:51.8221677Z To update references, rerun the tests and pass the `--bless` flag
2020-03-24T11:23:51.8222441Z To only update this specific test, also pass `--test-args tool_lints.rs`
2020-03-24T11:23:51.8223250Z error: 1 errors occurred comparing output.
2020-03-24T11:23:51.8223585Z status: exit code: 1
2020-03-24T11:23:51.8223585Z status: exit code: 1
2020-03-24T11:23:51.8225825Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary"
2020-03-24T11:23:51.8227760Z ------------------------------------------
2020-03-24T11:23:51.8228100Z 
2020-03-24T11:23:51.8228654Z ------------------------------------------
2020-03-24T11:23:51.8229157Z stderr:
2020-03-24T11:23:51.8229157Z stderr:
2020-03-24T11:23:51.8229721Z ------------------------------------------
2020-03-24T11:23:51.8230242Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T11:23:51.8230943Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T11:23:51.8231314Z    |
2020-03-24T11:23:51.8231637Z LL | #[warn(foo::bar)]
2020-03-24T11:23:51.8232272Z 
2020-03-24T11:23:51.8232637Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T11:23:51.8233627Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T11:23:51.8234012Z    |
2020-03-24T11:23:51.8234012Z    |
2020-03-24T11:23:51.8234298Z LL | #[warn(foo::bar)]
2020-03-24T11:23:51.8234811Z 
2020-03-24T11:23:51.8235190Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T11:23:51.8235861Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T11:23:51.8236218Z    |
2020-03-24T11:23:51.8236218Z    |
2020-03-24T11:23:51.8236516Z LL | #[warn(foo::bar)]
2020-03-24T11:23:51.8237043Z 
2020-03-24T11:23:51.8237340Z error: aborting due to 3 previous errors
2020-03-24T11:23:51.8237609Z 
2020-03-24T11:23:51.8238193Z For more information about this error, try `rustc --explain E0710`.
---
2020-03-24T11:23:51.8247932Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T11:23:51.8248692Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-24T11:23:51.8264053Z 
2020-03-24T11:23:51.8264397Z 
2020-03-24T11:23:51.8272369Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T11:23:51.8279221Z 
2020-03-24T11:23:51.8279446Z 
2020-03-24T11:23:51.8280198Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T11:23:51.8280616Z Build completed unsuccessfully in 1:06:31
2020-03-24T11:23:51.8280616Z Build completed unsuccessfully in 1:06:31
2020-03-24T11:23:51.8331374Z == clock drift check ==
2020-03-24T11:23:51.8361241Z   local time: Tue Mar 24 11:23:51 UTC 2020
2020-03-24T11:23:52.3875370Z   network time: Tue, 24 Mar 2020 11:23:52 GMT
2020-03-24T11:23:52.3881954Z == end clock drift check ==
2020-03-24T11:23:52.9985655Z 
2020-03-24T11:23:53.0057958Z ##[error]Bash exited with code '1'.
2020-03-24T11:23:53.0072399Z ##[section]Finishing: Run build
2020-03-24T11:23:53.0126532Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T11:23:53.0131888Z Task         : Get sources
2020-03-24T11:23:53.0132237Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T11:23:53.0132575Z Version      : 1.0.0
2020-03-24T11:23:53.0132810Z Author       : Microsoft
2020-03-24T11:23:53.0132810Z Author       : Microsoft
2020-03-24T11:23:53.0133170Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T11:23:53.0133808Z ==============================================================================
2020-03-24T11:23:53.3470131Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T11:23:53.3520333Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T11:23:53.3613285Z Cleaning up task key
2020-03-24T11:23:53.3614589Z Start cleaning up orphan processes.
2020-03-24T11:23:53.3964459Z Terminate orphan process: pid (3954) (python)
2020-03-24T11:23:53.4014279Z ##[section]Finishing: Finalize Job
