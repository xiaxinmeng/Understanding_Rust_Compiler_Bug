plain
2020-04-18T15:08:16.8230399Z ========================== Starting Command Output ===========================
2020-04-18T15:08:16.8235988Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/319c3062-cf33-4a30-bf57-5f04a45c4616.sh
2020-04-18T15:08:16.8236492Z 
2020-04-18T15:08:16.8242231Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T15:08:16.8262726Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T15:08:16.8266147Z Task         : Get sources
2020-04-18T15:08:16.8266467Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T15:08:16.8266759Z Version      : 1.0.0
2020-04-18T15:08:16.8266960Z Author       : Microsoft
---
2020-04-18T15:08:18.0646529Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T15:08:18.0660905Z ##[command]git config gc.auto 0
2020-04-18T15:08:18.0668307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T15:08:18.0674953Z ##[command]git config --get-all http.proxy
2020-04-18T15:08:18.0716062Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-18T15:11:11.6119813Z  ---> 318032b5f0e2
2020-04-18T15:11:11.6120988Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T15:11:11.6123987Z  ---> Using cache
2020-04-18T15:11:11.6124375Z  ---> d44a858fd1ce
2020-04-18T15:11:11.6125442Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T15:11:11.6130254Z  ---> 58b910f50f5a
2020-04-18T15:11:11.6130686Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T15:11:11.6142143Z  ---> Using cache
2020-04-18T15:11:11.6142853Z  ---> ee7702aadba1
---
2020-04-18T15:11:11.6573573Z Looks like docker image is the same as before, not uploading
2020-04-18T15:11:19.5845956Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T15:11:19.6137806Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T15:11:19.6163456Z == clock drift check ==
2020-04-18T15:11:19.6174124Z   local time: Sat Apr 18 15:11:19 UTC 2020
2020-04-18T15:11:19.8054111Z   network time: Sat, 18 Apr 2020 15:11:19 GMT
2020-04-18T15:11:19.8079894Z Starting sccache server...
2020-04-18T15:11:19.8936346Z configure: processing command line
2020-04-18T15:11:19.8936594Z configure: 
2020-04-18T15:11:19.8937946Z configure: rust.dist-src        := False
---
2020-04-18T15:16:56.1110723Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T15:16:57.7420262Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T15:16:59.4374161Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T15:17:01.4575491Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T15:17:10.5499418Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T15:17:13.8079031Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T15:17:18.5166906Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T15:17:23.0941346Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T15:17:33.1188766Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T15:41:44.6958194Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T15:41:46.4850502Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T15:41:48.4515663Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T15:41:49.6126580Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T15:41:59.9513595Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T15:42:04.3012625Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T15:42:09.5576118Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T15:42:14.5541282Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T15:42:24.4947596Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T16:06:55.0777385Z .................................................................................................... 1700/9912
2020-04-18T16:06:59.6029374Z .................................................................................................... 1800/9912
2020-04-18T16:07:08.1294605Z .................................................................................................... 1900/9912
2020-04-18T16:07:16.2775943Z ....i............................................................................................... 2000/9912
2020-04-18T16:07:22.6570917Z ..............................................................................................iiiii. 2100/9912
2020-04-18T16:07:43.2752497Z .................................................................................................... 2300/9912
2020-04-18T16:07:45.8673451Z .................................................................................................... 2400/9912
2020-04-18T16:07:47.8335538Z .................................................................................................... 2500/9912
2020-04-18T16:07:53.7548249Z .................................................................................................... 2600/9912
---
2020-04-18T16:10:55.3225517Z .................................................................................................... 5100/9912
2020-04-18T16:11:02.5876891Z .................................................................................................... 5200/9912
2020-04-18T16:11:07.3499031Z ................i................................................................................... 5300/9912
2020-04-18T16:11:17.5019466Z ......i............................................................................................. 5400/9912
2020-04-18T16:11:23.0074182Z ......ii.ii........i...i............................................................................ 5500/9912
2020-04-18T16:11:30.9611121Z ....................................................i............................................... 5700/9912
2020-04-18T16:11:40.1179901Z ....................................................................................ii.............. 5800/9912
2020-04-18T16:11:47.0705162Z .......................i............................................................................ 5900/9912
2020-04-18T16:11:52.5291625Z .................................................................................................... 6000/9912
2020-04-18T16:11:52.5291625Z .................................................................................................... 6000/9912
2020-04-18T16:12:03.2912365Z .................................................................................................... 6100/9912
2020-04-18T16:12:13.2452854Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-18T16:12:29.2615474Z .................................................................................................... 6400/9912
2020-04-18T16:12:36.0477230Z .................................................................................................... 6500/9912
2020-04-18T16:12:36.0477230Z .................................................................................................... 6500/9912
2020-04-18T16:12:51.2817285Z ...............................................i..ii................................................ 6600/9912
2020-04-18T16:13:15.2700821Z .................................................................................................... 6800/9912
2020-04-18T16:13:17.5720436Z ................................................i................................................... 6900/9912
2020-04-18T16:13:19.6425539Z .................................................................................................... 7000/9912
2020-04-18T16:13:21.6795371Z ........................................................................................i........... 7100/9912
---
2020-04-18T16:15:00.6284833Z .................................................................................................... 7900/9912
2020-04-18T16:15:07.0250262Z .................................................................................................... 8000/9912
2020-04-18T16:15:12.7713493Z ......................................................i............................................. 8100/9912
2020-04-18T16:15:22.7027991Z .................................................................................................... 8200/9912
2020-04-18T16:15:28.0374641Z ....iiiiiiiiiii.i................................................................................... 8300/9912
2020-04-18T16:15:41.6445705Z .................................................................................................... 8500/9912
2020-04-18T16:15:49.8914042Z .................................................................................................... 8600/9912
2020-04-18T16:16:04.0649350Z .................................................................................................... 8700/9912
2020-04-18T16:16:11.1195238Z .................................................................................................... 8800/9912
---
2020-04-18T16:18:32.2360002Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T16:18:32.2538500Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T16:18:32.4488856Z 
2020-04-18T16:18:32.4489181Z running 186 tests
2020-04-18T16:18:35.4452341Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-18T16:18:38.2178966Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-18T16:18:38.2187764Z 
2020-04-18T16:18:38.2194811Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/t finished in 5.964
2020-04-18T16:18:38.2198079Z ools/tidy"]
2020-04-18T16:18:38.2399541Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T16:18:40.4304351Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-18T16:18:40.4485605Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T16:18:40.6008850Z 
2020-04-18T16:18:40.6009550Z running 9 tests
2020-04-18T16:18:40.6011205Z iiiiiiiii
2020-04-18T16:18:40.6012666Z 
2020-04-18T16:18:40.6016679Z  finished in 0.153
2020-04-18T16:18:40.6022284Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-18T16:18:40.6230672Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T16:18:40.6230672Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T16:18:40.8355213Z 
2020-04-18T16:18:40.8355572Z running 119 tests
2020-04-18T16:18:57.4330788Z ............................FF.F.F..FFF.FFFFFFF.....FFF...........F.............................F... 100/119
2020-04-18T16:19:00.2988813Z failures:
2020-04-18T16:19:00.2989318Z 
2020-04-18T16:19:00.3002083Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-04-18T16:19:00.3008999Z 
2020-04-18T16:19:00.3008999Z 
2020-04-18T16:19:00.3009911Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3010366Z status: exit code: 101
2020-04-18T16:19:00.3013970Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2020-04-18T16:19:00.3016233Z ------------------------------------------
2020-04-18T16:19:00.3016546Z 
2020-04-18T16:19:00.3017323Z ------------------------------------------
2020-04-18T16:19:00.3017550Z stderr:
2020-04-18T16:19:00.3017550Z stderr:
2020-04-18T16:19:00.3018127Z ------------------------------------------
2020-04-18T16:19:00.3019079Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3023554Z 
2020-04-18T16:19:00.3023768Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3024313Z 
2020-04-18T16:19:00.3024545Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3024545Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3036808Z 
2020-04-18T16:19:00.3037894Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3038636Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3038856Z 
2020-04-18T16:19:00.3038856Z 
2020-04-18T16:19:00.3039661Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3040522Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3040802Z   left: `LLVMing`,
2020-04-18T16:19:00.3040802Z   left: `LLVMing`,
2020-04-18T16:19:00.3041225Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3041778Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3041990Z 
2020-04-18T16:19:00.3042176Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3042341Z 
2020-04-18T16:19:00.3042341Z 
2020-04-18T16:19:00.3042949Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3043627Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3043833Z 
2020-04-18T16:19:00.3043833Z 
2020-04-18T16:19:00.3044608Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3045101Z 
2020-04-18T16:19:00.3045553Z ------------------------------------------
2020-04-18T16:19:00.3045695Z 
2020-04-18T16:19:00.3045775Z 
2020-04-18T16:19:00.3045775Z 
2020-04-18T16:19:00.3046284Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2020-04-18T16:19:00.3046479Z 
2020-04-18T16:19:00.3046919Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3047187Z status: exit code: 101
2020-04-18T16:19:00.3049465Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2020-04-18T16:19:00.3051213Z ------------------------------------------
2020-04-18T16:19:00.3051361Z 
2020-04-18T16:19:00.3051692Z ------------------------------------------
2020-04-18T16:19:00.3051886Z stderr:
2020-04-18T16:19:00.3051886Z stderr:
2020-04-18T16:19:00.3052229Z ------------------------------------------
2020-04-18T16:19:00.3052838Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3053594Z 
2020-04-18T16:19:00.3053899Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3054071Z 
2020-04-18T16:19:00.3054277Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3054277Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3054451Z 
2020-04-18T16:19:00.3054991Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3055841Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3056049Z 
2020-04-18T16:19:00.3056049Z 
2020-04-18T16:19:00.3057163Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3058202Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3058481Z   left: `LLVMing`,
2020-04-18T16:19:00.3058481Z   left: `LLVMing`,
2020-04-18T16:19:00.3058915Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3059298Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3059593Z 
2020-04-18T16:19:00.3059785Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3059957Z 
2020-04-18T16:19:00.3059957Z 
2020-04-18T16:19:00.3060751Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3061511Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3061737Z 
2020-04-18T16:19:00.3061737Z 
2020-04-18T16:19:00.3062541Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3063072Z 
2020-04-18T16:19:00.3063417Z ------------------------------------------
2020-04-18T16:19:00.3063705Z 
2020-04-18T16:19:00.3063790Z 
2020-04-18T16:19:00.3063790Z 
2020-04-18T16:19:00.3064187Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2020-04-18T16:19:00.3064370Z 
2020-04-18T16:19:00.3064801Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3065064Z status: exit code: 101
2020-04-18T16:19:00.3067295Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2020-04-18T16:19:00.3069235Z ------------------------------------------
2020-04-18T16:19:00.3069382Z 
2020-04-18T16:19:00.3069713Z ------------------------------------------
2020-04-18T16:19:00.3069910Z stderr:
2020-04-18T16:19:00.3069910Z stderr:
2020-04-18T16:19:00.3070259Z ------------------------------------------
2020-04-18T16:19:00.3070868Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3071523Z 
2020-04-18T16:19:00.3071713Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3071885Z 
2020-04-18T16:19:00.3072091Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3072091Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3072263Z 
2020-04-18T16:19:00.3072897Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3073816Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3074031Z 
2020-04-18T16:19:00.3074031Z 
2020-04-18T16:19:00.3074830Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3075875Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3076163Z   left: `LLVMing`,
2020-04-18T16:19:00.3076163Z   left: `LLVMing`,
2020-04-18T16:19:00.3076599Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3076987Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3077182Z 
2020-04-18T16:19:00.3077380Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3077559Z 
2020-04-18T16:19:00.3077559Z 
2020-04-18T16:19:00.3078129Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3078823Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3079037Z 
2020-04-18T16:19:00.3079037Z 
2020-04-18T16:19:00.3080022Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3080556Z 
2020-04-18T16:19:00.3080922Z ------------------------------------------
2020-04-18T16:19:00.3081083Z 
2020-04-18T16:19:00.3081166Z 
2020-04-18T16:19:00.3081166Z 
2020-04-18T16:19:00.3081553Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2020-04-18T16:19:00.3081735Z 
2020-04-18T16:19:00.3082167Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3082428Z status: exit code: 101
2020-04-18T16:19:00.3084629Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2020-04-18T16:19:00.3086359Z ------------------------------------------
2020-04-18T16:19:00.3086508Z 
2020-04-18T16:19:00.3086845Z ------------------------------------------
2020-04-18T16:19:00.3087022Z stderr:
2020-04-18T16:19:00.3087022Z stderr:
2020-04-18T16:19:00.3087378Z ------------------------------------------
2020-04-18T16:19:00.3087984Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3088637Z 
2020-04-18T16:19:00.3088827Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3088998Z 
2020-04-18T16:19:00.3089203Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3089203Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3089376Z 
2020-04-18T16:19:00.3089917Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3090609Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3090820Z 
2020-04-18T16:19:00.3090820Z 
2020-04-18T16:19:00.3097630Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3098722Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3098991Z   left: `LLVMing`,
2020-04-18T16:19:00.3098991Z   left: `LLVMing`,
2020-04-18T16:19:00.3099585Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3100126Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3100297Z 
2020-04-18T16:19:00.3100503Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3100683Z 
2020-04-18T16:19:00.3100683Z 
2020-04-18T16:19:00.3101319Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3102038Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3102254Z 
2020-04-18T16:19:00.3102254Z 
2020-04-18T16:19:00.3103180Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3103698Z 
2020-04-18T16:19:00.3104035Z ------------------------------------------
2020-04-18T16:19:00.3104293Z 
2020-04-18T16:19:00.3104511Z 
2020-04-18T16:19:00.3104511Z 
2020-04-18T16:19:00.3104942Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2020-04-18T16:19:00.3105122Z 
2020-04-18T16:19:00.3105569Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3105849Z status: exit code: 101
2020-04-18T16:19:00.3108055Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2020-04-18T16:19:00.3109795Z ------------------------------------------
2020-04-18T16:19:00.3109970Z 
2020-04-18T16:19:00.3110317Z ------------------------------------------
2020-04-18T16:19:00.3110501Z stderr:
2020-04-18T16:19:00.3110501Z stderr:
2020-04-18T16:19:00.3110871Z ------------------------------------------
2020-04-18T16:19:00.3111510Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3112183Z 
2020-04-18T16:19:00.3112382Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3112560Z 
2020-04-18T16:19:00.3112758Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3112758Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3112953Z 
2020-04-18T16:19:00.3113520Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3114615Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3114840Z 
2020-04-18T16:19:00.3114840Z 
2020-04-18T16:19:00.3115642Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3116543Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3116813Z   left: `LLVMing`,
2020-04-18T16:19:00.3116813Z   left: `LLVMing`,
2020-04-18T16:19:00.3117263Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3117659Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3117838Z 
2020-04-18T16:19:00.3118052Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3118231Z 
2020-04-18T16:19:00.3118231Z 
2020-04-18T16:19:00.3118799Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3119676Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3119891Z 
2020-04-18T16:19:00.3119891Z 
2020-04-18T16:19:00.3120681Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3121418Z 
2020-04-18T16:19:00.3121807Z ------------------------------------------
2020-04-18T16:19:00.3121957Z 
2020-04-18T16:19:00.3122063Z 
2020-04-18T16:19:00.3122063Z 
2020-04-18T16:19:00.3122481Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2020-04-18T16:19:00.3122676Z 
2020-04-18T16:19:00.3123108Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3123396Z status: exit code: 101
2020-04-18T16:19:00.3144169Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2020-04-18T16:19:00.3146379Z ------------------------------------------
2020-04-18T16:19:00.3147770Z 
2020-04-18T16:19:00.3148293Z ------------------------------------------
2020-04-18T16:19:00.3148488Z stderr:
2020-04-18T16:19:00.3148488Z stderr:
2020-04-18T16:19:00.3148881Z ------------------------------------------
2020-04-18T16:19:00.3149530Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3151022Z 
2020-04-18T16:19:00.3151239Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3151418Z 
2020-04-18T16:19:00.3151617Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3151617Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3151812Z 
2020-04-18T16:19:00.3152502Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3153230Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3153446Z 
2020-04-18T16:19:00.3153446Z 
2020-04-18T16:19:00.3154506Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3155467Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3155740Z   left: `LLVMing`,
2020-04-18T16:19:00.3155740Z   left: `LLVMing`,
2020-04-18T16:19:00.3156196Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3156592Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3156771Z 
2020-04-18T16:19:00.3156984Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3157164Z 
2020-04-18T16:19:00.3157164Z 
2020-04-18T16:19:00.3157738Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3158473Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3158813Z 
2020-04-18T16:19:00.3158813Z 
2020-04-18T16:19:00.3159596Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3160294Z 
2020-04-18T16:19:00.3160677Z ------------------------------------------
2020-04-18T16:19:00.3160831Z 
2020-04-18T16:19:00.3160933Z 
2020-04-18T16:19:00.3160933Z 
2020-04-18T16:19:00.3161452Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2020-04-18T16:19:00.3161642Z 
2020-04-18T16:19:00.3162718Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3163015Z status: exit code: 101
2020-04-18T16:19:00.3165514Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2020-04-18T16:19:00.3167437Z ------------------------------------------
2020-04-18T16:19:00.3167610Z 
2020-04-18T16:19:00.3167956Z ------------------------------------------
2020-04-18T16:19:00.3168139Z stderr:
2020-04-18T16:19:00.3168139Z stderr:
2020-04-18T16:19:00.3168504Z ------------------------------------------
2020-04-18T16:19:00.3169359Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3170049Z 
2020-04-18T16:19:00.3170247Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3170426Z 
2020-04-18T16:19:00.3170624Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3170624Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3170819Z 
2020-04-18T16:19:00.3171528Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3172239Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3172456Z 
2020-04-18T16:19:00.3172456Z 
2020-04-18T16:19:00.3173354Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3174257Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3174535Z   left: `LLVMing`,
2020-04-18T16:19:00.3174535Z   left: `LLVMing`,
2020-04-18T16:19:00.3174976Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3175387Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3175565Z 
2020-04-18T16:19:00.3175778Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3175959Z 
2020-04-18T16:19:00.3175959Z 
2020-04-18T16:19:00.3176637Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3177582Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3177800Z 
2020-04-18T16:19:00.3177800Z 
2020-04-18T16:19:00.3178576Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3179228Z 
2020-04-18T16:19:00.3179585Z ------------------------------------------
2020-04-18T16:19:00.3179886Z 
2020-04-18T16:19:00.3179974Z 
2020-04-18T16:19:00.3179974Z 
2020-04-18T16:19:00.3180433Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2020-04-18T16:19:00.3180622Z 
2020-04-18T16:19:00.3181050Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3181338Z status: exit code: 101
2020-04-18T16:19:00.3183663Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2020-04-18T16:19:00.3185704Z ------------------------------------------
2020-04-18T16:19:00.3185860Z 
2020-04-18T16:19:00.3186226Z ------------------------------------------
2020-04-18T16:19:00.3186409Z stderr:
2020-04-18T16:19:00.3186409Z stderr:
2020-04-18T16:19:00.3186760Z ------------------------------------------
2020-04-18T16:19:00.3187409Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3188278Z 
2020-04-18T16:19:00.3188484Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3188655Z 
2020-04-18T16:19:00.3188845Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3188845Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3189032Z 
2020-04-18T16:19:00.3189621Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3190523Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3190762Z 
2020-04-18T16:19:00.3190762Z 
2020-04-18T16:19:00.3191543Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3192540Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3192801Z   left: `LLVMing`,
2020-04-18T16:19:00.3192801Z   left: `LLVMing`,
2020-04-18T16:19:00.3193213Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3193608Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3193779Z 
2020-04-18T16:19:00.3193978Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3194170Z 
2020-04-18T16:19:00.3194170Z 
2020-04-18T16:19:00.3194712Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3195681Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3195903Z 
2020-04-18T16:19:00.3195903Z 
2020-04-18T16:19:00.3196727Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3197291Z 
2020-04-18T16:19:00.3197648Z ------------------------------------------
2020-04-18T16:19:00.3197798Z 
2020-04-18T16:19:00.3197886Z 
2020-04-18T16:19:00.3197886Z 
2020-04-18T16:19:00.3198511Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2020-04-18T16:19:00.3198680Z 
2020-04-18T16:19:00.3199089Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3199601Z status: exit code: 101
2020-04-18T16:19:00.3202200Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2020-04-18T16:19:00.3203969Z ------------------------------------------
2020-04-18T16:19:00.3204199Z 
2020-04-18T16:19:00.3204578Z ------------------------------------------
2020-04-18T16:19:00.3204739Z stderr:
2020-04-18T16:19:00.3204739Z stderr:
2020-04-18T16:19:00.3205187Z ------------------------------------------
2020-04-18T16:19:00.3205917Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3206684Z 
2020-04-18T16:19:00.3206896Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3207074Z 
2020-04-18T16:19:00.3207270Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3207270Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3207610Z 
2020-04-18T16:19:00.3208237Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3208937Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3209172Z 
2020-04-18T16:19:00.3209172Z 
2020-04-18T16:19:00.3209981Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3210887Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3211155Z   left: `LLVMing`,
2020-04-18T16:19:00.3211155Z   left: `LLVMing`,
2020-04-18T16:19:00.3211583Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3211994Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3212172Z 
2020-04-18T16:19:00.3212369Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3212548Z 
2020-04-18T16:19:00.3212548Z 
2020-04-18T16:19:00.3213114Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3213819Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3214044Z 
2020-04-18T16:19:00.3214044Z 
2020-04-18T16:19:00.3214952Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3215604Z 
2020-04-18T16:19:00.3216092Z ------------------------------------------
2020-04-18T16:19:00.3216256Z 
2020-04-18T16:19:00.3216344Z 
2020-04-18T16:19:00.3216344Z 
2020-04-18T16:19:00.3217244Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2020-04-18T16:19:00.3217452Z 
2020-04-18T16:19:00.3217931Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3218203Z status: exit code: 101
2020-04-18T16:19:00.3220908Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
2020-04-18T16:19:00.3223077Z ------------------------------------------
2020-04-18T16:19:00.3223241Z 
2020-04-18T16:19:00.3223629Z ------------------------------------------
2020-04-18T16:19:00.3223816Z stderr:
2020-04-18T16:19:00.3223816Z stderr:
2020-04-18T16:19:00.3224173Z ------------------------------------------
2020-04-18T16:19:00.3224931Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3225709Z 
2020-04-18T16:19:00.3225913Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3226086Z 
2020-04-18T16:19:00.3226277Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3226277Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3226449Z 
2020-04-18T16:19:00.3227052Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3227740Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3227958Z 
2020-04-18T16:19:00.3227958Z 
2020-04-18T16:19:00.3228730Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3229924Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3230201Z   left: `LLVMing`,
2020-04-18T16:19:00.3230201Z   left: `LLVMing`,
2020-04-18T16:19:00.3230670Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3231065Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3231241Z 
2020-04-18T16:19:00.3231438Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3231616Z 
2020-04-18T16:19:00.3231616Z 
2020-04-18T16:19:00.3232190Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3233000Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3233232Z 
2020-04-18T16:19:00.3233232Z 
2020-04-18T16:19:00.3234215Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3234859Z 
2020-04-18T16:19:00.3235228Z ------------------------------------------
2020-04-18T16:19:00.3235380Z 
2020-04-18T16:19:00.3235467Z 
2020-04-18T16:19:00.3235467Z 
2020-04-18T16:19:00.3235868Z ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
2020-04-18T16:19:00.3236073Z 
2020-04-18T16:19:00.3236502Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3236782Z status: exit code: 101
2020-04-18T16:19:00.3239148Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
2020-04-18T16:19:00.3241133Z ------------------------------------------
2020-04-18T16:19:00.3241276Z 
2020-04-18T16:19:00.3241610Z ------------------------------------------
2020-04-18T16:19:00.3241781Z stderr:
2020-04-18T16:19:00.3241781Z stderr:
2020-04-18T16:19:00.3242108Z ------------------------------------------
2020-04-18T16:19:00.3242693Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3243320Z 
2020-04-18T16:19:00.3243504Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3243809Z 
2020-04-18T16:19:00.3244082Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3244082Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3244270Z 
2020-04-18T16:19:00.3244978Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3245683Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3245898Z 
2020-04-18T16:19:00.3245898Z 
2020-04-18T16:19:00.3246695Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3247689Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T16:19:00.3248063Z   left: `LLVMing`,
2020-04-18T16:19:00.3248063Z   left: `LLVMing`,
2020-04-18T16:19:00.3248459Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T16:19:00.3248827Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3248993Z 
2020-04-18T16:19:00.3249174Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3249454Z 
2020-04-18T16:19:00.3249454Z 
2020-04-18T16:19:00.3250004Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3250677Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3250907Z 
2020-04-18T16:19:00.3250907Z 
2020-04-18T16:19:00.3251799Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T16:19:00.3252323Z 
2020-04-18T16:19:00.3252789Z ------------------------------------------
2020-04-18T16:19:00.3252941Z 
2020-04-18T16:19:00.3253026Z 
2020-04-18T16:19:00.3253026Z 
2020-04-18T16:19:00.3253418Z ---- [incremental] incremental/hashes/match_expressions.rs stdout ----
2020-04-18T16:19:00.3253616Z 
2020-04-18T16:19:00.3254161Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T16:19:00.3254435Z status: exit code: 101
2020-04-18T16:19:00.3256925Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
2020-04-18T16:19:00.3258926Z ------------------------------------------
2020-04-18T16:19:00.3259080Z 
2020-04-18T16:19:00.3259423Z ------------------------------------------
2020-04-18T16:19:00.3259625Z stderr:
2020-04-18T16:19:00.3259625Z stderr:
2020-04-18T16:19:00.3259976Z ------------------------------------------
2020-04-18T16:19:00.3260606Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T16:19:00.3261287Z 
2020-04-18T16:19:00.3261672Z error: internal compiler error: unexpected panic
2020-04-18T16:19:00.3261870Z 
2020-04-18T16:19:00.3262070Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3262070Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T16:19:00.3262249Z 
2020-04-18T16:19:00.3262874Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T16:19:00.3263681Z note: rustc 1.44.0-nightly (3ae416b36 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T16:19:00.3263914Z 
2020-04-18T16:19:00.3263914Z 
---
2020-04-18T16:19:00.3446087Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T16:19:00.3446459Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T16:19:00.3446660Z 
2020-04-18T16:19:00.3446746Z 
2020-04-18T16:19:00.3450247Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T16:19:00.3452657Z 
2020-04-18T16:19:00.3452751Z 
2020-04-18T16:19:00.3453238Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T16:19:00.3453572Z Build completed unsuccessfully in 1:05:57
2020-04-18T16:19:00.3453572Z Build completed unsuccessfully in 1:05:57
2020-04-18T16:19:00.3453799Z == clock drift check ==
2020-04-18T16:19:00.3454041Z   local time: Sat Apr 18 16:19:00 UTC 2020
2020-04-18T16:19:00.5086732Z   network time: Sat, 18 Apr 2020 16:19:00 GMT
2020-04-18T16:19:03.2410021Z 
2020-04-18T16:19:03.2410021Z 
2020-04-18T16:19:03.2497632Z ##[error]Bash exited with code '1'.
2020-04-18T16:19:03.2515113Z ##[section]Finishing: Run build
2020-04-18T16:19:03.2569661Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T16:19:03.2575247Z Task         : Get sources
2020-04-18T16:19:03.2575604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T16:19:03.2575946Z Version      : 1.0.0
2020-04-18T16:19:03.2576178Z Author       : Microsoft
2020-04-18T16:19:03.2576178Z Author       : Microsoft
2020-04-18T16:19:03.2576542Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T16:19:03.2577277Z ==============================================================================
2020-04-18T16:19:03.6374441Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T16:19:03.6424518Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T16:19:03.6531258Z Cleaning up task key
2020-04-18T16:19:03.6532717Z Start cleaning up orphan processes.
2020-04-18T16:19:03.6759071Z Terminate orphan process: pid (3917) (python)
2020-04-18T16:19:03.6955636Z ##[section]Finishing: Finalize Job
