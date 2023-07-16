plain
2020-03-31T21:40:19.7911842Z ========================== Starting Command Output ===========================
2020-03-31T21:40:19.7913562Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d62656d7-44c9-4de5-a535-8479bf8b6083.sh
2020-03-31T21:40:19.7913767Z 
2020-03-31T21:40:19.7916647Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T21:40:19.7931325Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70636/merge to s
2020-03-31T21:40:19.7935441Z Task         : Get sources
2020-03-31T21:40:19.7935668Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T21:40:19.7935876Z Version      : 1.0.0
2020-03-31T21:40:19.7936014Z Author       : Microsoft
---
2020-03-31T21:40:20.8688906Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T21:40:20.9063820Z ##[command]git config gc.auto 0
2020-03-31T21:40:20.9067909Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T21:40:20.9071266Z ##[command]git config --get-all http.proxy
2020-03-31T21:40:20.9078329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70636/merge:refs/remotes/pull/70636/merge
---
2020-03-31T21:46:32.7889054Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:46:33.8291658Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:46:34.9606842Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:46:35.1165870Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:46:42.0055259Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:46:43.1241022Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:46:46.2307371Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:46:49.2175000Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:46:56.2590222Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T22:02:53.2266627Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T22:02:54.4930213Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T22:02:55.9654104Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T22:02:56.8317645Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T22:03:04.9293185Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T22:03:06.7595370Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T22:03:10.5770293Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T22:03:14.6199780Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T22:03:23.0175617Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T22:20:27.1506851Z .................................................................................................... 100/9862
2020-03-31T22:20:34.9888312Z .................................................................................................... 200/9862
2020-03-31T22:20:40.4154568Z .................................................................................................... 300/9862
2020-03-31T22:20:44.1897677Z ..................................................................................F......F.......... 400/9862
2020-03-31T22:20:51.3208766Z ....................F.F.......F....F....F.........F..............FF....F.F..F....................... 500/9862
2020-03-31T22:21:06.1914473Z .................................................................................................... 700/9862
2020-03-31T22:21:10.0685797Z .................................................................................................... 800/9862
2020-03-31T22:21:12.9278173Z .................................................................................................... 900/9862
2020-03-31T22:21:15.6946579Z .......................................................................i............................ 1000/9862
---
2020-03-31T22:21:52.4434041Z .................................................................................................... 1700/9862
2020-03-31T22:21:55.5323465Z .................................................................................................... 1800/9862
2020-03-31T22:22:02.2231593Z ...............................................................................................i.... 1900/9862
2020-03-31T22:22:08.1053742Z .................................................................................................... 2000/9862
2020-03-31T22:22:12.8449366Z .....................................................................................iiiii.......... 2100/9862
2020-03-31T22:22:28.2790224Z .................................................................................................... 2300/9862
2020-03-31T22:22:29.8958972Z .................................................................................................... 2400/9862
2020-03-31T22:22:31.5802271Z .................................................................................................... 2500/9862
2020-03-31T22:22:36.1362410Z .................................................................................................... 2600/9862
---
2020-03-31T22:24:42.5982587Z ...........................................................i...............i........................ 5000/9862
2020-03-31T22:24:48.0511918Z .................................................................................................... 5100/9862
2020-03-31T22:24:53.7290541Z .................................................................................................... 5200/9862
2020-03-31T22:24:57.3739213Z ....i............................................................................................... 5300/9862
2020-03-31T22:25:04.9014800Z ..........................................................................................ii.ii..... 5400/9862
2020-03-31T22:25:07.9458538Z ...i...i............................................................................................ 5500/9862
2020-03-31T22:25:14.3999023Z ...................................i................................................................ 5700/9862
2020-03-31T22:25:21.6841508Z .....................................................ii....................................i........ 5800/9862
2020-03-31T22:25:27.5680232Z .................................................................................................... 5900/9862
2020-03-31T22:25:31.1358603Z .................................................................................................... 6000/9862
2020-03-31T22:25:31.1358603Z .................................................................................................... 6000/9862
2020-03-31T22:25:37.9337387Z .....................................................................................ii...i..ii..... 6100/9862
2020-03-31T22:25:53.5950876Z .................................................................................................... 6300/9862
2020-03-31T22:25:58.8330225Z .................................................................................................... 6400/9862
2020-03-31T22:26:02.0722683Z .................................................................................................... 6500/9862
2020-03-31T22:26:02.0722683Z .................................................................................................... 6500/9862
2020-03-31T22:26:11.3869993Z ...............i..ii................................................................................ 6600/9862
2020-03-31T22:26:26.4752413Z .................................................................................................... 6800/9862
2020-03-31T22:26:28.0052645Z ...............i.................................................................................... 6900/9862
2020-03-31T22:26:29.5078761Z .................................................................................................... 7000/9862
2020-03-31T22:26:31.1434469Z .....................................................i.............................................. 7100/9862
---
2020-03-31T22:27:43.3779528Z .................................................................................................... 7800/9862
2020-03-31T22:27:47.1578229Z .................................................................................................... 7900/9862
2020-03-31T22:27:51.1339174Z .................................................................................................... 8000/9862
2020-03-31T22:27:57.1129383Z .............i...................................................................................... 8100/9862
2020-03-31T22:28:03.2196197Z ..............................................................iiiiiiiiii.i.......................... 8200/9862
2020-03-31T22:28:13.9678497Z ......i......i...................................................................................... 8400/9862
2020-03-31T22:28:17.5189518Z .................................................................................................... 8500/9862
2020-03-31T22:28:26.1389114Z .................................................................................................... 8600/9862
2020-03-31T22:28:34.1508371Z .................................................................................................... 8700/9862
---
2020-03-31T22:29:58.4373886Z ---- [ui] ui/async-await/async-await.rs stdout ----
2020-03-31T22:29:58.4374068Z 
2020-03-31T22:29:58.4374401Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4374638Z status: exit code: 101
2020-03-31T22:29:58.4376583Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await/auxiliary"
2020-03-31T22:29:58.4378059Z ------------------------------------------
2020-03-31T22:29:58.4378206Z 
2020-03-31T22:29:58.4378513Z ------------------------------------------
2020-03-31T22:29:58.4378679Z stderr:
2020-03-31T22:29:58.4378679Z stderr:
2020-03-31T22:29:58.4378972Z ------------------------------------------
2020-03-31T22:29:58.4379399Z error: internal compiler error: src/librustc_codegen_llvm/context.rs:851: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
2020-03-31T22:29:58.4380096Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-31T22:29:58.4380434Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.4380624Z 
2020-03-31T22:29:58.4380802Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4380802Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4380963Z 
2020-03-31T22:29:58.4381536Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4382147Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4382367Z 
2020-03-31T22:29:58.4382367Z 
2020-03-31T22:29:58.4382869Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4383304Z error: aborting due to previous error
2020-03-31T22:29:58.4383452Z 
2020-03-31T22:29:58.4383530Z 
2020-03-31T22:29:58.4383816Z ------------------------------------------
2020-03-31T22:29:58.4383816Z ------------------------------------------
2020-03-31T22:29:58.4383960Z 
2020-03-31T22:29:58.4384055Z 
2020-03-31T22:29:58.4384365Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-03-31T22:29:58.4384521Z 
2020-03-31T22:29:58.4384834Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4385054Z status: signal: 4
2020-03-31T22:29:58.4386561Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-03-31T22:29:58.4387821Z ------------------------------------------
2020-03-31T22:29:58.4387965Z 
2020-03-31T22:29:58.4391053Z ------------------------------------------
2020-03-31T22:29:58.4391254Z stderr:
---
2020-03-31T22:29:58.4402810Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4403162Z 
2020-03-31T22:29:58.4403399Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4403601Z 
2020-03-31T22:29:58.4404410Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4405210Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4405463Z 
2020-03-31T22:29:58.4405463Z 
2020-03-31T22:29:58.4406092Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4406674Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4407198Z   --> /checkout/src/test/ui/async-await/async-closure.rs:49:53
2020-03-31T22:29:58.4407460Z    |
2020-03-31T22:29:58.4407894Z LL |   fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4407894Z LL |   fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4408237Z    |  _____________________________________________________^
2020-03-31T22:29:58.4408711Z LL | |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4409004Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4409407Z LL | |     })(x)
2020-03-31T22:29:58.4409583Z LL | | }
2020-03-31T22:29:58.4409728Z    | |_^
2020-03-31T22:29:58.4409844Z 
2020-03-31T22:29:58.4409844Z 
2020-03-31T22:29:58.4410069Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4410587Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:5
2020-03-31T22:29:58.4410829Z    |
2020-03-31T22:29:58.4411200Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4411489Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4411883Z LL | |     })(x)
2020-03-31T22:29:58.4412070Z    | |_________^
2020-03-31T22:29:58.4412196Z 
2020-03-31T22:29:58.4412406Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4412406Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4412934Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:5
2020-03-31T22:29:58.4413184Z    |
2020-03-31T22:29:58.4413552Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4413832Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4414343Z LL | |     })(x)
2020-03-31T22:29:58.4414500Z    | |______^
2020-03-31T22:29:58.4414625Z 
2020-03-31T22:29:58.4414819Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4414819Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4415370Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:6
2020-03-31T22:29:58.4415576Z    |
2020-03-31T22:29:58.4415874Z LL |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4417087Z 
2020-03-31T22:29:58.4417274Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4417709Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:31
2020-03-31T22:29:58.4417905Z    |
2020-03-31T22:29:58.4417905Z    |
2020-03-31T22:29:58.4418216Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4418432Z    |  _______________________________^
2020-03-31T22:29:58.4418646Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4418985Z LL | |     })(x)
2020-03-31T22:29:58.4419119Z    | |_____^
2020-03-31T22:29:58.4419217Z 
2020-03-31T22:29:58.4419400Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4419400Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4419819Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:31
2020-03-31T22:29:58.4420016Z    |
2020-03-31T22:29:58.4420327Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4420542Z    |  _______________________________^
2020-03-31T22:29:58.4420750Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4421082Z LL | |     })(x)
2020-03-31T22:29:58.4421215Z    | |_____^
2020-03-31T22:29:58.4421312Z 
2020-03-31T22:29:58.4421495Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4421495Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4421912Z   --> /checkout/src/test/ui/async-await/async-closure.rs:50:31
2020-03-31T22:29:58.4422107Z    |
2020-03-31T22:29:58.4422494Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4422711Z    |  _______________________________^
2020-03-31T22:29:58.4422963Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4423298Z LL | |     })(x)
2020-03-31T22:29:58.4423433Z    | |_____^
2020-03-31T22:29:58.4423530Z 
2020-03-31T22:29:58.4423716Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4423716Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4424247Z   --> /checkout/src/test/ui/async-await/async-closure.rs:51:9
2020-03-31T22:29:58.4424414Z    |
2020-03-31T22:29:58.4424566Z LL |         wake_and_yield_once().await;
2020-03-31T22:29:58.4424862Z 
2020-03-31T22:29:58.4425007Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4425380Z   --> /checkout/src/test/ui/async-await/async-closure.rs:51:9
2020-03-31T22:29:58.4425547Z    |
2020-03-31T22:29:58.4425547Z    |
2020-03-31T22:29:58.4425682Z LL |         wake_and_yield_once().await;
2020-03-31T22:29:58.4425995Z 
2020-03-31T22:29:58.4425995Z 
2020-03-31T22:29:58.4426168Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4426732Z    |
2020-03-31T22:29:58.4426732Z    |
2020-03-31T22:29:58.4427028Z LL | / fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4427358Z LL | |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4427564Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4427837Z LL | |     })(x)
2020-03-31T22:29:58.4427961Z LL | | }
2020-03-31T22:29:58.4428061Z    | |_^
2020-03-31T22:29:58.4428138Z 
2020-03-31T22:29:58.4428138Z 
2020-03-31T22:29:58.4428423Z error: internal compiler error: broken MIR in DefId(0:27 ~ async_closure[317d]::async_closure[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4429102Z    |
2020-03-31T22:29:58.4429102Z    |
2020-03-31T22:29:58.4429400Z LL | / fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4429747Z LL | |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4429942Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4430231Z LL | |     })(x)
2020-03-31T22:29:58.4430341Z LL | | }
2020-03-31T22:29:58.4430441Z    | |_^
2020-03-31T22:29:58.4430533Z 
2020-03-31T22:29:58.4430533Z 
2020-03-31T22:29:58.4431528Z error: internal compiler error: broken MIR in DefId(0:27 ~ async_closure[317d]::async_closure[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:49:1: 54:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4432588Z    |
2020-03-31T22:29:58.4432588Z    |
2020-03-31T22:29:58.4432886Z LL | / fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4433222Z LL | |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4433433Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4433704Z LL | |     })(x)
2020-03-31T22:29:58.4433830Z LL | | }
2020-03-31T22:29:58.4433929Z    | |_^
2020-03-31T22:29:58.4434006Z 
2020-03-31T22:29:58.4434006Z 
2020-03-31T22:29:58.4435017Z error: internal compiler error: broken MIR in DefId(0:27 ~ async_closure[317d]::async_closure[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:49:1: 54:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4436075Z    |
2020-03-31T22:29:58.4436075Z    |
2020-03-31T22:29:58.4436372Z LL | / fn async_closure(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4436748Z LL | |     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4436955Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4437266Z LL | |     })(x)
2020-03-31T22:29:58.4437375Z LL | | }
2020-03-31T22:29:58.4437491Z    | |_^
2020-03-31T22:29:58.4437569Z 
2020-03-31T22:29:58.4437569Z 
2020-03-31T22:29:58.4438129Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.4438840Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4439210Z   --> /checkout/src/test/ui/async-await/async-closure.rs:56:69
2020-03-31T22:29:58.4439394Z    |
2020-03-31T22:29:58.4439394Z    |
2020-03-31T22:29:58.4439724Z LL |   fn async_closure_in_unsafe_block(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4440221Z LL | |     (unsafe {
2020-03-31T22:29:58.4440221Z LL | |     (unsafe {
2020-03-31T22:29:58.4440424Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4440618Z LL | |     })(x)
2020-03-31T22:29:58.4440842Z    | |_^
2020-03-31T22:29:58.4440919Z 
2020-03-31T22:29:58.4441221Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4441646Z   --> /checkout/src/test/ui/async-await/async-closure.rs:57:5
2020-03-31T22:29:58.4441646Z   --> /checkout/src/test/ui/async-await/async-closure.rs:57:5
2020-03-31T22:29:58.4441813Z    |
2020-03-31T22:29:58.4442018Z LL | /     (unsafe {
2020-03-31T22:29:58.4442243Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4442439Z LL | |     })(x)
2020-03-31T22:29:58.4442661Z 
2020-03-31T22:29:58.4442808Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4443211Z   --> /checkout/src/test/ui/async-await/async-closure.rs:57:5
2020-03-31T22:29:58.4443385Z    |
2020-03-31T22:29:58.4443385Z    |
2020-03-31T22:29:58.4443510Z LL | /     (unsafe {
2020-03-31T22:29:58.4443713Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4443998Z LL | |     })(x)
2020-03-31T22:29:58.4444215Z 
2020-03-31T22:29:58.4444360Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4444751Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:9
2020-03-31T22:29:58.4444915Z    |
2020-03-31T22:29:58.4444915Z    |
2020-03-31T22:29:58.4445088Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4445501Z 
2020-03-31T22:29:58.4445646Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4446005Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:9
2020-03-31T22:29:58.4446185Z    |
2020-03-31T22:29:58.4446185Z    |
2020-03-31T22:29:58.4446364Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4446683Z 
2020-03-31T22:29:58.4446845Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4448136Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:28
2020-03-31T22:29:58.4448338Z    |
2020-03-31T22:29:58.4448338Z    |
2020-03-31T22:29:58.4448559Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4449023Z 
2020-03-31T22:29:58.4449204Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4449650Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:28
2020-03-31T22:29:58.4449847Z    |
2020-03-31T22:29:58.4449847Z    |
2020-03-31T22:29:58.4450067Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4450526Z 
2020-03-31T22:29:58.4450695Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4451206Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:38
2020-03-31T22:29:58.4451400Z    |
2020-03-31T22:29:58.4451400Z    |
2020-03-31T22:29:58.4451649Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4452850Z 
2020-03-31T22:29:58.4453033Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4453532Z   --> /checkout/src/test/ui/async-await/async-closure.rs:58:38
2020-03-31T22:29:58.4453728Z    |
2020-03-31T22:29:58.4453728Z    |
2020-03-31T22:29:58.4453934Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4454411Z 
2020-03-31T22:29:58.4454411Z 
2020-03-31T22:29:58.4454607Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4455258Z    |
2020-03-31T22:29:58.4455258Z    |
2020-03-31T22:29:58.4455642Z LL | / fn async_closure_in_unsafe_block(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4455894Z LL | |     (unsafe {
2020-03-31T22:29:58.4456679Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4456910Z LL | |     })(x)
2020-03-31T22:29:58.4457170Z    | |_^
2020-03-31T22:29:58.4457261Z 
2020-03-31T22:29:58.4457261Z 
2020-03-31T22:29:58.4457619Z error: internal compiler error: broken MIR in DefId(0:31 ~ async_closure[317d]::async_closure_in_unsafe_block[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4458464Z    |
2020-03-31T22:29:58.4458464Z    |
2020-03-31T22:29:58.4458847Z LL | / fn async_closure_in_unsafe_block(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4459214Z LL | |     (unsafe {
2020-03-31T22:29:58.4459465Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4459691Z LL | |     })(x)
2020-03-31T22:29:58.4459956Z    | |_^
2020-03-31T22:29:58.4460047Z 
2020-03-31T22:29:58.4460047Z 
2020-03-31T22:29:58.4469771Z error: internal compiler error: broken MIR in DefId(0:31 ~ async_closure[317d]::async_closure_in_unsafe_block[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:56:1: 60:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4471251Z    |
2020-03-31T22:29:58.4471251Z    |
2020-03-31T22:29:58.4471665Z LL | / fn async_closure_in_unsafe_block(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4471915Z LL | |     (unsafe {
2020-03-31T22:29:58.4472151Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4472395Z LL | |     })(x)
2020-03-31T22:29:58.4472643Z    | |_^
2020-03-31T22:29:58.4472743Z 
2020-03-31T22:29:58.4472743Z 
2020-03-31T22:29:58.4474624Z error: internal compiler error: broken MIR in DefId(0:31 ~ async_closure[317d]::async_closure_in_unsafe_block[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:56:1: 60:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4476647Z    |
2020-03-31T22:29:58.4476647Z    |
2020-03-31T22:29:58.4477141Z LL | / fn async_closure_in_unsafe_block(x: u8) -> impl Future<Output = u8> {
2020-03-31T22:29:58.4477427Z LL | |     (unsafe {
2020-03-31T22:29:58.4477715Z LL | |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4477977Z LL | |     })(x)
2020-03-31T22:29:58.4478273Z    | |_^
2020-03-31T22:29:58.4478377Z 
2020-03-31T22:29:58.4478377Z 
2020-03-31T22:29:58.4479412Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.4480283Z 
2020-03-31T22:29:58.4480513Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4481299Z    |
2020-03-31T22:29:58.4481299Z    |
2020-03-31T22:29:58.4481647Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4481901Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4482284Z LL | |     })(x)
2020-03-31T22:29:58.4482439Z    | |______^
2020-03-31T22:29:58.4482551Z 
2020-03-31T22:29:58.4482551Z 
2020-03-31T22:29:58.4482981Z error: internal compiler error: broken MIR in DefId(0:29 ~ async_closure[317d]::async_closure[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4484024Z    |
2020-03-31T22:29:58.4484024Z    |
2020-03-31T22:29:58.4484398Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4484653Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4485117Z LL | |     })(x)
2020-03-31T22:29:58.4485272Z    | |______^
2020-03-31T22:29:58.4485384Z 
2020-03-31T22:29:58.4485384Z 
2020-03-31T22:29:58.4486846Z error: internal compiler error: broken MIR in DefId(0:29 ~ async_closure[317d]::async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4488101Z    |
2020-03-31T22:29:58.4488101Z    |
2020-03-31T22:29:58.4488406Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4488625Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4488962Z LL | |     })(x)
2020-03-31T22:29:58.4489099Z    | |______^
2020-03-31T22:29:58.4489196Z 
2020-03-31T22:29:58.4489196Z 
2020-03-31T22:29:58.4490398Z error: internal compiler error: broken MIR in DefId(0:29 ~ async_closure[317d]::async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4491646Z    |
2020-03-31T22:29:58.4491646Z    |
2020-03-31T22:29:58.4491944Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4492164Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4492616Z LL | |     })(x)
2020-03-31T22:29:58.4492750Z    | |______^
2020-03-31T22:29:58.4492847Z 
2020-03-31T22:29:58.4492847Z 
2020-03-31T22:29:58.4494061Z error: internal compiler error: broken MIR in DefId(0:29 ~ async_closure[317d]::async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4495631Z    |
2020-03-31T22:29:58.4495631Z    |
2020-03-31T22:29:58.4495972Z LL | /     (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4496412Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4496792Z LL | |     })(x)
2020-03-31T22:29:58.4497080Z    | |______^
2020-03-31T22:29:58.4497193Z 
2020-03-31T22:29:58.4497193Z 
2020-03-31T22:29:58.4497433Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4498317Z    |
2020-03-31T22:29:58.4498317Z    |
2020-03-31T22:29:58.4498693Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4498937Z    |  _______________________________^
2020-03-31T22:29:58.4499301Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4499681Z LL | |     })(x)
2020-03-31T22:29:58.4499833Z    | |_____^
2020-03-31T22:29:58.4499958Z 
2020-03-31T22:29:58.4499958Z 
2020-03-31T22:29:58.4500368Z error: internal compiler error: broken MIR in DefId(0:30 ~ async_closure[317d]::async_closure[0]::{{closure}}[1]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4501319Z    |
2020-03-31T22:29:58.4501319Z    |
2020-03-31T22:29:58.4501661Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4501906Z    |  _______________________________^
2020-03-31T22:29:58.4502159Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4502644Z LL | |     })(x)
2020-03-31T22:29:58.4502814Z    | |_____^
2020-03-31T22:29:58.4502926Z 
2020-03-31T22:29:58.4502926Z 
2020-03-31T22:29:58.4504317Z error: internal compiler error: broken MIR in DefId(0:30 ~ async_closure[317d]::async_closure[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4505885Z    |
2020-03-31T22:29:58.4505885Z    |
2020-03-31T22:29:58.4506230Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4506593Z    |  _______________________________^
2020-03-31T22:29:58.4506802Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4507137Z LL | |     })(x)
2020-03-31T22:29:58.4507270Z    | |_____^
2020-03-31T22:29:58.4507366Z 
2020-03-31T22:29:58.4507366Z 
2020-03-31T22:29:58.4508819Z error: internal compiler error: broken MIR in DefId(0:30 ~ async_closure[317d]::async_closure[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4510399Z    |
2020-03-31T22:29:58.4510399Z    |
2020-03-31T22:29:58.4510831Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4511045Z    |  _______________________________^
2020-03-31T22:29:58.4511252Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4511687Z LL | |     })(x)
2020-03-31T22:29:58.4511820Z    | |_____^
2020-03-31T22:29:58.4511934Z 
2020-03-31T22:29:58.4511934Z 
2020-03-31T22:29:58.4513138Z error: internal compiler error: broken MIR in DefId(0:30 ~ async_closure[317d]::async_closure[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4514511Z    |
2020-03-31T22:29:58.4514511Z    |
2020-03-31T22:29:58.4514812Z LL |       (async move |x: u8| -> u8 {
2020-03-31T22:29:58.4515026Z    |  _______________________________^
2020-03-31T22:29:58.4515247Z LL | |         wake_and_yield_once().await;
2020-03-31T22:29:58.4515562Z LL | |     })(x)
2020-03-31T22:29:58.4515710Z    | |_____^
2020-03-31T22:29:58.4515807Z 
2020-03-31T22:29:58.4515807Z 
2020-03-31T22:29:58.4516002Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4516712Z    |
2020-03-31T22:29:58.4516712Z    |
2020-03-31T22:29:58.4516953Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4517746Z 
2020-03-31T22:29:58.4517746Z 
2020-03-31T22:29:58.4518312Z error: internal compiler error: broken MIR in DefId(0:33 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4519469Z    |
2020-03-31T22:29:58.4519469Z    |
2020-03-31T22:29:58.4519706Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4520254Z 
2020-03-31T22:29:58.4520254Z 
2020-03-31T22:29:58.4521685Z error: internal compiler error: broken MIR in DefId(0:33 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:9: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4523234Z    |
2020-03-31T22:29:58.4523234Z    |
2020-03-31T22:29:58.4523441Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4523919Z 
2020-03-31T22:29:58.4523919Z 
2020-03-31T22:29:58.4525150Z error: internal compiler error: broken MIR in DefId(0:33 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:9: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4526434Z    |
2020-03-31T22:29:58.4526434Z    |
2020-03-31T22:29:58.4526653Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4527117Z 
2020-03-31T22:29:58.4527117Z 
2020-03-31T22:29:58.4528366Z error: internal compiler error: broken MIR in DefId(0:33 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:9: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4529652Z    |
2020-03-31T22:29:58.4529652Z    |
2020-03-31T22:29:58.4529859Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4530449Z 
2020-03-31T22:29:58.4530449Z 
2020-03-31T22:29:58.4530618Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4531186Z    |
2020-03-31T22:29:58.4531186Z    |
2020-03-31T22:29:58.4531364Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4531774Z 
2020-03-31T22:29:58.4531774Z 
2020-03-31T22:29:58.4532105Z error: internal compiler error: broken MIR in DefId(0:34 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[1]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4532834Z    |
2020-03-31T22:29:58.4532834Z    |
2020-03-31T22:29:58.4533060Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4533508Z 
2020-03-31T22:29:58.4533508Z 
2020-03-31T22:29:58.4534829Z error: internal compiler error: broken MIR in DefId(0:34 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:28: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4536248Z    |
2020-03-31T22:29:58.4536248Z    |
2020-03-31T22:29:58.4536606Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4537087Z 
2020-03-31T22:29:58.4537087Z 
2020-03-31T22:29:58.4538344Z error: internal compiler error: broken MIR in DefId(0:34 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:28: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4539630Z    |
2020-03-31T22:29:58.4539630Z    |
2020-03-31T22:29:58.4539850Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4540312Z 
2020-03-31T22:29:58.4540312Z 
2020-03-31T22:29:58.4541590Z error: internal compiler error: broken MIR in DefId(0:34 ~ async_closure[317d]::async_closure_in_unsafe_block[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/async-closure.rs:58:28: 58:63, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4542865Z    |
2020-03-31T22:29:58.4542865Z    |
2020-03-31T22:29:58.4543087Z LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)
2020-03-31T22:29:58.4543552Z 
2020-03-31T22:29:58.4543552Z 
2020-03-31T22:29:58.4544494Z error: internal compiler error: encountered ambiguity selecting `Binder(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:[type error] [type error]]> as std::future::Future>)` during codegen, presuming due to overflow or prior type error
2020-03-31T22:29:58.4545075Z 
2020-03-31T22:29:58.4545552Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.4545851Z stack backtrace:
2020-03-31T22:29:58.4546401Z    0:     0x7fd605082a26 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hedd7987763202137
2020-03-31T22:29:58.4546993Z    1:     0x7fd6050bec3c - core::fmt::write::hcbb365aadcb052db
2020-03-31T22:29:58.4547481Z    2:     0x7fd6050738b3 - std::io::Write::write_fmt::h47594015eba09c96
2020-03-31T22:29:58.4548112Z    3:     0x7fd605087ae4 - std::panicking::default_hook::{{closure}}::h0bfd476417993f67
2020-03-31T22:29:58.4548585Z    4:     0x7fd605087827 - std::panicking::default_hook::hcbdac96020d13bec
2020-03-31T22:29:58.4549017Z    5:     0x7fd6056b57fc - rustc_driver::report_ice::hdc3dc7b5a29a3c63
2020-03-31T22:29:58.4549467Z    6:     0x7fd605088268 - std::panicking::rust_panic_with_hook::h14f12fcd8b0ecabd
2020-03-31T22:29:58.4549932Z    7:     0x7fd60796d09e - std::panicking::begin_panic::hac43da55b48b3482
2020-03-31T22:29:58.4550498Z    8:     0x7fd607977bd4 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hc439f43c5cb6a94b
2020-03-31T22:29:58.4551012Z    9:     0x7fd6056f91f2 - core::ptr::drop_in_place::hbf138e6c6846e5f5
2020-03-31T22:29:58.4551518Z   10:     0x7fd6056fba1c - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h7a2b9f1cda01e4e7
2020-03-31T22:29:58.4551987Z   11:     0x7fd60571204c - core::ptr::drop_in_place::h0725d5e5e86463b4
2020-03-31T22:29:58.4552497Z   12:     0x7fd60570ef44 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h2bab91bb94f90ffb
2020-03-31T22:29:58.4553009Z   13:     0x7fd6056bcb5b - scoped_tls::ScopedKey<T>::set::h6911964dfbd9c9e0
2020-03-31T22:29:58.4553454Z   14:     0x7fd6056b9e36 - rustc_ast::attr::with_globals::h6c02a6cc13247e64
2020-03-31T22:29:58.4553971Z   15:     0x7fd6056c4457 - std::sys_common::backtrace::__rust_begin_short_backtrace::h4b9133bac4c50c08
2020-03-31T22:29:58.4554438Z   16:     0x7fd6056bd6c6 - std::panicking::try::h64db7b3b057eaefa
2020-03-31T22:29:58.4554927Z   17:     0x7fd60571164e - core::ops::function::FnOnce::call_once{{vtable.shim}}::ha1905f4e3cc1ac05
2020-03-31T22:29:58.4555510Z   18:     0x7fd60506282f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h05c3f6e2d16cce0c
2020-03-31T22:29:58.4556064Z   19:     0x7fd605098403 - std::sys::unix::thread::Thread::new::thread_start::h6001603e80018d44
2020-03-31T22:29:58.4556453Z   20:     0x7fd6003266db - start_thread
2020-03-31T22:29:58.4556762Z   21:     0x7fd604d4288f - __clone
2020-03-31T22:29:58.4557044Z   22:                0x0 - <unknown>
2020-03-31T22:29:58.4557312Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4558079Z 
2020-03-31T22:29:58.4558300Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4558500Z 
2020-03-31T22:29:58.4558500Z 
2020-03-31T22:29:58.4559169Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4560206Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4560478Z 
2020-03-31T22:29:58.4560478Z 
2020-03-31T22:29:58.4561170Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4561715Z thread panicked while panicking. aborting.
2020-03-31T22:29:58.4561888Z 
2020-03-31T22:29:58.4562262Z ------------------------------------------
2020-03-31T22:29:58.4562435Z 
2020-03-31T22:29:58.4562435Z 
2020-03-31T22:29:58.4562534Z 
2020-03-31T22:29:58.4562915Z ---- [ui] ui/async-await/issue-60709.rs stdout ----
2020-03-31T22:29:58.4563120Z 
2020-03-31T22:29:58.4563507Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4563772Z status: exit code: 101
2020-03-31T22:29:58.4565708Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-60709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=z" "-Cdebuginfo=2" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/auxiliary"
2020-03-31T22:29:58.4567286Z ------------------------------------------
2020-03-31T22:29:58.4567460Z 
2020-03-31T22:29:58.4567815Z ------------------------------------------
2020-03-31T22:29:58.4568033Z stderr:
2020-03-31T22:29:58.4568033Z stderr:
2020-03-31T22:29:58.4568398Z ------------------------------------------
2020-03-31T22:29:58.4568914Z error: internal compiler error: src/librustc_codegen_llvm/context.rs:851: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
2020-03-31T22:29:58.4570009Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-31T22:29:58.4570508Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.4570742Z 
2020-03-31T22:29:58.4571127Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4571127Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4571313Z 
2020-03-31T22:29:58.4571885Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4572595Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4572830Z 
2020-03-31T22:29:58.4572830Z 
2020-03-31T22:29:58.4573460Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C opt-level=z -C debuginfo=2
2020-03-31T22:29:58.4573988Z error: aborting due to previous error
2020-03-31T22:29:58.4574142Z 
2020-03-31T22:29:58.4574233Z 
2020-03-31T22:29:58.4574581Z ------------------------------------------
2020-03-31T22:29:58.4574581Z ------------------------------------------
2020-03-31T22:29:58.4574741Z 
2020-03-31T22:29:58.4574838Z 
2020-03-31T22:29:58.4575195Z ---- [ui] ui/async-await/issue-61793.rs stdout ----
2020-03-31T22:29:58.4575384Z 
2020-03-31T22:29:58.4575747Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4575991Z status: exit code: 101
2020-03-31T22:29:58.4581277Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61793.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793/auxiliary"
2020-03-31T22:29:58.4582789Z ------------------------------------------
2020-03-31T22:29:58.4582967Z 
2020-03-31T22:29:58.4583303Z ------------------------------------------
2020-03-31T22:29:58.4583509Z stderr:
2020-03-31T22:29:58.4583509Z stderr:
2020-03-31T22:29:58.4583852Z ------------------------------------------
2020-03-31T22:29:58.4584325Z error: internal compiler error: src/librustc_codegen_llvm/context.rs:851: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
2020-03-31T22:29:58.4585147Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-31T22:29:58.4585508Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.4585726Z 
2020-03-31T22:29:58.4585943Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4585943Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4586131Z 
2020-03-31T22:29:58.4586719Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4587487Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4587719Z 
2020-03-31T22:29:58.4587719Z 
2020-03-31T22:29:58.4588321Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4588823Z error: aborting due to previous error
2020-03-31T22:29:58.4588978Z 
2020-03-31T22:29:58.4589068Z 
2020-03-31T22:29:58.4589417Z ------------------------------------------
2020-03-31T22:29:58.4589417Z ------------------------------------------
2020-03-31T22:29:58.4589575Z 
2020-03-31T22:29:58.4589666Z 
2020-03-31T22:29:58.4590052Z ---- [ui] ui/async-await/issue-64130-4-async-move.rs stdout ----
2020-03-31T22:29:58.4590265Z 
2020-03-31T22:29:58.4590508Z error: Error: expected failure status (Some(1)) but received status Some(101).
---
2020-03-31T22:29:58.4617054Z    |
2020-03-31T22:29:58.4617238Z LL |                 let _x = get().await;
2020-03-31T22:29:58.4617481Z    |                          ^^^^^^^^^^^
2020-03-31T22:29:58.4617653Z 
2020-03-31T22:29:58.4617878Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4618684Z    |
2020-03-31T22:29:58.4619028Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4619028Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4619319Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4619647Z LL | |     let client = Client(Box::new(true));
2020-03-31T22:29:58.4619888Z LL | |     async move {
2020-03-31T22:29:58.4620200Z LL | |     }
2020-03-31T22:29:58.4620341Z LL | | }
2020-03-31T22:29:58.4620480Z    | |_^
2020-03-31T22:29:58.4620584Z 
2020-03-31T22:29:58.4620584Z 
2020-03-31T22:29:58.4620984Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_64130_4_async_move[317d]::foo[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4621936Z    |
2020-03-31T22:29:58.4622294Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4622294Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4622581Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4622890Z LL | |     let client = Client(Box::new(true));
2020-03-31T22:29:58.4623147Z LL | |     async move {
2020-03-31T22:29:58.4623440Z LL | |     }
2020-03-31T22:29:58.4623595Z LL | | }
2020-03-31T22:29:58.4623728Z    | |_^
2020-03-31T22:29:58.4623832Z 
2020-03-31T22:29:58.4623832Z 
2020-03-31T22:29:58.4625262Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_64130_4_async_move[317d]::foo[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-64130-4-async-move.rs:15:1: 26:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4626807Z    |
2020-03-31T22:29:58.4627767Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4627767Z LL | / pub fn foo() -> impl Future + Send {
2020-03-31T22:29:58.4628055Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4628364Z LL | |     let client = Client(Box::new(true));
2020-03-31T22:29:58.4628618Z LL | |     async move {
2020-03-31T22:29:58.4628910Z LL | |     }
2020-03-31T22:29:58.4629065Z LL | | }
2020-03-31T22:29:58.4629198Z    | |_^
2020-03-31T22:29:58.4629301Z 
2020-03-31T22:29:58.4629301Z 
2020-03-31T22:29:58.4630532Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Send>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.4631509Z 
2020-03-31T22:29:58.4631736Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4632572Z    |
2020-03-31T22:29:58.4632728Z LL |       async move {
2020-03-31T22:29:58.4632924Z    |  ________________^
2020-03-31T22:29:58.4633132Z LL | |         match client.status() {
2020-03-31T22:29:58.4633132Z LL | |         match client.status() {
2020-03-31T22:29:58.4633346Z LL | |             200 => {
2020-03-31T22:29:58.4633586Z LL | |                 let _x = get().await;
2020-03-31T22:29:58.4633780Z ...  |
2020-03-31T22:29:58.4633923Z LL | |         }
2020-03-31T22:29:58.4634092Z LL | |     }
2020-03-31T22:29:58.4634242Z    | |_____^
2020-03-31T22:29:58.4634354Z 
2020-03-31T22:29:58.4634768Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_64130_4_async_move[317d]::foo[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4635770Z    |
2020-03-31T22:29:58.4635923Z LL |       async move {
2020-03-31T22:29:58.4636121Z    |  ________________^
2020-03-31T22:29:58.4636552Z LL | |         match client.status() {
2020-03-31T22:29:58.4636552Z LL | |         match client.status() {
2020-03-31T22:29:58.4636768Z LL | |             200 => {
2020-03-31T22:29:58.4637007Z LL | |                 let _x = get().await;
2020-03-31T22:29:58.4637197Z ...  |
2020-03-31T22:29:58.4637340Z LL | |         }
2020-03-31T22:29:58.4637507Z LL | |     }
2020-03-31T22:29:58.4637776Z    | |_____^
2020-03-31T22:29:58.4637954Z 
2020-03-31T22:29:58.4639417Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_64130_4_async_move[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-64130-4-async-move.rs:18:16: 25:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4640913Z    |
2020-03-31T22:29:58.4641065Z LL |       async move {
2020-03-31T22:29:58.4641248Z    |  ________________^
2020-03-31T22:29:58.4641430Z LL | |         match client.status() {
2020-03-31T22:29:58.4641430Z LL | |         match client.status() {
2020-03-31T22:29:58.4641630Z LL | |             200 => {
2020-03-31T22:29:58.4641826Z LL | |                 let _x = get().await;
2020-03-31T22:29:58.4641992Z ...  |
2020-03-31T22:29:58.4642131Z LL | |         }
2020-03-31T22:29:58.4642265Z LL | |     }
2020-03-31T22:29:58.4642462Z    | |_____^
2020-03-31T22:29:58.4642559Z 
2020-03-31T22:29:58.4643868Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_64130_4_async_move[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-64130-4-async-move.rs:18:16: 25:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4645206Z    |
2020-03-31T22:29:58.4645342Z LL |       async move {
2020-03-31T22:29:58.4645502Z    |  ________________^
2020-03-31T22:29:58.4645699Z LL | |         match client.status() {
2020-03-31T22:29:58.4645699Z LL | |         match client.status() {
2020-03-31T22:29:58.4645884Z LL | |             200 => {
2020-03-31T22:29:58.4646079Z LL | |                 let _x = get().await;
2020-03-31T22:29:58.4646257Z ...  |
2020-03-31T22:29:58.4646382Z LL | |         }
2020-03-31T22:29:58.4646521Z LL | |     }
2020-03-31T22:29:58.4646661Z    | |_____^
2020-03-31T22:29:58.4646757Z 
2020-03-31T22:29:58.4647990Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_64130_4_async_move[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-64130-4-async-move.rs:18:16: 25:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4649325Z    |
2020-03-31T22:29:58.4649459Z LL |       async move {
2020-03-31T22:29:58.4649751Z    |  ________________^
2020-03-31T22:29:58.4649906Z LL | |         match client.status() {
2020-03-31T22:29:58.4649906Z LL | |         match client.status() {
2020-03-31T22:29:58.4650065Z LL | |             200 => {
2020-03-31T22:29:58.4650246Z LL | |                 let _x = get().await;
2020-03-31T22:29:58.4650391Z ...  |
2020-03-31T22:29:58.4650497Z LL | |         }
2020-03-31T22:29:58.4650626Z LL | |     }
2020-03-31T22:29:58.4650735Z    | |_____^
2020-03-31T22:29:58.4650821Z 
2020-03-31T22:29:58.4651226Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.4651842Z 
2020-03-31T22:29:58.4651992Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4652142Z 
2020-03-31T22:29:58.4652293Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4652293Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4652431Z 
2020-03-31T22:29:58.4652876Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4653400Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4653572Z 
2020-03-31T22:29:58.4653572Z 
2020-03-31T22:29:58.4654020Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4654338Z 
2020-03-31T22:29:58.4654585Z ------------------------------------------
2020-03-31T22:29:58.4654719Z 
2020-03-31T22:29:58.4654785Z 
2020-03-31T22:29:58.4654785Z 
2020-03-31T22:29:58.4655078Z ---- [ui] ui/async-await/issue-67252-unnamed-future.rs stdout ----
2020-03-31T22:29:58.4655228Z 
2020-03-31T22:29:58.4655424Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-31T22:29:58.4655637Z status: exit code: 101
2020-03-31T22:29:58.4657665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future/auxiliary"
2020-03-31T22:29:58.4659384Z ------------------------------------------
2020-03-31T22:29:58.4659548Z 
2020-03-31T22:29:58.4659880Z ------------------------------------------
2020-03-31T22:29:58.4660069Z stderr:
2020-03-31T22:29:58.4660069Z stderr:
2020-03-31T22:29:58.4660425Z ------------------------------------------
2020-03-31T22:29:58.4660687Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4661206Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16
2020-03-31T22:29:58.4661479Z    |
2020-03-31T22:29:58.4661635Z LL |   async fn foo() {
2020-03-31T22:29:58.4661818Z    |  ________________^
2020-03-31T22:29:58.4662107Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4662497Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4662803Z LL | |         AFuture.await;
2020-03-31T22:29:58.4663142Z LL | | }
2020-03-31T22:29:58.4663275Z    | |_^
2020-03-31T22:29:58.4663379Z 
2020-03-31T22:29:58.4663587Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4663587Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4664356Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16
2020-03-31T22:29:58.4664621Z    |
2020-03-31T22:29:58.4664791Z LL |   async fn foo() {
2020-03-31T22:29:58.4664972Z    |  ________________^
2020-03-31T22:29:58.4665245Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4665642Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4665942Z LL | |         AFuture.await;
2020-03-31T22:29:58.4666277Z LL | | }
2020-03-31T22:29:58.4666416Z    | |_^
2020-03-31T22:29:58.4666519Z 
2020-03-31T22:29:58.4666713Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4666713Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4667268Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16
2020-03-31T22:29:58.4667524Z    |
2020-03-31T22:29:58.4667677Z LL |   async fn foo() {
2020-03-31T22:29:58.4667876Z    |  ________________^
2020-03-31T22:29:58.4668150Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4668531Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4668849Z LL | |         AFuture.await;
2020-03-31T22:29:58.4669168Z LL | | }
2020-03-31T22:29:58.4669315Z    | |_^
2020-03-31T22:29:58.4669419Z 
2020-03-31T22:29:58.4669612Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4669612Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4670128Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:5
2020-03-31T22:29:58.4670392Z    |
2020-03-31T22:29:58.4670637Z LL |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4671025Z 
2020-03-31T22:29:58.4671305Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4671755Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:11
2020-03-31T22:29:58.4671974Z    |
2020-03-31T22:29:58.4671974Z    |
2020-03-31T22:29:58.4672200Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4672430Z    |  ___________^
2020-03-31T22:29:58.4672679Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4672957Z LL | |         AFuture.await;
2020-03-31T22:29:58.4673241Z    | |_____^
2020-03-31T22:29:58.4673352Z 
2020-03-31T22:29:58.4673523Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4673976Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4673976Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4674261Z    |
2020-03-31T22:29:58.4674472Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4674761Z    |  _________________^
2020-03-31T22:29:58.4675034Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4675297Z LL | |         AFuture.await;
2020-03-31T22:29:58.4675581Z    | |_____^
2020-03-31T22:29:58.4675691Z 
2020-03-31T22:29:58.4675858Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4676319Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4676319Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4676552Z    |
2020-03-31T22:29:58.4676765Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4677004Z    |  _________________^
2020-03-31T22:29:58.4677278Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4677538Z LL | |         AFuture.await;
2020-03-31T22:29:58.4677841Z    | |_____^
2020-03-31T22:29:58.4677937Z 
2020-03-31T22:29:58.4678108Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4678560Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4678560Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17
2020-03-31T22:29:58.4678795Z    |
2020-03-31T22:29:58.4679007Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4679245Z    |  _________________^
2020-03-31T22:29:58.4679515Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4679778Z LL | |         AFuture.await;
2020-03-31T22:29:58.4680076Z    | |_____^
2020-03-31T22:29:58.4680173Z 
2020-03-31T22:29:58.4680342Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4680791Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:20:9
2020-03-31T22:29:58.4680791Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:20:9
2020-03-31T22:29:58.4681022Z    |
2020-03-31T22:29:58.4681160Z LL |         AFuture.await;
2020-03-31T22:29:58.4681463Z 
2020-03-31T22:29:58.4681632Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4682084Z   --> /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:20:9
2020-03-31T22:29:58.4682302Z    |
2020-03-31T22:29:58.4682302Z    |
2020-03-31T22:29:58.4682455Z LL |         AFuture.await;
2020-03-31T22:29:58.4682742Z 
2020-03-31T22:29:58.4682742Z 
2020-03-31T22:29:58.4682952Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4683643Z    |
2020-03-31T22:29:58.4683790Z LL | / async fn foo() {
2020-03-31T22:29:58.4683790Z LL | / async fn foo() {
2020-03-31T22:29:58.4684031Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4684362Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4684637Z LL | |         AFuture.await;
2020-03-31T22:29:58.4684920Z LL | | }
2020-03-31T22:29:58.4685036Z    | |_^
2020-03-31T22:29:58.4685139Z 
2020-03-31T22:29:58.4685139Z 
2020-03-31T22:29:58.4685478Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_67252_unnamed_future[317d]::foo[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4686332Z    |
2020-03-31T22:29:58.4686465Z LL | / async fn foo() {
2020-03-31T22:29:58.4686465Z LL | / async fn foo() {
2020-03-31T22:29:58.4686703Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4687047Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4687309Z LL | |         AFuture.await;
2020-03-31T22:29:58.4687716Z LL | | }
2020-03-31T22:29:58.4687816Z    | |_^
2020-03-31T22:29:58.4687894Z 
2020-03-31T22:29:58.4687894Z 
2020-03-31T22:29:58.4688981Z error: internal compiler error: broken MIR in DefId(0:15 ~ issue_67252_unnamed_future[317d]::foo[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:1: 22:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4690130Z    |
2020-03-31T22:29:58.4690262Z LL | / async fn foo() {
2020-03-31T22:29:58.4690262Z LL | / async fn foo() {
2020-03-31T22:29:58.4690468Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4690769Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4690998Z LL | |         AFuture.await;
2020-03-31T22:29:58.4691415Z LL | | }
2020-03-31T22:29:58.4691546Z    | |_^
2020-03-31T22:29:58.4691636Z 
2020-03-31T22:29:58.4691636Z 
2020-03-31T22:29:58.4692289Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.4692941Z 
2020-03-31T22:29:58.4693138Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4693858Z    |
2020-03-31T22:29:58.4693992Z LL |   async fn foo() {
2020-03-31T22:29:58.4694153Z    |  ________________^
2020-03-31T22:29:58.4694153Z    |  ________________^
2020-03-31T22:29:58.4694404Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4694736Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4695001Z LL | |         AFuture.await;
2020-03-31T22:29:58.4695295Z LL | | }
2020-03-31T22:29:58.4695411Z    | |_^
2020-03-31T22:29:58.4695505Z 
2020-03-31T22:29:58.4695505Z 
2020-03-31T22:29:58.4695974Z error: internal compiler error: broken MIR in DefId(0:17 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4696890Z    |
2020-03-31T22:29:58.4697003Z LL |   async fn foo() {
2020-03-31T22:29:58.4697139Z    |  ________________^
2020-03-31T22:29:58.4697139Z    |  ________________^
2020-03-31T22:29:58.4697344Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4697645Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4697869Z LL | |         AFuture.await;
2020-03-31T22:29:58.4698122Z LL | | }
2020-03-31T22:29:58.4698221Z    | |_^
2020-03-31T22:29:58.4698298Z 
2020-03-31T22:29:58.4698298Z 
2020-03-31T22:29:58.4699390Z error: internal compiler error: broken MIR in DefId(0:17 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16: 22:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4700553Z    |
2020-03-31T22:29:58.4700670Z LL |   async fn foo() {
2020-03-31T22:29:58.4700806Z    |  ________________^
2020-03-31T22:29:58.4700806Z    |  ________________^
2020-03-31T22:29:58.4701025Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4701316Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4701545Z LL | |         AFuture.await;
2020-03-31T22:29:58.4701798Z LL | | }
2020-03-31T22:29:58.4701897Z    | |_^
2020-03-31T22:29:58.4701974Z 
2020-03-31T22:29:58.4701974Z 
2020-03-31T22:29:58.4703108Z error: internal compiler error: broken MIR in DefId(0:17 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16: 22:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4704306Z    |
2020-03-31T22:29:58.4704423Z LL |   async fn foo() {
2020-03-31T22:29:58.4704561Z    |  ________________^
2020-03-31T22:29:58.4704561Z    |  ________________^
2020-03-31T22:29:58.4704781Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4705068Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4705295Z LL | |         AFuture.await;
2020-03-31T22:29:58.4705552Z LL | | }
2020-03-31T22:29:58.4705656Z    | |_^
2020-03-31T22:29:58.4705749Z 
2020-03-31T22:29:58.4705749Z 
2020-03-31T22:29:58.4706823Z error: internal compiler error: broken MIR in DefId(0:17 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:17:16: 22:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4707982Z    |
2020-03-31T22:29:58.4708099Z LL |   async fn foo() {
2020-03-31T22:29:58.4708250Z    |  ________________^
2020-03-31T22:29:58.4708250Z    |  ________________^
2020-03-31T22:29:58.4708454Z LL | |     spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4708742Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4708983Z LL | |         AFuture.await;
2020-03-31T22:29:58.4709229Z LL | | }
2020-03-31T22:29:58.4709341Z    | |_^
2020-03-31T22:29:58.4709419Z 
2020-03-31T22:29:58.4709419Z 
2020-03-31T22:29:58.4709590Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4710204Z    |
2020-03-31T22:29:58.4710204Z    |
2020-03-31T22:29:58.4710387Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4710593Z    |  _________________^
2020-03-31T22:29:58.4710829Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4711055Z LL | |         AFuture.await;
2020-03-31T22:29:58.4711314Z    | |_____^
2020-03-31T22:29:58.4711398Z 
2020-03-31T22:29:58.4711398Z 
2020-03-31T22:29:58.4711735Z error: internal compiler error: broken MIR in DefId(0:18 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4712528Z    |
2020-03-31T22:29:58.4712528Z    |
2020-03-31T22:29:58.4712715Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4712934Z    |  _________________^
2020-03-31T22:29:58.4713157Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4713384Z LL | |         AFuture.await;
2020-03-31T22:29:58.4713642Z    | |_____^
2020-03-31T22:29:58.4713896Z 
2020-03-31T22:29:58.4713896Z 
2020-03-31T22:29:58.4715953Z error: internal compiler error: broken MIR in DefId(0:18 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17: 21:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4717787Z    |
2020-03-31T22:29:58.4717787Z    |
2020-03-31T22:29:58.4718071Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4718369Z    |  _________________^
2020-03-31T22:29:58.4718693Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4719035Z LL | |         AFuture.await;
2020-03-31T22:29:58.4719389Z    | |_____^
2020-03-31T22:29:58.4719524Z 
2020-03-31T22:29:58.4719524Z 
2020-03-31T22:29:58.4721121Z error: internal compiler error: broken MIR in DefId(0:18 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17: 21:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4722831Z    |
2020-03-31T22:29:58.4722831Z    |
2020-03-31T22:29:58.4723098Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4723412Z    |  _________________^
2020-03-31T22:29:58.4723730Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4724056Z LL | |         AFuture.await;
2020-03-31T22:29:58.4724424Z    | |_____^
2020-03-31T22:29:58.4724544Z 
2020-03-31T22:29:58.4724544Z 
2020-03-31T22:29:58.4726149Z error: internal compiler error: broken MIR in DefId(0:18 ~ issue_67252_unnamed_future[317d]::foo[0]::{{closure}}[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17: 21:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4727783Z    |
2020-03-31T22:29:58.4727783Z    |
2020-03-31T22:29:58.4727967Z LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4728173Z    |  _________________^
2020-03-31T22:29:58.4728407Z LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
2020-03-31T22:29:58.4728633Z LL | |         AFuture.await;
2020-03-31T22:29:58.4728894Z    | |_____^
2020-03-31T22:29:58.4728977Z 
2020-03-31T22:29:58.4728977Z 
2020-03-31T22:29:58.4729381Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.4729890Z 
2020-03-31T22:29:58.4730040Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4730179Z 
2020-03-31T22:29:58.4730344Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4730344Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4730487Z 
2020-03-31T22:29:58.4730914Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4731451Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4731626Z 
2020-03-31T22:29:58.4731626Z 
2020-03-31T22:29:58.4732060Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4732389Z 
2020-03-31T22:29:58.4732639Z ------------------------------------------
2020-03-31T22:29:58.4732759Z 
2020-03-31T22:29:58.4732827Z 
2020-03-31T22:29:58.4732827Z 
2020-03-31T22:29:58.4733119Z ---- [ui] ui/async-await/issues/issue-53249.rs stdout ----
2020-03-31T22:29:58.4733259Z 
2020-03-31T22:29:58.4733529Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4733769Z status: exit code: 101
2020-03-31T22:29:58.4735230Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-53249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-53249" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-53249/auxiliary"
2020-03-31T22:29:58.4736511Z ------------------------------------------
2020-03-31T22:29:58.4736634Z 
2020-03-31T22:29:58.4736896Z ------------------------------------------
2020-03-31T22:29:58.4737038Z stderr:
2020-03-31T22:29:58.4737038Z stderr:
2020-03-31T22:29:58.4737505Z ------------------------------------------
2020-03-31T22:29:58.4737756Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4738188Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:5
2020-03-31T22:29:58.4738394Z    |
2020-03-31T22:29:58.4738586Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4738981Z 
2020-03-31T22:29:58.4739151Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4739593Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:11
2020-03-31T22:29:58.4739796Z    |
2020-03-31T22:29:58.4739796Z    |
2020-03-31T22:29:58.4739974Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4740407Z 
2020-03-31T22:29:58.4740578Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4741024Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:11
2020-03-31T22:29:58.4741233Z    |
2020-03-31T22:29:58.4741233Z    |
2020-03-31T22:29:58.4741408Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4741818Z 
2020-03-31T22:29:58.4741985Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4742412Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:11
2020-03-31T22:29:58.4742627Z    |
2020-03-31T22:29:58.4742627Z    |
2020-03-31T22:29:58.4742801Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4743207Z 
2020-03-31T22:29:58.4743376Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4743804Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4744006Z    |
2020-03-31T22:29:58.4744006Z    |
2020-03-31T22:29:58.4744196Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4744568Z 
2020-03-31T22:29:58.4744756Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4745185Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4745391Z    |
2020-03-31T22:29:58.4745391Z    |
2020-03-31T22:29:58.4745579Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4745902Z 
2020-03-31T22:29:58.4746070Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4746512Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:23
2020-03-31T22:29:58.4746715Z    |
2020-03-31T22:29:58.4746715Z    |
2020-03-31T22:29:58.4746891Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4747276Z 
2020-03-31T22:29:58.4747446Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4747890Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:27
2020-03-31T22:29:58.4748093Z    |
2020-03-31T22:29:58.4748093Z    |
2020-03-31T22:29:58.4748268Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4748707Z 
2020-03-31T22:29:58.4748913Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4749432Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:33
2020-03-31T22:29:58.4749619Z    |
2020-03-31T22:29:58.4749619Z    |
2020-03-31T22:29:58.4749771Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4750089Z 
2020-03-31T22:29:58.4750248Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4750616Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:33
2020-03-31T22:29:58.4750790Z    |
2020-03-31T22:29:58.4750790Z    |
2020-03-31T22:29:58.4750959Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4751278Z 
2020-03-31T22:29:58.4751437Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4751802Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4751980Z    |
2020-03-31T22:29:58.4751980Z    |
2020-03-31T22:29:58.4752132Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4752483Z 
2020-03-31T22:29:58.4752627Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4753008Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4753182Z    |
2020-03-31T22:29:58.4753182Z    |
2020-03-31T22:29:58.4753333Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4753680Z 
2020-03-31T22:29:58.4753827Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4754192Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4754380Z    |
2020-03-31T22:29:58.4754380Z    |
2020-03-31T22:29:58.4754531Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4754881Z 
2020-03-31T22:29:58.4755028Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4755397Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4755586Z    |
2020-03-31T22:29:58.4755586Z    |
2020-03-31T22:29:58.4755738Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4756072Z 
2020-03-31T22:29:58.4756231Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4756597Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4756771Z    |
2020-03-31T22:29:58.4756771Z    |
2020-03-31T22:29:58.4756936Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4757269Z 
2020-03-31T22:29:58.4757415Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4757796Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4757973Z    |
2020-03-31T22:29:58.4757973Z    |
2020-03-31T22:29:58.4758124Z LL |     async { __receive(|_| async { () }).await };
2020-03-31T22:29:58.4758474Z 
2020-03-31T22:29:58.4758618Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4758999Z   --> /checkout/src/test/ui/async-await/issues/issue-53249.rs:44:13
2020-03-31T22:29:58.4759174Z    |
---
2020-03-31T22:29:58.4852220Z ---- [ui] ui/async-await/issues/issue-64477.rs stdout ----
2020-03-31T22:29:58.4852364Z 
2020-03-31T22:29:58.4852646Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4852828Z status: exit code: 101
2020-03-31T22:29:58.4854243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64477.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64477" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64477/auxiliary"
2020-03-31T22:29:58.4855409Z ------------------------------------------
2020-03-31T22:29:58.4855531Z 
2020-03-31T22:29:58.4855778Z ------------------------------------------
2020-03-31T22:29:58.4855957Z stderr:
2020-03-31T22:29:58.4855957Z stderr:
2020-03-31T22:29:58.4856379Z ------------------------------------------
2020-03-31T22:29:58.4856626Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4857001Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:16:69
2020-03-31T22:29:58.4857192Z    |
2020-03-31T22:29:58.4857523Z LL |   pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
2020-03-31T22:29:58.4857801Z    |  _____________________________________________________________________^
2020-03-31T22:29:58.4858031Z LL | |     async move { f(x).await }
2020-03-31T22:29:58.4858265Z    | |_^
2020-03-31T22:29:58.4858344Z 
2020-03-31T22:29:58.4858504Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4858874Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:5
2020-03-31T22:29:58.4858874Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:5
2020-03-31T22:29:58.4859047Z    |
2020-03-31T22:29:58.4859191Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4859467Z 
2020-03-31T22:29:58.4859627Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4859998Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16
2020-03-31T22:29:58.4860173Z    |
2020-03-31T22:29:58.4860173Z    |
2020-03-31T22:29:58.4860316Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4860589Z 
2020-03-31T22:29:58.4860733Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4861116Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16
2020-03-31T22:29:58.4861291Z    |
2020-03-31T22:29:58.4861291Z    |
2020-03-31T22:29:58.4861420Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4861707Z 
2020-03-31T22:29:58.4861852Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4862218Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16
2020-03-31T22:29:58.4862405Z    |
2020-03-31T22:29:58.4862405Z    |
2020-03-31T22:29:58.4862535Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4862823Z 
2020-03-31T22:29:58.4862971Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4863338Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:18
2020-03-31T22:29:58.4863512Z    |
2020-03-31T22:29:58.4863512Z    |
2020-03-31T22:29:58.4863654Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4863923Z 
2020-03-31T22:29:58.4864084Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4864453Z   --> /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:18
2020-03-31T22:29:58.4864628Z    |
2020-03-31T22:29:58.4864628Z    |
2020-03-31T22:29:58.4864771Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4865040Z 
2020-03-31T22:29:58.4865040Z 
2020-03-31T22:29:58.4865210Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4865792Z    |
2020-03-31T22:29:58.4865792Z    |
2020-03-31T22:29:58.4866117Z LL | / pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
2020-03-31T22:29:58.4866363Z LL | |     async move { f(x).await }
2020-03-31T22:29:58.4866598Z    | |_^
2020-03-31T22:29:58.4866676Z 
2020-03-31T22:29:58.4866676Z 
2020-03-31T22:29:58.4866954Z error: internal compiler error: broken MIR in DefId(0:7 ~ issue_64477[317d]::g[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4867627Z    |
2020-03-31T22:29:58.4867627Z    |
2020-03-31T22:29:58.4867953Z LL | / pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
2020-03-31T22:29:58.4868182Z LL | |     async move { f(x).await }
2020-03-31T22:29:58.4868429Z    | |_^
2020-03-31T22:29:58.4868507Z 
2020-03-31T22:29:58.4868507Z 
2020-03-31T22:29:58.4869516Z error: internal compiler error: broken MIR in DefId(0:7 ~ issue_64477[317d]::g[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-64477.rs:16:1: 18:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4871952Z    |
2020-03-31T22:29:58.4871952Z    |
2020-03-31T22:29:58.4872449Z LL | / pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
2020-03-31T22:29:58.4872781Z LL | |     async move { f(x).await }
2020-03-31T22:29:58.4873134Z    | |_^
2020-03-31T22:29:58.4873249Z 
2020-03-31T22:29:58.4873249Z 
2020-03-31T22:29:58.4874649Z error: internal compiler error: broken MIR in DefId(0:7 ~ issue_64477[317d]::g[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-64477.rs:16:1: 18:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4876176Z    |
2020-03-31T22:29:58.4876176Z    |
2020-03-31T22:29:58.4876645Z LL | / pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
2020-03-31T22:29:58.4876993Z LL | |     async move { f(x).await }
2020-03-31T22:29:58.4877331Z    | |_^
2020-03-31T22:29:58.4877458Z 
2020-03-31T22:29:58.4877458Z 
2020-03-31T22:29:58.4878522Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Send>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.4879583Z 
2020-03-31T22:29:58.4879824Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4880670Z    |
2020-03-31T22:29:58.4880670Z    |
2020-03-31T22:29:58.4880861Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4881256Z 
2020-03-31T22:29:58.4881256Z 
2020-03-31T22:29:58.4881688Z error: internal compiler error: broken MIR in DefId(0:10 ~ issue_64477[317d]::g[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4882695Z    |
2020-03-31T22:29:58.4882695Z    |
2020-03-31T22:29:58.4882879Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4883270Z 
2020-03-31T22:29:58.4883270Z 
2020-03-31T22:29:58.4884848Z error: internal compiler error: broken MIR in DefId(0:10 ~ issue_64477[317d]::g[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16: 17:30, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4885940Z    |
2020-03-31T22:29:58.4885940Z    |
2020-03-31T22:29:58.4886069Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4886356Z 
2020-03-31T22:29:58.4886356Z 
2020-03-31T22:29:58.4887372Z error: internal compiler error: broken MIR in DefId(0:10 ~ issue_64477[317d]::g[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16: 17:30, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4888454Z    |
2020-03-31T22:29:58.4888454Z    |
2020-03-31T22:29:58.4888637Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4888923Z 
2020-03-31T22:29:58.4888923Z 
2020-03-31T22:29:58.4889972Z error: internal compiler error: broken MIR in DefId(0:10 ~ issue_64477[317d]::g[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-64477.rs:17:16: 17:30, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4891064Z    |
2020-03-31T22:29:58.4891064Z    |
2020-03-31T22:29:58.4891193Z LL |     async move { f(x).await }
2020-03-31T22:29:58.4891480Z 
2020-03-31T22:29:58.4891480Z 
2020-03-31T22:29:58.4891881Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.4892394Z 
2020-03-31T22:29:58.4892546Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4892683Z 
2020-03-31T22:29:58.4892850Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4892850Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4892989Z 
2020-03-31T22:29:58.4893415Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4893951Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4894126Z 
2020-03-31T22:29:58.4894126Z 
2020-03-31T22:29:58.4894562Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4894888Z 
2020-03-31T22:29:58.4895135Z ------------------------------------------
2020-03-31T22:29:58.4895254Z 
2020-03-31T22:29:58.4895337Z 
2020-03-31T22:29:58.4895337Z 
2020-03-31T22:29:58.4895652Z ---- [ui] ui/async-await/issues/issue-65436-raw-ptr-not-send.rs stdout ----
2020-03-31T22:29:58.4895813Z 
2020-03-31T22:29:58.4895996Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-31T22:29:58.4896403Z status: exit code: 101
2020-03-31T22:29:58.4897927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send/auxiliary"
2020-03-31T22:29:58.4899154Z ------------------------------------------
2020-03-31T22:29:58.4899290Z 
2020-03-31T22:29:58.4899541Z ------------------------------------------
2020-03-31T22:29:58.4899683Z stderr:
---
2020-03-31T22:29:58.4916490Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:17
2020-03-31T22:29:58.4916736Z    |
2020-03-31T22:29:58.4916863Z LL |       assert_send(async {
2020-03-31T22:29:58.4917025Z    |  _________________^
2020-03-31T22:29:58.4917340Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4917575Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4917917Z    | |_____^
2020-03-31T22:29:58.4918004Z 
2020-03-31T22:29:58.4918152Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4918754Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4918754Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4918958Z    |
2020-03-31T22:29:58.4919083Z LL |       assert_send(async {
2020-03-31T22:29:58.4919252Z    |  _______________________^
2020-03-31T22:29:58.4919450Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4919683Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4919979Z    | |_____^
2020-03-31T22:29:58.4920063Z 
2020-03-31T22:29:58.4920209Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4920883Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4920883Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4921091Z    |
2020-03-31T22:29:58.4921218Z LL |       assert_send(async {
2020-03-31T22:29:58.4921386Z    |  _______________________^
2020-03-31T22:29:58.4921584Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4921817Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4922110Z    | |_____^
2020-03-31T22:29:58.4922193Z 
2020-03-31T22:29:58.4922339Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4922844Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4922844Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23
2020-03-31T22:29:58.4923046Z    |
2020-03-31T22:29:58.4923168Z LL |       assert_send(async {
2020-03-31T22:29:58.4923336Z    |  _______________________^
2020-03-31T22:29:58.4923535Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4923768Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4924064Z    | |_____^
2020-03-31T22:29:58.4924148Z 
2020-03-31T22:29:58.4924295Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4924719Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:9
2020-03-31T22:29:58.4924719Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:9
2020-03-31T22:29:58.4924920Z    |
2020-03-31T22:29:58.4925074Z LL |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4925414Z 
2020-03-31T22:29:58.4925560Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4925979Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:9
2020-03-31T22:29:58.4926179Z    |
2020-03-31T22:29:58.4926179Z    |
2020-03-31T22:29:58.4926334Z LL |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4926670Z 
2020-03-31T22:29:58.4926670Z 
2020-03-31T22:29:58.4926839Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4927484Z    |
2020-03-31T22:29:58.4927596Z LL | / fn main() {
2020-03-31T22:29:58.4927738Z LL | |     assert_send(async {
2020-03-31T22:29:58.4927738Z LL | |     assert_send(async {
2020-03-31T22:29:58.4927947Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4928179Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4928467Z LL | | }
2020-03-31T22:29:58.4928567Z    | |_^
2020-03-31T22:29:58.4928645Z 
2020-03-31T22:29:58.4928645Z 
2020-03-31T22:29:58.4928941Z error: internal compiler error: broken MIR in DefId(0:12 ~ issue_65436_raw_ptr_not_send[317d]::main[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4929711Z    |
2020-03-31T22:29:58.4929834Z LL | / fn main() {
2020-03-31T22:29:58.4929976Z LL | |     assert_send(async {
2020-03-31T22:29:58.4929976Z LL | |     assert_send(async {
2020-03-31T22:29:58.4930219Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4930451Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4930771Z LL | | }
2020-03-31T22:29:58.4930872Z    | |_^
2020-03-31T22:29:58.4930963Z 
2020-03-31T22:29:58.4930963Z 
2020-03-31T22:29:58.4932031Z error: internal compiler error: broken MIR in DefId(0:12 ~ issue_65436_raw_ptr_not_send[317d]::main[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:11:1: 16:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4933211Z    |
2020-03-31T22:29:58.4933321Z LL | / fn main() {
2020-03-31T22:29:58.4933475Z LL | |     assert_send(async {
2020-03-31T22:29:58.4933475Z LL | |     assert_send(async {
2020-03-31T22:29:58.4933677Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4933909Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4934197Z LL | | }
2020-03-31T22:29:58.4934296Z    | |_^
2020-03-31T22:29:58.4934374Z 
2020-03-31T22:29:58.4934374Z 
2020-03-31T22:29:58.4934556Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4935190Z    |
2020-03-31T22:29:58.4935328Z LL |       assert_send(async {
2020-03-31T22:29:58.4935482Z    |  _______________________^
2020-03-31T22:29:58.4935482Z    |  _______________________^
2020-03-31T22:29:58.4935679Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4935922Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4936314Z    | |_____^
2020-03-31T22:29:58.4936398Z 
2020-03-31T22:29:58.4936398Z 
2020-03-31T22:29:58.4936731Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_65436_raw_ptr_not_send[317d]::main[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4937543Z    |
2020-03-31T22:29:58.4937667Z LL |       assert_send(async {
2020-03-31T22:29:58.4937820Z    |  _______________________^
2020-03-31T22:29:58.4937820Z    |  _______________________^
2020-03-31T22:29:58.4938029Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4938291Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4938569Z    | |_____^
2020-03-31T22:29:58.4938667Z 
2020-03-31T22:29:58.4938667Z 
2020-03-31T22:29:58.4939771Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_65436_raw_ptr_not_send[317d]::main[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23: 15:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4940986Z    |
2020-03-31T22:29:58.4941110Z LL |       assert_send(async {
2020-03-31T22:29:58.4941278Z    |  _______________________^
2020-03-31T22:29:58.4941278Z    |  _______________________^
2020-03-31T22:29:58.4941476Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4941706Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4941997Z    | |_____^
2020-03-31T22:29:58.4942080Z 
2020-03-31T22:29:58.4942080Z 
2020-03-31T22:29:58.4943187Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_65436_raw_ptr_not_send[317d]::main[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23: 15:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4944478Z    |
2020-03-31T22:29:58.4944617Z LL |       assert_send(async {
2020-03-31T22:29:58.4944771Z    |  _______________________^
2020-03-31T22:29:58.4944771Z    |  _______________________^
2020-03-31T22:29:58.4944969Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4945218Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4945496Z    | |_____^
2020-03-31T22:29:58.4945592Z 
2020-03-31T22:29:58.4945592Z 
2020-03-31T22:29:58.4946687Z error: internal compiler error: broken MIR in DefId(0:13 ~ issue_65436_raw_ptr_not_send[317d]::main[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:23: 15:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.4947897Z    |
2020-03-31T22:29:58.4948020Z LL |       assert_send(async {
2020-03-31T22:29:58.4948185Z    |  _______________________^
2020-03-31T22:29:58.4948185Z    |  _______________________^
2020-03-31T22:29:58.4948382Z LL | |     //~^ ERROR future cannot be sent between threads safely
2020-03-31T22:29:58.4948615Z LL | |         bar(Foo(std::ptr::null())).await;
2020-03-31T22:29:58.4948907Z    | |_____^
2020-03-31T22:29:58.4948990Z 
2020-03-31T22:29:58.4948990Z 
2020-03-31T22:29:58.4949390Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.4949900Z 
2020-03-31T22:29:58.4950047Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.4950200Z 
2020-03-31T22:29:58.4950352Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4950352Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4950752Z 
2020-03-31T22:29:58.4951224Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4951757Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4951933Z 
2020-03-31T22:29:58.4951933Z 
2020-03-31T22:29:58.4952384Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4952697Z 
2020-03-31T22:29:58.4952944Z ------------------------------------------
2020-03-31T22:29:58.4953078Z 
2020-03-31T22:29:58.4953146Z 
2020-03-31T22:29:58.4953146Z 
2020-03-31T22:29:58.4953447Z ---- [ui] ui/async-await/issues/issue-66695-static-refs.rs stdout ----
2020-03-31T22:29:58.4953602Z 
2020-03-31T22:29:58.4953888Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4954077Z status: exit code: 101
2020-03-31T22:29:58.4955519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-66695-static-refs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-66695-static-refs/auxiliary"
2020-03-31T22:29:58.4956693Z ------------------------------------------
2020-03-31T22:29:58.4956829Z 
2020-03-31T22:29:58.4957076Z ------------------------------------------
2020-03-31T22:29:58.4957218Z stderr:
2020-03-31T22:29:58.4957218Z stderr:
2020-03-31T22:29:58.4957486Z ------------------------------------------
2020-03-31T22:29:58.4957921Z error: internal compiler error: src/librustc_codegen_llvm/context.rs:851: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
2020-03-31T22:29:58.4958543Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-31T22:29:58.4958814Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.4958976Z 
2020-03-31T22:29:58.4959140Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4959140Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4959279Z 
2020-03-31T22:29:58.4959689Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4960225Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4960401Z 
2020-03-31T22:29:58.4960401Z 
2020-03-31T22:29:58.4960835Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4961229Z error: aborting due to previous error
2020-03-31T22:29:58.4961347Z 
2020-03-31T22:29:58.4961416Z 
2020-03-31T22:29:58.4961676Z ------------------------------------------
2020-03-31T22:29:58.4961676Z ------------------------------------------
2020-03-31T22:29:58.4961797Z 
2020-03-31T22:29:58.4961864Z 
2020-03-31T22:29:58.4962171Z ---- [ui] ui/async-await/issues/issue-67611-static-mut-refs.rs stdout ----
2020-03-31T22:29:58.4962330Z 
2020-03-31T22:29:58.4962613Z error: test compilation failed although it shouldn't!
2020-03-31T22:29:58.4962796Z status: exit code: 101
2020-03-31T22:29:58.4964265Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-67611-static-mut-refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67611-static-mut-refs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67611-static-mut-refs/auxiliary"
2020-03-31T22:29:58.4965474Z ------------------------------------------
2020-03-31T22:29:58.4965596Z 
2020-03-31T22:29:58.4965845Z ------------------------------------------
2020-03-31T22:29:58.4965985Z stderr:
2020-03-31T22:29:58.4965985Z stderr:
2020-03-31T22:29:58.4966252Z ------------------------------------------
2020-03-31T22:29:58.4966606Z error: internal compiler error: src/librustc_codegen_llvm/context.rs:851: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
2020-03-31T22:29:58.4967217Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-31T22:29:58.4967484Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.4967650Z 
2020-03-31T22:29:58.4967814Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4967814Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.4967953Z 
2020-03-31T22:29:58.4968363Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.4968899Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.4969073Z 
2020-03-31T22:29:58.4969073Z 
2020-03-31T22:29:58.4969508Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.4969896Z error: aborting due to previous error
2020-03-31T22:29:58.4970010Z 
2020-03-31T22:29:58.4970078Z 
2020-03-31T22:29:58.4970338Z ------------------------------------------
2020-03-31T22:29:58.4970338Z ------------------------------------------
2020-03-31T22:29:58.4970458Z 
2020-03-31T22:29:58.4970526Z 
2020-03-31T22:29:58.4970822Z ---- [ui] ui/async-await/suggest-missing-await-closure.rs stdout ----
2020-03-31T22:29:58.4971014Z 
2020-03-31T22:29:58.4971154Z error: failed to compile fixed code
2020-03-31T22:29:58.4971315Z status: exit code: 101
2020-03-31T22:29:58.4972803Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/auxiliary"
2020-03-31T22:29:58.4973971Z ------------------------------------------
2020-03-31T22:29:58.4974093Z 
2020-03-31T22:29:58.4974339Z ------------------------------------------
2020-03-31T22:29:58.4974498Z stderr:
2020-03-31T22:29:58.4974498Z stderr:
2020-03-31T22:29:58.4974752Z ------------------------------------------
2020-03-31T22:29:58.4974951Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4975350Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.4975563Z    |
2020-03-31T22:29:58.4975712Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.4975911Z    |  ___________________________________________^
2020-03-31T22:29:58.4976091Z LL | |     async || {
2020-03-31T22:29:58.4976332Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4976496Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4976742Z LL | |     };
2020-03-31T22:29:58.4976850Z LL | | }
2020-03-31T22:29:58.4976964Z    | |_^
2020-03-31T22:29:58.4977042Z 
2020-03-31T22:29:58.4977042Z 
2020-03-31T22:29:58.4977189Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4977597Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.4977811Z    |
2020-03-31T22:29:58.4977958Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.4978159Z    |  ___________________________________________^
2020-03-31T22:29:58.4978342Z LL | |     async || {
2020-03-31T22:29:58.4978492Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4978655Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4978907Z LL | |     };
2020-03-31T22:29:58.4979014Z LL | | }
2020-03-31T22:29:58.4979127Z    | |_^
2020-03-31T22:29:58.4979205Z 
2020-03-31T22:29:58.4979205Z 
2020-03-31T22:29:58.4979350Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4979751Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.4979962Z    |
2020-03-31T22:29:58.4980110Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.4980309Z    |  ___________________________________________^
2020-03-31T22:29:58.4980489Z LL | |     async || {
2020-03-31T22:29:58.4980641Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4980803Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4981051Z LL | |     };
2020-03-31T22:29:58.4981159Z LL | | }
2020-03-31T22:29:58.4981259Z    | |_^
2020-03-31T22:29:58.4981351Z 
2020-03-31T22:29:58.4981351Z 
2020-03-31T22:29:58.4981497Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4981896Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
2020-03-31T22:29:58.4982105Z    |
2020-03-31T22:29:58.4982218Z LL | /     async || {
2020-03-31T22:29:58.4982366Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4982541Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4982724Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.4982930Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.4983134Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.4983389Z    | |_____^
2020-03-31T22:29:58.4983519Z 
2020-03-31T22:29:58.4983679Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4984136Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
---
2020-03-31T22:29:58.4985233Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.4985426Z    |
2020-03-31T22:29:58.4985538Z LL |       async || {
2020-03-31T22:29:58.4985684Z    |  ______________^
2020-03-31T22:29:58.4985833Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4985995Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4986190Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.4986398Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.4986590Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.4986862Z    | |_____^
2020-03-31T22:29:58.4986946Z 
2020-03-31T22:29:58.4987094Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4987508Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.4987508Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.4987707Z    |
2020-03-31T22:29:58.4987819Z LL |       async || {
2020-03-31T22:29:58.4987965Z    |  ______________^
2020-03-31T22:29:58.4988113Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4988276Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4988473Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.4988680Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.4988869Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.4989137Z    | |_____^
2020-03-31T22:29:58.4989220Z 
2020-03-31T22:29:58.4989364Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4989779Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.4989779Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.4989977Z    |
2020-03-31T22:29:58.4990092Z LL |       async || {
2020-03-31T22:29:58.4990238Z    |  ______________^
2020-03-31T22:29:58.4990385Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4990548Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4990741Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.4990946Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.4991138Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.4991407Z    | |_____^
2020-03-31T22:29:58.4991490Z 
2020-03-31T22:29:58.4991636Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4992050Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:16:18
2020-03-31T22:29:58.4992050Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:16:18
2020-03-31T22:29:58.4992247Z    |
2020-03-31T22:29:58.4992371Z LL |         take_u32(x.await)
2020-03-31T22:29:58.4992645Z 
2020-03-31T22:29:58.4992791Z error: internal compiler error: cat_expr Errd
2020-03-31T22:29:58.4993190Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:16:18
2020-03-31T22:29:58.4993400Z    |
2020-03-31T22:29:58.4993400Z    |
2020-03-31T22:29:58.4993524Z LL |         take_u32(x.await)
2020-03-31T22:29:58.4993795Z 
2020-03-31T22:29:58.4993795Z 
2020-03-31T22:29:58.4993965Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.4994384Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:1
2020-03-31T22:29:58.4994739Z LL | / async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.4994906Z LL | |     async || {
2020-03-31T22:29:58.4995055Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4995055Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4995231Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4995463Z LL | |     };
2020-03-31T22:29:58.4995584Z LL | | }
2020-03-31T22:29:58.4995722Z    | |_^
2020-03-31T22:29:58.4995800Z 
2020-03-31T22:29:58.4995800Z 
2020-03-31T22:29:58.4996161Z error: internal compiler error: broken MIR in DefId(0:7 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.4996767Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:1
2020-03-31T22:29:58.4997127Z LL | / async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.4997294Z LL | |     async || {
2020-03-31T22:29:58.4997444Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4997444Z LL | |         let x = make_u32();
2020-03-31T22:29:58.4997619Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.4997850Z LL | |     };
2020-03-31T22:29:58.4997957Z LL | | }
2020-03-31T22:29:58.4998071Z    | |_^
2020-03-31T22:29:58.4998150Z 
2020-03-31T22:29:58.4998150Z 
2020-03-31T22:29:58.4999260Z error: internal compiler error: broken MIR in DefId(0:7 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:1: 21:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5000266Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:1
2020-03-31T22:29:58.5000626Z LL | / async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5000794Z LL | |     async || {
2020-03-31T22:29:58.5000943Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5000943Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5001118Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5001349Z LL | |     };
2020-03-31T22:29:58.5001455Z LL | | }
2020-03-31T22:29:58.5001569Z    | |_^
2020-03-31T22:29:58.5001647Z 
2020-03-31T22:29:58.5001647Z 
2020-03-31T22:29:58.5002210Z error: internal compiler error: errors selecting obligation during MIR typeck: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::marker::Sized>)), depth=0),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[type error] as std::future::Future>)), depth=0),Ambiguity)]
2020-03-31T22:29:58.5002770Z 
2020-03-31T22:29:58.5002940Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.5003363Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.5003721Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5003920Z    |  ___________________________________________^
2020-03-31T22:29:58.5004100Z LL | |     async || {
2020-03-31T22:29:58.5004250Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5004250Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5004412Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5004657Z LL | |     };
2020-03-31T22:29:58.5004762Z LL | | }
2020-03-31T22:29:58.5004875Z    | |_^
2020-03-31T22:29:58.5004954Z 
2020-03-31T22:29:58.5004954Z 
2020-03-31T22:29:58.5005313Z error: internal compiler error: broken MIR in DefId(0:9 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.5005922Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.5006281Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5006482Z    |  ___________________________________________^
2020-03-31T22:29:58.5006666Z LL | |     async || {
2020-03-31T22:29:58.5006814Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5006814Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5006976Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5007221Z LL | |     };
2020-03-31T22:29:58.5007327Z LL | | }
2020-03-31T22:29:58.5007440Z    | |_^
2020-03-31T22:29:58.5007518Z 
2020-03-31T22:29:58.5007518Z 
2020-03-31T22:29:58.5008696Z error: internal compiler error: broken MIR in DefId(0:9 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43: 21:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5009754Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.5010103Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5010316Z    |  ___________________________________________^
2020-03-31T22:29:58.5010483Z LL | |     async || {
2020-03-31T22:29:58.5010631Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5010631Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5010806Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5011038Z LL | |     };
2020-03-31T22:29:58.5011158Z LL | | }
2020-03-31T22:29:58.5011256Z    | |_^
2020-03-31T22:29:58.5011334Z 
2020-03-31T22:29:58.5011334Z 
2020-03-31T22:29:58.5012487Z error: internal compiler error: broken MIR in DefId(0:9 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43: 21:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5013505Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.5013867Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5014066Z    |  ___________________________________________^
2020-03-31T22:29:58.5014246Z LL | |     async || {
2020-03-31T22:29:58.5014395Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5014395Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5014556Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5014800Z LL | |     };
2020-03-31T22:29:58.5014908Z LL | | }
2020-03-31T22:29:58.5015008Z    | |_^
2020-03-31T22:29:58.5015099Z 
2020-03-31T22:29:58.5015099Z 
2020-03-31T22:29:58.5016323Z error: internal compiler error: broken MIR in DefId(0:9 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43: 21:2, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5017357Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:13:43
2020-03-31T22:29:58.5017704Z LL |   async fn suggest_await_in_async_closure() {
2020-03-31T22:29:58.5017917Z    |  ___________________________________________^
2020-03-31T22:29:58.5018084Z LL | |     async || {
2020-03-31T22:29:58.5018232Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5018232Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5018412Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5018644Z LL | |     };
2020-03-31T22:29:58.5018750Z LL | | }
2020-03-31T22:29:58.5018865Z    | |_^
2020-03-31T22:29:58.5018944Z 
2020-03-31T22:29:58.5018944Z 
2020-03-31T22:29:58.5019113Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.5019552Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
2020-03-31T22:29:58.5019861Z LL | /     async || {
2020-03-31T22:29:58.5020024Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5020024Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5020187Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5020370Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5020576Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5020781Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5021035Z    | |_____^
2020-03-31T22:29:58.5021132Z 
2020-03-31T22:29:58.5021132Z 
2020-03-31T22:29:58.5021555Z error: internal compiler error: broken MIR in DefId(0:10 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[0]) ("return type"): bad type [type error]
2020-03-31T22:29:58.5022226Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
2020-03-31T22:29:58.5022551Z LL | /     async || {
2020-03-31T22:29:58.5022700Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5022700Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5022877Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5023060Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5023266Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5023469Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5023722Z    | |_____^
2020-03-31T22:29:58.5023805Z 
2020-03-31T22:29:58.5023805Z 
2020-03-31T22:29:58.5024995Z error: internal compiler error: broken MIR in DefId(0:10 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5: 20:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5026044Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
2020-03-31T22:29:58.5027834Z LL | /     async || {
2020-03-31T22:29:58.5028066Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5028066Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5028303Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5028567Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5028864Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5029157Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5029520Z    | |_____^
2020-03-31T22:29:58.5029654Z 
2020-03-31T22:29:58.5029654Z 
2020-03-31T22:29:58.5031411Z error: internal compiler error: broken MIR in DefId(0:10 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5: 20:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5032923Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:5
2020-03-31T22:29:58.5033373Z LL | /     async || {
2020-03-31T22:29:58.5033606Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5033606Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5033840Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5034106Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5034417Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5034695Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5035079Z    | |_____^
2020-03-31T22:29:58.5035202Z 
2020-03-31T22:29:58.5035202Z 
2020-03-31T22:29:58.5035449Z error: internal compiler error: PromoteTemps: MIR had errors
2020-03-31T22:29:58.5036062Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.5036529Z LL |       async || {
2020-03-31T22:29:58.5036720Z    |  ______________^
2020-03-31T22:29:58.5036949Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5036949Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5037183Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5037447Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5037788Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5038063Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5038443Z    | |_____^
2020-03-31T22:29:58.5038564Z 
2020-03-31T22:29:58.5038564Z 
2020-03-31T22:29:58.5039221Z error: internal compiler error: broken MIR in DefId(0:11 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[1]) ("return type"): bad type [type error]
2020-03-31T22:29:58.5040302Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.5040616Z LL |       async || {
2020-03-31T22:29:58.5040761Z    |  ______________^
2020-03-31T22:29:58.5040910Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5040910Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5041072Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5041268Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5041474Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5041664Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5041933Z    | |_____^
2020-03-31T22:29:58.5042016Z 
2020-03-31T22:29:58.5042016Z 
2020-03-31T22:29:58.5043201Z error: internal compiler error: broken MIR in DefId(0:11 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14: 20:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5044249Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.5044575Z LL |       async || {
2020-03-31T22:29:58.5044707Z    |  ______________^
2020-03-31T22:29:58.5044855Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5044855Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5045031Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5045212Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5045417Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5045622Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5045876Z    | |_____^
2020-03-31T22:29:58.5045962Z 
2020-03-31T22:29:58.5045962Z 
2020-03-31T22:29:58.5047144Z error: internal compiler error: broken MIR in DefId(0:11 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14: 20:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5048181Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.5048507Z LL |       async || {
2020-03-31T22:29:58.5048639Z    |  ______________^
2020-03-31T22:29:58.5048801Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5048801Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5048962Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5049144Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5049362Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5049556Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5049826Z    | |_____^
2020-03-31T22:29:58.5049910Z 
2020-03-31T22:29:58.5049910Z 
2020-03-31T22:29:58.5051073Z error: internal compiler error: broken MIR in DefId(0:11 ~ suggest_missing_await_closure[317d]::suggest_await_in_async_closure[0]::{{closure}}[0]::{{closure}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14: 20:6, scope: scope[0] } }): bad type [type error]
2020-03-31T22:29:58.5052127Z   --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed:14:14
2020-03-31T22:29:58.5052453Z LL |       async || {
2020-03-31T22:29:58.5052585Z    |  ______________^
2020-03-31T22:29:58.5052733Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5052733Z LL | |         let x = make_u32();
2020-03-31T22:29:58.5052952Z LL | |         take_u32(x.await)
2020-03-31T22:29:58.5053136Z LL | |         //~^ ERROR mismatched types [E0308]
2020-03-31T22:29:58.5053373Z LL | |         //~| HELP consider using `.await` here
2020-03-31T22:29:58.5053579Z LL | |         //~| SUGGESTION x.await
2020-03-31T22:29:58.5053833Z    | |_____^
2020-03-31T22:29:58.5053917Z 
2020-03-31T22:29:58.5053917Z 
2020-03-31T22:29:58.5054340Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-31T22:29:58.5054830Z 
2020-03-31T22:29:58.5054993Z error: internal compiler error: unexpected panic
2020-03-31T22:29:58.5055130Z 
2020-03-31T22:29:58.5055283Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.5055283Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T22:29:58.5055420Z 
2020-03-31T22:29:58.5055863Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T22:29:58.5056574Z note: rustc 1.44.0-nightly (e0c2aaa47 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T22:29:58.5056764Z 
2020-03-31T22:29:58.5056764Z 
2020-03-31T22:29:58.5057199Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-31T22:29:58.5057511Z 
2020-03-31T22:29:58.5057774Z ------------------------------------------
2020-03-31T22:29:58.5057894Z 
2020-03-31T22:29:58.5057962Z 
---
2020-03-31T22:29:58.5063700Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T22:29:58.5064041Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:29:58.5064203Z 
2020-03-31T22:29:58.5064270Z 
2020-03-31T22:29:58.5066914Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T22:29:58.5068818Z 
2020-03-31T22:29:58.5068901Z 
2020-03-31T22:29:58.5069255Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T22:29:58.5069517Z Build completed unsuccessfully in 0:45:57
2020-03-31T22:29:58.5069517Z Build completed unsuccessfully in 0:45:57
2020-03-31T22:29:58.5069706Z == clock drift check ==
2020-03-31T22:29:58.5069880Z   local time: Tue Mar 31 22:29:58 UTC 2020
2020-03-31T22:29:58.7392431Z   network time: Tue, 31 Mar 2020 22:29:58 GMT
2020-03-31T22:29:58.7392814Z == end clock drift check ==
2020-03-31T22:29:59.1874524Z 
2020-03-31T22:29:59.1947852Z ##[error]Bash exited with code '1'.
2020-03-31T22:29:59.1957481Z ##[section]Finishing: Run build
2020-03-31T22:29:59.1994893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70636/merge to s
2020-03-31T22:29:59.1998768Z Task         : Get sources
2020-03-31T22:29:59.1999025Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T22:29:59.1999246Z Version      : 1.0.0
2020-03-31T22:29:59.1999409Z Author       : Microsoft
2020-03-31T22:29:59.1999409Z Author       : Microsoft
2020-03-31T22:29:59.1999676Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T22:29:59.1999961Z ==============================================================================
2020-03-31T22:29:59.4356754Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T22:29:59.4387155Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70636/merge to s
2020-03-31T22:29:59.4456044Z Cleaning up task key
2020-03-31T22:29:59.4457158Z Start cleaning up orphan processes.
2020-03-31T22:29:59.4675142Z Terminate orphan process: pid (4526) (python)
2020-03-31T22:29:59.4697800Z ##[section]Finishing: Finalize Job
