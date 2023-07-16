plain
2020-02-09T16:49:27.3102444Z ========================== Starting Command Output ===========================
2020-02-09T16:49:27.3104353Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1212cf29-0961-421d-b376-3f232a79cfd2.sh
2020-02-09T16:49:27.3104395Z 
2020-02-09T16:49:27.3108280Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T16:49:27.3114628Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68944/merge to s
2020-02-09T16:49:27.3116420Z Task         : Get sources
2020-02-09T16:49:27.3116457Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T16:49:27.3116539Z Version      : 1.0.0
2020-02-09T16:49:27.3116577Z Author       : Microsoft
---
2020-02-09T16:49:28.1817031Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T16:49:28.1921452Z ##[command]git config gc.auto 0
2020-02-09T16:49:28.1970412Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T16:49:28.2021029Z ##[command]git config --get-all http.proxy
2020-02-09T16:49:28.2156413Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68944/merge:refs/remotes/pull/68944/merge
---
2020-02-09T17:44:50.0404417Z .................................................................................................... 1700/9621
2020-02-09T17:44:55.0735121Z .................................................................................................... 1800/9621
2020-02-09T17:45:07.3792112Z ............................i....................................................................... 1900/9621
2020-02-09T17:45:14.7015826Z .................................................................................................... 2000/9621
2020-02-09T17:45:28.8136213Z ..................iiiii............................................................................. 2100/9621
2020-02-09T17:45:38.6564145Z .................................................................................................... 2300/9621
2020-02-09T17:45:41.1379448Z .................................................................................................... 2400/9621
2020-02-09T17:45:46.1073312Z .................................................................................................... 2500/9621
2020-02-09T17:46:06.3049220Z .................................................................................................... 2600/9621
---
2020-02-09T17:48:39.6374385Z .....................................................................i...............i.............. 4900/9621
2020-02-09T17:48:47.3951651Z .................................................................................................... 5000/9621
2020-02-09T17:48:55.1226684Z .................................................................................................... 5100/9621
2020-02-09T17:48:59.8097665Z ...........i........................................................................................ 5200/9621
2020-02-09T17:49:10.5989351Z .....................................................................................ii.ii........i. 5300/9621
2020-02-09T17:49:14.3439369Z ..i................................................................................................. 5400/9621
2020-02-09T17:49:26.2859274Z .................................................................................................... 5600/9621
2020-02-09T17:49:34.4643556Z .........................................................................i.......................... 5700/9621
2020-02-09T17:49:41.9741015Z .................................................................................................... 5800/9621
2020-02-09T17:49:48.6404894Z .................................................................................................... 5900/9621
2020-02-09T17:49:48.6404894Z .................................................................................................... 5900/9621
2020-02-09T17:49:58.8231634Z .................................................................ii...i..ii...........i............. 6000/9621
2020-02-09T17:50:18.7177578Z .................................................................................................... 6200/9621
2020-02-09T17:50:22.6316577Z .................................................................................................... 6300/9621
2020-02-09T17:50:22.6316577Z .................................................................................................... 6300/9621
2020-02-09T17:50:26.7687543Z .............................................................................................i..ii.. 6400/9621
2020-02-09T17:50:49.0079362Z .................................................................................................... 6600/9621
2020-02-09T17:50:58.7141907Z ................................................................................i................... 6700/9621
2020-02-09T17:51:00.8813845Z .................................................................................................... 6800/9621
2020-02-09T17:51:03.0789257Z .......................................................................................i............ 6900/9621
---
2020-02-09T17:52:39.2251311Z .................................................................................................... 7600/9621
2020-02-09T17:52:43.3706941Z .................................................................................................... 7700/9621
2020-02-09T17:52:49.0156825Z .................................................................................................... 7800/9621
2020-02-09T17:52:57.8258930Z .................................................................................................... 7900/9621
2020-02-09T17:53:07.2756981Z ...............................................................iiiiiii.i............................ 8000/9621
2020-02-09T17:53:22.3607984Z ...i......i......................................................................................... 8200/9621
2020-02-09T17:53:27.9725186Z .................................................................................................... 8300/9621
2020-02-09T17:53:42.3630548Z .................................................................................................... 8400/9621
2020-02-09T17:53:50.7020204Z .................................................................................................... 8500/9621
---
2020-02-09T17:56:08.6604189Z  finished in 6.925
2020-02-09T17:56:08.6829051Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T17:56:09.5549757Z 
2020-02-09T17:56:09.5562390Z running 178 tests
2020-02-09T17:56:11.8777958Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-09T17:56:14.1512136Z ...i.i.i...iii...iiiiiiiiiiiiiiii......................iii............ii......
2020-02-09T17:56:14.1513528Z 
2020-02-09T17:56:14.1517027Z  finished in 5.469
2020-02-09T17:56:14.1709500Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T17:56:14.3407579Z 
---
2020-02-09T17:56:16.2761979Z  finished in 2.104
2020-02-09T17:56:16.2951824Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T17:56:17.0587608Z 
2020-02-09T17:56:17.0587848Z running 9 tests
2020-02-09T17:56:17.0588663Z iiiiiiiii
2020-02-09T17:56:17.0589070Z 
2020-02-09T17:56:17.0589119Z  finished in 0.159
2020-02-09T17:56:17.0589428Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T17:56:17.0589467Z 
2020-02-09T17:56:17.0589467Z 
2020-02-09T17:56:17.0589528Z running 115 tests
2020-02-09T17:56:32.8344249Z ...........................FF.FFFFFFFFFFFFFFFFFFFFFFFF.FFF..F....F.........F...........FF.F......... 100/115
2020-02-09T17:56:35.0166917Z .F.............
2020-02-09T17:56:35.0170819Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-09T17:56:35.0171160Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-09T17:56:35.0174213Z 
2020-02-09T17:56:35.0174951Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-02-09T17:56:35.0174951Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-02-09T17:56:35.0175133Z 
2020-02-09T17:56:35.0175528Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0175695Z status: exit code: 1
2020-02-09T17:56:35.0176949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2020-02-09T17:56:35.0177555Z ------------------------------------------
2020-02-09T17:56:35.0177698Z 
2020-02-09T17:56:35.0178056Z ------------------------------------------
2020-02-09T17:56:35.0178409Z stderr:
2020-02-09T17:56:35.0178409Z stderr:
2020-02-09T17:56:35.0178837Z ------------------------------------------
2020-02-09T17:56:35.0179219Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0179824Z    |
2020-02-09T17:56:35.0179824Z    |
2020-02-09T17:56:35.0179981Z LL | #[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
2020-02-09T17:56:35.0180286Z 
2020-02-09T17:56:35.0180662Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0180823Z   left: `LLVMing`,
2020-02-09T17:56:35.0180823Z   left: `LLVMing`,
2020-02-09T17:56:35.0181199Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0181495Z 
2020-02-09T17:56:35.0181656Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0181779Z 
2020-02-09T17:56:35.0181926Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0181926Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0182066Z 
2020-02-09T17:56:35.0182615Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0182767Z 
2020-02-09T17:56:35.0183183Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0183753Z 
2020-02-09T17:56:35.0184411Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0194025Z error: aborting due to previous error
2020-02-09T17:56:35.0194140Z 
2020-02-09T17:56:35.0194171Z 
2020-02-09T17:56:35.0194664Z ------------------------------------------
2020-02-09T17:56:35.0194664Z ------------------------------------------
2020-02-09T17:56:35.0194698Z 
2020-02-09T17:56:35.0194725Z 
2020-02-09T17:56:35.0195018Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2020-02-09T17:56:35.0195056Z 
2020-02-09T17:56:35.0195478Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0195553Z status: exit code: 1
2020-02-09T17:56:35.0196678Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2020-02-09T17:56:35.0197517Z ------------------------------------------
2020-02-09T17:56:35.0197554Z 
2020-02-09T17:56:35.0197805Z ------------------------------------------
2020-02-09T17:56:35.0197855Z stderr:
2020-02-09T17:56:35.0197855Z stderr:
2020-02-09T17:56:35.0198083Z ------------------------------------------
2020-02-09T17:56:35.0198337Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0198670Z    |
2020-02-09T17:56:35.0198670Z    |
2020-02-09T17:56:35.0198738Z LL | #[rustc_clean(cfg="cfail2", except="HirBody")]
2020-02-09T17:56:35.0198825Z 
2020-02-09T17:56:35.0199084Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0199151Z   left: `LLVMing`,
2020-02-09T17:56:35.0199151Z   left: `LLVMing`,
2020-02-09T17:56:35.0199402Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0199521Z 
2020-02-09T17:56:35.0199569Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0199601Z 
2020-02-09T17:56:35.0199650Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0199650Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0199696Z 
2020-02-09T17:56:35.0200080Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0200119Z 
2020-02-09T17:56:35.0200431Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0200471Z 
2020-02-09T17:56:35.0200918Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0201033Z error: aborting due to previous error
2020-02-09T17:56:35.0201067Z 
2020-02-09T17:56:35.0201094Z 
2020-02-09T17:56:35.0201348Z ------------------------------------------
2020-02-09T17:56:35.0201348Z ------------------------------------------
2020-02-09T17:56:35.0201381Z 
2020-02-09T17:56:35.0201409Z 
2020-02-09T17:56:35.0201655Z ---- [incremental] incremental/hashes/consts.rs stdout ----
2020-02-09T17:56:35.0201688Z 
2020-02-09T17:56:35.0201967Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0202021Z status: exit code: 1
2020-02-09T17:56:35.0203142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
2020-02-09T17:56:35.0203525Z ------------------------------------------
2020-02-09T17:56:35.0203559Z 
2020-02-09T17:56:35.0203791Z ------------------------------------------
2020-02-09T17:56:35.0203899Z stderr:
2020-02-09T17:56:35.0203899Z stderr:
2020-02-09T17:56:35.0204155Z ------------------------------------------
2020-02-09T17:56:35.0204387Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0204706Z    |
2020-02-09T17:56:35.0204706Z    |
2020-02-09T17:56:35.0204760Z LL | #[rustc_clean(cfg="cfail2", except="Hir,HirBody")]
2020-02-09T17:56:35.0204861Z 
2020-02-09T17:56:35.0205120Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0205172Z   left: `LLVMing`,
2020-02-09T17:56:35.0205172Z   left: `LLVMing`,
2020-02-09T17:56:35.0205427Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0205536Z 
2020-02-09T17:56:35.0205584Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0205631Z 
2020-02-09T17:56:35.0205680Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0205680Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0205720Z 
2020-02-09T17:56:35.0206060Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0206103Z 
2020-02-09T17:56:35.0206396Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0206432Z 
2020-02-09T17:56:35.0206893Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0207003Z error: aborting due to previous error
2020-02-09T17:56:35.0207049Z 
2020-02-09T17:56:35.0207077Z 
2020-02-09T17:56:35.0207306Z ------------------------------------------
2020-02-09T17:56:35.0207306Z ------------------------------------------
2020-02-09T17:56:35.0207337Z 
2020-02-09T17:56:35.0207364Z 
2020-02-09T17:56:35.0207636Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2020-02-09T17:56:35.0207681Z 
2020-02-09T17:56:35.0207942Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0207994Z status: exit code: 1
2020-02-09T17:56:35.0209102Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2020-02-09T17:56:35.0209472Z ------------------------------------------
2020-02-09T17:56:35.0209506Z 
2020-02-09T17:56:35.0209752Z ------------------------------------------
2020-02-09T17:56:35.0209799Z stderr:
2020-02-09T17:56:35.0209799Z stderr:
2020-02-09T17:56:35.0210027Z ------------------------------------------
2020-02-09T17:56:35.0210278Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0210595Z    |
2020-02-09T17:56:35.0210595Z    |
2020-02-09T17:56:35.0210724Z LL | #[rustc_clean(cfg="cfail2", except="HirBody,optimized_mir,mir_built")]
2020-02-09T17:56:35.0210820Z 
2020-02-09T17:56:35.0211085Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0211151Z   left: `LLVMing`,
2020-02-09T17:56:35.0211151Z   left: `LLVMing`,
2020-02-09T17:56:35.0211397Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0211562Z 
2020-02-09T17:56:35.0211611Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0211642Z 
2020-02-09T17:56:35.0211691Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0211691Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0211739Z 
2020-02-09T17:56:35.0212073Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0212111Z 
2020-02-09T17:56:35.0212412Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0212457Z 
2020-02-09T17:56:35.0212903Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0213017Z error: aborting due to previous error
2020-02-09T17:56:35.0213057Z 
2020-02-09T17:56:35.0213084Z 
2020-02-09T17:56:35.0213328Z ------------------------------------------
2020-02-09T17:56:35.0213328Z ------------------------------------------
2020-02-09T17:56:35.0213360Z 
2020-02-09T17:56:35.0213387Z 
2020-02-09T17:56:35.0213635Z ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
2020-02-09T17:56:35.0213669Z 
2020-02-09T17:56:35.0213945Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0213998Z status: exit code: 1
2020-02-09T17:56:35.0215053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
2020-02-09T17:56:35.0215437Z ------------------------------------------
2020-02-09T17:56:35.0215470Z 
2020-02-09T17:56:35.0215700Z ------------------------------------------
2020-02-09T17:56:35.0215748Z stderr:
2020-02-09T17:56:35.0215748Z stderr:
2020-02-09T17:56:35.0215997Z ------------------------------------------
2020-02-09T17:56:35.0216228Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0216551Z    |
2020-02-09T17:56:35.0216551Z    |
2020-02-09T17:56:35.0216603Z LL | #[rustc_clean(cfg="cfail2", except="Hir,HirBody")]
2020-02-09T17:56:35.0216711Z 
2020-02-09T17:56:35.0216967Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0217018Z   left: `LLVMing`,
2020-02-09T17:56:35.0217018Z   left: `LLVMing`,
2020-02-09T17:56:35.0217265Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0217373Z 
2020-02-09T17:56:35.0217420Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0217465Z 
2020-02-09T17:56:35.0217513Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0217513Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0217546Z 
2020-02-09T17:56:35.0217919Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0217976Z 
2020-02-09T17:56:35.0218997Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0219037Z 
2020-02-09T17:56:35.0219509Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0219699Z error: aborting due to previous error
2020-02-09T17:56:35.0219746Z 
2020-02-09T17:56:35.0219773Z 
2020-02-09T17:56:35.0220017Z ------------------------------------------
2020-02-09T17:56:35.0220017Z ------------------------------------------
2020-02-09T17:56:35.0220049Z 
2020-02-09T17:56:35.0220076Z 
2020-02-09T17:56:35.0220345Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2020-02-09T17:56:35.0220381Z 
2020-02-09T17:56:35.0220650Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0220703Z status: exit code: 1
2020-02-09T17:56:35.0221801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2020-02-09T17:56:35.0222180Z ------------------------------------------
2020-02-09T17:56:35.0222214Z 
2020-02-09T17:56:35.0222461Z ------------------------------------------
2020-02-09T17:56:35.0222508Z stderr:
2020-02-09T17:56:35.0222508Z stderr:
2020-02-09T17:56:35.0222735Z ------------------------------------------
2020-02-09T17:56:35.0222987Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0223311Z    |
2020-02-09T17:56:35.0223311Z    |
2020-02-09T17:56:35.0223379Z LL | #[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir")]
2020-02-09T17:56:35.0223468Z 
2020-02-09T17:56:35.0223723Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0223790Z   left: `LLVMing`,
2020-02-09T17:56:35.0223790Z   left: `LLVMing`,
2020-02-09T17:56:35.0224037Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0224154Z 
2020-02-09T17:56:35.0224202Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0224233Z 
2020-02-09T17:56:35.0224281Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0224281Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0224327Z 
2020-02-09T17:56:35.0224656Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0224706Z 
2020-02-09T17:56:35.0225009Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0225045Z 
2020-02-09T17:56:35.0225490Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0225604Z error: aborting due to previous error
2020-02-09T17:56:35.0225635Z 
2020-02-09T17:56:35.0225662Z 
2020-02-09T17:56:35.0225955Z ------------------------------------------
2020-02-09T17:56:35.0225955Z ------------------------------------------
2020-02-09T17:56:35.0226006Z 
2020-02-09T17:56:35.0226033Z 
2020-02-09T17:56:35.0226296Z ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
2020-02-09T17:56:35.0226331Z 
2020-02-09T17:56:35.0226607Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0226660Z status: exit code: 101
2020-02-09T17:56:35.0227794Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
2020-02-09T17:56:35.0228174Z ------------------------------------------
2020-02-09T17:56:35.0228207Z 
2020-02-09T17:56:35.0228447Z ------------------------------------------
2020-02-09T17:56:35.0228495Z stderr:
2020-02-09T17:56:35.0228495Z stderr:
2020-02-09T17:56:35.0228739Z ------------------------------------------
2020-02-09T17:56:35.0229060Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:378:24
2020-02-09T17:56:35.0229176Z 
2020-02-09T17:56:35.0229223Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0229254Z 
2020-02-09T17:56:35.0229324Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0229324Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0229358Z 
2020-02-09T17:56:35.0229681Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0229719Z 
2020-02-09T17:56:35.0230022Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0230058Z 
2020-02-09T17:56:35.0230504Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0230875Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0230927Z   left: `LLVMing`,
2020-02-09T17:56:35.0230927Z   left: `LLVMing`,
2020-02-09T17:56:35.0231190Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0231274Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0231305Z 
2020-02-09T17:56:35.0231377Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0231409Z 
2020-02-09T17:56:35.0231409Z 
2020-02-09T17:56:35.0231731Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0231781Z 
2020-02-09T17:56:35.0232069Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0232114Z 
2020-02-09T17:56:35.0232561Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0232653Z 
2020-02-09T17:56:35.0232883Z ------------------------------------------
2020-02-09T17:56:35.0232915Z 
2020-02-09T17:56:35.0232957Z 
2020-02-09T17:56:35.0232957Z 
2020-02-09T17:56:35.0233207Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2020-02-09T17:56:35.0233241Z 
2020-02-09T17:56:35.0233557Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0233628Z status: exit code: 1
2020-02-09T17:56:35.0234700Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2020-02-09T17:56:35.0235156Z ------------------------------------------
2020-02-09T17:56:35.0235204Z 
2020-02-09T17:56:35.0235438Z ------------------------------------------
2020-02-09T17:56:35.0235486Z stderr:
2020-02-09T17:56:35.0235486Z stderr:
2020-02-09T17:56:35.0235713Z ------------------------------------------
2020-02-09T17:56:35.0235964Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0236282Z    |
2020-02-09T17:56:35.0236282Z    |
2020-02-09T17:56:35.0236352Z LL | #[rustc_clean(cfg="cfail2", except="HirBody, mir_built, optimized_mir")]
2020-02-09T17:56:35.0236443Z 
2020-02-09T17:56:35.0236714Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0236766Z   left: `LLVMing`,
2020-02-09T17:56:35.0236766Z   left: `LLVMing`,
2020-02-09T17:56:35.0237015Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0237132Z 
2020-02-09T17:56:35.0237180Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0237211Z 
2020-02-09T17:56:35.0237259Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0237259Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0237306Z 
2020-02-09T17:56:35.0237626Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0237673Z 
2020-02-09T17:56:35.0237979Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0238016Z 
2020-02-09T17:56:35.0238463Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0238577Z error: aborting due to previous error
2020-02-09T17:56:35.0238609Z 
2020-02-09T17:56:35.0238636Z 
2020-02-09T17:56:35.0238889Z ------------------------------------------
2020-02-09T17:56:35.0238889Z ------------------------------------------
2020-02-09T17:56:35.0238922Z 
2020-02-09T17:56:35.0238949Z 
2020-02-09T17:56:35.0239209Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2020-02-09T17:56:35.0239260Z 
2020-02-09T17:56:35.0239521Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0239582Z status: exit code: 1
2020-02-09T17:56:35.0240754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2020-02-09T17:56:35.0241136Z ------------------------------------------
2020-02-09T17:56:35.0241229Z 
2020-02-09T17:56:35.0241468Z ------------------------------------------
2020-02-09T17:56:35.0241529Z stderr:
2020-02-09T17:56:35.0241529Z stderr:
2020-02-09T17:56:35.0241758Z ------------------------------------------
2020-02-09T17:56:35.0241988Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0242322Z    |
2020-02-09T17:56:35.0242322Z    |
2020-02-09T17:56:35.0242381Z LL |               except = "Hir, HirBody, mir_built, optimized_mir, typeck_tables_of, fn_sig")]
2020-02-09T17:56:35.0242504Z 
2020-02-09T17:56:35.0242759Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0242824Z   left: `LLVMing`,
2020-02-09T17:56:35.0242824Z   left: `LLVMing`,
2020-02-09T17:56:35.0243072Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0243172Z 
2020-02-09T17:56:35.0243235Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0243266Z 
2020-02-09T17:56:35.0243315Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0243315Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0243347Z 
2020-02-09T17:56:35.0243681Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0243719Z 
2020-02-09T17:56:35.0244006Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0244058Z 
2020-02-09T17:56:35.0244512Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0244626Z error: aborting due to previous error
2020-02-09T17:56:35.0244658Z 
2020-02-09T17:56:35.0244685Z 
2020-02-09T17:56:35.0244923Z ------------------------------------------
2020-02-09T17:56:35.0244923Z ------------------------------------------
2020-02-09T17:56:35.0244970Z 
2020-02-09T17:56:35.0244998Z 
2020-02-09T17:56:35.0245250Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2020-02-09T17:56:35.0245285Z 
2020-02-09T17:56:35.0245546Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0245612Z status: exit code: 1
2020-02-09T17:56:35.0246694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2020-02-09T17:56:35.0247065Z ------------------------------------------
2020-02-09T17:56:35.0247112Z 
2020-02-09T17:56:35.0247343Z ------------------------------------------
2020-02-09T17:56:35.0247391Z stderr:
2020-02-09T17:56:35.0247391Z stderr:
2020-02-09T17:56:35.0247633Z ------------------------------------------
2020-02-09T17:56:35.0247924Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0248264Z    |
2020-02-09T17:56:35.0248264Z    |
2020-02-09T17:56:35.0248319Z LL | #[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
2020-02-09T17:56:35.0248493Z 
2020-02-09T17:56:35.0248756Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0248806Z   left: `LLVMing`,
2020-02-09T17:56:35.0248806Z   left: `LLVMing`,
2020-02-09T17:56:35.0249053Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0249162Z 
2020-02-09T17:56:35.0249209Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0249241Z 
2020-02-09T17:56:35.0249304Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0249304Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0249337Z 
2020-02-09T17:56:35.0249668Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0249720Z 
2020-02-09T17:56:35.0250007Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0250044Z 
2020-02-09T17:56:35.0250504Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0250613Z error: aborting due to previous error
2020-02-09T17:56:35.0250645Z 
2020-02-09T17:56:35.0250685Z 
2020-02-09T17:56:35.0250916Z ------------------------------------------
2020-02-09T17:56:35.0250916Z ------------------------------------------
2020-02-09T17:56:35.0250948Z 
2020-02-09T17:56:35.0250975Z 
2020-02-09T17:56:35.0251248Z ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
2020-02-09T17:56:35.0251284Z 
2020-02-09T17:56:35.0251555Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0251606Z status: exit code: 1
2020-02-09T17:56:35.0252730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
2020-02-09T17:56:35.0253112Z ------------------------------------------
2020-02-09T17:56:35.0253145Z 
2020-02-09T17:56:35.0253392Z ------------------------------------------
2020-02-09T17:56:35.0253439Z stderr:
2020-02-09T17:56:35.0253439Z stderr:
2020-02-09T17:56:35.0253666Z ------------------------------------------
2020-02-09T17:56:35.0253909Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0254236Z    |
2020-02-09T17:56:35.0254236Z    |
2020-02-09T17:56:35.0254301Z LL | #[rustc_clean(label="Hir", cfg="cfail2")]
2020-02-09T17:56:35.0254382Z 
2020-02-09T17:56:35.0254637Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0254703Z   left: `LLVMing`,
2020-02-09T17:56:35.0254703Z   left: `LLVMing`,
2020-02-09T17:56:35.0254952Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0255101Z 
2020-02-09T17:56:35.0255164Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0255196Z 
2020-02-09T17:56:35.0255245Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0255245Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0255277Z 
2020-02-09T17:56:35.0255623Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0255718Z 
2020-02-09T17:56:35.0256015Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0256066Z 
2020-02-09T17:56:35.0256512Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0256627Z error: aborting due to previous error
2020-02-09T17:56:35.0256657Z 
2020-02-09T17:56:35.0256684Z 
2020-02-09T17:56:35.0256924Z ------------------------------------------
2020-02-09T17:56:35.0256924Z ------------------------------------------
2020-02-09T17:56:35.0256971Z 
2020-02-09T17:56:35.0256998Z 
2020-02-09T17:56:35.0257250Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2020-02-09T17:56:35.0257285Z 
2020-02-09T17:56:35.0257562Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0257614Z status: exit code: 1
2020-02-09T17:56:35.0258802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2020-02-09T17:56:35.0259172Z ------------------------------------------
2020-02-09T17:56:35.0259220Z 
2020-02-09T17:56:35.0259451Z ------------------------------------------
2020-02-09T17:56:35.0259506Z stderr:
2020-02-09T17:56:35.0259506Z stderr:
2020-02-09T17:56:35.0259750Z ------------------------------------------
2020-02-09T17:56:35.0259987Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0260309Z    |
2020-02-09T17:56:35.0260309Z    |
2020-02-09T17:56:35.0260365Z LL | #[rustc_clean(cfg="cfail2", except="HirBody, mir_built, optimized_mir")]
2020-02-09T17:56:35.0260456Z 
2020-02-09T17:56:35.0260735Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0260786Z   left: `LLVMing`,
2020-02-09T17:56:35.0260786Z   left: `LLVMing`,
2020-02-09T17:56:35.0261036Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0261144Z 
2020-02-09T17:56:35.0261191Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0261230Z 
2020-02-09T17:56:35.0261293Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0261293Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0261325Z 
2020-02-09T17:56:35.0261649Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0261702Z 
2020-02-09T17:56:35.0265184Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0267202Z 
2020-02-09T17:56:35.0268265Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0268390Z error: aborting due to previous error
2020-02-09T17:56:35.0268422Z 
2020-02-09T17:56:35.0268467Z 
2020-02-09T17:56:35.0268724Z ------------------------------------------
2020-02-09T17:56:35.0268724Z ------------------------------------------
2020-02-09T17:56:35.0268757Z 
2020-02-09T17:56:35.0268785Z 
2020-02-09T17:56:35.0270902Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2020-02-09T17:56:35.0270950Z 
2020-02-09T17:56:35.0271223Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0271279Z status: exit code: 1
2020-02-09T17:56:35.0272389Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2020-02-09T17:56:35.0272767Z ------------------------------------------
2020-02-09T17:56:35.0272802Z 
2020-02-09T17:56:35.0273049Z ------------------------------------------
2020-02-09T17:56:35.0273097Z stderr:
2020-02-09T17:56:35.0273097Z stderr:
2020-02-09T17:56:35.0273326Z ------------------------------------------
2020-02-09T17:56:35.0273557Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0273896Z    |
2020-02-09T17:56:35.0273896Z    |
2020-02-09T17:56:35.0273952Z LL | #[rustc_clean(cfg="cfail2", except="Hir,HirBody,associated_item_def_ids")]
2020-02-09T17:56:35.0274060Z 
2020-02-09T17:56:35.0274317Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0274393Z   left: `LLVMing`,
2020-02-09T17:56:35.0274393Z   left: `LLVMing`,
2020-02-09T17:56:35.0274642Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0274738Z 
2020-02-09T17:56:35.0274802Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0274834Z 
---
2020-02-09T17:56:35.0391472Z    |
2020-02-09T17:56:35.0391517Z LL |     fn baz() {
2020-02-09T17:56:35.0391563Z    |        ^^^
2020-02-09T17:56:35.0391606Z 
2020-02-09T17:56:35.0391829Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0392139Z    |
2020-02-09T17:56:35.0392139Z    |
2020-02-09T17:56:35.0392201Z LL |     #[rustc_clean(label="Hir", cfg="rpass2")]
2020-02-09T17:56:35.0392284Z 
2020-02-09T17:56:35.0392559Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0392612Z   left: `LLVMing`,
2020-02-09T17:56:35.0392612Z   left: `LLVMing`,
2020-02-09T17:56:35.0392853Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0392965Z 
2020-02-09T17:56:35.0393012Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0393053Z 
2020-02-09T17:56:35.0393103Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0393103Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0393150Z 
2020-02-09T17:56:35.0393506Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0393545Z 
2020-02-09T17:56:35.0393847Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0393883Z 
2020-02-09T17:56:35.0394306Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0394419Z error: aborting due to previous error
2020-02-09T17:56:35.0394450Z 
2020-02-09T17:56:35.0394477Z 
2020-02-09T17:56:35.0394717Z ------------------------------------------
2020-02-09T17:56:35.0394717Z ------------------------------------------
2020-02-09T17:56:35.0394749Z 
2020-02-09T17:56:35.0394777Z 
2020-02-09T17:56:35.0395029Z ---- [incremental] incremental/ich_nested_items.rs stdout ----
2020-02-09T17:56:35.0395063Z 
2020-02-09T17:56:35.0395333Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0395386Z status: exit code: 1
2020-02-09T17:56:35.0396411Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_nested_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/ich_nested_items.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/auxiliary"
2020-02-09T17:56:35.0396770Z ------------------------------------------
2020-02-09T17:56:35.0396803Z 
2020-02-09T17:56:35.0397029Z ------------------------------------------
2020-02-09T17:56:35.0397076Z stderr:
2020-02-09T17:56:35.0397076Z stderr:
2020-02-09T17:56:35.0397315Z ------------------------------------------
2020-02-09T17:56:35.0397366Z warning: function is never used: `bar`
2020-02-09T17:56:35.0397617Z   --> /checkout/src/test/incremental/ich_nested_items.rs:18:12
2020-02-09T17:56:35.0397681Z    |
2020-02-09T17:56:35.0397978Z LL |     pub fn bar() { } // but that doesn't matter.
2020-02-09T17:56:35.0398092Z    |
2020-02-09T17:56:35.0398139Z    = note: `#[warn(dead_code)]` on by default
2020-02-09T17:56:35.0398172Z 
2020-02-09T17:56:35.0398218Z warning: function is never used: `baz`
---
2020-02-09T17:56:35.0398739Z 
2020-02-09T17:56:35.0398784Z warning: function is never used: `bap`
2020-02-09T17:56:35.0399044Z   --> /checkout/src/test/incremental/ich_nested_items.rs:23:12
2020-02-09T17:56:35.0399106Z    |
2020-02-09T17:56:35.0399155Z LL |     pub fn bap() { } // neither does adding a new item
2020-02-09T17:56:35.0399247Z 
2020-02-09T17:56:35.0399247Z 
2020-02-09T17:56:35.0399474Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0399779Z    |
2020-02-09T17:56:35.0399779Z    |
2020-02-09T17:56:35.0399842Z LL | #[rustc_clean(label="Hir", cfg="cfail2")]
2020-02-09T17:56:35.0399923Z 
2020-02-09T17:56:35.0400186Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0400238Z   left: `LLVMing`,
2020-02-09T17:56:35.0400238Z   left: `LLVMing`,
2020-02-09T17:56:35.0400488Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0400598Z 
2020-02-09T17:56:35.0400647Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0400678Z 
2020-02-09T17:56:35.0400727Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0400727Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0400774Z 
2020-02-09T17:56:35.0401092Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0401131Z 
2020-02-09T17:56:35.0401439Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0401475Z 
2020-02-09T17:56:35.0401869Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0401987Z error: aborting due to previous error
2020-02-09T17:56:35.0402017Z 
2020-02-09T17:56:35.0402044Z 
2020-02-09T17:56:35.0402283Z ------------------------------------------
2020-02-09T17:56:35.0402283Z ------------------------------------------
2020-02-09T17:56:35.0402316Z 
2020-02-09T17:56:35.0402343Z 
2020-02-09T17:56:35.0402590Z ---- [incremental] incremental/ich_resolve_results.rs stdout ----
2020-02-09T17:56:35.0402638Z 
2020-02-09T17:56:35.0402687Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0402737Z status: exit code: 1
2020-02-09T17:56:35.0403772Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/auxiliary"
2020-02-09T17:56:35.0404133Z ------------------------------------------
2020-02-09T17:56:35.0404166Z 
2020-02-09T17:56:35.0404391Z ------------------------------------------
2020-02-09T17:56:35.0404452Z stderr:
---
2020-02-09T17:56:35.0407269Z    |
2020-02-09T17:56:35.0407313Z LL |     fn in_type() {
2020-02-09T17:56:35.0407373Z    |        ^^^^^^^
2020-02-09T17:56:35.0407402Z 
2020-02-09T17:56:35.0407623Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0407940Z    |
2020-02-09T17:56:35.0407940Z    |
2020-02-09T17:56:35.0407990Z LL |     #[rustc_clean(label="Hir", cfg="rpass2")]
2020-02-09T17:56:35.0408095Z 
2020-02-09T17:56:35.0408346Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0408397Z   left: `LLVMing`,
2020-02-09T17:56:35.0408397Z   left: `LLVMing`,
2020-02-09T17:56:35.0408656Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0408761Z 
2020-02-09T17:56:35.0408810Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0408855Z 
2020-02-09T17:56:35.0408904Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0408904Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0408936Z 
2020-02-09T17:56:35.0409266Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0409305Z 
2020-02-09T17:56:35.0409589Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0409626Z 
2020-02-09T17:56:35.0410042Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0410137Z error: aborting due to previous error
2020-02-09T17:56:35.0410183Z 
2020-02-09T17:56:35.0410211Z 
2020-02-09T17:56:35.0410438Z ------------------------------------------
2020-02-09T17:56:35.0410438Z ------------------------------------------
2020-02-09T17:56:35.0410478Z 
2020-02-09T17:56:35.0410506Z 
2020-02-09T17:56:35.0410762Z ---- [incremental] incremental/issue-38222.rs stdout ----
2020-02-09T17:56:35.0410795Z 
2020-02-09T17:56:35.0410844Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0410908Z status: exit code: 1
2020-02-09T17:56:35.0411985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
2020-02-09T17:56:35.0412397Z ------------------------------------------
2020-02-09T17:56:35.0412431Z 
2020-02-09T17:56:35.0412672Z ------------------------------------------
2020-02-09T17:56:35.0412719Z stderr:
2020-02-09T17:56:35.0412719Z stderr:
2020-02-09T17:56:35.0412942Z ------------------------------------------
2020-02-09T17:56:35.0413227Z error: CGU-reuse for `issue_38222-mod1` is `No` but should be at least `PreLto`
2020-02-09T17:56:35.0413539Z    |
2020-02-09T17:56:35.0413539Z    |
2020-02-09T17:56:35.0413819Z LL | #![rustc_partition_reused(module = "issue_38222-mod1", cfg = "rpass2")]
2020-02-09T17:56:35.0413912Z 
2020-02-09T17:56:35.0413959Z error: aborting due to previous error
2020-02-09T17:56:35.0414003Z 
2020-02-09T17:56:35.0414039Z 
2020-02-09T17:56:35.0414039Z 
2020-02-09T17:56:35.0414263Z ------------------------------------------
2020-02-09T17:56:35.0414294Z 
2020-02-09T17:56:35.0414322Z 
2020-02-09T17:56:35.0414576Z ---- [incremental] incremental/issue-42602.rs stdout ----
2020-02-09T17:56:35.0414609Z 
2020-02-09T17:56:35.0414863Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0414929Z status: exit code: 101
2020-02-09T17:56:35.0415943Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
2020-02-09T17:56:35.0416300Z ------------------------------------------
2020-02-09T17:56:35.0416333Z 
2020-02-09T17:56:35.0416572Z ------------------------------------------
2020-02-09T17:56:35.0416619Z stderr:
2020-02-09T17:56:35.0416619Z stderr:
2020-02-09T17:56:35.0416841Z ------------------------------------------
2020-02-09T17:56:35.0417180Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:378:24
2020-02-09T17:56:35.0417283Z 
2020-02-09T17:56:35.0417344Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0417376Z 
2020-02-09T17:56:35.0417427Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0417427Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0417468Z 
2020-02-09T17:56:35.0417803Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0417841Z 
2020-02-09T17:56:35.0418125Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0418161Z 
2020-02-09T17:56:35.0418842Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0419229Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0419289Z   left: `LLVMing`,
2020-02-09T17:56:35.0419289Z   left: `LLVMing`,
2020-02-09T17:56:35.0419543Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0419641Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0419672Z 
2020-02-09T17:56:35.0419720Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0419815Z 
2020-02-09T17:56:35.0419815Z 
2020-02-09T17:56:35.0420162Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0420201Z 
2020-02-09T17:56:35.0420485Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0420535Z 
2020-02-09T17:56:35.0420945Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0421028Z 
2020-02-09T17:56:35.0421266Z ------------------------------------------
2020-02-09T17:56:35.0421299Z 
2020-02-09T17:56:35.0421326Z 
2020-02-09T17:56:35.0421326Z 
2020-02-09T17:56:35.0421590Z ---- [incremental] incremental/issue-62649-path-collisions-happen.rs stdout ----
2020-02-09T17:56:35.0421638Z 
2020-02-09T17:56:35.0421686Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0421745Z status: exit code: 101
2020-02-09T17:56:35.0422875Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-62649-path-collisions-happen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-62649-path-collisions-happen/issue-62649-path-collisions-happen.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-62649-path-collisions-happen/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-62649-path-collisions-happen/auxiliary"
2020-02-09T17:56:35.0423234Z ------------------------------------------
2020-02-09T17:56:35.0423276Z 
2020-02-09T17:56:35.0423501Z ------------------------------------------
2020-02-09T17:56:35.0423563Z stderr:
---
2020-02-09T17:56:35.0424243Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0424274Z 
2020-02-09T17:56:35.0424343Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0424376Z 
2020-02-09T17:56:35.0424694Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0424744Z 
2020-02-09T17:56:35.0425028Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0425065Z 
2020-02-09T17:56:35.0425466Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0425556Z 
2020-02-09T17:56:35.0425782Z ------------------------------------------
2020-02-09T17:56:35.0425814Z 
2020-02-09T17:56:35.0425856Z 
2020-02-09T17:56:35.0425856Z 
2020-02-09T17:56:35.0426102Z ---- [incremental] incremental/source_loc_macros.rs stdout ----
2020-02-09T17:56:35.0426136Z 
2020-02-09T17:56:35.0426184Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0426306Z status: exit code: 1
2020-02-09T17:56:35.0427349Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/source_loc_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/source_loc_macros.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/auxiliary"
2020-02-09T17:56:35.0427768Z ------------------------------------------
2020-02-09T17:56:35.0427816Z 
2020-02-09T17:56:35.0428052Z ------------------------------------------
2020-02-09T17:56:35.0428098Z stderr:
2020-02-09T17:56:35.0428098Z stderr:
2020-02-09T17:56:35.0428322Z ------------------------------------------
2020-02-09T17:56:35.0428561Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0428863Z    |
2020-02-09T17:56:35.0428863Z    |
2020-02-09T17:56:35.0428927Z LL | #[rustc_clean(label="Hir", cfg="rpass2")]
2020-02-09T17:56:35.0429017Z 
2020-02-09T17:56:35.0429283Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0429335Z   left: `LLVMing`,
2020-02-09T17:56:35.0429335Z   left: `LLVMing`,
2020-02-09T17:56:35.0429576Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0429683Z 
2020-02-09T17:56:35.0429731Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0429763Z 
2020-02-09T17:56:35.0429819Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0429819Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0429865Z 
2020-02-09T17:56:35.0430183Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0430222Z 
2020-02-09T17:56:35.0430522Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0430557Z 
2020-02-09T17:56:35.0430978Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0431092Z error: aborting due to previous error
2020-02-09T17:56:35.0431123Z 
2020-02-09T17:56:35.0431151Z 
2020-02-09T17:56:35.0431394Z ------------------------------------------
2020-02-09T17:56:35.0431394Z ------------------------------------------
2020-02-09T17:56:35.0431426Z 
2020-02-09T17:56:35.0431453Z 
2020-02-09T17:56:35.0431703Z ---- [incremental] incremental/span_hash_stable/main.rs stdout ----
2020-02-09T17:56:35.0431759Z 
2020-02-09T17:56:35.0431808Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0431857Z status: exit code: 1
2020-02-09T17:56:35.0433198Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/span_hash_stable/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/auxiliary"
2020-02-09T17:56:35.0433641Z ------------------------------------------
2020-02-09T17:56:35.0433677Z 
2020-02-09T17:56:35.0433915Z ------------------------------------------
2020-02-09T17:56:35.0433976Z stderr:
2020-02-09T17:56:35.0433976Z stderr:
2020-02-09T17:56:35.0434201Z ------------------------------------------
2020-02-09T17:56:35.0434427Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0434907Z   --> /checkout/src/test/incremental/span_hash_stable/auxiliary/sub1.rs:1:15
2020-02-09T17:56:35.0435035Z    |
2020-02-09T17:56:35.0435087Z LL | #[rustc_clean(label="Hir", cfg="rpass2")]
2020-02-09T17:56:35.0435182Z 
2020-02-09T17:56:35.0435466Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0435517Z   left: `LLVMing`,
2020-02-09T17:56:35.0435517Z   left: `LLVMing`,
2020-02-09T17:56:35.0435773Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0435868Z 
2020-02-09T17:56:35.0435923Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0435969Z 
2020-02-09T17:56:35.0436020Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0436020Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0436052Z 
2020-02-09T17:56:35.0436395Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0436434Z 
2020-02-09T17:56:35.0436720Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0436764Z 
2020-02-09T17:56:35.0437176Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
2020-02-09T17:56:35.0437275Z error: aborting due to previous error
2020-02-09T17:56:35.0437319Z 
2020-02-09T17:56:35.0437348Z 
2020-02-09T17:56:35.0437574Z ------------------------------------------
2020-02-09T17:56:35.0437574Z ------------------------------------------
2020-02-09T17:56:35.0437606Z 
2020-02-09T17:56:35.0437633Z 
2020-02-09T17:56:35.0437912Z ---- [incremental] incremental/spans_significant_w_debuginfo.rs stdout ----
2020-02-09T17:56:35.0437947Z 
2020-02-09T17:56:35.0437994Z error in revision `rpass2`: compilation failed!
2020-02-09T17:56:35.0438043Z status: exit code: 1
2020-02-09T17:56:35.0439136Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/spans_significant_w_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/auxiliary"
2020-02-09T17:56:35.0439499Z ------------------------------------------
2020-02-09T17:56:35.0439532Z 
2020-02-09T17:56:35.0439773Z ------------------------------------------
2020-02-09T17:56:35.0439820Z stderr:
2020-02-09T17:56:35.0439820Z stderr:
2020-02-09T17:56:35.0440051Z ------------------------------------------
2020-02-09T17:56:35.0440291Z error: dep-node label `Hir` not recognized
2020-02-09T17:56:35.0440608Z    |
2020-02-09T17:56:35.0440608Z    |
2020-02-09T17:56:35.0440673Z LL | #[rustc_dirty(label="Hir", cfg="rpass2")]
2020-02-09T17:56:35.0440754Z 
2020-02-09T17:56:35.0441005Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0441072Z   left: `LLVMing`,
2020-02-09T17:56:35.0441072Z   left: `LLVMing`,
2020-02-09T17:56:35.0441368Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0441479Z 
2020-02-09T17:56:35.0441528Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0441559Z 
2020-02-09T17:56:35.0441608Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0441608Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0441688Z 
2020-02-09T17:56:35.0442030Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0442068Z 
2020-02-09T17:56:35.0442363Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0442400Z 
2020-02-09T17:56:35.0442795Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
2020-02-09T17:56:35.0442905Z error: aborting due to previous error
2020-02-09T17:56:35.0442944Z 
2020-02-09T17:56:35.0442972Z 
2020-02-09T17:56:35.0443198Z ------------------------------------------
2020-02-09T17:56:35.0443198Z ------------------------------------------
2020-02-09T17:56:35.0443244Z 
2020-02-09T17:56:35.0443273Z 
2020-02-09T17:56:35.0443517Z ---- [incremental] incremental/string_constant.rs stdout ----
2020-02-09T17:56:35.0443553Z 
2020-02-09T17:56:35.0443824Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-09T17:56:35.0443885Z status: exit code: 1
2020-02-09T17:56:35.0444910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/string_constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/string_constant.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/auxiliary"
2020-02-09T17:56:35.0445263Z ------------------------------------------
2020-02-09T17:56:35.0445318Z 
2020-02-09T17:56:35.0445544Z ------------------------------------------
2020-02-09T17:56:35.0445591Z stderr:
2020-02-09T17:56:35.0445591Z stderr:
2020-02-09T17:56:35.0445829Z ------------------------------------------
2020-02-09T17:56:35.0446059Z error: dep-node label `HirBody` not recognized
2020-02-09T17:56:35.0446373Z    |
2020-02-09T17:56:35.0446373Z    |
2020-02-09T17:56:35.0446425Z LL |     #[rustc_dirty(label="HirBody", cfg="cfail2")]
2020-02-09T17:56:35.0446507Z 
2020-02-09T17:56:35.0446780Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-09T17:56:35.0446830Z   left: `LLVMing`,
2020-02-09T17:56:35.0446830Z   left: `LLVMing`,
2020-02-09T17:56:35.0447072Z  right: `Codegenning`', <::std::macros::panic macros>:5:6
2020-02-09T17:56:35.0447180Z 
2020-02-09T17:56:35.0447226Z error: internal compiler error: unexpected panic
2020-02-09T17:56:35.0447266Z 
2020-02-09T17:56:35.0447328Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0447328Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-09T17:56:35.0447361Z 
2020-02-09T17:56:35.0447678Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-09T17:56:35.0447730Z 
2020-02-09T17:56:35.0448014Z note: rustc 1.43.0-nightly (6f643a020 2020-02-09) running on x86_64-unknown-linux-gnu
2020-02-09T17:56:35.0448050Z 
2020-02-09T17:56:35.0448515Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-09T17:56:35.0448631Z error: aborting due to previous error
2020-02-09T17:56:35.0448663Z 
2020-02-09T17:56:35.0448702Z 
2020-02-09T17:56:35.0448936Z ------------------------------------------
---
2020-02-09T17:56:35.0452276Z test result: FAILED. 79 passed; 36 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-09T17:56:35.0452313Z 
2020-02-09T17:56:35.0452340Z 
2020-02-09T17:56:35.0452367Z 
2020-02-09T17:56:35.0454007Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-09T17:56:35.0454276Z 
2020-02-09T17:56:35.0454306Z 
2020-02-09T17:56:35.5542534Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-09T17:56:35.5543474Z Build completed unsuccessfully in 1:01:00
2020-02-09T17:56:35.5543474Z Build completed unsuccessfully in 1:01:00
2020-02-09T17:56:35.5543610Z == clock drift check ==
2020-02-09T17:56:35.5543709Z   local time: Sun Feb  9 17:56:35 UTC 2020
2020-02-09T17:56:35.5791788Z   network time: Sun, 09 Feb 2020 17:56:35 GMT
2020-02-09T17:56:35.5791923Z == end clock drift check ==
2020-02-09T17:56:38.1633890Z 
2020-02-09T17:56:38.1737146Z ##[error]Bash exited with code '1'.
2020-02-09T17:56:38.1749940Z ##[section]Finishing: Run build
2020-02-09T17:56:38.1774317Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68944/merge to s
2020-02-09T17:56:38.1776118Z Task         : Get sources
2020-02-09T17:56:38.1776171Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T17:56:38.1776224Z Version      : 1.0.0
2020-02-09T17:56:38.1776285Z Author       : Microsoft
2020-02-09T17:56:38.1776285Z Author       : Microsoft
2020-02-09T17:56:38.1776338Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T17:56:38.1776394Z ==============================================================================
2020-02-09T17:56:38.6026152Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T17:56:38.6074258Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68944/merge to s
2020-02-09T17:56:38.6209876Z Cleaning up task key
2020-02-09T17:56:38.6210705Z Start cleaning up orphan processes.
2020-02-09T17:56:38.6318239Z Terminate orphan process: pid (3588) (python)
2020-02-09T17:56:38.6585453Z ##[section]Finishing: Finalize Job
