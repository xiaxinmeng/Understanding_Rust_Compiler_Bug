plain
2020-04-18T17:59:35.5419083Z ========================== Starting Command Output ===========================
2020-04-18T17:59:35.5423733Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad89af3a-a1a0-4a42-b0b8-1f19216b1bf4.sh
2020-04-18T17:59:35.5424173Z 
2020-04-18T17:59:35.5428877Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T17:59:35.5447529Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T17:59:35.5450800Z Task         : Get sources
2020-04-18T17:59:35.5451070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T17:59:35.5451348Z Version      : 1.0.0
2020-04-18T17:59:35.5451530Z Author       : Microsoft
---
2020-04-18T17:59:36.5247951Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T17:59:36.5253786Z ##[command]git config gc.auto 0
2020-04-18T17:59:36.5257714Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T17:59:36.5261256Z ##[command]git config --get-all http.proxy
2020-04-18T17:59:36.5277870Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-18T18:01:56.4114304Z  ---> 318032b5f0e2
2020-04-18T18:01:56.4115496Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T18:01:56.4118294Z  ---> Using cache
2020-04-18T18:01:56.4119053Z  ---> d44a858fd1ce
2020-04-18T18:01:56.4120395Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T18:01:56.4123674Z  ---> 58b910f50f5a
2020-04-18T18:01:56.4124193Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T18:01:56.4126899Z  ---> Using cache
2020-04-18T18:01:56.4127645Z  ---> ee7702aadba1
---
2020-04-18T18:01:56.4852345Z Looks like docker image is the same as before, not uploading
2020-04-18T18:02:04.4076166Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T18:02:04.4385873Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T18:02:04.4421570Z == clock drift check ==
2020-04-18T18:02:04.4429980Z   local time: Sat Apr 18 18:02:04 UTC 2020
2020-04-18T18:02:04.5351284Z   network time: Sat, 18 Apr 2020 18:02:04 GMT
2020-04-18T18:02:04.5375570Z Starting sccache server...
2020-04-18T18:02:04.6318562Z configure: processing command line
2020-04-18T18:02:04.6319220Z configure: 
2020-04-18T18:02:04.6320330Z configure: rust.dist-src        := False
---
2020-04-18T18:07:56.9076816Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T18:07:58.6273652Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T18:08:00.4191310Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T18:08:01.5874404Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T18:08:11.6667387Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T18:08:14.1857392Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T18:08:19.0353792Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T18:08:23.7403899Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T18:08:34.7753900Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T18:33:42.5082741Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T18:33:44.3019227Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T18:33:46.2988055Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T18:33:47.8149461Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T18:33:57.9775558Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T18:34:02.0324883Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T18:34:07.3694367Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T18:34:12.3248158Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T18:34:22.5936040Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T18:59:42.8550579Z .................................................................................................... 1700/9913
2020-04-18T18:59:47.5724384Z .................................................................................................... 1800/9913
2020-04-18T18:59:56.7150665Z .................................................................................................... 1900/9913
2020-04-18T19:00:05.1553384Z ....i............................................................................................... 2000/9913
2020-04-18T19:00:11.9504328Z ..............................................................................................iiiii. 2100/9913
2020-04-18T19:00:33.5123127Z .................................................................................................... 2300/9913
2020-04-18T19:00:35.8490472Z .................................................................................................... 2400/9913
2020-04-18T19:00:38.2942344Z .................................................................................................... 2500/9913
2020-04-18T19:00:44.6124685Z .................................................................................................... 2600/9913
---
2020-04-18T19:03:52.7361516Z .................................................................................................... 5100/9913
2020-04-18T19:04:00.1844131Z .................................................................................................... 5200/9913
2020-04-18T19:04:05.0857301Z ................i................................................................................... 5300/9913
2020-04-18T19:04:15.4458196Z ......i............................................................................................. 5400/9913
2020-04-18T19:04:20.8088943Z ......ii.ii........i...i............................................................................ 5500/9913
2020-04-18T19:04:28.6041947Z .....................................................i.............................................. 5700/9913
2020-04-18T19:04:37.8727108Z .....................................................................................ii............. 5800/9913
2020-04-18T19:04:45.4992496Z ........................i........................................................................... 5900/9913
2020-04-18T19:04:50.6015367Z .................................................................................................... 6000/9913
2020-04-18T19:04:50.6015367Z .................................................................................................... 6000/9913
2020-04-18T19:05:01.5459383Z .................................................................................................... 6100/9913
2020-04-18T19:05:11.7100216Z ..................ii...i..ii...........i............................................................ 6200/9913
2020-04-18T19:05:27.9582331Z .................................................................................................... 6400/9913
2020-04-18T19:05:34.9520253Z .................................................................................................... 6500/9913
2020-04-18T19:05:34.9520253Z .................................................................................................... 6500/9913
2020-04-18T19:05:55.3408760Z ................................................i..ii............................................... 6600/9913
2020-04-18T19:06:17.6998179Z .................................................................................................... 6800/9913
2020-04-18T19:06:19.9655121Z .................................................i.................................................. 6900/9913
2020-04-18T19:06:22.1404553Z .................................................................................................... 7000/9913
2020-04-18T19:06:24.2450221Z .........................................................................................i.......... 7100/9913
---
2020-04-18T19:08:07.5894444Z .................................................................................................... 7900/9913
2020-04-18T19:08:14.7345339Z .................................................................................................... 8000/9913
2020-04-18T19:08:20.9889814Z .......................................................i............................................ 8100/9913
2020-04-18T19:08:32.0679323Z .................................................................................................... 8200/9913
2020-04-18T19:08:37.9139805Z ....iiiiii.iiiii.i.................................................................................. 8300/9913
2020-04-18T19:08:52.3238507Z .................................................................................................... 8500/9913
2020-04-18T19:09:00.6775809Z .................................................................................................... 8600/9913
2020-04-18T19:09:15.6431780Z .................................................................................................... 8700/9913
2020-04-18T19:09:22.4537292Z .................................................................................................... 8800/9913
---
2020-04-18T19:11:43.9922556Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T19:11:44.0121990Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T19:11:44.2271653Z 
2020-04-18T19:11:44.2271991Z running 186 tests
2020-04-18T19:11:47.2226935Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-18T19:11:49.9513033Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-18T19:11:49.9515807Z 
2020-04-18T19:11:49.9516565Z  finished in 5.940
2020-04-18T19:11:49.9526478Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-18T19:11:49.9716666Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T19:11:52.1402866Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-18T19:11:52.1544863Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T19:11:52.3149258Z 
2020-04-18T19:11:52.3149596Z running 9 tests
2020-04-18T19:11:52.3150721Z iiiiiiiii
2020-04-18T19:11:52.3151817Z 
2020-04-18T19:11:52.3156976Z  finished in 0.160
2020-04-18T19:11:52.3157798Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-18T19:11:52.3352546Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T19:11:52.3352546Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T19:11:52.5587029Z 
2020-04-18T19:11:52.5587404Z running 119 tests
2020-04-18T19:12:09.6708524Z ............................FF..FF..FFF.FFFFFFF.....FFF...........F............................F.... 100/119
2020-04-18T19:12:12.5453559Z failures:
2020-04-18T19:12:12.5453697Z 
2020-04-18T19:12:12.5506061Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-04-18T19:12:12.5506455Z 
2020-04-18T19:12:12.5506455Z 
2020-04-18T19:12:12.5507037Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5507489Z status: exit code: 101
2020-04-18T19:12:12.5510393Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2020-04-18T19:12:12.5512927Z ------------------------------------------
2020-04-18T19:12:12.5513100Z 
2020-04-18T19:12:12.5514467Z ------------------------------------------
2020-04-18T19:12:12.5514985Z stderr:
2020-04-18T19:12:12.5514985Z stderr:
2020-04-18T19:12:12.5515692Z ------------------------------------------
2020-04-18T19:12:12.5516717Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5517726Z 
2020-04-18T19:12:12.5518053Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5518254Z 
2020-04-18T19:12:12.5518458Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5518458Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5518764Z 
2020-04-18T19:12:12.5519504Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5520455Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5520684Z 
2020-04-18T19:12:12.5520684Z 
2020-04-18T19:12:12.5522000Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5523002Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5523499Z   left: `LLVMing`,
2020-04-18T19:12:12.5523499Z   left: `LLVMing`,
2020-04-18T19:12:12.5523996Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5524411Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5525293Z 
2020-04-18T19:12:12.5525516Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5526226Z 
2020-04-18T19:12:12.5526226Z 
2020-04-18T19:12:12.5528155Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5529006Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5529234Z 
2020-04-18T19:12:12.5529234Z 
2020-04-18T19:12:12.5530084Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5530624Z 
2020-04-18T19:12:12.5530993Z ------------------------------------------
2020-04-18T19:12:12.5531154Z 
2020-04-18T19:12:12.5531244Z 
2020-04-18T19:12:12.5531244Z 
2020-04-18T19:12:12.5531666Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2020-04-18T19:12:12.5531871Z 
2020-04-18T19:12:12.5532326Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5532616Z status: exit code: 101
2020-04-18T19:12:12.5535221Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2020-04-18T19:12:12.5537088Z ------------------------------------------
2020-04-18T19:12:12.5537397Z 
2020-04-18T19:12:12.5537796Z ------------------------------------------
2020-04-18T19:12:12.5538008Z stderr:
2020-04-18T19:12:12.5538008Z stderr:
2020-04-18T19:12:12.5538372Z ------------------------------------------
2020-04-18T19:12:12.5539030Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5539717Z 
2020-04-18T19:12:12.5539917Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5540097Z 
2020-04-18T19:12:12.5540314Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5540314Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5540495Z 
2020-04-18T19:12:12.5541096Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5541809Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5542040Z 
2020-04-18T19:12:12.5542040Z 
2020-04-18T19:12:12.5542859Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5543833Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5544135Z   left: `LLVMing`,
2020-04-18T19:12:12.5544135Z   left: `LLVMing`,
2020-04-18T19:12:12.5544608Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5545009Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5545205Z 
2020-04-18T19:12:12.5545404Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5545586Z 
2020-04-18T19:12:12.5545586Z 
2020-04-18T19:12:12.5546170Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5546882Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5547114Z 
2020-04-18T19:12:12.5547114Z 
2020-04-18T19:12:12.5548134Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5548691Z 
2020-04-18T19:12:12.5549118Z ------------------------------------------
2020-04-18T19:12:12.5549281Z 
2020-04-18T19:12:12.5549372Z 
2020-04-18T19:12:12.5549372Z 
2020-04-18T19:12:12.5549788Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2020-04-18T19:12:12.5549989Z 
2020-04-18T19:12:12.5550444Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5550729Z status: exit code: 101
2020-04-18T19:12:12.5553058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2020-04-18T19:12:12.5554829Z ------------------------------------------
2020-04-18T19:12:12.5554990Z 
2020-04-18T19:12:12.5555338Z ------------------------------------------
2020-04-18T19:12:12.5555546Z stderr:
2020-04-18T19:12:12.5555546Z stderr:
2020-04-18T19:12:12.5555910Z ------------------------------------------
2020-04-18T19:12:12.5556555Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5557365Z 
2020-04-18T19:12:12.5557563Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5557742Z 
2020-04-18T19:12:12.5558014Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5558014Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5558196Z 
2020-04-18T19:12:12.5558813Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5560291Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5560517Z 
2020-04-18T19:12:12.5560517Z 
2020-04-18T19:12:12.5561344Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5562560Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5562856Z   left: `LLVMing`,
2020-04-18T19:12:12.5562856Z   left: `LLVMing`,
2020-04-18T19:12:12.5563296Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5563690Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5564012Z 
2020-04-18T19:12:12.5564224Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5564403Z 
2020-04-18T19:12:12.5564403Z 
2020-04-18T19:12:12.5565047Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5565767Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5565993Z 
2020-04-18T19:12:12.5565993Z 
2020-04-18T19:12:12.5566816Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5567361Z 
2020-04-18T19:12:12.5567707Z ------------------------------------------
2020-04-18T19:12:12.5567882Z 
2020-04-18T19:12:12.5567973Z 
2020-04-18T19:12:12.5567973Z 
2020-04-18T19:12:12.5568773Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2020-04-18T19:12:12.5568980Z 
2020-04-18T19:12:12.5569450Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5569734Z status: exit code: 101
2020-04-18T19:12:12.5572031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2020-04-18T19:12:12.5573869Z ------------------------------------------
2020-04-18T19:12:12.5574253Z 
2020-04-18T19:12:12.5574674Z ------------------------------------------
2020-04-18T19:12:12.5574885Z stderr:
2020-04-18T19:12:12.5574885Z stderr:
2020-04-18T19:12:12.5575250Z ------------------------------------------
2020-04-18T19:12:12.5575901Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5576588Z 
2020-04-18T19:12:12.5576786Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5577091Z 
2020-04-18T19:12:12.5577309Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5577309Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5577490Z 
2020-04-18T19:12:12.5578113Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5578858Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5579087Z 
2020-04-18T19:12:12.5579087Z 
2020-04-18T19:12:12.5579894Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5580793Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5581065Z   left: `LLVMing`,
2020-04-18T19:12:12.5581065Z   left: `LLVMing`,
2020-04-18T19:12:12.5581517Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5581914Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5582114Z 
2020-04-18T19:12:12.5582313Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5582494Z 
2020-04-18T19:12:12.5582494Z 
2020-04-18T19:12:12.5583075Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5583871Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5584108Z 
2020-04-18T19:12:12.5584108Z 
2020-04-18T19:12:12.5584973Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5585512Z 
2020-04-18T19:12:12.5585860Z ------------------------------------------
2020-04-18T19:12:12.5586034Z 
2020-04-18T19:12:12.5586125Z 
2020-04-18T19:12:12.5586125Z 
2020-04-18T19:12:12.5586543Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2020-04-18T19:12:12.5586753Z 
2020-04-18T19:12:12.5587207Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5587492Z status: exit code: 101
2020-04-18T19:12:12.5589843Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2020-04-18T19:12:12.5591662Z ------------------------------------------
2020-04-18T19:12:12.5591824Z 
2020-04-18T19:12:12.5592173Z ------------------------------------------
2020-04-18T19:12:12.5592364Z stderr:
2020-04-18T19:12:12.5592364Z stderr:
2020-04-18T19:12:12.5592739Z ------------------------------------------
2020-04-18T19:12:12.5593387Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5594073Z 
2020-04-18T19:12:12.5594273Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5594451Z 
2020-04-18T19:12:12.5594687Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5594687Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5594869Z 
2020-04-18T19:12:12.5595441Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5596307Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5596534Z 
2020-04-18T19:12:12.5596534Z 
2020-04-18T19:12:12.5597349Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5598254Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5598526Z   left: `LLVMing`,
2020-04-18T19:12:12.5598526Z   left: `LLVMing`,
2020-04-18T19:12:12.5598977Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5599374Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5599553Z 
2020-04-18T19:12:12.5599768Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5599948Z 
2020-04-18T19:12:12.5599948Z 
2020-04-18T19:12:12.5600514Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5601248Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5601692Z 
2020-04-18T19:12:12.5601692Z 
2020-04-18T19:12:12.5602678Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5608935Z 
2020-04-18T19:12:12.5609694Z ------------------------------------------
2020-04-18T19:12:12.5609887Z 
2020-04-18T19:12:12.5610017Z 
2020-04-18T19:12:12.5610017Z 
2020-04-18T19:12:12.5610426Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2020-04-18T19:12:12.5610616Z 
2020-04-18T19:12:12.5611072Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5611528Z status: exit code: 101
2020-04-18T19:12:12.5613816Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2020-04-18T19:12:12.5654758Z ------------------------------------------
2020-04-18T19:12:12.5654956Z 
2020-04-18T19:12:12.5655326Z ------------------------------------------
2020-04-18T19:12:12.5655522Z stderr:
2020-04-18T19:12:12.5655522Z stderr:
2020-04-18T19:12:12.5655907Z ------------------------------------------
2020-04-18T19:12:12.5656581Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5657267Z 
2020-04-18T19:12:12.5657473Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5657654Z 
2020-04-18T19:12:12.5657855Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5657855Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5658052Z 
2020-04-18T19:12:12.5658654Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5659394Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5659620Z 
2020-04-18T19:12:12.5659620Z 
2020-04-18T19:12:12.5660425Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5664206Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5664483Z   left: `LLVMing`,
2020-04-18T19:12:12.5664483Z   left: `LLVMing`,
2020-04-18T19:12:12.5664940Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5665351Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5665533Z 
2020-04-18T19:12:12.5665747Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5665928Z 
2020-04-18T19:12:12.5665928Z 
2020-04-18T19:12:12.5666527Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5667265Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5667492Z 
2020-04-18T19:12:12.5667492Z 
2020-04-18T19:12:12.5668299Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5668862Z 
2020-04-18T19:12:12.5669212Z ------------------------------------------
2020-04-18T19:12:12.5669372Z 
2020-04-18T19:12:12.5669479Z 
2020-04-18T19:12:12.5669479Z 
2020-04-18T19:12:12.5670017Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2020-04-18T19:12:12.5670229Z 
2020-04-18T19:12:12.5699756Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5700145Z status: exit code: 101
2020-04-18T19:12:12.5703418Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2020-04-18T19:12:12.5707177Z ------------------------------------------
2020-04-18T19:12:12.5707394Z 
2020-04-18T19:12:12.5707878Z ------------------------------------------
2020-04-18T19:12:12.5708100Z stderr:
2020-04-18T19:12:12.5708100Z stderr:
2020-04-18T19:12:12.5708485Z ------------------------------------------
2020-04-18T19:12:12.5709145Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5709833Z 
2020-04-18T19:12:12.5710046Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5710225Z 
2020-04-18T19:12:12.5710425Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5710425Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5710622Z 
2020-04-18T19:12:12.5711222Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5711961Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5712189Z 
2020-04-18T19:12:12.5712189Z 
2020-04-18T19:12:12.5713000Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5713898Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5714170Z   left: `LLVMing`,
2020-04-18T19:12:12.5714170Z   left: `LLVMing`,
2020-04-18T19:12:12.5714622Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5715193Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5715372Z 
2020-04-18T19:12:12.5715590Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5715771Z 
2020-04-18T19:12:12.5715771Z 
2020-04-18T19:12:12.5716390Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5717209Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5717438Z 
2020-04-18T19:12:12.5717438Z 
2020-04-18T19:12:12.5718246Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5718801Z 
2020-04-18T19:12:12.5719154Z ------------------------------------------
2020-04-18T19:12:12.5719313Z 
2020-04-18T19:12:12.5719421Z 
2020-04-18T19:12:12.5719421Z 
2020-04-18T19:12:12.5719830Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2020-04-18T19:12:12.5720034Z 
2020-04-18T19:12:12.5720473Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5720774Z status: exit code: 101
2020-04-18T19:12:12.5723356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2020-04-18T19:12:12.5725192Z ------------------------------------------
2020-04-18T19:12:12.5725374Z 
2020-04-18T19:12:12.5725724Z ------------------------------------------
2020-04-18T19:12:12.5725917Z stderr:
2020-04-18T19:12:12.5725917Z stderr:
2020-04-18T19:12:12.5726283Z ------------------------------------------
2020-04-18T19:12:12.5726951Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5727893Z 
2020-04-18T19:12:12.5728097Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5728276Z 
2020-04-18T19:12:12.5728475Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5728475Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5728671Z 
2020-04-18T19:12:12.5729317Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5730062Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5730290Z 
2020-04-18T19:12:12.5730290Z 
2020-04-18T19:12:12.5731101Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5732002Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5732276Z   left: `LLVMing`,
2020-04-18T19:12:12.5732276Z   left: `LLVMing`,
2020-04-18T19:12:12.5732709Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5733124Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5733302Z 
2020-04-18T19:12:12.5744303Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5744551Z 
2020-04-18T19:12:12.5744551Z 
2020-04-18T19:12:12.5745324Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5746300Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5746528Z 
2020-04-18T19:12:12.5746528Z 
2020-04-18T19:12:12.5747345Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5751444Z 
2020-04-18T19:12:12.5751840Z ------------------------------------------
2020-04-18T19:12:12.5752001Z 
2020-04-18T19:12:12.5752093Z 
2020-04-18T19:12:12.5752093Z 
2020-04-18T19:12:12.5752515Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2020-04-18T19:12:12.5752706Z 
2020-04-18T19:12:12.5753146Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5753661Z status: exit code: 101
2020-04-18T19:12:12.5756444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2020-04-18T19:12:12.5758308Z ------------------------------------------
2020-04-18T19:12:12.5758473Z 
2020-04-18T19:12:12.5758849Z ------------------------------------------
2020-04-18T19:12:12.5759040Z stderr:
2020-04-18T19:12:12.5759040Z stderr:
2020-04-18T19:12:12.5760077Z ------------------------------------------
2020-04-18T19:12:12.5760788Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5761604Z 
2020-04-18T19:12:12.5761835Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5762016Z 
2020-04-18T19:12:12.5762218Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5762218Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5762399Z 
2020-04-18T19:12:12.5763042Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5763760Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5764003Z 
2020-04-18T19:12:12.5764003Z 
2020-04-18T19:12:12.5764809Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5765871Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5766144Z   left: `LLVMing`,
2020-04-18T19:12:12.5766144Z   left: `LLVMing`,
2020-04-18T19:12:12.5766580Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5766999Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5767179Z 
2020-04-18T19:12:12.5767379Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5767576Z 
2020-04-18T19:12:12.5767576Z 
2020-04-18T19:12:12.5768146Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5769650Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5769886Z 
2020-04-18T19:12:12.5769886Z 
2020-04-18T19:12:12.5770701Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5771604Z 
2020-04-18T19:12:12.5772034Z ------------------------------------------
2020-04-18T19:12:12.5772195Z 
2020-04-18T19:12:12.5772286Z 
2020-04-18T19:12:12.5772286Z 
2020-04-18T19:12:12.5774162Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2020-04-18T19:12:12.5774371Z 
2020-04-18T19:12:12.5774827Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5775111Z status: exit code: 101
2020-04-18T19:12:12.5777788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
2020-04-18T19:12:12.5780527Z ------------------------------------------
2020-04-18T19:12:12.5780705Z 
2020-04-18T19:12:12.5781093Z ------------------------------------------
2020-04-18T19:12:12.5781288Z stderr:
2020-04-18T19:12:12.5781288Z stderr:
2020-04-18T19:12:12.5781648Z ------------------------------------------
2020-04-18T19:12:12.5782316Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5782998Z 
2020-04-18T19:12:12.5783214Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5783394Z 
2020-04-18T19:12:12.5783747Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5783747Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5783935Z 
2020-04-18T19:12:12.5784589Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5785323Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5785568Z 
2020-04-18T19:12:12.5785568Z 
2020-04-18T19:12:12.5786379Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5787276Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5787700Z   left: `LLVMing`,
2020-04-18T19:12:12.5787700Z   left: `LLVMing`,
2020-04-18T19:12:12.5788183Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5788606Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5788785Z 
2020-04-18T19:12:12.5788983Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5789162Z 
2020-04-18T19:12:12.5789162Z 
2020-04-18T19:12:12.5789760Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5790672Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5790903Z 
2020-04-18T19:12:12.5790903Z 
2020-04-18T19:12:12.5791717Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5792270Z 
2020-04-18T19:12:12.5792618Z ------------------------------------------
2020-04-18T19:12:12.5792777Z 
2020-04-18T19:12:12.5792870Z 
2020-04-18T19:12:12.5792870Z 
2020-04-18T19:12:12.5793485Z ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
2020-04-18T19:12:12.5793687Z 
2020-04-18T19:12:12.5794127Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5794410Z status: exit code: 101
2020-04-18T19:12:12.5797507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
2020-04-18T19:12:12.5799607Z ------------------------------------------
2020-04-18T19:12:12.5799774Z 
2020-04-18T19:12:12.5800144Z ------------------------------------------
2020-04-18T19:12:12.5800461Z stderr:
2020-04-18T19:12:12.5800461Z stderr:
2020-04-18T19:12:12.5801233Z ------------------------------------------
2020-04-18T19:12:12.5802151Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5802832Z 
2020-04-18T19:12:12.5803050Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5803230Z 
2020-04-18T19:12:12.5803430Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5803430Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5803611Z 
2020-04-18T19:12:12.5804394Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5805138Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5805364Z 
2020-04-18T19:12:12.5805364Z 
2020-04-18T19:12:12.5806189Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5807087Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-18T19:12:12.5807358Z   left: `LLVMing`,
2020-04-18T19:12:12.5807358Z   left: `LLVMing`,
2020-04-18T19:12:12.5807793Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-18T19:12:12.5808206Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5808385Z 
2020-04-18T19:12:12.5808584Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5808767Z 
2020-04-18T19:12:12.5808767Z 
2020-04-18T19:12:12.5809355Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5810062Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5810505Z 
2020-04-18T19:12:12.5810505Z 
2020-04-18T19:12:12.5811390Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-18T19:12:12.5812772Z 
2020-04-18T19:12:12.5813261Z ------------------------------------------
2020-04-18T19:12:12.5813422Z 
2020-04-18T19:12:12.5813513Z 
2020-04-18T19:12:12.5813513Z 
2020-04-18T19:12:12.5813929Z ---- [incremental] incremental/hashes/match_expressions.rs stdout ----
2020-04-18T19:12:12.5814147Z 
2020-04-18T19:12:12.5814588Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-18T19:12:12.5814872Z status: exit code: 101
2020-04-18T19:12:12.5818948Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
2020-04-18T19:12:12.5821299Z ------------------------------------------
2020-04-18T19:12:12.5821466Z 
2020-04-18T19:12:12.5821839Z ------------------------------------------
2020-04-18T19:12:12.5822042Z stderr:
2020-04-18T19:12:12.5822042Z stderr:
2020-04-18T19:12:12.5822776Z ------------------------------------------
2020-04-18T19:12:12.5823528Z thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-18T19:12:12.5824644Z 
2020-04-18T19:12:12.5824847Z error: internal compiler error: unexpected panic
2020-04-18T19:12:12.5825043Z 
2020-04-18T19:12:12.5825244Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5825244Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T19:12:12.5825425Z 
2020-04-18T19:12:12.5826179Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T19:12:12.5826910Z note: rustc 1.44.0-nightly (d583cb959 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T19:12:12.5827135Z 
2020-04-18T19:12:12.5827135Z 
---
2020-04-18T19:12:12.5992605Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T19:12:12.5992993Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T19:12:12.5993205Z 
2020-04-18T19:12:12.5993295Z 
2020-04-18T19:12:12.5996815Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T19:12:12.6000527Z 
2020-04-18T19:12:12.6000623Z 
2020-04-18T19:12:12.6001323Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T19:12:12.6001977Z Build completed unsuccessfully in 1:08:20
2020-04-18T19:12:12.6001977Z Build completed unsuccessfully in 1:08:20
2020-04-18T19:12:12.6002212Z == clock drift check ==
2020-04-18T19:12:12.6002458Z   local time: Sat Apr 18 19:12:12 UTC 2020
2020-04-18T19:12:12.6703830Z   network time: Sat, 18 Apr 2020 19:12:12 GMT
2020-04-18T19:12:15.4141651Z 
2020-04-18T19:12:15.4141651Z 
2020-04-18T19:12:15.4221170Z ##[error]Bash exited with code '1'.
2020-04-18T19:12:15.4238388Z ##[section]Finishing: Run build
2020-04-18T19:12:15.4291828Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T19:12:15.4298642Z Task         : Get sources
2020-04-18T19:12:15.4298995Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T19:12:15.4299336Z Version      : 1.0.0
2020-04-18T19:12:15.4299568Z Author       : Microsoft
2020-04-18T19:12:15.4299568Z Author       : Microsoft
2020-04-18T19:12:15.4299931Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T19:12:15.4300363Z ==============================================================================
2020-04-18T19:12:15.8071715Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T19:12:15.8125316Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T19:12:15.8221915Z Cleaning up task key
2020-04-18T19:12:15.8223143Z Start cleaning up orphan processes.
2020-04-18T19:12:15.8453999Z Terminate orphan process: pid (3498) (python)
2020-04-18T19:12:15.8626492Z ##[section]Finishing: Finalize Job
