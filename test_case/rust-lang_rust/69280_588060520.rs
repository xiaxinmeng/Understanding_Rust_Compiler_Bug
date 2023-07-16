plain
2020-02-19T05:34:53.4400327Z ========================== Starting Command Output ===========================
2020-02-19T05:34:53.4401858Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a7891c47-61b2-4dcd-9278-1721be26313d.sh
2020-02-19T05:34:53.4401891Z 
2020-02-19T05:34:53.4404614Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T05:34:53.4410631Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69280/merge to s
2020-02-19T05:34:53.4412106Z Task         : Get sources
2020-02-19T05:34:53.4412140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T05:34:53.4412218Z Version      : 1.0.0
2020-02-19T05:34:53.4412251Z Author       : Microsoft
---
2020-02-19T05:34:54.4436592Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T05:34:54.4447859Z ##[command]git config gc.auto 0
2020-02-19T05:34:54.4451705Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T05:34:54.4454801Z ##[command]git config --get-all http.proxy
2020-02-19T05:34:54.4464371Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69280/merge:refs/remotes/pull/69280/merge
---
2020-02-19T06:28:37.0476163Z .................................................................................................... 1700/9675
2020-02-19T06:28:41.3858396Z .................................................................................................... 1800/9675
2020-02-19T06:28:51.9646493Z ...................................i................................................................ 1900/9675
2020-02-19T06:28:58.8323586Z .................................................................................................... 2000/9675
2020-02-19T06:29:11.4241421Z .........................iiiii...................................................................... 2100/9675
2020-02-19T06:29:20.0926706Z .................................................................................................... 2300/9675
2020-02-19T06:29:22.2309214Z .................................................................................................... 2400/9675
2020-02-19T06:29:26.3173044Z .................................................................................................... 2500/9675
2020-02-19T06:29:44.9547546Z .................................................................................................... 2600/9675
---
2020-02-19T06:32:10.5857282Z i................................................................................................... 5000/9675
2020-02-19T06:32:18.4538715Z .................................................................................................... 5100/9675
2020-02-19T06:32:22.6567192Z ...........................i........................................................................ 5200/9675
2020-02-19T06:32:31.8245362Z .................................................................................................... 5300/9675
2020-02-19T06:32:36.5966098Z ...iiii........i...i................................................................................ 5400/9675
2020-02-19T06:32:45.1174632Z .................................................................................................... 5600/9675
2020-02-19T06:32:54.4409875Z ............................................................................................i....... 5700/9675
2020-02-19T06:33:01.7405067Z .................................................................................................... 5800/9675
2020-02-19T06:33:06.3247106Z ..........................................................................................i......... 5900/9675
2020-02-19T06:33:06.3247106Z ..........................................................................................i......... 5900/9675
2020-02-19T06:33:14.9661290Z ...................................................................................ii...i..ii....... 6000/9675
2020-02-19T06:33:35.3168323Z .................................................................................................... 6200/9675
2020-02-19T06:33:41.0094020Z .................................................................................................... 6300/9675
2020-02-19T06:33:44.5052913Z .................................................................................................... 6400/9675
2020-02-19T06:33:44.5052913Z .................................................................................................... 6400/9675
2020-02-19T06:33:56.1661766Z ...........i..ii.................................................................................... 6500/9675
2020-02-19T06:34:14.0700167Z .................................................................................................... 6700/9675
2020-02-19T06:34:16.0380476Z ...i................................................................................................ 6800/9675
2020-02-19T06:34:18.1132169Z .................................................................................................... 6900/9675
2020-02-19T06:34:20.2385444Z .........................i.......................................................................... 7000/9675
---
2020-02-19T06:35:50.4224243Z .................................................................................................... 7700/9675
2020-02-19T06:35:55.1287745Z .................................................................................................... 7800/9675
2020-02-19T06:36:00.9669631Z .................................................................................................... 7900/9675
2020-02-19T06:36:10.4292628Z .................................................................................................... 8000/9675
2020-02-19T06:36:15.8378245Z ...........iiiiiii.i................................................................................ 8100/9675
2020-02-19T06:36:24.6494603Z ..........................................F.F.F....i.....Fi......................................... 8200/9675
2020-02-19T06:36:37.1213173Z .................................................................................................... 8400/9675
2020-02-19T06:36:48.8857860Z .................................................................................................... 8500/9675
2020-02-19T06:36:55.0621750Z .................................................................................................... 8600/9675
2020-02-19T06:37:03.4391531Z .................................................................................................... 8700/9675
---
2020-02-19T06:38:37.2690579Z ---- [ui] ui/issues/issue-38074.rs stdout ----
2020-02-19T06:38:37.2690808Z 
2020-02-19T06:38:37.2691212Z error: test compilation failed although it shouldn't!
2020-02-19T06:38:37.2691420Z status: exit code: 101
2020-02-19T06:38:37.2692615Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38074.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38074/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38074/auxiliary"
2020-02-19T06:38:37.2693269Z ------------------------------------------
2020-02-19T06:38:37.2693449Z 
2020-02-19T06:38:37.2693835Z ------------------------------------------
2020-02-19T06:38:37.2694029Z stderr:
2020-02-19T06:38:37.2694029Z stderr:
2020-02-19T06:38:37.2694384Z ------------------------------------------
2020-02-19T06:38:37.2694609Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:622: shuffle indices must be constant
2020-02-19T06:38:37.2695224Z    |
2020-02-19T06:38:37.2695224Z    |
2020-02-19T06:38:37.2695624Z LL |     let r: u64x2 = unsafe { simd_shuffle2(a, a, [0-0, 0-0]) };
2020-02-19T06:38:37.2696015Z 
2020-02-19T06:38:37.2696404Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-02-19T06:38:37.2696609Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2696785Z 
2020-02-19T06:38:37.2696785Z 
2020-02-19T06:38:37.2696964Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-19T06:38:37.2697120Z 
2020-02-19T06:38:37.2697683Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-19T06:38:37.2697869Z 
2020-02-19T06:38:37.2698278Z note: rustc 1.43.0-nightly (616e4f6ff 2020-02-19) running on x86_64-unknown-linux-gnu
2020-02-19T06:38:37.2698489Z 
2020-02-19T06:38:37.2698955Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-19T06:38:37.2699351Z error: aborting due to previous error
2020-02-19T06:38:37.2699507Z 
2020-02-19T06:38:37.2699660Z 
2020-02-19T06:38:37.2700017Z ------------------------------------------
2020-02-19T06:38:37.2700017Z ------------------------------------------
2020-02-19T06:38:37.2700537Z 
2020-02-19T06:38:37.2701104Z 
2020-02-19T06:38:37.2701582Z ---- [ui] ui/simd-intrinsic/simd-intrinsic-generic-elements.rs stdout ----
2020-02-19T06:38:37.2702415Z 
2020-02-19T06:38:37.2702588Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-19T06:38:37.2702739Z status: exit code: 101
2020-02-19T06:38:37.2703724Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-elements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-elements" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-elements/auxiliary"
2020-02-19T06:38:37.2704260Z ------------------------------------------
2020-02-19T06:38:37.2704401Z 
2020-02-19T06:38:37.2704703Z ------------------------------------------
2020-02-19T06:38:37.2704838Z stderr:
2020-02-19T06:38:37.2704838Z stderr:
2020-02-19T06:38:37.2705154Z ------------------------------------------
2020-02-19T06:38:37.2705541Z error[E0511]: invalid monomorphization of `simd_insert` intrinsic: expected SIMD input type, found non-SIMD `i32`
2020-02-19T06:38:37.2706183Z    |
2020-02-19T06:38:37.2706183Z    |
2020-02-19T06:38:37.2706307Z LL |         simd_insert(0, 0, 0);
2020-02-19T06:38:37.2706555Z 
2020-02-19T06:38:37.2706555Z 
2020-02-19T06:38:37.2706747Z error[E0511]: invalid monomorphization of `simd_insert` intrinsic: expected inserted type `i32` (element of input `i32x4`), found `f64`
2020-02-19T06:38:37.2707273Z    |
2020-02-19T06:38:37.2707273Z    |
2020-02-19T06:38:37.2707395Z LL |         simd_insert(x, 0, 1.0);
2020-02-19T06:38:37.2707642Z 
2020-02-19T06:38:37.2707642Z 
2020-02-19T06:38:37.2707776Z error[E0511]: invalid monomorphization of `simd_extract` intrinsic: expected return type `i32` (element of input `i32x4`), found `f32`
2020-02-19T06:38:37.2708276Z    |
2020-02-19T06:38:37.2708276Z    |
2020-02-19T06:38:37.2708413Z LL |         simd_extract::<_, f32>(x, 0);
2020-02-19T06:38:37.2708657Z 
2020-02-19T06:38:37.2708657Z 
2020-02-19T06:38:37.2708895Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:622: shuffle indices must be constant
2020-02-19T06:38:37.2709332Z    |
2020-02-19T06:38:37.2709332Z    |
2020-02-19T06:38:37.2709457Z LL |         simd_shuffle2::<i32, i32>(0, 0, [0; 2]);
2020-02-19T06:38:37.2709663Z 
2020-02-19T06:38:37.2709959Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-02-19T06:38:37.2710279Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2711707Z 
2020-02-19T06:38:37.2711707Z 
2020-02-19T06:38:37.2711887Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-19T06:38:37.2712107Z 
2020-02-19T06:38:37.2712656Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-19T06:38:37.2713217Z 
2020-02-19T06:38:37.2713534Z note: rustc 1.43.0-nightly (616e4f6ff 2020-02-19) running on x86_64-unknown-linux-gnu
2020-02-19T06:38:37.2713642Z 
2020-02-19T06:38:37.2713957Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-19T06:38:37.2714149Z error: aborting due to 4 previous errors
2020-02-19T06:38:37.2714228Z 
2020-02-19T06:38:37.2714667Z For more information about this error, try `rustc --explain E0511`.
2020-02-19T06:38:37.2714774Z 
2020-02-19T06:38:37.2714774Z 
2020-02-19T06:38:37.2715036Z ------------------------------------------
2020-02-19T06:38:37.2715155Z 
2020-02-19T06:38:37.2715246Z 
2020-02-19T06:38:37.2715538Z ---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs stdout ----
2020-02-19T06:38:37.2715643Z 
2020-02-19T06:38:37.2715931Z error: test compilation failed although it shouldn't!
2020-02-19T06:38:37.2716051Z status: exit code: 101
2020-02-19T06:38:37.2716869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/auxiliary"
2020-02-19T06:38:37.2717311Z ------------------------------------------
2020-02-19T06:38:37.2717418Z 
2020-02-19T06:38:37.2717677Z ------------------------------------------
2020-02-19T06:38:37.2717808Z stderr:
2020-02-19T06:38:37.2717808Z stderr:
2020-02-19T06:38:37.2718064Z ------------------------------------------
2020-02-19T06:38:37.2718326Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:622: shuffle indices must be constant
2020-02-19T06:38:37.2718831Z    |
2020-02-19T06:38:37.2718831Z    |
2020-02-19T06:38:37.2718959Z LL |     simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 3])
2020-02-19T06:38:37.2719166Z 
2020-02-19T06:38:37.2719473Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-02-19T06:38:37.2719604Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2719699Z 
2020-02-19T06:38:37.2719699Z 
2020-02-19T06:38:37.2719824Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-19T06:38:37.2719917Z 
2020-02-19T06:38:37.2720266Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-19T06:38:37.2720393Z 
2020-02-19T06:38:37.2720704Z note: rustc 1.43.0-nightly (616e4f6ff 2020-02-19) running on x86_64-unknown-linux-gnu
2020-02-19T06:38:37.2720811Z 
2020-02-19T06:38:37.2721207Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z mir-opt-level=3 -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-19T06:38:37.2721429Z error: aborting due to previous error
2020-02-19T06:38:37.2721536Z 
2020-02-19T06:38:37.2721623Z 
2020-02-19T06:38:37.2721880Z ------------------------------------------
2020-02-19T06:38:37.2721880Z ------------------------------------------
2020-02-19T06:38:37.2721997Z 
2020-02-19T06:38:37.2722085Z 
2020-02-19T06:38:37.2722368Z ---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs stdout ----
2020-02-19T06:38:37.2722476Z 
2020-02-19T06:38:37.2722756Z error: test compilation failed although it shouldn't!
2020-02-19T06:38:37.2722875Z status: exit code: 101
2020-02-19T06:38:37.2723809Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/auxiliary"
2020-02-19T06:38:37.2725784Z ------------------------------------------
2020-02-19T06:38:37.2725994Z 
2020-02-19T06:38:37.2726287Z ------------------------------------------
2020-02-19T06:38:37.2726425Z stderr:
2020-02-19T06:38:37.2726425Z stderr:
2020-02-19T06:38:37.2726685Z ------------------------------------------
2020-02-19T06:38:37.2727665Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:622: shuffle indices must be constant
2020-02-19T06:38:37.2734135Z    |
2020-02-19T06:38:37.2734135Z    |
2020-02-19T06:38:37.2734436Z LL |         let p_res: Simd2 = simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 1]);
2020-02-19T06:38:37.2734705Z 
2020-02-19T06:38:37.2735286Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-02-19T06:38:37.2735444Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2735589Z 
2020-02-19T06:38:37.2735589Z 
2020-02-19T06:38:37.2735977Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-19T06:38:37.2736114Z 
2020-02-19T06:38:37.2736627Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-19T06:38:37.2736768Z 
2020-02-19T06:38:37.2737154Z note: rustc 1.43.0-nightly (616e4f6ff 2020-02-19) running on x86_64-unknown-linux-gnu
2020-02-19T06:38:37.2737451Z 
2020-02-19T06:38:37.2737981Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z mir-opt-level=3 -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-19T06:38:37.2738306Z error: aborting due to previous error
2020-02-19T06:38:37.2738420Z 
2020-02-19T06:38:37.2738529Z 
2020-02-19T06:38:37.2738880Z ------------------------------------------
2020-02-19T06:38:37.2738880Z ------------------------------------------
2020-02-19T06:38:37.2739011Z 
2020-02-19T06:38:37.2739121Z 
2020-02-19T06:38:37.2739482Z ---- [ui] ui/simd/simd-intrinsic-generic-elements.rs stdout ----
2020-02-19T06:38:37.2739614Z 
2020-02-19T06:38:37.2739947Z error: test compilation failed although it shouldn't!
2020-02-19T06:38:37.2740099Z status: exit code: 101
2020-02-19T06:38:37.2741027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/simd-intrinsic-generic-elements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-intrinsic-generic-elements/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-intrinsic-generic-elements/auxiliary"
2020-02-19T06:38:37.2741584Z ------------------------------------------
2020-02-19T06:38:37.2741713Z 
2020-02-19T06:38:37.2742028Z ------------------------------------------
2020-02-19T06:38:37.2742192Z stderr:
2020-02-19T06:38:37.2742192Z stderr:
2020-02-19T06:38:37.2742514Z ------------------------------------------
2020-02-19T06:38:37.2742693Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:622: shuffle indices must be constant
2020-02-19T06:38:37.2743228Z    |
2020-02-19T06:38:37.2743228Z    |
2020-02-19T06:38:37.2743368Z LL |         all_eq!(simd_shuffle2(x2, y2, [3, 0]), i32x2(121, 20));
2020-02-19T06:38:37.2743637Z 
2020-02-19T06:38:37.2744015Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-02-19T06:38:37.2744193Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2744307Z 
2020-02-19T06:38:37.2744307Z 
2020-02-19T06:38:37.2744440Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-19T06:38:37.2744571Z 
2020-02-19T06:38:37.2744994Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-19T06:38:37.2745128Z 
2020-02-19T06:38:37.2745507Z note: rustc 1.43.0-nightly (616e4f6ff 2020-02-19) running on x86_64-unknown-linux-gnu
2020-02-19T06:38:37.2745657Z 
2020-02-19T06:38:37.2746092Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-19T06:38:37.2746374Z error: aborting due to previous error
2020-02-19T06:38:37.2746489Z 
2020-02-19T06:38:37.2746598Z 
2020-02-19T06:38:37.2746946Z ------------------------------------------
---
2020-02-19T06:38:37.2754261Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-19T06:38:37.2754637Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:38:37.2754810Z 
2020-02-19T06:38:37.2755062Z 
2020-02-19T06:38:37.2756874Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-19T06:38:37.2757582Z 
2020-02-19T06:38:37.2757749Z 
2020-02-19T06:38:37.2757938Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-19T06:38:37.2758148Z Build completed unsuccessfully in 0:56:48
2020-02-19T06:38:37.2758148Z Build completed unsuccessfully in 0:56:48
2020-02-19T06:38:37.2800541Z == clock drift check ==
2020-02-19T06:38:37.2818496Z   local time: Wed Feb 19 06:38:37 UTC 2020
2020-02-19T06:38:37.8156638Z   network time: Wed, 19 Feb 2020 06:38:37 GMT
2020-02-19T06:38:37.8156820Z == end clock drift check ==
2020-02-19T06:38:38.2732077Z 
2020-02-19T06:38:38.2825662Z ##[error]Bash exited with code '1'.
2020-02-19T06:38:38.2863374Z ##[section]Finishing: Run build
2020-02-19T06:38:38.2886751Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69280/merge to s
2020-02-19T06:38:38.2888609Z Task         : Get sources
2020-02-19T06:38:38.2888678Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T06:38:38.2888728Z Version      : 1.0.0
2020-02-19T06:38:38.2888773Z Author       : Microsoft
2020-02-19T06:38:38.2888773Z Author       : Microsoft
2020-02-19T06:38:38.2889308Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-19T06:38:38.2889364Z ==============================================================================
2020-02-19T06:38:38.6924068Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-19T06:38:38.6977216Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69280/merge to s
2020-02-19T06:38:38.7080557Z Cleaning up task key
2020-02-19T06:38:38.7081329Z Start cleaning up orphan processes.
2020-02-19T06:38:38.7176465Z Terminate orphan process: pid (7031) (python)
2020-02-19T06:38:38.7465070Z ##[section]Finishing: Finalize Job
