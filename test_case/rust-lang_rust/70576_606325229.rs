plain
2020-03-30T23:33:23.6755379Z ========================== Starting Command Output ===========================
2020-03-30T23:33:23.6757986Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/35a9938c-8785-4d53-9c0b-ba7189de7cbf.sh
2020-03-30T23:33:23.6758252Z 
2020-03-30T23:33:23.6761608Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T23:33:23.6779582Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-30T23:33:23.6783102Z Task         : Get sources
2020-03-30T23:33:23.6783411Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T23:33:23.6783723Z Version      : 1.0.0
2020-03-30T23:33:23.6783923Z Author       : Microsoft
---
2020-03-30T23:33:24.9755589Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T23:33:24.9835941Z ##[command]git config gc.auto 0
2020-03-30T23:33:24.9839259Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T23:33:24.9841856Z ##[command]git config --get-all http.proxy
2020-03-30T23:33:24.9846891Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70576/merge:refs/remotes/pull/70576/merge
---
2020-03-30T23:40:01.7168987Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T23:40:02.9262168Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T23:40:04.2918922Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T23:40:04.6209994Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T23:40:13.1418421Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T23:40:14.6033436Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T23:40:18.3704200Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T23:40:21.8771339Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T23:40:30.7529487Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T23:59:58.2253895Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T23:59:59.7770410Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T00:00:01.6591927Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T00:00:03.0913315Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T00:00:13.0229231Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T00:00:15.6596969Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T00:00:20.3591549Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T00:00:25.3599590Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T00:00:35.3645589Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T00:23:12.6193017Z .................................................................................................... 1700/9856
2020-03-31T00:23:16.0533615Z .................................................................................................... 1800/9856
2020-03-31T00:23:25.1791188Z ..........................................................................................i......... 1900/9856
2020-03-31T00:23:31.1328874Z .................................................................................................... 2000/9856
2020-03-31T00:23:36.6047153Z ................................................................................iiiii............... 2100/9856
2020-03-31T00:23:54.6383019Z .................................................................................................... 2300/9856
2020-03-31T00:23:56.5597913Z .................................................................................................... 2400/9856
2020-03-31T00:23:58.5900332Z .................................................................................................... 2500/9856
2020-03-31T00:24:04.3472482Z .................................................................................................... 2600/9856
2020-03-31T00:24:04.3472482Z .................................................................................................... 2600/9856
2020-03-31T00:24:20.1739166Z .................................................................................................... 2700/9856
2020-03-31T00:24:20.4523639Z .............- error[E0658]: is unstable
2020-03-31T00:24:20.4525876Z + error[E0658]: kind="link_cfg" is unstable, see https://github.com/rust-lang/rfcs/pull/1721
2020-03-31T00:24:20.4529074Z 3    |
2020-03-31T00:24:20.4529074Z 3    |
2020-03-31T00:24:20.4530255Z 4 LL | #[link(name = "foo", cfg(foo))]
2020-03-31T00:24:23.6492161Z F...................................................................................... 2800/9856
2020-03-31T00:24:32.5951124Z .................................................i.................................................. 2900/9856
2020-03-31T00:24:37.9624399Z .................................................................................................... 3000/9856
2020-03-31T00:24:44.5154676Z .................................................................................................... 3100/9856
---
2020-03-31T00:26:41.1549261Z ......................................................i...............i............................. 5000/9856
2020-03-31T00:26:48.2565665Z .................................................................................................... 5100/9856
2020-03-31T00:26:54.7077556Z ...................................................................................................i 5200/9856
2020-03-31T00:26:59.3793979Z .................................................................................................... 5300/9856
2020-03-31T00:27:09.7410680Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T00:27:13.3193987Z ..i................................................................................................. 5500/9856
2020-03-31T00:27:22.1748886Z ..............................i..................................................................... 5700/9856
2020-03-31T00:27:31.4920391Z ................................................ii....................................i............. 5800/9856
2020-03-31T00:27:38.8666532Z .................................................................................................... 5900/9856
2020-03-31T00:27:43.8445002Z .................................................................................................... 6000/9856
2020-03-31T00:27:43.8445002Z .................................................................................................... 6000/9856
2020-03-31T00:27:52.5377139Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T00:28:03.6091227Z .i.................................................................................................. 6200/9856
2020-03-31T00:28:15.7411766Z .................................................................................................... 6400/9856
2020-03-31T00:28:18.8916142Z .................................................................................................... 6500/9856
2020-03-31T00:28:18.8916142Z .................................................................................................... 6500/9856
2020-03-31T00:28:30.1140492Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T00:28:48.1850360Z .................................................................................................... 6800/9856
2020-03-31T00:28:50.1550069Z ..........i......................................................................................... 6900/9856
2020-03-31T00:28:52.0285587Z .................................................................................................... 7000/9856
2020-03-31T00:28:53.9483212Z ...............................................i.................................................... 7100/9856
---
2020-03-31T00:30:22.3315213Z .................................................................................................... 7800/9856
2020-03-31T00:30:27.6257637Z .................................................................................................... 7900/9856
2020-03-31T00:30:33.3690823Z .................................................................................................... 8000/9856
2020-03-31T00:30:40.7570798Z .......i............................................................................................ 8100/9856
2020-03-31T00:30:48.4562306Z ........................................................iiiiiiiiii.i................................ 8200/9856
2020-03-31T00:31:01.1038499Z i......i............................................................................................ 8400/9856
2020-03-31T00:31:05.8372871Z .................................................................................................... 8500/9856
2020-03-31T00:31:17.3991797Z .................................................................................................... 8600/9856
2020-03-31T00:31:26.5682547Z .................................................................................................... 8700/9856
---
2020-03-31T00:33:07.2049819Z diff of stderr:
2020-03-31T00:33:07.2050066Z 
2020-03-31T00:33:07.2050286Z 
2020-03-31T00:33:07.2050620Z The actual stderr differed from the expected stderr.
2020-03-31T00:33:07.2051526Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-link_cfg/feature-gate-link_cfg.stderr
2020-03-31T00:33:07.2052364Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T00:33:07.2053347Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-link_cfg.rs`
2020-03-31T00:33:07.2053947Z error: 1 errors occurred comparing output.
2020-03-31T00:33:07.2054251Z status: exit code: 1
2020-03-31T00:33:07.2054251Z status: exit code: 1
2020-03-31T00:33:07.2056041Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-link_cfg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-link_cfg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-link_cfg/auxiliary"
2020-03-31T00:33:07.2057737Z ------------------------------------------
2020-03-31T00:33:07.2057986Z 
2020-03-31T00:33:07.2058427Z ------------------------------------------
2020-03-31T00:33:07.2058701Z stderr:
2020-03-31T00:33:07.2058701Z stderr:
2020-03-31T00:33:07.2059120Z ------------------------------------------
2020-03-31T00:33:07.2059753Z error[E0658]: kind="link_cfg" is unstable, see https://github.com/rust-lang/rfcs/pull/1721
2020-03-31T00:33:07.2060751Z    |
2020-03-31T00:33:07.2060751Z    |
2020-03-31T00:33:07.2061007Z LL | #[link(name = "foo", cfg(foo))]
2020-03-31T00:33:07.2061796Z    |
2020-03-31T00:33:07.2061796Z    |
2020-03-31T00:33:07.2062498Z    = note: see issue #37406 <***/issues/37406> for more information
2020-03-31T00:33:07.2062916Z    = help: add `#![feature(link_cfg)]` to the crate attributes to enable
2020-03-31T00:33:07.2063455Z error: aborting due to previous error
2020-03-31T00:33:07.2063703Z 
2020-03-31T00:33:07.2064190Z For more information about this error, try `rustc --explain E0658`.
2020-03-31T00:33:07.2064477Z 
---
2020-03-31T00:33:07.2072258Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T00:33:07.2072788Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T00:33:07.2084484Z 
2020-03-31T00:33:07.2084783Z 
2020-03-31T00:33:07.2090792Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T00:33:07.2093095Z 
2020-03-31T00:33:07.2093942Z 
2020-03-31T00:33:07.2098561Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T00:33:07.2099770Z Build completed unsuccessfully in 0:56:13
2020-03-31T00:33:07.2099770Z Build completed unsuccessfully in 0:56:13
2020-03-31T00:33:07.2144675Z == clock drift check ==
2020-03-31T00:33:07.2162630Z   local time: Tue Mar 31 00:33:07 UTC 2020
2020-03-31T00:33:07.4893571Z   network time: Tue, 31 Mar 2020 00:33:07 GMT
2020-03-31T00:33:07.4902670Z == end clock drift check ==
2020-03-31T00:33:07.9842655Z 
2020-03-31T00:33:07.9884383Z ##[error]Bash exited with code '1'.
2020-03-31T00:33:07.9896726Z ##[section]Finishing: Run build
2020-03-31T00:33:07.9939451Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-31T00:33:07.9943772Z Task         : Get sources
2020-03-31T00:33:07.9944036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T00:33:07.9944266Z Version      : 1.0.0
2020-03-31T00:33:07.9944444Z Author       : Microsoft
2020-03-31T00:33:07.9944444Z Author       : Microsoft
2020-03-31T00:33:07.9944698Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T00:33:07.9945005Z ==============================================================================
2020-03-31T00:33:08.2868056Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T00:33:08.2912530Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70576/merge to s
2020-03-31T00:33:08.2992143Z Cleaning up task key
2020-03-31T00:33:08.2993050Z Start cleaning up orphan processes.
2020-03-31T00:33:08.3131458Z Terminate orphan process: pid (4444) (python)
2020-03-31T00:33:08.3275106Z ##[section]Finishing: Finalize Job
