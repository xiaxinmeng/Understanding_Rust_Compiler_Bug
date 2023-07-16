plain
2020-04-18T16:36:55.5270754Z ========================== Starting Command Output ===========================
2020-04-18T16:36:55.5272926Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/50d7f9b8-0815-4668-93dd-8b96c6b90cd0.sh
2020-04-18T16:36:55.5273172Z 
2020-04-18T16:36:55.5276824Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T16:36:55.5298118Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T16:36:55.5301223Z Task         : Get sources
2020-04-18T16:36:55.5301498Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T16:36:55.5301782Z Version      : 1.0.0
2020-04-18T16:36:55.5301966Z Author       : Microsoft
---
2020-04-18T16:36:56.8320353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T16:36:56.8338588Z ##[command]git config gc.auto 0
2020-04-18T16:36:56.8343992Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T16:36:56.8348869Z ##[command]git config --get-all http.proxy
2020-04-18T16:36:56.8357558Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-18T16:39:16.2821345Z  ---> 318032b5f0e2
2020-04-18T16:39:16.2822102Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T16:39:16.2826740Z  ---> Using cache
2020-04-18T16:39:16.2827087Z  ---> d44a858fd1ce
2020-04-18T16:39:16.2827928Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T16:39:16.2832726Z  ---> 58b910f50f5a
2020-04-18T16:39:16.2832939Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T16:39:16.2836799Z  ---> Using cache
2020-04-18T16:39:16.2837138Z  ---> ee7702aadba1
---
2020-04-18T16:39:16.3226863Z Looks like docker image is the same as before, not uploading
2020-04-18T16:39:22.2526263Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:39:22.2765337Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:39:22.2791417Z == clock drift check ==
2020-04-18T16:39:22.2802112Z   local time: Sat Apr 18 16:39:22 UTC 2020
2020-04-18T16:39:22.3723351Z   network time: Sat, 18 Apr 2020 16:39:22 GMT
2020-04-18T16:39:22.3760175Z Starting sccache server...
2020-04-18T16:39:22.4513074Z configure: processing command line
2020-04-18T16:39:22.4513596Z configure: 
2020-04-18T16:39:22.4514600Z configure: rust.dist-src        := False
---
2020-04-18T16:44:08.7307725Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T16:44:10.0355327Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T16:44:11.4944550Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T16:44:12.5681404Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T16:44:20.4682394Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T16:44:22.8271330Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T16:44:26.8253790Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T16:44:30.6870909Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T16:44:39.2808899Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T17:04:41.8210710Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T17:04:43.2538002Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T17:04:44.9154765Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T17:04:44.9453909Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T17:04:54.5877572Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T17:04:56.2896282Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T17:05:00.5863700Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T17:05:04.5930048Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T17:05:14.4292198Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T17:25:49.2802007Z .................................................................................................... 1700/9912
2020-04-18T17:25:53.2138191Z .................................................................................................... 1800/9912
2020-04-18T17:26:00.9035611Z .................................................................................................... 1900/9912
2020-04-18T17:26:08.3466085Z ....i............................................................................................... 2000/9912
2020-04-18T17:26:14.2493004Z ..............................................................................................iiiii. 2100/9912
2020-04-18T17:26:31.8355368Z .................................................................................................... 2300/9912
2020-04-18T17:26:33.7969792Z .................................................................................................... 2400/9912
2020-04-18T17:26:35.7896332Z .................................................................................................... 2500/9912
2020-04-18T17:26:40.9537614Z .................................................................................................... 2600/9912
---
2020-04-18T17:29:24.1157728Z .................................................................................................... 5100/9912
2020-04-18T17:29:30.7569978Z .................................................................................................... 5200/9912
2020-04-18T17:29:35.2730588Z ................i................................................................................... 5300/9912
2020-04-18T17:29:44.3005806Z ......i............................................................................................. 5400/9912
2020-04-18T17:29:49.0901541Z ......ii.ii........i...i............................................................................ 5500/9912
2020-04-18T17:29:56.2647241Z ....................................................i............................................... 5700/9912
2020-04-18T17:30:04.5058869Z ....................................................................................ii.............. 5800/9912
2020-04-18T17:30:10.9269391Z .......................i............................................................................ 5900/9912
2020-04-18T17:30:15.8368370Z .................................................................................................... 6000/9912
2020-04-18T17:30:15.8368370Z .................................................................................................... 6000/9912
2020-04-18T17:30:25.3899047Z .................................................................................................... 6100/9912
2020-04-18T17:30:34.4213623Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-18T17:30:47.0713915Z .................................................................................................... 6400/9912
2020-04-18T17:30:50.1907826Z .................................................................................................... 6500/9912
2020-04-18T17:30:50.1907826Z .................................................................................................... 6500/9912
2020-04-18T17:30:58.7165430Z ...............................................i..ii................................................ 6600/9912
2020-04-18T17:31:19.2027145Z .................................................................................................... 6800/9912
2020-04-18T17:31:21.2236138Z ................................................i................................................... 6900/9912
2020-04-18T17:31:23.0817824Z .................................................................................................... 7000/9912
2020-04-18T17:31:24.9727151Z ........................................................................................i........... 7100/9912
---
2020-04-18T17:32:50.3842468Z .................................................................................................... 7900/9912
2020-04-18T17:32:56.0648195Z .................................................................................................... 8000/9912
2020-04-18T17:33:01.1162974Z ......................................................i............................................. 8100/9912
2020-04-18T17:33:09.9239908Z .................................................................................................... 8200/9912
2020-04-18T17:33:14.6068932Z ....iiiiiiiiiii.i................................................................................... 8300/9912
2020-04-18T17:33:26.4056349Z .................................................................................................... 8500/9912
2020-04-18T17:33:33.3945496Z .................................................................................................... 8600/9912
2020-04-18T17:33:45.1622205Z .................................................................................................... 8700/9912
2020-04-18T17:33:51.0121331Z .................................................................................................... 8800/9912
---
2020-04-18T17:35:50.5735536Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T17:35:50.5921341Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:35:50.7886215Z 
2020-04-18T17:35:50.7887670Z running 186 tests
2020-04-18T17:35:53.4053017Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-18T17:35:55.7826524Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-18T17:35:55.7828775Z 
2020-04-18T17:35:55.7832565Z  finished in 5.191
2020-04-18T17:35:55.7838367Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-18T17:35:55.8021418Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T17:35:57.7227889Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-18T17:35:57.7408108Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:35:57.8866046Z 
2020-04-18T17:35:57.8866949Z running 9 tests
2020-04-18T17:35:57.8868387Z iiiiiiiii
2020-04-18T17:35:57.8871712Z 
2020-04-18T17:35:57.8872183Z  finished in 0.146
2020-04-18T17:35:57.8878294Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-18T17:35:57.9057967Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:35:57.9057967Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:35:58.0781739Z 
2020-04-18T17:35:58.0782551Z running 119 tests
2020-04-18T17:36:12.9932067Z ............................F.F.FF..FFF.FFFFFF.F....FFF.........F..............................F.... 100/119
2020-04-18T17:36:15.5356309Z failures:
2020-04-18T17:36:15.5386313Z 
2020-04-18T17:36:15.5387431Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-04-18T17:36:15.5387742Z 
2020-04-18T17:36:15.5387742Z 
2020-04-18T17:36:15.5388317Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5388688Z status: exit code: 101
2020-04-18T17:36:15.5390969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2020-04-18T17:36:15.5394853Z ------------------------------------------
2020-04-18T17:36:15.5395019Z 
2020-04-18T17:36:15.5395344Z ------------------------------------------
2020-04-18T17:36:15.5395521Z stderr:
2020-04-18T17:36:15.5395521Z stderr:
2020-04-18T17:36:15.5395866Z ------------------------------------------
2020-04-18T17:36:15.5396472Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5397100Z 
2020-04-18T17:36:15.5397281Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5397443Z 
2020-04-18T17:36:15.5399463Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5399463Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5399654Z 
2020-04-18T17:36:15.5400325Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5400996Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5401201Z 
2020-04-18T17:36:15.5401201Z 
2020-04-18T17:36:15.5402512Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5404281Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5404854Z   left: `LLVMing`,
2020-04-18T17:36:15.5404854Z   left: `LLVMing`,
2020-04-18T17:36:15.5405577Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5406312Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5406952Z 
2020-04-18T17:36:15.5407172Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5407347Z 
2020-04-18T17:36:15.5407347Z 
2020-04-18T17:36:15.5407980Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5408702Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5409583Z 
2020-04-18T17:36:15.5409583Z 
2020-04-18T17:36:15.5411016Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5411579Z 
2020-04-18T17:36:15.5411943Z ------------------------------------------
2020-04-18T17:36:15.5412099Z 
2020-04-18T17:36:15.5412214Z 
2020-04-18T17:36:15.5412214Z 
2020-04-18T17:36:15.5412635Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2020-04-18T17:36:15.5412833Z 
2020-04-18T17:36:15.5413264Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5413558Z status: exit code: 101
2020-04-18T17:36:15.5415847Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2020-04-18T17:36:15.5417608Z ------------------------------------------
2020-04-18T17:36:15.5417780Z 
2020-04-18T17:36:15.5418126Z ------------------------------------------
2020-04-18T17:36:15.5418315Z stderr:
2020-04-18T17:36:15.5418315Z stderr:
2020-04-18T17:36:15.5418684Z ------------------------------------------
2020-04-18T17:36:15.5419323Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5419997Z 
2020-04-18T17:36:15.5420192Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5420368Z 
2020-04-18T17:36:15.5420564Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5420564Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5420756Z 
2020-04-18T17:36:15.5421326Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5422038Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5422259Z 
2020-04-18T17:36:15.5422259Z 
2020-04-18T17:36:15.5423079Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5423965Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5424232Z   left: `LLVMing`,
2020-04-18T17:36:15.5424232Z   left: `LLVMing`,
2020-04-18T17:36:15.5424673Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5425200Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5425375Z 
2020-04-18T17:36:15.5425585Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5425822Z 
2020-04-18T17:36:15.5425822Z 
2020-04-18T17:36:15.5426392Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5427102Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5427321Z 
2020-04-18T17:36:15.5427321Z 
2020-04-18T17:36:15.5428106Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5428645Z 
2020-04-18T17:36:15.5428986Z ------------------------------------------
2020-04-18T17:36:15.5429140Z 
2020-04-18T17:36:15.5429235Z 
2020-04-18T17:36:15.5429235Z 
2020-04-18T17:36:15.5429658Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2020-04-18T17:36:15.5429854Z 
2020-04-18T17:36:15.5430282Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5430578Z status: exit code: 101
2020-04-18T17:36:15.5432843Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2020-04-18T17:36:15.5434575Z ------------------------------------------
2020-04-18T17:36:15.5434752Z 
2020-04-18T17:36:15.5435097Z ------------------------------------------
2020-04-18T17:36:15.5435284Z stderr:
2020-04-18T17:36:15.5435284Z stderr:
2020-04-18T17:36:15.5435637Z ------------------------------------------
2020-04-18T17:36:15.5436291Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5436942Z 
2020-04-18T17:36:15.5437155Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5437329Z 
2020-04-18T17:36:15.5437523Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5437523Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5437699Z 
2020-04-18T17:36:15.5438264Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5438981Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5439205Z 
2020-04-18T17:36:15.5439205Z 
2020-04-18T17:36:15.5439998Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5440879Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5441145Z   left: `LLVMing`,
2020-04-18T17:36:15.5441145Z   left: `LLVMing`,
2020-04-18T17:36:15.5441573Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5441977Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5442153Z 
2020-04-18T17:36:15.5442348Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5442539Z 
2020-04-18T17:36:15.5442539Z 
2020-04-18T17:36:15.5443160Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5443870Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5444138Z 
2020-04-18T17:36:15.5444138Z 
2020-04-18T17:36:15.5444935Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5445474Z 
2020-04-18T17:36:15.5445813Z ------------------------------------------
2020-04-18T17:36:15.5445967Z 
2020-04-18T17:36:15.5446057Z 
2020-04-18T17:36:15.5446057Z 
2020-04-18T17:36:15.5447646Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2020-04-18T17:36:15.5447853Z 
2020-04-18T17:36:15.5448681Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5449124Z status: exit code: 101
2020-04-18T17:36:15.5451415Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2020-04-18T17:36:15.5453132Z ------------------------------------------
2020-04-18T17:36:15.5453290Z 
2020-04-18T17:36:15.5453651Z ------------------------------------------
2020-04-18T17:36:15.5453843Z stderr:
2020-04-18T17:36:15.5453843Z stderr:
2020-04-18T17:36:15.5454198Z ------------------------------------------
2020-04-18T17:36:15.5454851Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5455500Z 
2020-04-18T17:36:15.5465615Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5465811Z 
2020-04-18T17:36:15.5466011Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5466011Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5466189Z 
2020-04-18T17:36:15.5466965Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5467737Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5467960Z 
2020-04-18T17:36:15.5467960Z 
2020-04-18T17:36:15.5468694Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5471259Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5471509Z   left: `LLVMing`,
2020-04-18T17:36:15.5471509Z   left: `LLVMing`,
2020-04-18T17:36:15.5471908Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5472285Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5472448Z 
2020-04-18T17:36:15.5472866Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5473049Z 
2020-04-18T17:36:15.5473049Z 
2020-04-18T17:36:15.5474273Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5474942Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5475147Z 
2020-04-18T17:36:15.5475147Z 
2020-04-18T17:36:15.5475878Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5476576Z 
2020-04-18T17:36:15.5477097Z ------------------------------------------
2020-04-18T17:36:15.5477244Z 
2020-04-18T17:36:15.5477328Z 
2020-04-18T17:36:15.5477328Z 
2020-04-18T17:36:15.5477705Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2020-04-18T17:36:15.5477877Z 
2020-04-18T17:36:15.5478276Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5478533Z status: exit code: 101
2020-04-18T17:36:15.5481231Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2020-04-18T17:36:15.5483230Z ------------------------------------------
2020-04-18T17:36:15.5483622Z 
2020-04-18T17:36:15.5484030Z ------------------------------------------
2020-04-18T17:36:15.5484350Z stderr:
2020-04-18T17:36:15.5484350Z stderr:
2020-04-18T17:36:15.5484738Z ------------------------------------------
2020-04-18T17:36:15.5485538Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5492607Z 
2020-04-18T17:36:15.5492866Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5493046Z 
2020-04-18T17:36:15.5493264Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5493264Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5493443Z 
2020-04-18T17:36:15.5494274Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5494999Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5495238Z 
2020-04-18T17:36:15.5495238Z 
2020-04-18T17:36:15.5496035Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5496918Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5497185Z   left: `LLVMing`,
2020-04-18T17:36:15.5497185Z   left: `LLVMing`,
2020-04-18T17:36:15.5497624Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5498141Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5498305Z 
2020-04-18T17:36:15.5498490Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5509730Z 
2020-04-18T17:36:15.5509730Z 
2020-04-18T17:36:15.5510742Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5511536Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5511759Z 
2020-04-18T17:36:15.5511759Z 
2020-04-18T17:36:15.5512560Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5513105Z 
2020-04-18T17:36:15.5513454Z ------------------------------------------
2020-04-18T17:36:15.5513793Z 
2020-04-18T17:36:15.5513884Z 
2020-04-18T17:36:15.5513884Z 
2020-04-18T17:36:15.5514324Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2020-04-18T17:36:15.5514524Z 
2020-04-18T17:36:15.5515023Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5515307Z status: exit code: 101
2020-04-18T17:36:15.5517622Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2020-04-18T17:36:15.5519375Z ------------------------------------------
2020-04-18T17:36:15.5519532Z 
2020-04-18T17:36:15.5520011Z ------------------------------------------
2020-04-18T17:36:15.5520200Z stderr:
2020-04-18T17:36:15.5520200Z stderr:
2020-04-18T17:36:15.5520616Z ------------------------------------------
2020-04-18T17:36:15.5521454Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5522110Z 
2020-04-18T17:36:15.5522323Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5522499Z 
2020-04-18T17:36:15.5522695Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5522695Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5522878Z 
2020-04-18T17:36:15.5523621Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5524343Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5524578Z 
2020-04-18T17:36:15.5524578Z 
2020-04-18T17:36:15.5525454Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5526366Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5526857Z   left: `LLVMing`,
2020-04-18T17:36:15.5526857Z   left: `LLVMing`,
2020-04-18T17:36:15.5527306Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5527709Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5527885Z 
2020-04-18T17:36:15.5528189Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5528359Z 
2020-04-18T17:36:15.5528359Z 
2020-04-18T17:36:15.5528896Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5529556Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5529928Z 
2020-04-18T17:36:15.5529928Z 
2020-04-18T17:36:15.5530716Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5531256Z 
2020-04-18T17:36:15.5531599Z ------------------------------------------
2020-04-18T17:36:15.5531752Z 
2020-04-18T17:36:15.5531842Z 
2020-04-18T17:36:15.5531842Z 
2020-04-18T17:36:15.5532257Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2020-04-18T17:36:15.5532447Z 
2020-04-18T17:36:15.5533069Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5533439Z status: exit code: 101
2020-04-18T17:36:15.5535754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2020-04-18T17:36:15.5537474Z ------------------------------------------
2020-04-18T17:36:15.5537636Z 
2020-04-18T17:36:15.5537992Z ------------------------------------------
2020-04-18T17:36:15.5538180Z stderr:
2020-04-18T17:36:15.5538180Z stderr:
2020-04-18T17:36:15.5538534Z ------------------------------------------
2020-04-18T17:36:15.5539183Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5539834Z 
2020-04-18T17:36:15.5540044Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5540220Z 
2020-04-18T17:36:15.5540418Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5540418Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5540595Z 
2020-04-18T17:36:15.5541162Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5541860Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5542079Z 
2020-04-18T17:36:15.5542079Z 
2020-04-18T17:36:15.5542889Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5543772Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5544036Z   left: `LLVMing`,
2020-04-18T17:36:15.5544036Z   left: `LLVMing`,
2020-04-18T17:36:15.5544464Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5544865Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5545040Z 
2020-04-18T17:36:15.5545236Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5545412Z 
2020-04-18T17:36:15.5545412Z 
2020-04-18T17:36:15.5546083Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5546722Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5546945Z 
2020-04-18T17:36:15.5546945Z 
2020-04-18T17:36:15.5547681Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5548165Z 
2020-04-18T17:36:15.5548497Z ------------------------------------------
2020-04-18T17:36:15.5548640Z 
2020-04-18T17:36:15.5548723Z 
2020-04-18T17:36:15.5548723Z 
2020-04-18T17:36:15.5549093Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2020-04-18T17:36:15.5549284Z 
2020-04-18T17:36:15.5549728Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5549986Z status: exit code: 101
2020-04-18T17:36:15.5552113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2020-04-18T17:36:15.5553732Z ------------------------------------------
2020-04-18T17:36:15.5553877Z 
2020-04-18T17:36:15.5554209Z ------------------------------------------
2020-04-18T17:36:15.5554385Z stderr:
2020-04-18T17:36:15.5554385Z stderr:
2020-04-18T17:36:15.5554713Z ------------------------------------------
2020-04-18T17:36:15.5555300Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5555925Z 
2020-04-18T17:36:15.5556103Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5556282Z 
2020-04-18T17:36:15.5556464Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5556464Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5556627Z 
2020-04-18T17:36:15.5557152Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5557795Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5557999Z 
2020-04-18T17:36:15.5557999Z 
2020-04-18T17:36:15.5558744Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5559536Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5559800Z   left: `LLVMing`,
2020-04-18T17:36:15.5559800Z   left: `LLVMing`,
2020-04-18T17:36:15.5560196Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5560573Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5560737Z 
2020-04-18T17:36:15.5560917Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5561079Z 
2020-04-18T17:36:15.5561079Z 
2020-04-18T17:36:15.5561597Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5562236Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5562454Z 
2020-04-18T17:36:15.5562454Z 
2020-04-18T17:36:15.5563375Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5563902Z 
2020-04-18T17:36:15.5564258Z ------------------------------------------
2020-04-18T17:36:15.5564411Z 
2020-04-18T17:36:15.5564499Z 
2020-04-18T17:36:15.5564499Z 
2020-04-18T17:36:15.5564887Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2020-04-18T17:36:15.5565091Z 
2020-04-18T17:36:15.5565957Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5566658Z status: exit code: 101
2020-04-18T17:36:15.5569015Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2020-04-18T17:36:15.5570856Z ------------------------------------------
2020-04-18T17:36:15.5571013Z 
2020-04-18T17:36:15.5571358Z ------------------------------------------
2020-04-18T17:36:15.5571560Z stderr:
2020-04-18T17:36:15.5571560Z stderr:
2020-04-18T17:36:15.5571914Z ------------------------------------------
2020-04-18T17:36:15.5572550Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5573216Z 
2020-04-18T17:36:15.5573411Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5573598Z 
2020-04-18T17:36:15.5573794Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5573794Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5573976Z 
2020-04-18T17:36:15.5574555Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5575255Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5575475Z 
2020-04-18T17:36:15.5575475Z 
2020-04-18T17:36:15.5576279Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5577141Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5577422Z   left: `LLVMing`,
2020-04-18T17:36:15.5577422Z   left: `LLVMing`,
2020-04-18T17:36:15.5577849Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5578250Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5578425Z 
2020-04-18T17:36:15.5578626Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5578802Z 
2020-04-18T17:36:15.5578802Z 
2020-04-18T17:36:15.5579363Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5580061Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5580281Z 
2020-04-18T17:36:15.5580281Z 
2020-04-18T17:36:15.5581084Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5581607Z 
2020-04-18T17:36:15.5581963Z ------------------------------------------
2020-04-18T17:36:15.5582118Z 
2020-04-18T17:36:15.5582209Z 
2020-04-18T17:36:15.5582209Z 
2020-04-18T17:36:15.5582611Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2020-04-18T17:36:15.5582804Z 
2020-04-18T17:36:15.5583249Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5583530Z status: exit code: 101
2020-04-18T17:36:15.5585785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
2020-04-18T17:36:15.5587574Z ------------------------------------------
2020-04-18T17:36:15.5587730Z 
2020-04-18T17:36:15.5588071Z ------------------------------------------
2020-04-18T17:36:15.5588272Z stderr:
2020-04-18T17:36:15.5588272Z stderr:
2020-04-18T17:36:15.5588630Z ------------------------------------------
2020-04-18T17:36:15.5589310Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5589981Z 
2020-04-18T17:36:15.5590175Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5590350Z 
2020-04-18T17:36:15.5590558Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5590558Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5590735Z 
2020-04-18T17:36:15.5591298Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5592007Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5592231Z 
2020-04-18T17:36:15.5592231Z 
2020-04-18T17:36:15.5593039Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5593899Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5594176Z   left: `LLVMing`,
2020-04-18T17:36:15.5594176Z   left: `LLVMing`,
2020-04-18T17:36:15.5594873Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5595255Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5595442Z 
2020-04-18T17:36:15.5595637Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5595811Z 
2020-04-18T17:36:15.5595811Z 
2020-04-18T17:36:15.5596372Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5597064Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5597288Z 
2020-04-18T17:36:15.5597288Z 
2020-04-18T17:36:15.5598097Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5598617Z 
2020-04-18T17:36:15.5598958Z ------------------------------------------
2020-04-18T17:36:15.5599131Z 
2020-04-18T17:36:15.5599220Z 
2020-04-18T17:36:15.5599220Z 
2020-04-18T17:36:15.5599625Z ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
2020-04-18T17:36:15.5599819Z 
2020-04-18T17:36:15.5600267Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5600542Z status: exit code: 101
2020-04-18T17:36:15.5602786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
2020-04-18T17:36:15.5604635Z ------------------------------------------
2020-04-18T17:36:15.5604791Z 
2020-04-18T17:36:15.5605131Z ------------------------------------------
2020-04-18T17:36:15.5605331Z stderr:
2020-04-18T17:36:15.5605331Z stderr:
2020-04-18T17:36:15.5605685Z ------------------------------------------
2020-04-18T17:36:15.5606316Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5607702Z 
2020-04-18T17:36:15.5607975Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5608142Z 
2020-04-18T17:36:15.5608336Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5608336Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5608499Z 
2020-04-18T17:36:15.5609051Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5609710Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5609914Z 
2020-04-18T17:36:15.5609914Z 
2020-04-18T17:36:15.5610643Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5611460Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T17:36:15.5611721Z   left: `LLVMing`,
2020-04-18T17:36:15.5611721Z   left: `LLVMing`,
2020-04-18T17:36:15.5612120Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T17:36:15.5612483Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5612659Z 
2020-04-18T17:36:15.5612839Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5613001Z 
2020-04-18T17:36:15.5613001Z 
2020-04-18T17:36:15.5613521Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5614163Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5614366Z 
2020-04-18T17:36:15.5614366Z 
2020-04-18T17:36:15.5615106Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T17:36:15.5615593Z 
2020-04-18T17:36:15.5616007Z ------------------------------------------
2020-04-18T17:36:15.5616165Z 
2020-04-18T17:36:15.5616248Z 
2020-04-18T17:36:15.5616248Z 
2020-04-18T17:36:15.5616630Z ---- [incremental] incremental/hashes/match_expressions.rs stdout ----
2020-04-18T17:36:15.5616812Z 
2020-04-18T17:36:15.5617222Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T17:36:15.5617478Z status: exit code: 101
2020-04-18T17:36:15.5619786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
2020-04-18T17:36:15.5621534Z ------------------------------------------
2020-04-18T17:36:15.5621691Z 
2020-04-18T17:36:15.5622033Z ------------------------------------------
2020-04-18T17:36:15.5622221Z stderr:
2020-04-18T17:36:15.5622221Z stderr:
2020-04-18T17:36:15.5622591Z ------------------------------------------
2020-04-18T17:36:15.5623224Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T17:36:15.5623887Z 
2020-04-18T17:36:15.5624082Z error: internal compiler error: unexpected panic
2020-04-18T17:36:15.5624323Z 
2020-04-18T17:36:15.5624533Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5624533Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T17:36:15.5624711Z 
2020-04-18T17:36:15.5625324Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T17:36:15.5626047Z note: rustc 1.44.0-nightly (a98ebef39 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T17:36:15.5626266Z 
2020-04-18T17:36:15.5626266Z 
---
2020-04-18T17:36:15.5768533Z test result: FAILED. 99 passed; 20 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-18T17:36:15.5768783Z 
2020-04-18T17:36:15.5768872Z 
2020-04-18T17:36:15.5768961Z 
2020-04-18T17:36:15.5772381Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T17:36:15.5774698Z 
2020-04-18T17:36:15.5774789Z 
2020-04-18T17:36:15.5775255Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T17:36:15.5775635Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T17:36:15.5775635Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T17:36:15.5776215Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T17:36:15.5776558Z Build completed unsuccessfully in 0:55:22
2020-04-18T17:36:15.5776780Z == clock drift check ==
2020-04-18T17:36:15.5777005Z   local time: Sat Apr 18 17:36:15 UTC 2020
2020-04-18T17:36:15.6388423Z   network time: Sat, 18 Apr 2020 17:36:15 GMT
2020-04-18T17:36:18.4951295Z 
2020-04-18T17:36:18.4951295Z 
2020-04-18T17:36:18.5012786Z ##[error]Bash exited with code '1'.
2020-04-18T17:36:18.5025777Z ##[section]Finishing: Run build
2020-04-18T17:36:18.5071482Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T17:36:18.5076037Z Task         : Get sources
2020-04-18T17:36:18.5076325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T17:36:18.5076599Z Version      : 1.0.0
2020-04-18T17:36:18.5076790Z Author       : Microsoft
2020-04-18T17:36:18.5076790Z Author       : Microsoft
2020-04-18T17:36:18.5077085Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T17:36:18.5077434Z ==============================================================================
2020-04-18T17:36:18.8204883Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T17:36:18.8247714Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T17:36:18.8348078Z Cleaning up task key
2020-04-18T17:36:18.8349312Z Start cleaning up orphan processes.
2020-04-18T17:36:18.8516092Z Terminate orphan process: pid (3489) (python)
2020-04-18T17:36:18.8670643Z ##[section]Finishing: Finalize Job
