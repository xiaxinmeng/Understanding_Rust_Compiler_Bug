plain
2020-04-19T08:26:25.9979424Z ========================== Starting Command Output ===========================
2020-04-19T08:26:25.9984446Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4e8ef2ea-1acd-45b1-8aeb-d30f208c4fbe.sh
2020-04-19T08:26:25.9984905Z 
2020-04-19T08:26:25.9989861Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T08:26:26.0009257Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T08:26:26.0012558Z Task         : Get sources
2020-04-19T08:26:26.0012855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T08:26:26.0013165Z Version      : 1.0.0
2020-04-19T08:26:26.0013366Z Author       : Microsoft
---
2020-04-19T08:26:26.9863998Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T08:26:26.9869558Z ##[command]git config gc.auto 0
2020-04-19T08:26:26.9873308Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T08:26:26.9876750Z ##[command]git config --get-all http.proxy
2020-04-19T08:26:26.9883238Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-19T08:29:31.6155055Z  ---> 318032b5f0e2
2020-04-19T08:29:31.6155925Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T08:29:31.6162920Z  ---> Using cache
2020-04-19T08:29:31.6163381Z  ---> d44a858fd1ce
2020-04-19T08:29:31.6164426Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T08:29:31.6177774Z  ---> 58b910f50f5a
2020-04-19T08:29:31.6178560Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T08:29:31.6179358Z  ---> Using cache
2020-04-19T08:29:31.6179790Z  ---> ee7702aadba1
---
2020-04-19T08:29:31.6864248Z Looks like docker image is the same as before, not uploading
2020-04-19T08:29:40.3739966Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T08:29:40.4096813Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T08:29:40.4130629Z == clock drift check ==
2020-04-19T08:29:40.4142751Z   local time: Sun Apr 19 08:29:40 UTC 2020
2020-04-19T08:29:40.4632687Z   network time: Sun, 19 Apr 2020 08:29:40 GMT
2020-04-19T08:29:40.4676104Z Starting sccache server...
2020-04-19T08:29:40.5564312Z configure: processing command line
2020-04-19T08:29:40.5564845Z configure: 
2020-04-19T08:29:40.5565859Z configure: rust.dist-src        := False
---
2020-04-19T08:35:17.8582118Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T08:35:19.5068332Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T08:35:21.1592470Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T08:35:21.9325179Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T08:35:31.8722162Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T08:35:34.2882925Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T08:35:39.0085748Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T08:35:43.4793557Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T08:35:54.2675773Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T08:59:57.9303918Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T08:59:59.7567814Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T09:00:01.7866452Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T09:00:02.6992347Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T09:00:13.4305253Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T09:00:16.8936333Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T09:00:21.9419326Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T09:00:26.8014781Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T09:00:37.6669455Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T09:25:02.2051521Z .................................................................................................... 1700/9914
2020-04-19T09:25:06.4771845Z .................................................................................................... 1800/9914
2020-04-19T09:25:14.9881987Z .................................................................................................... 1900/9914
2020-04-19T09:25:23.1659826Z .....i.............................................................................................. 2000/9914
2020-04-19T09:25:29.6161937Z ...............................................................................................iiiii 2100/9914
2020-04-19T09:25:49.9438385Z .................................................................................................... 2300/9914
2020-04-19T09:25:52.1527643Z .................................................................................................... 2400/9914
2020-04-19T09:25:54.3640888Z .................................................................................................... 2500/9914
2020-04-19T09:26:00.0820515Z .................................................................................................... 2600/9914
---
2020-04-19T09:28:56.6123308Z .................................................................................................... 5100/9914
2020-04-19T09:29:03.8287817Z .................................................................................................... 5200/9914
2020-04-19T09:29:08.2833327Z .................i.................................................................................. 5300/9914
2020-04-19T09:29:18.1782534Z .......i............................................................................................ 5400/9914
2020-04-19T09:29:23.4960097Z .......ii.ii........i...i........................................................................... 5500/9914
2020-04-19T09:29:30.9878238Z ......................................................i............................................. 5700/9914
2020-04-19T09:29:39.9574589Z ......................................................................................ii............ 5800/9914
2020-04-19T09:29:46.7504338Z .........................i.......................................................................... 5900/9914
2020-04-19T09:29:52.0835088Z .................................................................................................... 6000/9914
2020-04-19T09:29:52.0835088Z .................................................................................................... 6000/9914
2020-04-19T09:30:02.5608870Z .................................................................................................... 6100/9914
2020-04-19T09:30:12.3829369Z ...................ii...i..ii...........i........................................................... 6200/9914
2020-04-19T09:30:28.1195298Z .................................................................................................... 6400/9914
2020-04-19T09:30:34.7750578Z .................................................................................................... 6500/9914
2020-04-19T09:30:34.7750578Z .................................................................................................... 6500/9914
2020-04-19T09:30:55.4967245Z .................................................i..ii.............................................. 6600/9914
2020-04-19T09:31:16.6679937Z .................................................................................................... 6800/9914
2020-04-19T09:31:18.9391362Z ..................................................i................................................. 6900/9914
2020-04-19T09:31:20.9862956Z .................................................................................................... 7000/9914
2020-04-19T09:31:23.0718237Z ..........................................................................................i......... 7100/9914
---
2020-04-19T09:33:02.9319599Z .................................................................................................... 7900/9914
2020-04-19T09:33:09.2919305Z .................................................................................................... 8000/9914
2020-04-19T09:33:14.9823800Z .........................................................i.......................................... 8100/9914
2020-04-19T09:33:24.8962951Z .................................................................................................... 8200/9914
2020-04-19T09:33:30.2961571Z .....iiiiii.iiiii.i................................................................................. 8300/9914
2020-04-19T09:33:43.8132658Z .................................................................................................... 8500/9914
2020-04-19T09:33:51.8277456Z .................................................................................................... 8600/9914
2020-04-19T09:34:05.0075494Z .................................................................................................... 8700/9914
2020-04-19T09:34:11.5888732Z .................................................................................................... 8800/9914
---
2020-04-19T09:36:30.5041336Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-19T09:36:30.5250248Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T09:36:30.7569283Z 
2020-04-19T09:36:30.7574828Z running 186 tests
2020-04-19T09:36:33.6827067Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-19T09:36:36.3847438Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-19T09:36:36.3850273Z 
2020-04-19T09:36:36.3855067Z  finished in 5.860
2020-04-19T09:36:36.3863365Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-19T09:36:36.4051725Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T09:36:38.5373255Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-19T09:36:38.5565707Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T09:36:38.7090158Z 
2020-04-19T09:36:38.7090487Z running 9 tests
2020-04-19T09:36:38.7091579Z iiiiiiiii
2020-04-19T09:36:38.7094817Z 
2020-04-19T09:36:38.7095166Z  finished in 0.152
2020-04-19T09:36:38.7096213Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-19T09:36:38.7275044Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T09:36:38.7275044Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T09:36:38.9167628Z 
2020-04-19T09:36:38.9167942Z running 120 tests
2020-04-19T09:36:55.3516994Z F............................F.F.FF..FFF.FFFFFF.F....FFF...........F..........................F..... 100/120
2020-04-19T09:36:58.4095353Z failures:
2020-04-19T09:36:58.4096395Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T09:36:58.4096817Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T09:36:58.4097896Z 
2020-04-19T09:36:58.4097896Z 
2020-04-19T09:36:58.4098637Z ---- [incremental] incremental/__check/issue-42602.rs stdout ----
2020-04-19T09:36:58.4098913Z 
2020-04-19T09:36:58.4099392Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4099669Z status: exit code: 101
2020-04-19T09:36:58.4102014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/__check/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/__check/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/__check/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/__check/issue-42602/auxiliary"
2020-04-19T09:36:58.4103774Z ------------------------------------------
2020-04-19T09:36:58.4103935Z 
2020-04-19T09:36:58.4104279Z ------------------------------------------
2020-04-19T09:36:58.4104465Z stderr:
2020-04-19T09:36:58.4104465Z stderr:
2020-04-19T09:36:58.4104838Z ------------------------------------------
2020-04-19T09:36:58.4105573Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `mir_built`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4106306Z 
2020-04-19T09:36:58.4106499Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4106674Z 
2020-04-19T09:36:58.4106882Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4106882Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4107058Z 
2020-04-19T09:36:58.4107690Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4108405Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4108625Z 
2020-04-19T09:36:58.4108625Z 
2020-04-19T09:36:58.4109353Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4110194Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4110461Z   left: `LLVMing`,
2020-04-19T09:36:58.4110461Z   left: `LLVMing`,
2020-04-19T09:36:58.4110903Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4111290Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4111465Z 
2020-04-19T09:36:58.4111675Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4111851Z 
2020-04-19T09:36:58.4111851Z 
2020-04-19T09:36:58.4112407Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4113112Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4113334Z 
2020-04-19T09:36:58.4113334Z 
2020-04-19T09:36:58.4114070Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4114686Z 
2020-04-19T09:36:58.4115075Z ------------------------------------------
2020-04-19T09:36:58.4115231Z 
2020-04-19T09:36:58.4115318Z 
2020-04-19T09:36:58.4115318Z 
2020-04-19T09:36:58.4115719Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2020-04-19T09:36:58.4115913Z 
2020-04-19T09:36:58.4116352Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4126975Z status: exit code: 101
2020-04-19T09:36:58.4129729Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2020-04-19T09:36:58.4131623Z ------------------------------------------
2020-04-19T09:36:58.4131790Z 
2020-04-19T09:36:58.4132145Z ------------------------------------------
2020-04-19T09:36:58.4132355Z stderr:
2020-04-19T09:36:58.4132355Z stderr:
2020-04-19T09:36:58.4132718Z ------------------------------------------
2020-04-19T09:36:58.4133476Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4134251Z 
2020-04-19T09:36:58.4134453Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4134635Z 
2020-04-19T09:36:58.4134853Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4134853Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4135035Z 
2020-04-19T09:36:58.4135640Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4136380Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4136609Z 
2020-04-19T09:36:58.4136609Z 
2020-04-19T09:36:58.4137438Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4138329Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4138623Z   left: `LLVMing`,
2020-04-19T09:36:58.4138623Z   left: `LLVMing`,
2020-04-19T09:36:58.4139180Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4139570Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4139760Z 
2020-04-19T09:36:58.4139956Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4140131Z 
2020-04-19T09:36:58.4140131Z 
2020-04-19T09:36:58.4140829Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4141550Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4141778Z 
2020-04-19T09:36:58.4141778Z 
2020-04-19T09:36:58.4142818Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4143364Z 
2020-04-19T09:36:58.4143841Z ------------------------------------------
2020-04-19T09:36:58.4143997Z 
2020-04-19T09:36:58.4144198Z 
2020-04-19T09:36:58.4144198Z 
2020-04-19T09:36:58.4144633Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2020-04-19T09:36:58.4144850Z 
2020-04-19T09:36:58.4145274Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4145548Z status: exit code: 101
2020-04-19T09:36:58.4148166Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2020-04-19T09:36:58.4150033Z ------------------------------------------
2020-04-19T09:36:58.4150185Z 
2020-04-19T09:36:58.4150759Z ------------------------------------------
2020-04-19T09:36:58.4150968Z stderr:
2020-04-19T09:36:58.4150968Z stderr:
2020-04-19T09:36:58.4151325Z ------------------------------------------
2020-04-19T09:36:58.4152067Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `mir_built`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4152825Z 
2020-04-19T09:36:58.4153025Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4153225Z 
2020-04-19T09:36:58.4153429Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4153429Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4153618Z 
2020-04-19T09:36:58.4154441Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4155114Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4155327Z 
2020-04-19T09:36:58.4155327Z 
2020-04-19T09:36:58.4156102Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4157059Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4157342Z   left: `LLVMing`,
2020-04-19T09:36:58.4157342Z   left: `LLVMing`,
2020-04-19T09:36:58.4157762Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4158163Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4158339Z 
2020-04-19T09:36:58.4158537Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4158717Z 
2020-04-19T09:36:58.4158717Z 
2020-04-19T09:36:58.4159279Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4160069Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4160297Z 
2020-04-19T09:36:58.4160297Z 
2020-04-19T09:36:58.4161050Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4161578Z 
2020-04-19T09:36:58.4162027Z ------------------------------------------
2020-04-19T09:36:58.4162182Z 
2020-04-19T09:36:58.4162270Z 
2020-04-19T09:36:58.4162270Z 
2020-04-19T09:36:58.4162679Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2020-04-19T09:36:58.4162877Z 
2020-04-19T09:36:58.4171400Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4171880Z status: exit code: 101
2020-04-19T09:36:58.4174616Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2020-04-19T09:36:58.4176627Z ------------------------------------------
2020-04-19T09:36:58.4176802Z 
2020-04-19T09:36:58.4177292Z ------------------------------------------
2020-04-19T09:36:58.4177595Z stderr:
2020-04-19T09:36:58.4177595Z stderr:
2020-04-19T09:36:58.4177946Z ------------------------------------------
2020-04-19T09:36:58.4178691Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4179413Z 
2020-04-19T09:36:58.4179621Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4179796Z 
2020-04-19T09:36:58.4179990Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4179990Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4180164Z 
2020-04-19T09:36:58.4180856Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4181533Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4181763Z 
2020-04-19T09:36:58.4181763Z 
2020-04-19T09:36:58.4182641Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4183808Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4184078Z   left: `LLVMing`,
2020-04-19T09:36:58.4184078Z   left: `LLVMing`,
2020-04-19T09:36:58.4184502Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4185027Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4185208Z 
2020-04-19T09:36:58.4185406Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4185587Z 
2020-04-19T09:36:58.4185587Z 
2020-04-19T09:36:58.4186274Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4186985Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4187211Z 
2020-04-19T09:36:58.4187211Z 
2020-04-19T09:36:58.4188118Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4188688Z 
2020-04-19T09:36:58.4189035Z ------------------------------------------
2020-04-19T09:36:58.4189195Z 
2020-04-19T09:36:58.4189286Z 
2020-04-19T09:36:58.4189286Z 
2020-04-19T09:36:58.4189710Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2020-04-19T09:36:58.4189910Z 
2020-04-19T09:36:58.4190344Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4190646Z status: exit code: 101
2020-04-19T09:36:58.4193132Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2020-04-19T09:36:58.4195094Z ------------------------------------------
2020-04-19T09:36:58.4195252Z 
2020-04-19T09:36:58.4195604Z ------------------------------------------
2020-04-19T09:36:58.4195788Z stderr:
2020-04-19T09:36:58.4195788Z stderr:
2020-04-19T09:36:58.4196130Z ------------------------------------------
2020-04-19T09:36:58.4197030Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4197741Z 
2020-04-19T09:36:58.4197944Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4198112Z 
2020-04-19T09:36:58.4198419Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4198419Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4198594Z 
2020-04-19T09:36:58.4199188Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4199890Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4200110Z 
2020-04-19T09:36:58.4200110Z 
2020-04-19T09:36:58.4200895Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4201872Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4202130Z   left: `LLVMing`,
2020-04-19T09:36:58.4202130Z   left: `LLVMing`,
2020-04-19T09:36:58.4202532Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4202919Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4203086Z 
2020-04-19T09:36:58.4203272Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4203457Z 
2020-04-19T09:36:58.4203457Z 
2020-04-19T09:36:58.4204248Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4205888Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4206118Z 
2020-04-19T09:36:58.4206118Z 
2020-04-19T09:36:58.4212913Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4216608Z 
2020-04-19T09:36:58.4217222Z ------------------------------------------
2020-04-19T09:36:58.4217392Z 
2020-04-19T09:36:58.4217512Z 
2020-04-19T09:36:58.4217512Z 
2020-04-19T09:36:58.4217948Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2020-04-19T09:36:58.4218142Z 
2020-04-19T09:36:58.4218729Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4219030Z status: exit code: 101
2020-04-19T09:36:58.4221443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2020-04-19T09:36:58.4224585Z ------------------------------------------
2020-04-19T09:36:58.4224782Z 
2020-04-19T09:36:58.4225162Z ------------------------------------------
2020-04-19T09:36:58.4225358Z stderr:
2020-04-19T09:36:58.4225358Z stderr:
2020-04-19T09:36:58.4225754Z ------------------------------------------
2020-04-19T09:36:58.4226794Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4227886Z 
2020-04-19T09:36:58.4228224Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4228416Z 
2020-04-19T09:36:58.4228620Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4228620Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4228820Z 
2020-04-19T09:36:58.4229785Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4231371Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4231622Z 
2020-04-19T09:36:58.4231622Z 
2020-04-19T09:36:58.4232553Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4233496Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4233775Z   left: `LLVMing`,
2020-04-19T09:36:58.4233775Z   left: `LLVMing`,
2020-04-19T09:36:58.4234253Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4234673Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4234977Z 
2020-04-19T09:36:58.4235311Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4235493Z 
2020-04-19T09:36:58.4235493Z 
2020-04-19T09:36:58.4236111Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4236878Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4237108Z 
2020-04-19T09:36:58.4237108Z 
2020-04-19T09:36:58.4237936Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4238506Z 
2020-04-19T09:36:58.4238877Z ------------------------------------------
2020-04-19T09:36:58.4239039Z 
2020-04-19T09:36:58.4239149Z 
2020-04-19T09:36:58.4239149Z 
2020-04-19T09:36:58.4239889Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2020-04-19T09:36:58.4240707Z 
2020-04-19T09:36:58.4241298Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4241589Z status: exit code: 101
2020-04-19T09:36:58.4244455Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2020-04-19T09:36:58.4247207Z ------------------------------------------
2020-04-19T09:36:58.4247389Z 
2020-04-19T09:36:58.4247816Z ------------------------------------------
2020-04-19T09:36:58.4248265Z stderr:
2020-04-19T09:36:58.4248265Z stderr:
2020-04-19T09:36:58.4248926Z ------------------------------------------
2020-04-19T09:36:58.4249706Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4253739Z 
2020-04-19T09:36:58.4253991Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4254173Z 
2020-04-19T09:36:58.4254400Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4254400Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4254585Z 
2020-04-19T09:36:58.4255526Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4256919Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4257160Z 
2020-04-19T09:36:58.4257160Z 
2020-04-19T09:36:58.4258010Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4258956Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4259237Z   left: `LLVMing`,
2020-04-19T09:36:58.4259237Z   left: `LLVMing`,
2020-04-19T09:36:58.4260029Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4260451Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4261073Z 
2020-04-19T09:36:58.4261307Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4261600Z 
2020-04-19T09:36:58.4261600Z 
2020-04-19T09:36:58.4262305Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4263093Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4263324Z 
2020-04-19T09:36:58.4263324Z 
2020-04-19T09:36:58.4264605Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4265169Z 
2020-04-19T09:36:58.4265598Z ------------------------------------------
2020-04-19T09:36:58.4265781Z 
2020-04-19T09:36:58.4265873Z 
2020-04-19T09:36:58.4265873Z 
2020-04-19T09:36:58.4266680Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2020-04-19T09:36:58.4266898Z 
2020-04-19T09:36:58.4267458Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4267746Z status: exit code: 101
2020-04-19T09:36:58.4270441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2020-04-19T09:36:58.4281197Z ------------------------------------------
2020-04-19T09:36:58.4281388Z 
2020-04-19T09:36:58.4281825Z ------------------------------------------
2020-04-19T09:36:58.4282567Z stderr:
2020-04-19T09:36:58.4282567Z stderr:
2020-04-19T09:36:58.4283023Z ------------------------------------------
2020-04-19T09:36:58.4283807Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4284849Z 
2020-04-19T09:36:58.4285060Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4285242Z 
2020-04-19T09:36:58.4285698Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4285698Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4285865Z 
2020-04-19T09:36:58.4286496Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4287216Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4287432Z 
2020-04-19T09:36:58.4287432Z 
2020-04-19T09:36:58.4288484Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4292782Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4293093Z   left: `LLVMing`,
2020-04-19T09:36:58.4293093Z   left: `LLVMing`,
2020-04-19T09:36:58.4293560Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4293952Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4294145Z 
2020-04-19T09:36:58.4294340Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4294646Z 
2020-04-19T09:36:58.4294646Z 
2020-04-19T09:36:58.4295287Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4296050Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4296281Z 
2020-04-19T09:36:58.4296281Z 
2020-04-19T09:36:58.4297153Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4297838Z 
2020-04-19T09:36:58.4298222Z ------------------------------------------
2020-04-19T09:36:58.4298503Z 
2020-04-19T09:36:58.4298593Z 
2020-04-19T09:36:58.4298593Z 
2020-04-19T09:36:58.4299019Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2020-04-19T09:36:58.4299231Z 
2020-04-19T09:36:58.4299694Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4299982Z status: exit code: 101
2020-04-19T09:36:58.4305757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2020-04-19T09:36:58.4310679Z ------------------------------------------
2020-04-19T09:36:58.4310987Z 
2020-04-19T09:36:58.4311492Z ------------------------------------------
2020-04-19T09:36:58.4311714Z stderr:
2020-04-19T09:36:58.4311714Z stderr:
2020-04-19T09:36:58.4312093Z ------------------------------------------
2020-04-19T09:36:58.4312861Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4313774Z 
2020-04-19T09:36:58.4314077Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4314259Z 
2020-04-19T09:36:58.4314443Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4314443Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4314612Z 
2020-04-19T09:36:58.4315257Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4316077Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4316291Z 
2020-04-19T09:36:58.4316291Z 
2020-04-19T09:36:58.4317119Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4318096Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4318401Z   left: `LLVMing`,
2020-04-19T09:36:58.4318401Z   left: `LLVMing`,
2020-04-19T09:36:58.4318883Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4319377Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4319564Z 
2020-04-19T09:36:58.4319751Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4319918Z 
2020-04-19T09:36:58.4319918Z 
2020-04-19T09:36:58.4320501Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4321337Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4321579Z 
2020-04-19T09:36:58.4321579Z 
2020-04-19T09:36:58.4322381Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4322911Z 
2020-04-19T09:36:58.4323302Z ------------------------------------------
2020-04-19T09:36:58.4323459Z 
2020-04-19T09:36:58.4323546Z 
2020-04-19T09:36:58.4323546Z 
2020-04-19T09:36:58.4323955Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2020-04-19T09:36:58.4324276Z 
2020-04-19T09:36:58.4324903Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4325181Z status: exit code: 101
2020-04-19T09:36:58.4327581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2020-04-19T09:36:58.4329455Z ------------------------------------------
2020-04-19T09:36:58.4329610Z 
2020-04-19T09:36:58.4333875Z ------------------------------------------
2020-04-19T09:36:58.4334116Z stderr:
2020-04-19T09:36:58.4334116Z stderr:
2020-04-19T09:36:58.4334622Z ------------------------------------------
2020-04-19T09:36:58.4335346Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `mir_built`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4336069Z 
2020-04-19T09:36:58.4336258Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4336444Z 
2020-04-19T09:36:58.4336771Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4336771Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4337134Z 
2020-04-19T09:36:58.4337814Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4338546Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4338769Z 
2020-04-19T09:36:58.4338769Z 
2020-04-19T09:36:58.4339602Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4340584Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4340861Z   left: `LLVMing`,
2020-04-19T09:36:58.4340861Z   left: `LLVMing`,
2020-04-19T09:36:58.4341290Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4341684Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4342077Z 
2020-04-19T09:36:58.4342294Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4342461Z 
2020-04-19T09:36:58.4342461Z 
2020-04-19T09:36:58.4343097Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4343828Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4344070Z 
2020-04-19T09:36:58.4344070Z 
2020-04-19T09:36:58.4344877Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4345530Z 
2020-04-19T09:36:58.4345883Z ------------------------------------------
2020-04-19T09:36:58.4346503Z 
2020-04-19T09:36:58.4346602Z 
2020-04-19T09:36:58.4346602Z 
2020-04-19T09:36:58.4347246Z ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
2020-04-19T09:36:58.4347449Z 
2020-04-19T09:36:58.4347917Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4348201Z status: exit code: 101
2020-04-19T09:36:58.4350655Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
2020-04-19T09:36:58.4352511Z ------------------------------------------
2020-04-19T09:36:58.4352676Z 
2020-04-19T09:36:58.4353066Z ------------------------------------------
2020-04-19T09:36:58.4353261Z stderr:
2020-04-19T09:36:58.4353261Z stderr:
2020-04-19T09:36:58.4353640Z ------------------------------------------
2020-04-19T09:36:58.4354431Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4355185Z 
2020-04-19T09:36:58.4355446Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4355631Z 
2020-04-19T09:36:58.4355833Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4355833Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4356143Z 
2020-04-19T09:36:58.4356769Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4357648Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4358038Z 
2020-04-19T09:36:58.4358038Z 
2020-04-19T09:36:58.4358926Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4359862Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-19T09:36:58.4360142Z   left: `LLVMing`,
2020-04-19T09:36:58.4360142Z   left: `LLVMing`,
2020-04-19T09:36:58.4360601Z  right: `Codegenning`', /checkout/src/libstd/macros.rs:16:9
2020-04-19T09:36:58.4361023Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4361205Z 
2020-04-19T09:36:58.4361406Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4361587Z 
2020-04-19T09:36:58.4361587Z 
2020-04-19T09:36:58.4362204Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4363049Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T09:36:58.4363292Z 
2020-04-19T09:36:58.4363292Z 
2020-04-19T09:36:58.4364158Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-19T09:36:58.4364972Z 
2020-04-19T09:36:58.4365391Z ------------------------------------------
2020-04-19T09:36:58.4365554Z 
2020-04-19T09:36:58.4365646Z 
2020-04-19T09:36:58.4365646Z 
2020-04-19T09:36:58.4366398Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2020-04-19T09:36:58.4366602Z 
2020-04-19T09:36:58.4367064Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-04-19T09:36:58.4367371Z status: exit code: 101
2020-04-19T09:36:58.4369718Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
2020-04-19T09:36:58.4371533Z ------------------------------------------
2020-04-19T09:36:58.4371697Z 
2020-04-19T09:36:58.4372084Z ------------------------------------------
2020-04-19T09:36:58.4372279Z stderr:
2020-04-19T09:36:58.4372279Z stderr:
2020-04-19T09:36:58.4372657Z ------------------------------------------
2020-04-19T09:36:58.4373458Z thread 'rustc' panicked at 'internal error: entered unreachable code: missing `DepNode` for `promoted_mir`', src/librustc_incremental/persist/dirty_clean.rs:380:24
2020-04-19T09:36:58.4374209Z 
2020-04-19T09:36:58.4374431Z error: internal compiler error: unexpected panic
2020-04-19T09:36:58.4374613Z 
2020-04-19T09:36:58.4374814Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4374814Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T09:36:58.4374996Z 
2020-04-19T09:36:58.4375613Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T09:36:58.4376352Z note: rustc 1.44.0-nightly (7d0d716f9 2020-04-19) running on x86_64-unknown-linux-gnu
---
2020-04-19T09:36:58.4562531Z test result: FAILED. 99 passed; 21 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T09:36:58.4562785Z 
2020-04-19T09:36:58.4562874Z 
2020-04-19T09:36:58.4562961Z 
2020-04-19T09:36:58.4567028Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T09:36:58.4569645Z 
2020-04-19T09:36:58.4569736Z 
2020-04-19T09:36:58.4570296Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T09:36:58.4570624Z Build completed unsuccessfully in 1:05:34
2020-04-19T09:36:58.4570624Z Build completed unsuccessfully in 1:05:34
2020-04-19T09:36:58.4570974Z == clock drift check ==
2020-04-19T09:36:58.4571267Z   local time: Sun Apr 19 09:36:58 UTC 2020
2020-04-19T09:36:58.7174397Z   network time: Sun, 19 Apr 2020 09:36:58 GMT
2020-04-19T09:37:01.4904648Z 
2020-04-19T09:37:01.4904648Z 
2020-04-19T09:37:01.4995861Z ##[error]Bash exited with code '1'.
2020-04-19T09:37:01.5012720Z ##[section]Finishing: Run build
2020-04-19T09:37:01.5067162Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T09:37:01.5072669Z Task         : Get sources
2020-04-19T09:37:01.5073028Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T09:37:01.5073372Z Version      : 1.0.0
2020-04-19T09:37:01.5073620Z Author       : Microsoft
2020-04-19T09:37:01.5073620Z Author       : Microsoft
2020-04-19T09:37:01.5073986Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T09:37:01.5074408Z ==============================================================================
2020-04-19T09:37:01.8757779Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T09:37:01.8812732Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T09:37:01.8892910Z Cleaning up task key
2020-04-19T09:37:01.8894063Z Start cleaning up orphan processes.
2020-04-19T09:37:01.9070366Z Terminate orphan process: pid (5291) (python)
2020-04-19T09:37:01.9460460Z ##[section]Finishing: Finalize Job
