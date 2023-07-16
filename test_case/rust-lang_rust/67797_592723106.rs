plain
2020-02-28T19:35:43.6889115Z ========================== Starting Command Output ===========================
2020-02-28T19:35:43.6891401Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fbe8b974-1e72-4bd2-b591-03cab168c661.sh
2020-02-28T19:35:43.6891612Z 
2020-02-28T19:35:43.6895519Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-28T19:35:43.6915703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67797/merge to s
2020-02-28T19:35:43.6919658Z Task         : Get sources
2020-02-28T19:35:43.6919922Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T19:35:43.6920235Z Version      : 1.0.0
2020-02-28T19:35:43.6920408Z Author       : Microsoft
---
2020-02-28T19:35:44.7020422Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-28T19:35:44.7028549Z ##[command]git config gc.auto 0
2020-02-28T19:35:44.7036545Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-28T19:35:44.7043494Z ##[command]git config --get-all http.proxy
2020-02-28T19:35:44.7051079Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67797/merge:refs/remotes/pull/67797/merge
---
2020-02-28T20:40:28.5650832Z .................................................................................................... 1700/9735
2020-02-28T20:40:33.1975239Z .................................................................................................... 1800/9735
2020-02-28T20:40:45.2197603Z .......................................................................i............................ 1900/9735
2020-02-28T20:40:51.7987856Z .................................................................................................... 2000/9735
2020-02-28T20:41:07.7115103Z ............................................................iiiii................................... 2100/9735
2020-02-28T20:41:18.3533076Z .................................................................................................... 2300/9735
2020-02-28T20:41:20.6356492Z .................................................................................................... 2400/9735
2020-02-28T20:41:23.6363735Z .................................................................................................... 2500/9735
2020-02-28T20:41:43.5028096Z .................................................................................................... 2600/9735
---
2020-02-28T20:44:23.3854423Z .....................i...............i.............................................................. 5000/9735
2020-02-28T20:44:33.4851974Z .................................................................................................... 5100/9735
2020-02-28T20:44:39.0826959Z ................................................................i................................... 5200/9735
2020-02-28T20:44:45.8975906Z .................................................................................................... 5300/9735
2020-02-28T20:44:55.3346237Z .........................................ii.ii........i...i......................................... 5400/9735
2020-02-28T20:45:04.5416852Z .................................................................................................... 5600/9735
2020-02-28T20:45:14.5559791Z .................................................................................................... 5700/9735
2020-02-28T20:45:21.5616858Z ................................i................................................................... 5800/9735
2020-02-28T20:45:27.7449906Z .................................................................................................... 5900/9735
2020-02-28T20:45:27.7449906Z .................................................................................................... 5900/9735
2020-02-28T20:45:38.8612549Z .................................................................................................... 6000/9735
2020-02-28T20:45:49.0843017Z .......................ii...i..ii...........i....................................................... 6100/9735
2020-02-28T20:46:06.2700155Z .................................................................................................... 6300/9735
2020-02-28T20:46:13.6604886Z .................................................................................................... 6400/9735
2020-02-28T20:46:13.6604886Z .................................................................................................... 6400/9735
2020-02-28T20:46:30.1843645Z ......................................................i..ii......................................... 6500/9735
2020-02-28T20:47:02.2031713Z .................................................................................................... 6700/9735
2020-02-28T20:47:04.6058738Z ..............................................i..................................................... 6800/9735
2020-02-28T20:47:06.7100388Z .................................................................................................... 6900/9735
2020-02-28T20:47:08.8554470Z ............................................................................i....................... 7000/9735
---
2020-02-28T20:48:49.6437728Z .................................................................................................... 7700/9735
2020-02-28T20:48:54.2932359Z .................................................................................................... 7800/9735
2020-02-28T20:48:59.6121523Z .................................................................................................... 7900/9735
2020-02-28T20:49:07.8938384Z .....................i.............................................................................. 8000/9735
2020-02-28T20:49:16.6679217Z ......................................................................iiiiiii.i..................... 8100/9735
2020-02-28T20:49:32.0585961Z ...........i......i................................................................................. 8300/9735
2020-02-28T20:49:37.5454039Z .................................................................................................... 8400/9735
2020-02-28T20:49:51.3274614Z .................................................................................................... 8500/9735
2020-02-28T20:50:01.1210951Z .................................................................................................... 8600/9735
---
2020-02-28T20:52:31.4061649Z  finished in 7.583
2020-02-28T20:52:31.4239156Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-28T20:52:31.5843341Z 
2020-02-28T20:52:31.5845272Z running 178 tests
2020-02-28T20:52:34.6258441Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-28T20:52:36.9777512Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-28T20:52:36.9780203Z 
2020-02-28T20:52:36.9785104Z  finished in 5.554
2020-02-28T20:52:36.9982865Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-28T20:52:37.1625917Z 
---
2020-02-28T20:52:39.1257219Z  finished in 2.127
2020-02-28T20:52:39.1454763Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-28T20:52:39.3110838Z 
2020-02-28T20:52:39.3111224Z running 9 tests
2020-02-28T20:52:39.3112258Z iiiiiiiii
2020-02-28T20:52:39.3113226Z 
2020-02-28T20:52:39.3113369Z  finished in 0.165
2020-02-28T20:52:39.3319881Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-28T20:52:39.4954099Z 
2020-02-28T20:52:39.4954099Z 
2020-02-28T20:52:39.4955223Z running 116 tests
2020-02-28T20:52:57.1754130Z ....................F............................................................................... 100/116
2020-02-28T20:52:59.6848946Z ................
2020-02-28T20:52:59.6851476Z failures:
2020-02-28T20:52:59.6852591Z 
2020-02-28T20:52:59.6854018Z ---- [incremental] incremental/const-generics/issue-68477.rs stdout ----
2020-02-28T20:52:59.6854488Z 
2020-02-28T20:52:59.6854969Z error in revision `rpass1`: compilation failed!
2020-02-28T20:52:59.6855557Z status: exit code: 101
2020-02-28T20:52:59.6858337Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/const-generics/issue-68477.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/issue-68477/issue-68477.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/issue-68477/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/issue-68477/auxiliary"
2020-02-28T20:52:59.6862492Z ------------------------------------------
2020-02-28T20:52:59.6862957Z 
2020-02-28T20:52:59.6863424Z ------------------------------------------
2020-02-28T20:52:59.6863748Z stderr:
---
2020-02-28T20:52:59.6866305Z    |            ^^^^^^^^^^^^^^
2020-02-28T20:52:59.6866556Z    |
2020-02-28T20:52:59.6866871Z    = note: `#[warn(incomplete_features)]` on by default
2020-02-28T20:52:59.6867508Z 
2020-02-28T20:52:59.6868436Z error: internal compiler error: src/librustc/ich/impls_ty.rs:99: StableHasher: unexpected region RePlaceholder(Placeholder { universe: U9, name: BrAnon(1) })
2020-02-28T20:52:59.6869642Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-02-28T20:52:59.6870411Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T20:52:59.6870819Z 
2020-02-28T20:52:59.6871409Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-28T20:52:59.6871409Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-28T20:52:59.6871760Z 
2020-02-28T20:52:59.6873401Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-28T20:52:59.6874494Z note: rustc 1.43.0-nightly (8c69dc0f7 2020-02-28) running on x86_64-unknown-linux-gnu
2020-02-28T20:52:59.6874890Z 
2020-02-28T20:52:59.6874890Z 
2020-02-28T20:52:59.6875899Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-28T20:52:59.6877335Z error: aborting due to previous error
2020-02-28T20:52:59.6877491Z 
2020-02-28T20:52:59.6877585Z 
2020-02-28T20:52:59.6878004Z ------------------------------------------
---
2020-02-28T20:52:59.6955038Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-28T20:52:59.6955496Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T20:52:59.6955763Z 
2020-02-28T20:52:59.6955866Z 
2020-02-28T20:52:59.6960243Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-28T20:52:59.6963306Z 
2020-02-28T20:52:59.6963405Z 
2020-02-28T20:52:59.6963640Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-28T20:52:59.6964166Z Build completed unsuccessfully in 1:10:10
2020-02-28T20:52:59.6964166Z Build completed unsuccessfully in 1:10:10
2020-02-28T20:52:59.7010627Z == clock drift check ==
2020-02-28T20:52:59.7026291Z   local time: Fri Feb 28 20:52:59 UTC 2020
2020-02-28T20:53:00.2461484Z   network time: Fri, 28 Feb 2020 20:53:00 GMT
2020-02-28T20:53:00.2463612Z == end clock drift check ==
2020-02-28T20:53:02.4953922Z 
2020-02-28T20:53:02.5026555Z ##[error]Bash exited with code '1'.
2020-02-28T20:53:02.5041169Z ##[section]Finishing: Run build
2020-02-28T20:53:02.5089268Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67797/merge to s
2020-02-28T20:53:02.5094354Z Task         : Get sources
2020-02-28T20:53:02.5094661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T20:53:02.5094966Z Version      : 1.0.0
2020-02-28T20:53:02.5095164Z Author       : Microsoft
2020-02-28T20:53:02.5095164Z Author       : Microsoft
2020-02-28T20:53:02.5095476Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-28T20:53:02.5095861Z ==============================================================================
2020-02-28T20:53:02.8482140Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-28T20:53:02.8530646Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67797/merge to s
2020-02-28T20:53:02.8622198Z Cleaning up task key
2020-02-28T20:53:02.8623425Z Start cleaning up orphan processes.
2020-02-28T20:53:02.8819261Z Terminate orphan process: pid (3577) (python)
2020-02-28T20:53:02.9007621Z ##[section]Finishing: Finalize Job
