plain
2020-04-07T03:18:13.6132711Z ========================== Starting Command Output ===========================
2020-04-07T03:18:13.6136917Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2176e986-a10c-4460-8d13-e3194d3d5a87.sh
2020-04-07T03:18:13.6137656Z 
2020-04-07T03:18:13.6142382Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T03:18:13.6160625Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-04-07T03:18:13.6164845Z Task         : Get sources
2020-04-07T03:18:13.6165112Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T03:18:13.6165429Z Version      : 1.0.0
2020-04-07T03:18:13.6165783Z Author       : Microsoft
---
2020-04-07T03:18:14.8190748Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T03:18:14.8197758Z ##[command]git config gc.auto 0
2020-04-07T03:18:14.8204988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T03:18:14.8211217Z ##[command]git config --get-all http.proxy
2020-04-07T03:18:14.8219868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67343/merge:refs/remotes/pull/67343/merge
---
2020-04-07T03:20:16.6097064Z Looks like docker image is the same as before, not uploading
2020-04-07T03:20:24.2655599Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T03:20:24.2980245Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T03:20:24.3009978Z == clock drift check ==
2020-04-07T03:20:24.3021758Z   local time: Tue Apr  7 03:20:24 UTC 2020
2020-04-07T03:20:24.5946420Z   network time: Tue, 07 Apr 2020 03:20:24 GMT
2020-04-07T03:20:24.5971118Z Starting sccache server...
2020-04-07T03:20:24.6847065Z configure: processing command line
2020-04-07T03:20:24.6847548Z configure: 
2020-04-07T03:20:24.6848659Z configure: rust.dist-src        := False
---
2020-04-07T03:26:36.7174364Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T03:26:38.3089590Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T03:26:39.9953477Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T03:26:41.8422886Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T03:26:50.8638618Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T03:26:54.4588525Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T03:26:59.1294712Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T03:27:03.7190538Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T03:27:13.3912425Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T03:51:11.6294145Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T03:51:13.5474576Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T03:51:15.6589798Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T03:51:17.5774004Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T03:51:29.1861140Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T03:51:32.2323997Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T03:51:37.9873801Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T03:51:43.7919818Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T03:51:55.9722276Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T04:18:59.4286898Z .................................................................................................F.. 1700/9885
2020-04-07T04:19:04.1834453Z .................................................................................................... 1800/9885
2020-04-07T04:19:13.3856921Z .................................................................................................... 1900/9885
2020-04-07T04:19:21.9979481Z ....i............................................................................................... 2000/9885
2020-04-07T04:19:28.5942296Z ..............................................................................................iiiii. 2100/9885
2020-04-07T04:19:50.7206449Z .................................................................................................... 2300/9885
2020-04-07T04:19:52.8774080Z .................................................................................................... 2400/9885
2020-04-07T04:19:55.2318275Z .................................................................................................... 2500/9885
2020-04-07T04:20:01.2186812Z .................................................................................................... 2600/9885
---
2020-04-07T04:23:11.5841995Z .................................................................................................... 5100/9885
2020-04-07T04:23:19.5991156Z .........................................F.......................................................... 5200/9885
2020-04-07T04:23:24.9999751Z .............i...................................................................................... 5300/9885
2020-04-07T04:23:35.1629588Z .................................................................................................... 5400/9885
2020-04-07T04:23:40.5349080Z ..ii.ii........i...i................................................................................ 5500/9885
2020-04-07T04:23:48.3795946Z ...............................................i.................................................... 5700/9885
2020-04-07T04:23:59.0802313Z ...................................................................ii............................... 5800/9885
2020-04-07T04:24:05.6212581Z ......i............................................................................................. 5900/9885
2020-04-07T04:24:11.5887503Z .................................................................................................... 6000/9885
2020-04-07T04:24:11.5887503Z .................................................................................................... 6000/9885
2020-04-07T04:24:21.7854688Z .................................................................................................... 6100/9885
2020-04-07T04:24:33.3783538Z ii...i..ii...........i.............................................................................. 6200/9885
2020-04-07T04:24:48.9762287Z .................................................................................................... 6400/9885
2020-04-07T04:24:55.5535878Z .................................................................................................... 6500/9885
2020-04-07T04:24:55.5535878Z .................................................................................................... 6500/9885
2020-04-07T04:25:12.0130877Z ..............................i..ii................................................................. 6600/9885
2020-04-07T04:25:34.3357560Z .................................................................................................... 6800/9885
2020-04-07T04:25:36.4974809Z ..............................i..................................................................... 6900/9885
2020-04-07T04:25:38.6511111Z .................................................................................................... 7000/9885
2020-04-07T04:25:40.9502098Z .....................................................................i.............................. 7100/9885
---
2020-04-07T04:27:23.3208608Z .................................................................................................... 7800/9885
2020-04-07T04:27:27.4473371Z .................................................................................................... 7900/9885
2020-04-07T04:27:34.1168930Z .................................................................................................... 8000/9885
2020-04-07T04:27:41.2969374Z ..................................i................................................................. 8100/9885
2020-04-07T04:27:50.4809106Z ...................................................................................iiiiiiiiii.i..... 8200/9885
2020-04-07T04:28:06.8651347Z ...........................i......i................................................................. 8400/9885
2020-04-07T04:28:10.5983991Z .................................................................................................... 8500/9885
2020-04-07T04:28:21.9105729Z .................................................................................................... 8600/9885
2020-04-07T04:28:35.0968306Z .................................................................................................... 8700/9885
---
2020-04-07T04:30:39.2070419Z failures:
2020-04-07T04:30:39.2089693Z 
2020-04-07T04:30:39.2090305Z ---- [ui] ui/consts/const_in_pattern/issue-65466.rs stdout ----
2020-04-07T04:30:39.2090501Z 
2020-04-07T04:30:39.2091112Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-04-07T04:30:39.2091570Z status: exit code: 101
2020-04-07T04:30:39.2094270Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/issue-65466.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-65466" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-65466/auxiliary"
2020-04-07T04:30:39.2096746Z ------------------------------------------
2020-04-07T04:30:39.2096916Z 
2020-04-07T04:30:39.2097283Z ------------------------------------------
2020-04-07T04:30:39.2097485Z stderr:
2020-04-07T04:30:39.2097485Z stderr:
2020-04-07T04:30:39.2097885Z ------------------------------------------
2020-04-07T04:30:39.2099644Z error: internal compiler error: src/librustc_trait_selection/traits/codegen/mod.rs:107: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<B as std::cmp::PartialEq>)), depth=2),Unimplemented)]` resolving bounds after type-checking
2020-04-07T04:30:39.2100597Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-04-07T04:30:39.2102734Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T04:30:39.2102950Z 
2020-04-07T04:30:39.2103142Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T04:30:39.2103142Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T04:30:39.2103340Z 
2020-04-07T04:30:39.2104031Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-07T04:30:39.2104733Z note: rustc 1.44.0-nightly (307111f0c 2020-04-07) running on x86_64-unknown-linux-gnu
2020-04-07T04:30:39.2104963Z 
2020-04-07T04:30:39.2104963Z 
2020-04-07T04:30:39.2105694Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-07T04:30:39.2106188Z error: aborting due to previous error
2020-04-07T04:30:39.2106336Z 
2020-04-07T04:30:39.2106422Z 
2020-04-07T04:30:39.2106937Z ------------------------------------------
---
2020-04-07T04:30:39.2108098Z 
2020-04-07T04:30:39.2109282Z - warning: to use a constant of type `std::cell::Cell` in a pattern, `std::cell::Cell` must be annotated with `#[derive(PartialEq, Eq)]`
2020-04-07T04:30:39.2110478Z -   --> $DIR/issue-55511.rs:16:9
2020-04-07T04:30:39.2112827Z -    |
2020-04-07T04:30:39.2113263Z - LL |         <() as Foo<'static>>::C => { }
2020-04-07T04:30:39.2114602Z -    |
2020-04-07T04:30:39.2115164Z - note: the lint level is defined here
2020-04-07T04:30:39.2115943Z -   --> $DIR/issue-55511.rs:1:9
2020-04-07T04:30:39.2116855Z -    |
2020-04-07T04:30:39.2116855Z -    |
2020-04-07T04:30:39.2117398Z - LL | #![warn(indirect_structural_match)]
2020-04-07T04:30:39.2118665Z -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-07T04:30:39.2119437Z -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-04-07T04:30:39.2120413Z -    = note: for more information, see issue #62411 <***/issues/62411>
2020-04-07T04:30:39.2121035Z 15 error[E0597]: `a` does not live long enough
2020-04-07T04:30:39.2121620Z 16   --> $DIR/issue-55511.rs:13:28
2020-04-07T04:30:39.2121821Z 17    |
2020-04-07T04:30:39.2121924Z 
2020-04-07T04:30:39.2121924Z 
2020-04-07T04:30:39.2122164Z 
2020-04-07T04:30:39.2122363Z The actual stderr differed from the expected stderr.
2020-04-07T04:30:39.2124034Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/issue-55511.stderr
2020-04-07T04:30:39.2124812Z To update references, rerun the tests and pass the `--bless` flag
2020-04-07T04:30:39.2126331Z To only update this specific test, also pass `--test-args issues/issue-55511.rs`
2020-04-07T04:30:39.2126748Z error: 1 errors occurred comparing output.
2020-04-07T04:30:39.2126987Z status: exit code: 1
2020-04-07T04:30:39.2126987Z status: exit code: 1
2020-04-07T04:30:39.2128910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55511.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/auxiliary"
2020-04-07T04:30:39.2130425Z ------------------------------------------
2020-04-07T04:30:39.2130588Z 
2020-04-07T04:30:39.2131165Z ------------------------------------------
2020-04-07T04:30:39.2131356Z stderr:
2020-04-07T04:30:39.2131356Z stderr:
2020-04-07T04:30:39.2131696Z ------------------------------------------
2020-04-07T04:30:39.2131962Z error[E0597]: `a` does not live long enough
2020-04-07T04:30:39.2132415Z   --> /checkout/src/test/ui/issues/issue-55511.rs:13:28
2020-04-07T04:30:39.2132630Z    |
2020-04-07T04:30:39.2132838Z LL |     let b = Some(Cell::new(&a));
2020-04-07T04:30:39.2133140Z    |                            ^^ borrowed value does not live long enough
2020-04-07T04:30:39.2133369Z ...
2020-04-07T04:30:39.2133756Z LL |         <() as Foo<'static>>::C => { }
2020-04-07T04:30:39.2134472Z    |         ----------------------- type annotation requires that `a` is borrowed for `'static`
2020-04-07T04:30:39.2135017Z LL | }
2020-04-07T04:30:39.2135424Z    | - `a` dropped here while still borrowed
2020-04-07T04:30:39.2135590Z 
2020-04-07T04:30:39.2135758Z error: aborting due to previous error
---
2020-04-07T04:30:39.2139483Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-07T04:30:39.2139871Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T04:30:39.2140104Z 
2020-04-07T04:30:39.2140192Z 
2020-04-07T04:30:39.2144031Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T04:30:39.2146585Z 
2020-04-07T04:30:39.2146673Z 
2020-04-07T04:30:39.2147167Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T04:30:39.2147522Z Build completed unsuccessfully in 1:07:54
2020-04-07T04:30:39.2147522Z Build completed unsuccessfully in 1:07:54
2020-04-07T04:30:39.2201145Z == clock drift check ==
2020-04-07T04:30:39.2228828Z   local time: Tue Apr  7 04:30:39 UTC 2020
2020-04-07T04:30:39.7722395Z   network time: Tue, 07 Apr 2020 04:30:39 GMT
2020-04-07T04:30:40.1253690Z 
2020-04-07T04:30:40.1253690Z 
2020-04-07T04:30:40.1326184Z ##[error]Bash exited with code '1'.
2020-04-07T04:30:40.1350637Z ##[section]Finishing: Run build
2020-04-07T04:30:40.1402781Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-04-07T04:30:40.1407482Z Task         : Get sources
2020-04-07T04:30:40.1407791Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T04:30:40.1408095Z Version      : 1.0.0
2020-04-07T04:30:40.1408293Z Author       : Microsoft
2020-04-07T04:30:40.1408293Z Author       : Microsoft
2020-04-07T04:30:40.1408609Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T04:30:40.1408993Z ==============================================================================
2020-04-07T04:30:40.4899104Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T04:30:40.4949865Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-04-07T04:30:40.5045421Z Cleaning up task key
2020-04-07T04:30:40.5046683Z Start cleaning up orphan processes.
2020-04-07T04:30:40.5263390Z Terminate orphan process: pid (3263) (python)
2020-04-07T04:30:40.5511101Z ##[section]Finishing: Finalize Job
