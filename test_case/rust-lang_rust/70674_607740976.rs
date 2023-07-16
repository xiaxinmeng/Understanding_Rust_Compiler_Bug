plain
2020-04-02T08:45:01.3188962Z ========================== Starting Command Output ===========================
2020-04-02T08:45:01.3191398Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/04833375-2f7d-4467-a83e-65b9667e2c8b.sh
2020-04-02T08:45:01.3191697Z 
2020-04-02T08:45:01.3195874Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T08:45:01.3214541Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-02T08:45:01.3217277Z Task         : Get sources
2020-04-02T08:45:01.3217526Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T08:45:01.3217766Z Version      : 1.0.0
2020-04-02T08:45:01.3217942Z Author       : Microsoft
---
2020-04-02T08:45:02.1377946Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T08:45:02.1383281Z ##[command]git config gc.auto 0
2020-04-02T08:45:02.1387162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T08:45:02.1390475Z ##[command]git config --get-all http.proxy
2020-04-02T08:45:02.1396408Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70674/merge:refs/remotes/pull/70674/merge
---
2020-04-02T08:47:34.8649531Z Looks like docker image is the same as before, not uploading
2020-04-02T08:47:42.5066944Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T08:47:42.5289164Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T08:47:42.5318378Z == clock drift check ==
2020-04-02T08:47:42.5327724Z   local time: Thu Apr  2 08:47:42 UTC 2020
2020-04-02T08:47:42.8241922Z   network time: Thu, 02 Apr 2020 08:47:42 GMT
2020-04-02T08:47:42.8277053Z Starting sccache server...
2020-04-02T08:47:42.9118457Z configure: processing command line
2020-04-02T08:47:42.9118903Z configure: 
2020-04-02T08:47:42.9122169Z configure: rust.dist-src        := False
---
2020-04-02T08:52:22.1708637Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T08:52:23.5080745Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T08:52:24.8465423Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T08:52:24.9438019Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T08:52:33.8480544Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T08:52:35.4013008Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T08:52:39.4007908Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T08:52:43.0268473Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T08:52:51.9742330Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T09:13:23.3671568Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T09:13:24.9033395Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T09:13:26.6547193Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T09:13:28.8102839Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T09:13:38.0144781Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T09:13:40.9826971Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T09:13:45.7662203Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T09:13:50.6262783Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T09:14:00.0874902Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T09:37:54.7954172Z .................................................................................................... 1700/9868
2020-04-02T09:37:58.4978743Z .................................................................................................... 1800/9868
2020-04-02T09:38:06.5775125Z ...............................................................................................i.... 1900/9868
2020-04-02T09:38:13.6374707Z .................................................................................................... 2000/9868
2020-04-02T09:38:19.5541172Z .....................................................................................iiiii.......... 2100/9868
2020-04-02T09:38:39.0996303Z .................................................................................................... 2300/9868
2020-04-02T09:38:41.1440571Z .................................................................................................... 2400/9868
2020-04-02T09:38:43.2748275Z .................................................................................................... 2500/9868
2020-04-02T09:38:48.8442042Z .................................................................................................... 2600/9868
---
2020-04-02T09:41:28.4093568Z ...........................................................i...............i........................ 5000/9868
2020-04-02T09:41:35.1440479Z .................................................................................................... 5100/9868
2020-04-02T09:41:42.3202221Z .................................................................................................... 5200/9868
2020-04-02T09:41:46.8479176Z ....i............................................................................................... 5300/9868
2020-04-02T09:41:56.2327329Z ..........................................................................................ii.ii..... 5400/9868
2020-04-02T09:41:59.8964796Z ...i...i............................................................................................ 5500/9868
2020-04-02T09:42:07.8769180Z ...................................i................................................................ 5700/9868
2020-04-02T09:42:17.2543862Z .......................................................ii....................................i...... 5800/9868
2020-04-02T09:42:23.9348398Z .................................................................................................... 5900/9868
2020-04-02T09:42:28.2935543Z .................................................................................................... 6000/9868
2020-04-02T09:42:28.2935543Z .................................................................................................... 6000/9868
2020-04-02T09:42:36.7884151Z .......................................................................................ii...i..ii... 6100/9868
2020-04-02T09:42:55.7461015Z .................................................................................................... 6300/9868
2020-04-02T09:43:03.2637413Z .................................................................................................... 6400/9868
2020-04-02T09:43:05.7545022Z .................................................................................................... 6500/9868
2020-04-02T09:43:05.7545022Z .................................................................................................... 6500/9868
2020-04-02T09:43:17.7666048Z .................i..ii.............................................................................. 6600/9868
2020-04-02T09:43:36.0247299Z .................................................................................................... 6800/9868
2020-04-02T09:43:37.9065247Z .................i.................................................................................. 6900/9868
2020-04-02T09:43:39.8688454Z .................................................................................................... 7000/9868
2020-04-02T09:43:41.9138943Z ........................................................i........................................... 7100/9868
---
2020-04-02T09:45:11.8922937Z .................................................................................................... 7800/9868
2020-04-02T09:45:16.6030783Z .................................................................................................... 7900/9868
2020-04-02T09:45:21.7041153Z .................................................................................................... 8000/9868
2020-04-02T09:45:29.0959756Z ..................i................................................................................. 8100/9868
2020-04-02T09:45:37.1524807Z ...................................................................iiiiiiiiii.i..................... 8200/9868
2020-04-02T09:45:51.5654573Z ...........i......i................................................................................. 8400/9868
2020-04-02T09:45:55.9080065Z .................................................................................................... 8500/9868
2020-04-02T09:46:06.3682094Z .................................................................................................... 8600/9868
2020-04-02T09:46:17.3728375Z .................................................................................................... 8700/9868
---
2020-04-02T09:48:29.9489232Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-02T09:48:29.9670538Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-02T09:48:30.1636801Z 
2020-04-02T09:48:30.1638514Z running 183 tests
2020-04-02T09:48:32.5977164Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-04-02T09:48:34.8688314Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-04-02T09:48:34.8692138Z 
2020-04-02T09:48:34.8692574Z  finished in 4.902
2020-04-02T09:48:34.8700352Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-02T09:48:34.8866254Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-02T09:48:36.8089182Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-02T09:48:36.8266302Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-02T09:48:36.9664767Z 
2020-04-02T09:48:36.9666175Z running 9 tests
2020-04-02T09:48:36.9667296Z iiiiiiiii
2020-04-02T09:48:36.9668394Z 
2020-04-02T09:48:36.9672987Z  finished in 0.139
2020-04-02T09:48:36.9678694Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-02T09:48:36.9874694Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-02T09:48:36.9874694Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-02T09:48:37.1739455Z 
2020-04-02T09:48:37.1739953Z running 117 tests
2020-04-02T09:48:48.4219654Z F.FFFFFFFFFFFFF.F.....F.F.FFFF.FFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFF.F.FF...FF.FFFFFFFF.FFFF.FFFFFF..FFFF 100/117
2020-04-02T09:48:50.0404432Z FFFFFFFFFFFFFFFFF
2020-04-02T09:48:50.0445345Z 
2020-04-02T09:48:50.0447110Z ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
2020-04-02T09:48:50.0464678Z 
2020-04-02T09:48:50.0464678Z 
2020-04-02T09:48:50.0546485Z error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
2020-04-02T09:48:50.0562051Z status: exit code: 101
2020-04-02T09:48:50.0564922Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
2020-04-02T09:48:50.0566917Z ------------------------------------------
2020-04-02T09:48:50.0567103Z 
2020-04-02T09:48:50.0567449Z ------------------------------------------
2020-04-02T09:48:50.0567643Z stderr:
2020-04-02T09:48:50.0567643Z stderr:
2020-04-02T09:48:50.0568011Z ------------------------------------------
2020-04-02T09:48:50.0569261Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0570406Z 
2020-04-02T09:48:50.0570613Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0570798Z 
2020-04-02T09:48:50.0571019Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0571019Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0571209Z 
2020-04-02T09:48:50.0571915Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0572663Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0572902Z 
2020-04-02T09:48:50.0572902Z 
2020-04-02T09:48:50.0573851Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0574397Z 
2020-04-02T09:48:50.0575288Z ------------------------------------------
2020-04-02T09:48:50.0575457Z 
2020-04-02T09:48:50.0575568Z 
2020-04-02T09:48:50.0575568Z 
2020-04-02T09:48:50.0576089Z ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
2020-04-02T09:48:50.0576308Z 
2020-04-02T09:48:50.0576743Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0577055Z status: exit code: 101
2020-04-02T09:48:50.0579906Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
2020-04-02T09:48:50.0582047Z ------------------------------------------
2020-04-02T09:48:50.0582232Z 
2020-04-02T09:48:50.0582574Z ------------------------------------------
2020-04-02T09:48:50.0582765Z stderr:
2020-04-02T09:48:50.0582765Z stderr:
2020-04-02T09:48:50.0583130Z ------------------------------------------
2020-04-02T09:48:50.0592367Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0602989Z 
2020-04-02T09:48:50.0603216Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0603420Z 
2020-04-02T09:48:50.0603665Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0603665Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0603870Z 
2020-04-02T09:48:50.0604706Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0605681Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0605943Z 
2020-04-02T09:48:50.0605943Z 
2020-04-02T09:48:50.0606876Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0607436Z 
2020-04-02T09:48:50.0607786Z ------------------------------------------
2020-04-02T09:48:50.0607953Z 
2020-04-02T09:48:50.0608044Z 
2020-04-02T09:48:50.0608044Z 
2020-04-02T09:48:50.0608478Z ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
2020-04-02T09:48:50.0608821Z 
2020-04-02T09:48:50.0609188Z error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
2020-04-02T09:48:50.0609617Z status: exit code: 101
2020-04-02T09:48:50.0612105Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
2020-04-02T09:48:50.0613995Z ------------------------------------------
2020-04-02T09:48:50.0614180Z 
2020-04-02T09:48:50.0614520Z ------------------------------------------
2020-04-02T09:48:50.0614712Z stderr:
2020-04-02T09:48:50.0614712Z stderr:
2020-04-02T09:48:50.0615203Z ------------------------------------------
2020-04-02T09:48:50.0616441Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0617948Z 
2020-04-02T09:48:50.0618153Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0618338Z 
2020-04-02T09:48:50.0618561Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0618561Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0618749Z 
2020-04-02T09:48:50.0619342Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0620080Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0620319Z 
2020-04-02T09:48:50.0620319Z 
2020-04-02T09:48:50.0621179Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0621741Z 
2020-04-02T09:48:50.0622095Z ------------------------------------------
2020-04-02T09:48:50.0622260Z 
2020-04-02T09:48:50.0622353Z 
2020-04-02T09:48:50.0622353Z 
2020-04-02T09:48:50.0622776Z ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
2020-04-02T09:48:50.0622983Z 
2020-04-02T09:48:50.0623762Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0624076Z status: exit code: 101
2020-04-02T09:48:50.0626799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
2020-04-02T09:48:50.0628724Z ------------------------------------------
2020-04-02T09:48:50.0628902Z 
2020-04-02T09:48:50.0629288Z ------------------------------------------
2020-04-02T09:48:50.0629495Z stderr:
2020-04-02T09:48:50.0629495Z stderr:
2020-04-02T09:48:50.0629873Z ------------------------------------------
2020-04-02T09:48:50.0631215Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0632574Z 
2020-04-02T09:48:50.0632802Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0632999Z 
2020-04-02T09:48:50.0633225Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0633225Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0633446Z 
2020-04-02T09:48:50.0634065Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0634859Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0635117Z 
2020-04-02T09:48:50.0635117Z 
2020-04-02T09:48:50.0635994Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C panic=unwind
2020-04-02T09:48:50.0636610Z 
2020-04-02T09:48:50.0636976Z ------------------------------------------
2020-04-02T09:48:50.0637152Z 
2020-04-02T09:48:50.0637251Z 
2020-04-02T09:48:50.0637251Z 
2020-04-02T09:48:50.0637708Z ---- [incremental] incremental/change_crate_order/main.rs stdout ----
2020-04-02T09:48:50.0638015Z 
2020-04-02T09:48:50.0638443Z error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
2020-04-02T09:48:50.0639152Z status: exit code: 101
2020-04-02T09:48:50.0643089Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
2020-04-02T09:48:50.0645132Z ------------------------------------------
2020-04-02T09:48:50.0645311Z 
2020-04-02T09:48:50.0645699Z ------------------------------------------
2020-04-02T09:48:50.0645908Z stderr:
2020-04-02T09:48:50.0645908Z stderr:
2020-04-02T09:48:50.0646283Z ------------------------------------------
2020-04-02T09:48:50.0647626Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0648853Z 
2020-04-02T09:48:50.0649072Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0649273Z 
2020-04-02T09:48:50.0649502Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0649502Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0649723Z 
2020-04-02T09:48:50.0650343Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0651136Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0651394Z 
2020-04-02T09:48:50.0651394Z 
2020-04-02T09:48:50.0652197Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0652782Z 
2020-04-02T09:48:50.0653147Z ------------------------------------------
2020-04-02T09:48:50.0653324Z 
2020-04-02T09:48:50.0653424Z 
2020-04-02T09:48:50.0653424Z 
2020-04-02T09:48:50.0653893Z ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
2020-04-02T09:48:50.0654126Z 
2020-04-02T09:48:50.0654344Z error in revision `rpass2`: compilation failed!
2020-04-02T09:48:50.0654631Z status: exit code: 101
2020-04-02T09:48:50.0657317Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
2020-04-02T09:48:50.0659315Z ------------------------------------------
2020-04-02T09:48:50.0659482Z 
2020-04-02T09:48:50.0659960Z ------------------------------------------
2020-04-02T09:48:50.0660152Z stderr:
2020-04-02T09:48:50.0660152Z stderr:
2020-04-02T09:48:50.0660553Z ------------------------------------------
2020-04-02T09:48:50.0661805Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0662944Z 
2020-04-02T09:48:50.0663148Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0663504Z 
2020-04-02T09:48:50.0663705Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0663705Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0663902Z 
2020-04-02T09:48:50.0664472Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0665189Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0665421Z 
2020-04-02T09:48:50.0665421Z 
2020-04-02T09:48:50.0666098Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0666610Z 
2020-04-02T09:48:50.0666949Z ------------------------------------------
2020-04-02T09:48:50.0667111Z 
2020-04-02T09:48:50.0667204Z 
2020-04-02T09:48:50.0667204Z 
2020-04-02T09:48:50.0667765Z ---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----
2020-04-02T09:48:50.0667969Z 
2020-04-02T09:48:50.0668301Z error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_implementation_cross_crate/auxiliary/a.rs" failed to compile: 
2020-04-02T09:48:50.0668667Z status: exit code: 101
2020-04-02T09:48:50.0670827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
2020-04-02T09:48:50.0672498Z ------------------------------------------
2020-04-02T09:48:50.0672641Z 
2020-04-02T09:48:50.0672952Z ------------------------------------------
2020-04-02T09:48:50.0673123Z stderr:
2020-04-02T09:48:50.0673123Z stderr:
2020-04-02T09:48:50.0673425Z ------------------------------------------
2020-04-02T09:48:50.0674054Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(std[ce3e]::time[0]::Instant[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0674954Z 
2020-04-02T09:48:50.0675173Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0675359Z 
2020-04-02T09:48:50.0675562Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0675562Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0675748Z 
2020-04-02T09:48:50.0676317Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0677029Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0677285Z 
2020-04-02T09:48:50.0677285Z 
2020-04-02T09:48:50.0678026Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0678677Z 
2020-04-02T09:48:50.0679025Z ------------------------------------------
2020-04-02T09:48:50.0679188Z 
2020-04-02T09:48:50.0679280Z 
2020-04-02T09:48:50.0679280Z 
2020-04-02T09:48:50.0679717Z ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
2020-04-02T09:48:50.0679939Z 
2020-04-02T09:48:50.0680367Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0680659Z status: exit code: 101
2020-04-02T09:48:50.0683255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
2020-04-02T09:48:50.0685263Z ------------------------------------------
2020-04-02T09:48:50.0685442Z 
2020-04-02T09:48:50.0685821Z ------------------------------------------
2020-04-02T09:48:50.0686028Z stderr:
2020-04-02T09:48:50.0686028Z stderr:
2020-04-02T09:48:50.0686401Z ------------------------------------------
2020-04-02T09:48:50.0687161Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(core[94f6]::pin[0]::Pin[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0687959Z 
2020-04-02T09:48:50.0688192Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0688394Z 
2020-04-02T09:48:50.0688617Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0688617Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0688818Z 
2020-04-02T09:48:50.0689432Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0690435Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0690694Z 
2020-04-02T09:48:50.0690694Z 
2020-04-02T09:48:50.0691575Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0692107Z 
2020-04-02T09:48:50.0692460Z ------------------------------------------
2020-04-02T09:48:50.0692622Z 
2020-04-02T09:48:50.0692713Z 
2020-04-02T09:48:50.0692713Z 
2020-04-02T09:48:50.0693140Z ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
2020-04-02T09:48:50.0693385Z 
2020-04-02T09:48:50.0693754Z error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
2020-04-02T09:48:50.0694346Z status: exit code: 101
2020-04-02T09:48:50.0697198Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
2020-04-02T09:48:50.0699205Z ------------------------------------------
2020-04-02T09:48:50.0699370Z 
2020-04-02T09:48:50.0699724Z ------------------------------------------
2020-04-02T09:48:50.0699914Z stderr:
2020-04-02T09:48:50.0699914Z stderr:
2020-04-02T09:48:50.0700261Z ------------------------------------------
2020-04-02T09:48:50.0700950Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(core[94f6]::pin[0]::Pin[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0701701Z 
2020-04-02T09:48:50.0701903Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0702104Z 
2020-04-02T09:48:50.0702309Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0702309Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0702498Z 
2020-04-02T09:48:50.0703081Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0704036Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0704278Z 
2020-04-02T09:48:50.0704278Z 
2020-04-02T09:48:50.0705038Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0705566Z 
2020-04-02T09:48:50.0705918Z ------------------------------------------
2020-04-02T09:48:50.0706079Z 
2020-04-02T09:48:50.0706171Z 
2020-04-02T09:48:50.0706171Z 
2020-04-02T09:48:50.0706613Z ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
2020-04-02T09:48:50.0706867Z 
2020-04-02T09:48:50.0707507Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0707801Z status: exit code: 101
2020-04-02T09:48:50.0710684Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
2020-04-02T09:48:50.0712622Z ------------------------------------------
2020-04-02T09:48:50.0712788Z 
2020-04-02T09:48:50.0713131Z ------------------------------------------
2020-04-02T09:48:50.0713340Z stderr:
2020-04-02T09:48:50.0713340Z stderr:
2020-04-02T09:48:50.0713692Z ------------------------------------------
2020-04-02T09:48:50.0714389Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(alloc[db9d]::sync[0]::Weak[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0715143Z 
2020-04-02T09:48:50.0715346Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0715549Z 
2020-04-02T09:48:50.0715756Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0715756Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0715944Z 
2020-04-02T09:48:50.0716518Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0717236Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0717477Z 
2020-04-02T09:48:50.0717477Z 
2020-04-02T09:48:50.0718383Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0718962Z 
2020-04-02T09:48:50.0719328Z ------------------------------------------
2020-04-02T09:48:50.0719493Z 
2020-04-02T09:48:50.0719585Z 
2020-04-02T09:48:50.0719585Z 
2020-04-02T09:48:50.0720043Z ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
2020-04-02T09:48:50.0720286Z 
2020-04-02T09:48:50.0720732Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0721024Z status: exit code: 101
2020-04-02T09:48:50.0723543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
2020-04-02T09:48:50.0725503Z ------------------------------------------
2020-04-02T09:48:50.0725666Z 
2020-04-02T09:48:50.0726002Z ------------------------------------------
2020-04-02T09:48:50.0726212Z stderr:
2020-04-02T09:48:50.0726212Z stderr:
2020-04-02T09:48:50.0726560Z ------------------------------------------
2020-04-02T09:48:50.0727250Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(alloc[db9d]::sync[0]::Weak[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0728020Z 
2020-04-02T09:48:50.0728222Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0728421Z 
2020-04-02T09:48:50.0728626Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0728626Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0728813Z 
2020-04-02T09:48:50.0729383Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0730153Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0730360Z 
2020-04-02T09:48:50.0730360Z 
2020-04-02T09:48:50.0731021Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0731484Z 
2020-04-02T09:48:50.0731968Z ------------------------------------------
2020-04-02T09:48:50.0732151Z 
2020-04-02T09:48:50.0732243Z 
2020-04-02T09:48:50.0732243Z 
2020-04-02T09:48:50.0732699Z ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
2020-04-02T09:48:50.0732938Z 
2020-04-02T09:48:50.0733336Z error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
2020-04-02T09:48:50.0733758Z status: exit code: 101
2020-04-02T09:48:50.0736417Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
2020-04-02T09:48:50.0738160Z ------------------------------------------
2020-04-02T09:48:50.0738303Z 
2020-04-02T09:48:50.0738596Z ------------------------------------------
2020-04-02T09:48:50.0738779Z stderr:
2020-04-02T09:48:50.0738779Z stderr:
2020-04-02T09:48:50.0739079Z ------------------------------------------
2020-04-02T09:48:50.0739682Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(alloc[db9d]::sync[0]::Weak[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0740344Z 
2020-04-02T09:48:50.0740520Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0740685Z 
2020-04-02T09:48:50.0740881Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0740881Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0741045Z 
2020-04-02T09:48:50.0741541Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0742162Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0742371Z 
2020-04-02T09:48:50.0742371Z 
2020-04-02T09:48:50.0743028Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2020-04-02T09:48:50.0743636Z 
2020-04-02T09:48:50.0743935Z ------------------------------------------
2020-04-02T09:48:50.0744087Z 
2020-04-02T09:48:50.0744163Z 
2020-04-02T09:48:50.0744163Z 
2020-04-02T09:48:50.0744548Z ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
2020-04-02T09:48:50.0744751Z 
2020-04-02T09:48:50.0745138Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-02T09:48:50.0745396Z status: exit code: 101
2020-04-02T09:48:50.0747789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
2020-04-02T09:48:50.0749715Z ------------------------------------------
2020-04-02T09:48:50.0749880Z 
2020-04-02T09:48:50.0750216Z ------------------------------------------
2020-04-02T09:48:50.0750426Z stderr:
2020-04-02T09:48:50.0750426Z stderr:
2020-04-02T09:48:50.0750950Z ------------------------------------------
2020-04-02T09:48:50.0751705Z thread 'rustc' panicked at 'found unstable fingerprints for adt_def(core[94f6]::cell[0]::RefMut[0])', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0752518Z 
2020-04-02T09:48:50.0752737Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0752938Z 
2020-04-02T09:48:50.0753175Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0753175Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0753377Z 
2020-04-02T09:48:50.0753972Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0754913Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0755174Z 
2020-04-02T09:48:50.0755174Z 
2020-04-02T09:48:50.0755994Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0756558Z 
2020-04-02T09:48:50.0756923Z ------------------------------------------
2020-04-02T09:48:50.0757115Z 
2020-04-02T09:48:50.0757215Z 
2020-04-02T09:48:50.0757215Z 
2020-04-02T09:48:50.0757662Z ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
2020-04-02T09:48:50.0757895Z 
2020-04-02T09:48:50.0758127Z error in revision `rpass2`: compilation failed!
2020-04-02T09:48:50.0758392Z status: exit code: 101
2020-04-02T09:48:50.0760961Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
2020-04-02T09:48:50.0762852Z ------------------------------------------
2020-04-02T09:48:50.0762995Z 
2020-04-02T09:48:50.0763288Z ------------------------------------------
2020-04-02T09:48:50.0763455Z stderr:
2020-04-02T09:48:50.0763455Z stderr:
2020-04-02T09:48:50.0763770Z ------------------------------------------
2020-04-02T09:48:50.0764854Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0765843Z 
2020-04-02T09:48:50.0766022Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0766299Z 
2020-04-02T09:48:50.0766467Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0766467Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0766606Z 
2020-04-02T09:48:50.0767023Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0767572Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0767752Z 
2020-04-02T09:48:50.0767752Z 
2020-04-02T09:48:50.0768528Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0768991Z 
2020-04-02T09:48:50.0769286Z ------------------------------------------
2020-04-02T09:48:50.0769443Z 
2020-04-02T09:48:50.0769524Z 
2020-04-02T09:48:50.0769524Z 
2020-04-02T09:48:50.0769861Z ---- [incremental] incremental/commandline-args.rs stdout ----
2020-04-02T09:48:50.0770034Z 
2020-04-02T09:48:50.0770221Z error in revision `rpass3`: compilation failed!
2020-04-02T09:48:50.0770436Z status: exit code: 101
2020-04-02T09:48:50.0772438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
2020-04-02T09:48:50.0774035Z ------------------------------------------
2020-04-02T09:48:50.0774194Z 
2020-04-02T09:48:50.0774487Z ------------------------------------------
2020-04-02T09:48:50.0774653Z stderr:
2020-04-02T09:48:50.0774653Z stderr:
2020-04-02T09:48:50.0774968Z ------------------------------------------
2020-04-02T09:48:50.0776031Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0777023Z 
2020-04-02T09:48:50.0777198Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0777359Z 
2020-04-02T09:48:50.0777551Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0777551Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0777715Z 
2020-04-02T09:48:50.0778199Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0778838Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0779045Z 
2020-04-02T09:48:50.0779045Z 
2020-04-02T09:48:50.0779690Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
2020-04-02T09:48:50.0780167Z 
2020-04-02T09:48:50.0780458Z ------------------------------------------
2020-04-02T09:48:50.0780599Z 
2020-04-02T09:48:50.0780694Z 
2020-04-02T09:48:50.0780694Z 
2020-04-02T09:48:50.0781038Z ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
2020-04-02T09:48:50.0781213Z 
2020-04-02T09:48:50.0781388Z error in revision `rpass2`: compilation failed!
2020-04-02T09:48:50.0781618Z status: exit code: 101
2020-04-02T09:48:50.0783756Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
2020-04-02T09:48:50.0785336Z ------------------------------------------
2020-04-02T09:48:50.0785495Z 
2020-04-02T09:48:50.0785790Z ------------------------------------------
2020-04-02T09:48:50.0785956Z stderr:
2020-04-02T09:48:50.0785956Z stderr:
2020-04-02T09:48:50.0786257Z ------------------------------------------
2020-04-02T09:48:50.0787343Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0788593Z 
2020-04-02T09:48:50.0788795Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0789041Z 
2020-04-02T09:48:50.0789307Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0789307Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0789496Z 
2020-04-02T09:48:50.0790064Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0790995Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0791477Z 
2020-04-02T09:48:50.0791477Z 
2020-04-02T09:48:50.0792297Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0792889Z 
2020-04-02T09:48:50.0793252Z ------------------------------------------
2020-04-02T09:48:50.0793429Z 
2020-04-02T09:48:50.0793529Z 
2020-04-02T09:48:50.0793529Z 
2020-04-02T09:48:50.0793951Z ---- [incremental] incremental/dirty_clean.rs stdout ----
2020-04-02T09:48:50.0794155Z 
2020-04-02T09:48:50.0794948Z error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
2020-04-02T09:48:50.0795485Z status: exit code: 101
2020-04-02T09:48:50.0797849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
2020-04-02T09:48:50.0799732Z ------------------------------------------
2020-04-02T09:48:50.0799916Z 
2020-04-02T09:48:50.0800306Z ------------------------------------------
2020-04-02T09:48:50.0800515Z stderr:
2020-04-02T09:48:50.0800515Z stderr:
2020-04-02T09:48:50.0800887Z ------------------------------------------
2020-04-02T09:48:50.0802229Z thread 'rustc' panicked at 'missing specialization: `<rustc_middle::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', /checkout/src/libstd/macros.rs:16:9
2020-04-02T09:48:50.0803453Z 
2020-04-02T09:48:50.0803672Z error: internal compiler error: unexpected panic
2020-04-02T09:48:50.0803872Z 
2020-04-02T09:48:50.0804094Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0804094Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T09:48:50.0804313Z 
2020-04-02T09:48:50.0804915Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T09:48:50.0805718Z note: rustc 1.44.0-nightly (0c64dde1f 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T09:48:50.0805978Z 
2020-04-02T09:48:50.0805978Z 
2020-04-02T09:48:50.0806781Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-02T09:48:50.0807397Z 
2020-04-02T09:48:50.0807692Z ------------------------------------------
2020-04-02T09:48:50.0807835Z 
2020-04-02T09:48:50.0807915Z 
2020-04-02T09:48:50.0807915Z 
2020-04-02T09:48:50.0808289Z ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
2020-04-02T09:48:50.0808473Z 
2020-04-02T09:48:50.0808649Z error in revision `rpass2`: compilation failed!
2020-04-02T09:48:50.0808861Z status: exit code: 101
2020-04-02T09:48:50.0810866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
2020-04-02T09:48:50.0812468Z ------------------------------------------
2020-04-02T09:48:50.0812612Z 
2020-04-02T09:48:50.0812920Z ------------------------------------------
---
2020-04-02T09:48:50.1935979Z 
2020-04-02T09:48:50.1936482Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-02T09:48:50.1936749Z 
2020-04-02T09:48:50.1936848Z 
2020-04-02T09:48:50.1940878Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-02T09:48:50.1943776Z 
2020-04-02T09:48:50.1943874Z 
2020-04-02T09:48:50.1944360Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-02T09:48:50.1944717Z Build completed unsuccessfully in 0:59:44
2020-04-02T09:48:50.1944717Z Build completed unsuccessfully in 0:59:44
2020-04-02T09:48:50.1945066Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-02T09:48:50.1945497Z == clock drift check ==
2020-04-02T09:48:50.1945703Z   local time: Thu Apr  2 09:48:50 UTC 2020
2020-04-02T09:48:50.1945971Z   network time: Thu, 02 Apr 2020 09:48:50 GMT
2020-04-02T09:48:53.0360543Z 
2020-04-02T09:48:53.0360543Z 
2020-04-02T09:48:53.0392187Z ##[error]Bash exited with code '1'.
2020-04-02T09:48:53.0403220Z ##[section]Finishing: Run build
2020-04-02T09:48:53.0449549Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-02T09:48:53.0455699Z Task         : Get sources
2020-04-02T09:48:53.0456058Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T09:48:53.0456385Z Version      : 1.0.0
2020-04-02T09:48:53.0456629Z Author       : Microsoft
2020-04-02T09:48:53.0456629Z Author       : Microsoft
2020-04-02T09:48:53.0456998Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T09:48:53.0457419Z ==============================================================================
2020-04-02T09:48:53.3340135Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T09:48:53.3381973Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-02T09:48:53.3462687Z Cleaning up task key
2020-04-02T09:48:53.3464181Z Start cleaning up orphan processes.
2020-04-02T09:48:53.3640576Z Terminate orphan process: pid (3532) (python)
2020-04-02T09:48:53.3807252Z ##[section]Finishing: Finalize Job
