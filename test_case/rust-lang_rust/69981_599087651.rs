plain
2020-03-14T14:57:09.5196886Z ========================== Starting Command Output ===========================
2020-03-14T14:57:09.5213047Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/58df88da-e1fa-4d2b-a653-364c71f6ea70.sh
2020-03-14T14:57:09.7651509Z 
2020-03-14T14:57:09.7686384Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T14:57:09.7723984Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T14:57:09.7734753Z Task         : Get sources
2020-03-14T14:57:09.7735786Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T14:57:09.7736311Z Version      : 1.0.0
2020-03-14T14:57:09.7736747Z Author       : Microsoft
---
2020-03-14T14:57:11.8939362Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T14:57:11.9111259Z ##[command]git config gc.auto 0
2020-03-14T14:57:11.9138881Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T14:57:11.9169375Z ##[command]git config --get-all http.proxy
2020-03-14T14:57:11.9256500Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-14T15:58:18.2244089Z .................................................................................................... 1700/9771
2020-03-14T15:58:22.7512339Z .................................................................................................... 1800/9771
2020-03-14T15:58:34.3934562Z ...................................................................i................................ 1900/9771
2020-03-14T15:58:41.2872163Z .................................................................................................... 2000/9771
2020-03-14T15:58:55.4998420Z .........................................................iiiii...................................... 2100/9771
2020-03-14T15:59:06.2445608Z .................................................................................................... 2300/9771
2020-03-14T15:59:08.4049493Z .................................................................................................... 2400/9771
2020-03-14T15:59:11.5654220Z .................................................................................................... 2500/9771
2020-03-14T15:59:33.1973458Z .................................................................................................... 2600/9771
---
2020-03-14T16:02:07.9533825Z .............................i...............i...................................................... 5000/9771
2020-03-14T16:02:17.2383400Z .................................................................................................... 5100/9771
2020-03-14T16:02:22.9713234Z ........................................................................i........................... 5200/9771
2020-03-14T16:02:28.4278443Z .................................................................................................... 5300/9771
2020-03-14T16:02:37.9574481Z .....................................................ii.ii........i...i............................. 5400/9771
2020-03-14T16:02:46.4395863Z .................................................................................................... 5600/9771
2020-03-14T16:02:56.0938177Z .................................................................................................... 5700/9771
2020-03-14T16:03:02.4160314Z .............................................i...................................................... 5800/9771
2020-03-14T16:03:09.0998048Z .................................................................................................... 5900/9771
2020-03-14T16:03:09.0998048Z .................................................................................................... 5900/9771
2020-03-14T16:03:19.6578408Z .................................................................................................... 6000/9771
2020-03-14T16:03:25.5289939Z .......................................ii...i..ii...........i....................................... 6100/9771
2020-03-14T16:03:45.6145638Z .................................................................................................... 6300/9771
2020-03-14T16:03:52.5059186Z .................................................................................................... 6400/9771
2020-03-14T16:03:52.5059186Z .................................................................................................... 6400/9771
2020-03-14T16:04:01.7735406Z .....................................................................i..ii.......................... 6500/9771
2020-03-14T16:04:24.4365236Z .................................................................................................... 6700/9771
2020-03-14T16:04:32.7435463Z ...................................................................i................................ 6800/9771
2020-03-14T16:04:34.7101691Z .................................................................................................... 6900/9771
2020-03-14T16:04:36.8681023Z .................................................................................................... 7000/9771
---
2020-03-14T16:06:20.8712951Z .................................................................................................... 7800/9771
2020-03-14T16:06:27.0607944Z .................................................................................................... 7900/9771
2020-03-14T16:06:32.8302365Z ...................................................i................................................ 8000/9771
2020-03-14T16:06:43.4410195Z .................................................................................................... 8100/9771
2020-03-14T16:06:48.8608572Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-14T16:07:02.7246273Z .................................................................................................... 8400/9771
2020-03-14T16:07:13.9419922Z .................................................................................................... 8500/9771
2020-03-14T16:07:26.7605754Z .................................................................................................... 8600/9771
2020-03-14T16:07:32.6952276Z .................................................................................................... 8700/9771
---
2020-03-14T16:09:24.2821601Z -   --> $DIR/issue-62504.rs:18:25
2020-03-14T16:09:24.2821886Z + error[E0308]: mismatched types
2020-03-14T16:09:24.2822373Z +   --> $DIR/issue-62504.rs:18:21
2020-03-14T16:09:24.2822595Z 3    |
2020-03-14T16:09:24.2822847Z 4 LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T16:09:24.2824036Z +    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T16:09:24.2824366Z +    |
2020-03-14T16:09:24.2824366Z +    |
2020-03-14T16:09:24.2824745Z +    = note: expected array `[u32; _]`
2020-03-14T16:09:24.2825032Z +               found array `[u32; _]`
2020-03-14T16:09:24.2826201Z 7 error: aborting due to previous error
2020-03-14T16:09:24.2826439Z 8 
2020-03-14T16:09:24.2826556Z 
2020-03-14T16:09:24.2827171Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T16:09:24.2827171Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T16:09:24.2827473Z 9 
2020-03-14T16:09:24.2827589Z 
2020-03-14T16:09:24.2827699Z 
2020-03-14T16:09:24.2827931Z The actual stderr differed from the expected stderr.
2020-03-14T16:09:24.2828728Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/issue-62504.stderr
2020-03-14T16:09:24.2829455Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T16:09:24.2830181Z To only update this specific test, also pass `--test-args const-generics/issues/issue-62504.rs`
2020-03-14T16:09:24.2830698Z error: 1 errors occurred comparing output.
2020-03-14T16:09:24.2831258Z status: exit code: 1
2020-03-14T16:09:24.2831258Z status: exit code: 1
2020-03-14T16:09:24.2833586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/auxiliary"
2020-03-14T16:09:24.2835455Z ------------------------------------------
2020-03-14T16:09:24.2835663Z 
2020-03-14T16:09:24.2836090Z ------------------------------------------
2020-03-14T16:09:24.2836322Z stderr:
2020-03-14T16:09:24.2836322Z stderr:
2020-03-14T16:09:24.2836743Z ------------------------------------------
2020-03-14T16:09:24.2837026Z error[E0308]: mismatched types
2020-03-14T16:09:24.2837616Z   --> /checkout/src/test/ui/const-generics/issues/issue-62504.rs:18:21
2020-03-14T16:09:24.2837910Z    |
2020-03-14T16:09:24.2838154Z LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T16:09:24.2838584Z    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T16:09:24.2838924Z    |
2020-03-14T16:09:24.2839139Z    = note: expected array `[u32; _]`
2020-03-14T16:09:24.2839438Z               found array `[u32; _]`
2020-03-14T16:09:24.2839825Z error: aborting due to previous error
2020-03-14T16:09:24.2840011Z 
2020-03-14T16:09:24.2840518Z For more information about this error, try `rustc --explain E0308`.
2020-03-14T16:09:24.2840765Z 
2020-03-14T16:09:24.2840765Z 
2020-03-14T16:09:24.2841172Z ------------------------------------------
2020-03-14T16:09:24.2841367Z 
2020-03-14T16:09:24.2841495Z 
2020-03-14T16:09:24.2841958Z ---- [ui] ui/const-generics/issues/issue-67739.rs stdout ----
2020-03-14T16:09:24.2842197Z 
2020-03-14T16:09:24.2842488Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-14T16:09:24.2842851Z status: exit code: 101
2020-03-14T16:09:24.2845240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739/auxiliary"
2020-03-14T16:09:24.2847152Z ------------------------------------------
2020-03-14T16:09:24.2847461Z 
2020-03-14T16:09:24.2847898Z ------------------------------------------
2020-03-14T16:09:24.2848128Z stderr:
2020-03-14T16:09:24.2848128Z stderr:
2020-03-14T16:09:24.2848539Z ------------------------------------------
2020-03-14T16:09:24.2849523Z error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:35: could not fully normalize `fn() -> usize {std::mem::size_of::<<Self as Trait>::Associated>}`
2020-03-14T16:09:24.2850563Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-14T16:09:24.2851017Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T16:09:24.2851282Z 
2020-03-14T16:09:24.2851527Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T16:09:24.2851527Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T16:09:24.2851769Z 
2020-03-14T16:09:24.2852533Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-14T16:09:24.2853429Z note: rustc 1.43.0-nightly (83cb194a9 2020-03-14) running on x86_64-unknown-linux-gnu
2020-03-14T16:09:24.2853719Z 
2020-03-14T16:09:24.2853719Z 
2020-03-14T16:09:24.2854425Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-14T16:09:24.2856225Z error: aborting due to previous error
2020-03-14T16:09:24.2856412Z 
2020-03-14T16:09:24.2856520Z 
2020-03-14T16:09:24.2856996Z ------------------------------------------
---
2020-03-14T16:09:24.2882456Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T16:09:24.2882954Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T16:09:24.2895802Z 
2020-03-14T16:09:24.2895957Z 
2020-03-14T16:09:24.2899294Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T16:09:24.2901661Z 
2020-03-14T16:09:24.2901749Z 
2020-03-14T16:09:24.2977708Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T16:09:24.2978200Z Build completed unsuccessfully in 1:05:08
2020-03-14T16:09:24.2978200Z Build completed unsuccessfully in 1:05:08
2020-03-14T16:09:24.2980251Z == clock drift check ==
2020-03-14T16:09:24.3007952Z   local time: Sat Mar 14 16:09:24 UTC 2020
2020-03-14T16:09:24.6026204Z   network time: Sat, 14 Mar 2020 16:09:24 GMT
2020-03-14T16:09:24.6032352Z == end clock drift check ==
2020-03-14T16:09:25.1719411Z 
2020-03-14T16:09:25.1799766Z ##[error]Bash exited with code '1'.
2020-03-14T16:09:25.1816262Z ##[section]Finishing: Run build
2020-03-14T16:09:25.1864334Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T16:09:25.1868896Z Task         : Get sources
2020-03-14T16:09:25.1869213Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T16:09:25.1869486Z Version      : 1.0.0
2020-03-14T16:09:25.1869680Z Author       : Microsoft
2020-03-14T16:09:25.1869680Z Author       : Microsoft
2020-03-14T16:09:25.1870002Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T16:09:25.1870354Z ==============================================================================
2020-03-14T16:09:25.5631034Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T16:09:25.5673466Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T16:09:25.5772947Z Cleaning up task key
2020-03-14T16:09:25.5774008Z Start cleaning up orphan processes.
2020-03-14T16:09:25.5942441Z Terminate orphan process: pid (4079) (python)
2020-03-14T16:09:25.6177283Z ##[section]Finishing: Finalize Job
