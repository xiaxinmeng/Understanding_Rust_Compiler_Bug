plain
2020-04-07T01:43:27.0629649Z ========================== Starting Command Output ===========================
2020-04-07T01:43:27.0632270Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/10f007ac-d591-4408-88cb-e65785e7f07b.sh
2020-04-07T01:43:27.0632555Z 
2020-04-07T01:43:27.0637305Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T01:43:27.0660429Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70872/merge to s
2020-04-07T01:43:27.0663838Z Task         : Get sources
2020-04-07T01:43:27.0664182Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T01:43:27.0664497Z Version      : 1.0.0
2020-04-07T01:43:27.0664710Z Author       : Microsoft
---
2020-04-07T01:43:29.2822710Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T01:43:29.2829225Z ##[command]git config gc.auto 0
2020-04-07T01:43:29.2832942Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T01:43:29.2837084Z ##[command]git config --get-all http.proxy
2020-04-07T01:43:29.2844352Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70872/merge:refs/remotes/pull/70872/merge
---
2020-04-07T01:45:50.3203757Z Looks like docker image is the same as before, not uploading
2020-04-07T01:45:57.3861786Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T01:45:57.4182080Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T01:45:57.4210827Z == clock drift check ==
2020-04-07T01:45:57.4218770Z   local time: Tue Apr  7 01:45:57 UTC 2020
2020-04-07T01:45:57.4904929Z   network time: Tue, 07 Apr 2020 01:45:57 GMT
2020-04-07T01:45:57.4936118Z Starting sccache server...
2020-04-07T01:45:57.5818563Z configure: processing command line
2020-04-07T01:45:57.5818867Z configure: 
2020-04-07T01:45:57.5823082Z configure: rust.dist-src        := False
---
2020-04-07T01:51:25.6986488Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T01:51:27.1314904Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T01:51:28.7142208Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T01:51:29.7681945Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T01:51:38.6729207Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T01:51:40.8568738Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T01:51:45.2526221Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T01:51:49.3166976Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T01:51:58.6816056Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T02:14:27.3160940Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T02:14:29.0831097Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T02:14:31.1432132Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T02:14:31.5844618Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T02:14:43.9161251Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T02:14:45.8098434Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T02:14:51.3172781Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T02:14:56.8944035Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T02:15:07.9919142Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T02:41:09.8990821Z .................................................................................................... 1700/9879
2020-04-07T02:41:14.0134051Z ...........F.................................................................F...................... 1800/9879
2020-04-07T02:41:23.1576000Z ..................................................................................................i. 1900/9879
2020-04-07T02:41:31.4060925Z .................................................................................................... 2000/9879
2020-04-07T02:41:37.8958098Z ........................................................................................iiiii....... 2100/9879
2020-04-07T02:41:59.4480258Z .................................................................................................... 2300/9879
2020-04-07T02:42:02.4371427Z .................................................................................................... 2400/9879
2020-04-07T02:42:03.9284435Z .................................................................................................... 2500/9879
2020-04-07T02:42:10.0330047Z .................................................................................................... 2600/9879
---
2020-04-07T02:45:06.0997551Z ..............................................................i...............i..................... 5000/9879
2020-04-07T02:45:13.5798929Z .................................................................................................... 5100/9879
2020-04-07T02:45:21.1842520Z .................................................................................................... 5200/9879
2020-04-07T02:45:26.6678926Z .......i............................................................................................ 5300/9879
2020-04-07T02:45:37.0013536Z ................................................................................................ii.i 5400/9879
2020-04-07T02:45:42.3084523Z i........i...i...................................................................................... 5500/9879
2020-04-07T02:45:51.6083514Z .........................................i.......................................................... 5700/9879
2020-04-07T02:46:01.9889467Z .............................................................ii..................................... 5800/9879
2020-04-07T02:46:09.4719099Z i................................................................................................... 5900/9879
2020-04-07T02:46:14.9397458Z .................................................................................................... 6000/9879
2020-04-07T02:46:14.9397458Z .................................................................................................... 6000/9879
2020-04-07T02:46:25.2901175Z ..............................................................................................ii...i 6100/9879
2020-04-07T02:46:37.5844018Z ..ii...........i.................................................................................... 6200/9879
2020-04-07T02:46:53.7708890Z .................................................................................................... 6400/9879
2020-04-07T02:46:56.7903294Z .................................................................................................... 6500/9879
2020-04-07T02:46:56.7903294Z .................................................................................................... 6500/9879
2020-04-07T02:47:09.4662207Z ........................i..ii....................................................................... 6600/9879
2020-04-07T02:47:30.8460547Z .................................................................................................... 6800/9879
2020-04-07T02:47:33.0104173Z ........................i........................................................................... 6900/9879
2020-04-07T02:47:35.1879369Z .................................................................................................... 7000/9879
2020-04-07T02:47:37.5512203Z ...............................................................i.................................... 7100/9879
---
2020-04-07T02:49:11.6676257Z .................................................................................................... 7700/9879
2020-04-07T02:49:16.7495459Z .................................................................................................... 7800/9879
2020-04-07T02:49:21.2338982Z .................................................................................................... 7900/9879
2020-04-07T02:49:27.4170946Z .................................................................................................... 8000/9879
2020-04-07T02:49:34.3204652Z ............................i.....................FFFF.F.FFFF..FFFF................................. 8100/9879
2020-04-07T02:49:42.8541462Z .............................................................................iiiiiiiiii.i........... 8200/9879
2020-04-07T02:49:59.4409934Z .....................i......i....................................................................... 8400/9879
2020-04-07T02:50:04.3804659Z .................................................................................................... 8500/9879
2020-04-07T02:50:16.0643828Z .................................................................................................... 8600/9879
2020-04-07T02:50:29.3351772Z .................................................................................................... 8700/9879
---
2020-04-07T02:52:28.5782870Z ---- [ui] ui/const-generics/fn-const-param-call.rs stdout ----
2020-04-07T02:52:28.5783342Z 
2020-04-07T02:52:28.5784047Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5784580Z status: exit code: 1
2020-04-07T02:52:28.5786859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/fn-const-param-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/fn-const-param-call/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/fn-const-param-call/auxiliary"
2020-04-07T02:52:28.5789910Z ------------------------------------------
2020-04-07T02:52:28.5790267Z 
2020-04-07T02:52:28.5790796Z ------------------------------------------
2020-04-07T02:52:28.5791139Z stderr:
---
2020-04-07T02:52:28.5795833Z 
2020-04-07T02:52:28.5796403Z error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-07T02:52:28.5797998Z   --> /checkout/src/test/ui/const-generics/fn-const-param-call.rs:10:25
2020-04-07T02:52:28.5798451Z    |
2020-04-07T02:52:28.5799019Z LL | struct Wrapper<const F: fn() -> u32>;
2020-04-07T02:52:28.5799800Z    |                         ^^^^^^^^^^^ `fn() -> u32` doesn't derive both `PartialEq` and `Eq`
2020-04-07T02:52:28.5801049Z error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-07T02:52:28.5806215Z   --> /checkout/src/test/ui/const-generics/fn-const-param-call.rs:12:15
2020-04-07T02:52:28.5806502Z    |
2020-04-07T02:52:28.5806502Z    |
2020-04-07T02:52:28.5806914Z LL | impl<const F: fn() -> u32> Wrapper<F> {
2020-04-07T02:52:28.5807525Z    |               ^^^^^^^^^^^ `fn() -> u32` doesn't derive both `PartialEq` and `Eq`
2020-04-07T02:52:28.5807987Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.5808170Z 
2020-04-07T02:52:28.5808642Z For more information about this error, try `rustc --explain E0741`.
2020-04-07T02:52:28.5808871Z 
---
2020-04-07T02:52:28.5815970Z error: /checkout/src/test/ui/const-generics/fn-const-param-infer.rs:25: expected message not found: mismatched types
2020-04-07T02:52:28.5816329Z 
2020-04-07T02:52:28.5816573Z error: 1 unexpected errors found, 4 expected errors not found
2020-04-07T02:52:28.5816874Z status: exit code: 1
2020-04-07T02:52:28.5818995Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/fn-const-param-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/fn-const-param-infer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/fn-const-param-infer/auxiliary"
2020-04-07T02:52:28.5820651Z     Error {
2020-04-07T02:52:28.5820853Z         line_num: 4,
2020-04-07T02:52:28.5821090Z         kind: Some(
2020-04-07T02:52:28.5821293Z             Error,
2020-04-07T02:52:28.5821293Z             Error,
2020-04-07T02:52:28.5821468Z         ),
2020-04-07T02:52:28.5821838Z         msg: "4:25: 4:42: the types of const generic parameters must derive `PartialEq` and `Eq` [E0741]",
2020-04-07T02:52:28.5822321Z ]
2020-04-07T02:52:28.5822445Z 
2020-04-07T02:52:28.5822653Z not found errors (from test file): [
2020-04-07T02:52:28.5822882Z     Error {
---
2020-04-07T02:52:28.5828961Z ---- [ui] ui/const-generics/raw-ptr-const-param-deref.rs stdout ----
2020-04-07T02:52:28.5829191Z 
2020-04-07T02:52:28.5829604Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5829876Z status: exit code: 1
2020-04-07T02:52:28.5832018Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/raw-ptr-const-param-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param-deref/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param-deref/auxiliary"
2020-04-07T02:52:28.5833694Z ------------------------------------------
2020-04-07T02:52:28.5833879Z 
2020-04-07T02:52:28.5834274Z ------------------------------------------
2020-04-07T02:52:28.5834488Z stderr:
---
2020-04-07T02:52:28.5837471Z 
2020-04-07T02:52:28.5837757Z error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-07T02:52:28.5838409Z   --> /checkout/src/test/ui/const-generics/raw-ptr-const-param-deref.rs:7:23
2020-04-07T02:52:28.5838714Z    |
2020-04-07T02:52:28.5838933Z LL | struct Const<const P: *const u32>;
2020-04-07T02:52:28.5839532Z    |                       ^^^^^^^^^^ `*const u32` doesn't derive both `PartialEq` and `Eq`
2020-04-07T02:52:28.5840106Z error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-07T02:52:28.5840757Z   --> /checkout/src/test/ui/const-generics/raw-ptr-const-param-deref.rs:9:15
2020-04-07T02:52:28.5841060Z    |
2020-04-07T02:52:28.5841060Z    |
2020-04-07T02:52:28.5841280Z LL | impl<const P: *const u32> Const<P> {
2020-04-07T02:52:28.5841860Z    |               ^^^^^^^^^^ `*const u32` doesn't derive both `PartialEq` and `Eq`
2020-04-07T02:52:28.5842338Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.5842517Z 
2020-04-07T02:52:28.5842966Z For more information about this error, try `rustc --explain E0741`.
2020-04-07T02:52:28.5843194Z 
---
2020-04-07T02:52:28.5846446Z error: /checkout/src/test/ui/const-generics/raw-ptr-const-param.rs:7: expected message not found: mismatched types
2020-04-07T02:52:28.5846792Z 
2020-04-07T02:52:28.5847030Z error: 1 unexpected errors found, 1 expected errors not found
2020-04-07T02:52:28.5847317Z status: exit code: 1
2020-04-07T02:52:28.5849494Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/raw-ptr-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param/auxiliary"
2020-04-07T02:52:28.5851501Z     Error {
2020-04-07T02:52:28.5851910Z         line_num: 4,
2020-04-07T02:52:28.5852141Z         kind: Some(
2020-04-07T02:52:28.5852341Z             Error,
2020-04-07T02:52:28.5852341Z             Error,
2020-04-07T02:52:28.5852514Z         ),
2020-04-07T02:52:28.5853424Z         msg: "4:23: 4:33: the types of const generic parameters must derive `PartialEq` and `Eq` [E0741]",
2020-04-07T02:52:28.5855196Z ]
2020-04-07T02:52:28.5855308Z 
2020-04-07T02:52:28.5855536Z not found errors (from test file): [
2020-04-07T02:52:28.5855766Z     Error {
---
2020-04-07T02:52:28.5858626Z ---- [ui] ui/consts/consts-in-patterns.rs stdout ----
2020-04-07T02:52:28.5858834Z 
2020-04-07T02:52:28.5859250Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5859525Z status: exit code: 1
2020-04-07T02:52:28.5861458Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/consts-in-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/consts-in-patterns/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/consts-in-patterns/auxiliary"
2020-04-07T02:52:28.5863774Z ------------------------------------------
2020-04-07T02:52:28.5863961Z 
2020-04-07T02:52:28.5864345Z ------------------------------------------
2020-04-07T02:52:28.5864577Z stderr:
---
2020-04-07T02:52:28.5891478Z error: /checkout/src/test/ui/consts/match_ice.rs:16: expected error not found: non-exhaustive patterns: `&T` not covered
2020-04-07T02:52:28.5892061Z 
2020-04-07T02:52:28.5892307Z error: 2 unexpected errors found, 1 expected errors not found
2020-04-07T02:52:28.5892595Z status: exit code: 1
2020-04-07T02:52:28.5894801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/match_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice/auxiliary"
2020-04-07T02:52:28.5896384Z     Error {
2020-04-07T02:52:28.5896587Z         line_num: 17,
2020-04-07T02:52:28.5896801Z         kind: Some(
2020-04-07T02:52:28.5897021Z             Error,
---
2020-04-07T02:52:28.5907945Z error: /checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs:5: unexpected error: '5:26: 5:36: the types of const generic parameters must derive `PartialEq` and `Eq` [E0741]'
2020-04-07T02:52:28.5908468Z 
2020-04-07T02:52:28.5908712Z error: 2 unexpected errors found, 0 expected errors not found
2020-04-07T02:52:28.5908996Z status: exit code: 1
2020-04-07T02:52:28.5911410Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_generics-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-ptr/auxiliary"
2020-04-07T02:52:28.5913403Z     Error {
2020-04-07T02:52:28.5913600Z         line_num: 1,
2020-04-07T02:52:28.5913829Z         kind: Some(
2020-04-07T02:52:28.5914030Z             Error,
2020-04-07T02:52:28.5914030Z             Error,
2020-04-07T02:52:28.5914205Z         ),
2020-04-07T02:52:28.5914573Z         msg: "1:25: 1:29: the types of const generic parameters must derive `PartialEq` and `Eq` [E0741]",
2020-04-07T02:52:28.5915074Z     Error {
2020-04-07T02:52:28.5915280Z         line_num: 5,
2020-04-07T02:52:28.5915614Z         kind: Some(
2020-04-07T02:52:28.5915817Z             Error,
2020-04-07T02:52:28.5915817Z             Error,
2020-04-07T02:52:28.5916252Z         ),
2020-04-07T02:52:28.5916622Z         msg: "5:26: 5:36: the types of const generic parameters must derive `PartialEq` and `Eq` [E0741]",
2020-04-07T02:52:28.5917097Z ]
2020-04-07T02:52:28.5917222Z 
2020-04-07T02:52:28.5918015Z thread '[ui] ui/feature-gates/feature-gate-const_generics-ptr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-07T02:52:28.5918395Z 
2020-04-07T02:52:28.5918395Z 
2020-04-07T02:52:28.5918965Z ---- [ui] ui/issues/issue-34784.rs stdout ----
2020-04-07T02:52:28.5919160Z 
2020-04-07T02:52:28.5919590Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5919863Z status: exit code: 1
2020-04-07T02:52:28.5921752Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34784.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34784/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34784/auxiliary"
2020-04-07T02:52:28.5923334Z ------------------------------------------
2020-04-07T02:52:28.5923671Z 
2020-04-07T02:52:28.5924066Z ------------------------------------------
2020-04-07T02:52:28.5924296Z stderr:
2020-04-07T02:52:28.5924296Z stderr:
2020-04-07T02:52:28.5924681Z ------------------------------------------
2020-04-07T02:52:28.5925004Z error: constants of type `*const u8` cannot be used in a pattern
2020-04-07T02:52:28.5925816Z    |
2020-04-07T02:52:28.5925985Z LL |         C => {}
2020-04-07T02:52:28.5926189Z    |         ^
2020-04-07T02:52:28.5926321Z 
---
2020-04-07T02:52:28.5927723Z    |         ^
2020-04-07T02:52:28.5927894Z    |
2020-04-07T02:52:28.5928136Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.5928349Z 
2020-04-07T02:52:28.5928607Z error: constants of type `*const [u8; 4]` cannot be used in a pattern
2020-04-07T02:52:28.5929444Z    |
2020-04-07T02:52:28.5929738Z LL |         D => {}
2020-04-07T02:52:28.5929941Z    |         ^
2020-04-07T02:52:28.5930071Z 
2020-04-07T02:52:28.5930071Z 
2020-04-07T02:52:28.5930249Z warning: unreachable pattern
2020-04-07T02:52:28.5930764Z   --> /checkout/src/test/ui/issues/issue-34784.rs:17:9
2020-04-07T02:52:28.5931005Z    |
2020-04-07T02:52:28.5931172Z LL |         _ => {}
2020-04-07T02:52:28.5931357Z    |         ^
2020-04-07T02:52:28.5931505Z 
2020-04-07T02:52:28.5932439Z error: constants of type `*const u8` cannot be used in a pattern
2020-04-07T02:52:28.5933271Z    |
2020-04-07T02:52:28.5933439Z LL |         C => {}
2020-04-07T02:52:28.5933622Z    |         ^
2020-04-07T02:52:28.5933752Z 
2020-04-07T02:52:28.5933752Z 
2020-04-07T02:52:28.5934025Z error: constants of type `*const [u8; 4]` cannot be used in a pattern
2020-04-07T02:52:28.5934942Z    |
2020-04-07T02:52:28.5935124Z LL |         D => {}
2020-04-07T02:52:28.5935308Z    |         ^
2020-04-07T02:52:28.5935439Z 
---
2020-04-07T02:52:28.5937022Z ---- [ui] ui/issues/issue-44333.rs stdout ----
2020-04-07T02:52:28.5937215Z 
2020-04-07T02:52:28.5937624Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5938063Z status: exit code: 1
2020-04-07T02:52:28.5940022Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44333.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44333/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44333/auxiliary"
2020-04-07T02:52:28.5941612Z ------------------------------------------
2020-04-07T02:52:28.5941797Z 
2020-04-07T02:52:28.5942194Z ------------------------------------------
2020-04-07T02:52:28.5942409Z stderr:
---
2020-04-07T02:52:28.5955601Z ---- [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded.rs stdout ----
2020-04-07T02:52:28.5955855Z 
2020-04-07T02:52:28.5956282Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5956555Z status: exit code: 1
2020-04-07T02:52:28.5958864Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded/auxiliary"
2020-04-07T02:52:28.5960736Z ------------------------------------------
2020-04-07T02:52:28.5960920Z 
2020-04-07T02:52:28.5961299Z ------------------------------------------
2020-04-07T02:52:28.5961516Z stderr:
2020-04-07T02:52:28.5961516Z stderr:
2020-04-07T02:52:28.5961926Z ------------------------------------------
2020-04-07T02:52:28.5962260Z error: constants of type `*const NoDerive` cannot be used in a pattern
2020-04-07T02:52:28.5963270Z    |
2020-04-07T02:52:28.5963270Z    |
2020-04-07T02:52:28.5963584Z LL |         WRAP_UNSAFE_EMBEDDED => { println!("WRAP_UNSAFE_EMBEDDED correctly matched itself"); }
2020-04-07T02:52:28.5964140Z 
2020-04-07T02:52:28.5964320Z warning: unreachable pattern
2020-04-07T02:52:28.5964916Z   --> /checkout/src/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded.rs:22:9
2020-04-07T02:52:28.5965247Z    |
2020-04-07T02:52:28.5965247Z    |
2020-04-07T02:52:28.5965528Z LL |         _ => { panic!("WRAP_UNSAFE_EMBEDDED did not match itself"); }
2020-04-07T02:52:28.5965982Z    |
2020-04-07T02:52:28.5966228Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.5966439Z 
2020-04-07T02:52:28.5966439Z 
2020-04-07T02:52:28.5966691Z error: constants of type `*const NoDerive` cannot be used in a pattern
2020-04-07T02:52:28.5967699Z    |
2020-04-07T02:52:28.5967699Z    |
2020-04-07T02:52:28.5968014Z LL |         WRAP_UNSAFE_EMBEDDED => { println!("WRAP_UNSAFE_EMBEDDED correctly matched itself"); }
2020-04-07T02:52:28.5968559Z 
2020-04-07T02:52:28.5968760Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.5968939Z 
2020-04-07T02:52:28.5969058Z 
2020-04-07T02:52:28.5969058Z 
2020-04-07T02:52:28.5969438Z ------------------------------------------
2020-04-07T02:52:28.5969620Z 
2020-04-07T02:52:28.5969723Z 
2020-04-07T02:52:28.5970230Z ---- [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs stdout ----
2020-04-07T02:52:28.5970481Z 
2020-04-07T02:52:28.5970896Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5971169Z status: exit code: 1
2020-04-07T02:52:28.5973789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param/auxiliary"
2020-04-07T02:52:28.5975540Z ------------------------------------------
2020-04-07T02:52:28.5975794Z 
2020-04-07T02:52:28.5976591Z ------------------------------------------
2020-04-07T02:52:28.5976809Z stderr:
2020-04-07T02:52:28.5976809Z stderr:
2020-04-07T02:52:28.5977197Z ------------------------------------------
2020-04-07T02:52:28.5977530Z error: constants of type `*const NoDerive` cannot be used in a pattern
2020-04-07T02:52:28.5978523Z    |
2020-04-07T02:52:28.5978523Z    |
2020-04-07T02:52:28.5978839Z LL |         WRAP_UNSAFE_PARAM => { println!("WRAP_UNSAFE_PARAM correctly matched itself"); }
2020-04-07T02:52:28.5979351Z 
2020-04-07T02:52:28.5979529Z warning: unreachable pattern
2020-04-07T02:52:28.5980134Z   --> /checkout/src/test/ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs:22:9
2020-04-07T02:52:28.5980449Z    |
2020-04-07T02:52:28.5980449Z    |
2020-04-07T02:52:28.5980713Z LL |         _ => { panic!("WRAP_UNSAFE_PARAM did not match itself"); }
2020-04-07T02:52:28.5981249Z    |
2020-04-07T02:52:28.5981492Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.5981724Z 
2020-04-07T02:52:28.5981724Z 
2020-04-07T02:52:28.5981979Z error: constants of type `*const NoDerive` cannot be used in a pattern
2020-04-07T02:52:28.5982976Z    |
2020-04-07T02:52:28.5982976Z    |
2020-04-07T02:52:28.5983281Z LL |         WRAP_UNSAFE_PARAM => { println!("WRAP_UNSAFE_PARAM correctly matched itself"); }
2020-04-07T02:52:28.5983785Z 
2020-04-07T02:52:28.5984003Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.5984186Z 
2020-04-07T02:52:28.5984290Z 
2020-04-07T02:52:28.5984290Z 
2020-04-07T02:52:28.5984670Z ------------------------------------------
2020-04-07T02:52:28.5984871Z 
2020-04-07T02:52:28.5984977Z 
2020-04-07T02:52:28.5985467Z ---- [ui] ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs stdout ----
2020-04-07T02:52:28.5985738Z 
2020-04-07T02:52:28.5986167Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.5986440Z status: exit code: 1
2020-04-07T02:52:28.5988606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded/auxiliary"
2020-04-07T02:52:28.5990375Z ------------------------------------------
2020-04-07T02:52:28.5990577Z 
2020-04-07T02:52:28.5990959Z ------------------------------------------
2020-04-07T02:52:28.5991177Z stderr:
2020-04-07T02:52:28.5991177Z stderr:
2020-04-07T02:52:28.5992241Z ------------------------------------------
2020-04-07T02:52:28.5992578Z error: constants of type `WrapEmbedded` cannot be used in a pattern
2020-04-07T02:52:28.5993607Z    |
2020-04-07T02:52:28.5993607Z    |
2020-04-07T02:52:28.5993923Z LL |         WRAP_UNSAFE_EMBEDDED => { println!("WRAP_UNSAFE_EMBEDDED correctly matched itself"); }
2020-04-07T02:52:28.5994467Z 
2020-04-07T02:52:28.5994648Z warning: unreachable pattern
2020-04-07T02:52:28.5995250Z   --> /checkout/src/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs:22:9
2020-04-07T02:52:28.5995590Z    |
2020-04-07T02:52:28.5995590Z    |
2020-04-07T02:52:28.5995895Z LL |         _ => { panic!("WRAP_UNSAFE_EMBEDDED did not match itself"); }
2020-04-07T02:52:28.5996338Z    |
2020-04-07T02:52:28.5996699Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.5996914Z 
2020-04-07T02:52:28.5996914Z 
2020-04-07T02:52:28.5997164Z error: constants of type `WrapEmbedded` cannot be used in a pattern
2020-04-07T02:52:28.5998636Z    |
2020-04-07T02:52:28.5998636Z    |
2020-04-07T02:52:28.5998953Z LL |         WRAP_UNSAFE_EMBEDDED => { println!("WRAP_UNSAFE_EMBEDDED correctly matched itself"); }
2020-04-07T02:52:28.5999494Z 
2020-04-07T02:52:28.5999694Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.5999876Z 
2020-04-07T02:52:28.5999997Z 
2020-04-07T02:52:28.5999997Z 
2020-04-07T02:52:28.6000419Z ------------------------------------------
2020-04-07T02:52:28.6000601Z 
2020-04-07T02:52:28.6000705Z 
2020-04-07T02:52:28.6001195Z ---- [ui] ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-param.rs stdout ----
2020-04-07T02:52:28.6001461Z 
2020-04-07T02:52:28.6001962Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6002243Z status: exit code: 1
2020-04-07T02:52:28.6004391Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-param/auxiliary"
2020-04-07T02:52:28.6006160Z ------------------------------------------
2020-04-07T02:52:28.6006347Z 
2020-04-07T02:52:28.6006726Z ------------------------------------------
2020-04-07T02:52:28.6006964Z stderr:
2020-04-07T02:52:28.6006964Z stderr:
2020-04-07T02:52:28.6007360Z ------------------------------------------
2020-04-07T02:52:28.6007704Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6008732Z    |
2020-04-07T02:52:28.6008732Z    |
2020-04-07T02:52:28.6009036Z LL |         WRAP_UNSAFE_PARAM => { println!("WRAP_UNSAFE_PARAM correctly matched itself"); }
2020-04-07T02:52:28.6009561Z 
2020-04-07T02:52:28.6009741Z warning: unreachable pattern
2020-04-07T02:52:28.6010355Z   --> /checkout/src/test/ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-param.rs:22:9
2020-04-07T02:52:28.6010673Z    |
2020-04-07T02:52:28.6010673Z    |
2020-04-07T02:52:28.6010935Z LL |         _ => { panic!("WRAP_UNSAFE_PARAM did not match itself"); }
2020-04-07T02:52:28.6011384Z    |
2020-04-07T02:52:28.6011625Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6011977Z 
2020-04-07T02:52:28.6011977Z 
2020-04-07T02:52:28.6012255Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6013394Z    |
2020-04-07T02:52:28.6013394Z    |
2020-04-07T02:52:28.6013713Z LL |         WRAP_UNSAFE_PARAM => { println!("WRAP_UNSAFE_PARAM correctly matched itself"); }
2020-04-07T02:52:28.6014219Z 
2020-04-07T02:52:28.6014437Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6014618Z 
2020-04-07T02:52:28.6014723Z 
2020-04-07T02:52:28.6014723Z 
2020-04-07T02:52:28.6015392Z ------------------------------------------
2020-04-07T02:52:28.6015581Z 
2020-04-07T02:52:28.6015700Z 
2020-04-07T02:52:28.6016169Z ---- [ui] ui/rfc1445/allow-use-behind-cousin-variant.rs stdout ----
2020-04-07T02:52:28.6016402Z 
2020-04-07T02:52:28.6016810Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6017099Z status: exit code: 1
2020-04-07T02:52:28.6019459Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-use-behind-cousin-variant/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/allow-use-behind-cousin-variant/auxiliary"
2020-04-07T02:52:28.6021178Z ------------------------------------------
2020-04-07T02:52:28.6021362Z 
2020-04-07T02:52:28.6021836Z ------------------------------------------
2020-04-07T02:52:28.6022051Z stderr:
2020-04-07T02:52:28.6022051Z stderr:
2020-04-07T02:52:28.6022445Z ------------------------------------------
2020-04-07T02:52:28.6022867Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6023504Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:49:13
2020-04-07T02:52:28.6023797Z    |
2020-04-07T02:52:28.6024042Z LL |         & & EEK_ZERO => { println!("Hello 3"); }
2020-04-07T02:52:28.6024463Z 
2020-04-07T02:52:28.6024712Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6025322Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:50:13
2020-04-07T02:52:28.6025615Z    |
2020-04-07T02:52:28.6025615Z    |
2020-04-07T02:52:28.6025856Z LL |         & & EEK_ONE => { println!("Gbye"); }
2020-04-07T02:52:28.6026264Z 
2020-04-07T02:52:28.6026441Z warning: unreachable pattern
2020-04-07T02:52:28.6027337Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:50:9
2020-04-07T02:52:28.6027633Z    |
2020-04-07T02:52:28.6027633Z    |
2020-04-07T02:52:28.6028153Z LL |         & & EEK_ONE => { println!("Gbye"); }
2020-04-07T02:52:28.6028613Z    |
2020-04-07T02:52:28.6028854Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6029068Z 
2020-04-07T02:52:28.6029262Z warning: unreachable pattern
2020-04-07T02:52:28.6029262Z warning: unreachable pattern
2020-04-07T02:52:28.6029841Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:51:9
2020-04-07T02:52:28.6030128Z    |
2020-04-07T02:52:28.6030350Z LL |         _ => { println!("Gbye"); }
2020-04-07T02:52:28.6030703Z 
2020-04-07T02:52:28.6030937Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6031569Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:55:13
2020-04-07T02:52:28.6031859Z    |
2020-04-07T02:52:28.6031859Z    |
2020-04-07T02:52:28.6032083Z LL |         & & EEK_ZERO => { println!("Gby"); }
2020-04-07T02:52:28.6032510Z 
2020-04-07T02:52:28.6032743Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6033377Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:56:13
2020-04-07T02:52:28.6033669Z    |
2020-04-07T02:52:28.6033669Z    |
2020-04-07T02:52:28.6033900Z LL |         & & EEK_ONE => { println!("Hello 4"); }
2020-04-07T02:52:28.6034329Z 
2020-04-07T02:52:28.6034508Z warning: unreachable pattern
2020-04-07T02:52:28.6035062Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:56:9
2020-04-07T02:52:28.6035370Z    |
2020-04-07T02:52:28.6035370Z    |
2020-04-07T02:52:28.6035601Z LL |         & & EEK_ONE => { println!("Hello 4"); }
2020-04-07T02:52:28.6036010Z 
2020-04-07T02:52:28.6036203Z warning: unreachable pattern
2020-04-07T02:52:28.6036756Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:57:9
2020-04-07T02:52:28.6037046Z    |
2020-04-07T02:52:28.6037046Z    |
2020-04-07T02:52:28.6037269Z LL |         _ => { println!("Gbye"); }
2020-04-07T02:52:28.6037622Z 
2020-04-07T02:52:28.6037879Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6038580Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:49:13
2020-04-07T02:52:28.6038869Z    |
2020-04-07T02:52:28.6038869Z    |
2020-04-07T02:52:28.6039124Z LL |         & & EEK_ZERO => { println!("Hello 3"); }
2020-04-07T02:52:28.6039544Z 
2020-04-07T02:52:28.6039777Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6040403Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:50:13
2020-04-07T02:52:28.6040697Z    |
2020-04-07T02:52:28.6040697Z    |
2020-04-07T02:52:28.6040922Z LL |         & & EEK_ONE => { println!("Gbye"); }
2020-04-07T02:52:28.6041345Z 
2020-04-07T02:52:28.6041575Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6042184Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:55:13
2020-04-07T02:52:28.6042493Z    |
2020-04-07T02:52:28.6042493Z    |
2020-04-07T02:52:28.6042774Z LL |         & & EEK_ZERO => { println!("Gby"); }
2020-04-07T02:52:28.6043210Z 
2020-04-07T02:52:28.6043442Z error: constants of type `Eek` cannot be used in a pattern
2020-04-07T02:52:28.6044058Z   --> /checkout/src/test/ui/rfc1445/allow-use-behind-cousin-variant.rs:56:13
2020-04-07T02:52:28.6044365Z    |
2020-04-07T02:52:28.6044365Z    |
2020-04-07T02:52:28.6044594Z LL |         & & EEK_ONE => { println!("Hello 4"); }
2020-04-07T02:52:28.6045005Z 
2020-04-07T02:52:28.6045221Z error: aborting due to 8 previous errors
2020-04-07T02:52:28.6045404Z 
2020-04-07T02:52:28.6045509Z 
2020-04-07T02:52:28.6045509Z 
2020-04-07T02:52:28.6045886Z ------------------------------------------
2020-04-07T02:52:28.6046087Z 
2020-04-07T02:52:28.6046191Z 
2020-04-07T02:52:28.6046665Z ---- [ui] ui/rfc1445/cant-hide-behind-doubly-indirect-embedded.rs stdout ----
2020-04-07T02:52:28.6046916Z 
2020-04-07T02:52:28.6047351Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6047856Z status: exit code: 1
2020-04-07T02:52:28.6050289Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded/auxiliary"
2020-04-07T02:52:28.6053283Z ------------------------------------------
2020-04-07T02:52:28.6053500Z 
2020-04-07T02:52:28.6053935Z ------------------------------------------
2020-04-07T02:52:28.6054149Z stderr:
2020-04-07T02:52:28.6054149Z stderr:
2020-04-07T02:52:28.6054561Z ------------------------------------------
2020-04-07T02:52:28.6054889Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6055540Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded.rs:24:9
2020-04-07T02:52:28.6055874Z    |
2020-04-07T02:52:28.6056195Z LL |         WRAP_DOUBLY_INDIRECT_INLINE => { panic!("WRAP_DOUBLY_INDIRECT_INLINE matched itself"); }
2020-04-07T02:52:28.6057095Z 
2020-04-07T02:52:28.6057280Z warning: unreachable pattern
2020-04-07T02:52:28.6057887Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded.rs:27:9
2020-04-07T02:52:28.6058215Z    |
2020-04-07T02:52:28.6058215Z    |
2020-04-07T02:52:28.6058517Z LL |         _ => { println!("WRAP_DOUBLY_INDIRECT_INLINE correctly did not match itself"); }
2020-04-07T02:52:28.6059007Z    |
2020-04-07T02:52:28.6059249Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6059464Z 
2020-04-07T02:52:28.6059857Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6059857Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6060540Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-embedded.rs:24:9
2020-04-07T02:52:28.6060850Z    |
2020-04-07T02:52:28.6061167Z LL |         WRAP_DOUBLY_INDIRECT_INLINE => { panic!("WRAP_DOUBLY_INDIRECT_INLINE matched itself"); }
2020-04-07T02:52:28.6061745Z 
2020-04-07T02:52:28.6061944Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6062125Z 
2020-04-07T02:52:28.6062246Z 
2020-04-07T02:52:28.6062246Z 
2020-04-07T02:52:28.6062625Z ------------------------------------------
2020-04-07T02:52:28.6063464Z 
2020-04-07T02:52:28.6063570Z 
2020-04-07T02:52:28.6064083Z ---- [ui] ui/rfc1445/cant-hide-behind-doubly-indirect-param.rs stdout ----
2020-04-07T02:52:28.6064326Z 
2020-04-07T02:52:28.6064744Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6065017Z status: exit code: 1
2020-04-07T02:52:28.6067861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-doubly-indirect-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-doubly-indirect-param/auxiliary"
2020-04-07T02:52:28.6069654Z ------------------------------------------
2020-04-07T02:52:28.6069840Z 
2020-04-07T02:52:28.6070217Z ------------------------------------------
2020-04-07T02:52:28.6070446Z stderr:
2020-04-07T02:52:28.6070446Z stderr:
2020-04-07T02:52:28.6070830Z ------------------------------------------
2020-04-07T02:52:28.6071174Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6072166Z    |
2020-04-07T02:52:28.6072166Z    |
2020-04-07T02:52:28.6072499Z LL |         WRAP_DOUBLY_INDIRECT_PARAM => { panic!("WRAP_DOUBLY_INDIRECT_PARAM matched itself"); }
2020-04-07T02:52:28.6073352Z 
2020-04-07T02:52:28.6073530Z warning: unreachable pattern
2020-04-07T02:52:28.6074137Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-doubly-indirect-param.rs:27:9
2020-04-07T02:52:28.6074446Z    |
2020-04-07T02:52:28.6074446Z    |
2020-04-07T02:52:28.6074747Z LL |         _ => { println!("WRAP_DOUBLY_INDIRECT_PARAM correctly did not match itself"); }
2020-04-07T02:52:28.6075236Z    |
2020-04-07T02:52:28.6075477Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6075708Z 
2020-04-07T02:52:28.6075708Z 
2020-04-07T02:52:28.6075976Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6076972Z    |
2020-04-07T02:52:28.6076972Z    |
2020-04-07T02:52:28.6077284Z LL |         WRAP_DOUBLY_INDIRECT_PARAM => { panic!("WRAP_DOUBLY_INDIRECT_PARAM matched itself"); }
2020-04-07T02:52:28.6077832Z 
2020-04-07T02:52:28.6078052Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6078232Z 
2020-04-07T02:52:28.6078336Z 
2020-04-07T02:52:28.6078336Z 
2020-04-07T02:52:28.6078714Z ------------------------------------------
2020-04-07T02:52:28.6078914Z 
2020-04-07T02:52:28.6079676Z 
2020-04-07T02:52:28.6080273Z ---- [ui] ui/rfc1445/cant-hide-behind-indirect-struct-embedded.rs stdout ----
2020-04-07T02:52:28.6080527Z 
2020-04-07T02:52:28.6080953Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6081226Z status: exit code: 1
2020-04-07T02:52:28.6083367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded/auxiliary"
2020-04-07T02:52:28.6085233Z ------------------------------------------
2020-04-07T02:52:28.6085433Z 
2020-04-07T02:52:28.6085816Z ------------------------------------------
2020-04-07T02:52:28.6086034Z stderr:
2020-04-07T02:52:28.6086034Z stderr:
2020-04-07T02:52:28.6086438Z ------------------------------------------
2020-04-07T02:52:28.6086762Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6087476Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded.rs:24:9
2020-04-07T02:52:28.6087820Z    |
2020-04-07T02:52:28.6088112Z LL |         WRAP_INDIRECT_INLINE => { panic!("WRAP_INDIRECT_INLINE matched itself"); }
2020-04-07T02:52:28.6088635Z 
2020-04-07T02:52:28.6088816Z warning: unreachable pattern
2020-04-07T02:52:28.6089408Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded.rs:27:9
2020-04-07T02:52:28.6089743Z    |
2020-04-07T02:52:28.6089743Z    |
2020-04-07T02:52:28.6090014Z LL |         _ => { println!("WRAP_INDIRECT_INLINE did not match itself"); }
2020-04-07T02:52:28.6090457Z    |
2020-04-07T02:52:28.6090716Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6090931Z 
2020-04-07T02:52:28.6091174Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6091174Z error: constants of type `WrapInline` cannot be used in a pattern
2020-04-07T02:52:28.6091971Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-embedded.rs:24:9
2020-04-07T02:52:28.6092288Z    |
2020-04-07T02:52:28.6092579Z LL |         WRAP_INDIRECT_INLINE => { panic!("WRAP_INDIRECT_INLINE matched itself"); }
2020-04-07T02:52:28.6093139Z 
2020-04-07T02:52:28.6093340Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6093522Z 
2020-04-07T02:52:28.6093627Z 
2020-04-07T02:52:28.6093627Z 
2020-04-07T02:52:28.6094302Z ------------------------------------------
2020-04-07T02:52:28.6094486Z 
2020-04-07T02:52:28.6094590Z 
2020-04-07T02:52:28.6095064Z ---- [ui] ui/rfc1445/cant-hide-behind-indirect-struct-param.rs stdout ----
2020-04-07T02:52:28.6095580Z 
2020-04-07T02:52:28.6096028Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6096300Z status: exit code: 1
2020-04-07T02:52:28.6098423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-indirect-struct-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/cant-hide-behind-indirect-struct-param/auxiliary"
2020-04-07T02:52:28.6100151Z ------------------------------------------
2020-04-07T02:52:28.6100336Z 
2020-04-07T02:52:28.6100712Z ------------------------------------------
2020-04-07T02:52:28.6100945Z stderr:
2020-04-07T02:52:28.6100945Z stderr:
2020-04-07T02:52:28.6101333Z ------------------------------------------
2020-04-07T02:52:28.6101674Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6102657Z    |
2020-04-07T02:52:28.6102657Z    |
2020-04-07T02:52:28.6103031Z LL |         WRAP_INDIRECT_PARAM => { panic!("WRAP_INDIRECT_PARAM matched itself"); }
2020-04-07T02:52:28.6103547Z 
2020-04-07T02:52:28.6103724Z warning: unreachable pattern
2020-04-07T02:52:28.6104327Z   --> /checkout/src/test/ui/rfc1445/cant-hide-behind-indirect-struct-param.rs:27:9
2020-04-07T02:52:28.6104631Z    |
2020-04-07T02:52:28.6104631Z    |
2020-04-07T02:52:28.6104918Z LL |         _ => { println!("WRAP_INDIRECT_PARAM correctly did not match itself"); }
2020-04-07T02:52:28.6105389Z    |
2020-04-07T02:52:28.6105630Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6105842Z 
2020-04-07T02:52:28.6105842Z 
2020-04-07T02:52:28.6106120Z error: constants of type `WrapParam<NoDerive>` cannot be used in a pattern
2020-04-07T02:52:28.6107844Z    |
2020-04-07T02:52:28.6107844Z    |
2020-04-07T02:52:28.6108239Z LL |         WRAP_INDIRECT_PARAM => { panic!("WRAP_INDIRECT_PARAM matched itself"); }
2020-04-07T02:52:28.6108747Z 
2020-04-07T02:52:28.6108946Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6109141Z 
2020-04-07T02:52:28.6109245Z 
---
2020-04-07T02:52:28.6114019Z error: /checkout/src/test/ui/rfc1445/issue-61188-match-slice-forbidden-without-eq.rs:15: expected error not found: must be annotated with `#[derive(PartialEq, Eq)]`
2020-04-07T02:52:28.6114469Z 
2020-04-07T02:52:28.6114860Z error: 2 unexpected errors found, 1 expected errors not found
2020-04-07T02:52:28.6115163Z status: exit code: 1
2020-04-07T02:52:28.6117443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/issue-61188-match-slice-forbidden-without-eq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-61188-match-slice-forbidden-without-eq" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-61188-match-slice-forbidden-without-eq/auxiliary"
2020-04-07T02:52:28.6119202Z     Error {
2020-04-07T02:52:28.6119400Z         line_num: 15,
2020-04-07T02:52:28.6119633Z         kind: Some(
2020-04-07T02:52:28.6119835Z             Error,
---
2020-04-07T02:52:28.6126099Z ---- [ui] ui/rfc1445/fn-ptr-is-structurally-matchable.rs stdout ----
2020-04-07T02:52:28.6126333Z 
2020-04-07T02:52:28.6126746Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6127018Z status: exit code: 1
2020-04-07T02:52:28.6129140Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/fn-ptr-is-structurally-matchable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/fn-ptr-is-structurally-matchable/auxiliary"
2020-04-07T02:52:28.6130856Z ------------------------------------------
2020-04-07T02:52:28.6131040Z 
2020-04-07T02:52:28.6131438Z ------------------------------------------
2020-04-07T02:52:28.6131819Z stderr:
2020-04-07T02:52:28.6131819Z stderr:
2020-04-07T02:52:28.6132217Z ------------------------------------------
2020-04-07T02:52:28.6132533Z error: constants of type `fn()` cannot be used in a pattern
2020-04-07T02:52:28.6133165Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:43:14
2020-04-07T02:52:28.6133473Z    |
2020-04-07T02:52:28.6133693Z LL |         Wrap(CFN1) => count += 1,
2020-04-07T02:52:28.6134072Z 
2020-04-07T02:52:28.6134251Z warning: unreachable pattern
2020-04-07T02:52:28.6134824Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:44:9
2020-04-07T02:52:28.6135117Z    |
2020-04-07T02:52:28.6135117Z    |
2020-04-07T02:52:28.6135297Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6135523Z    |         ^^^^^^^
2020-04-07T02:52:28.6135690Z    |
2020-04-07T02:52:28.6135932Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6136144Z 
2020-04-07T02:52:28.6136430Z error: constants of type `fn(main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6137363Z    |
2020-04-07T02:52:28.6137363Z    |
2020-04-07T02:52:28.6137582Z LL |         Wrap(CFN2) => count += 1,
2020-04-07T02:52:28.6137967Z 
2020-04-07T02:52:28.6138151Z warning: unreachable pattern
2020-04-07T02:52:28.6138723Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:52:9
2020-04-07T02:52:28.6139016Z    |
2020-04-07T02:52:28.6139016Z    |
2020-04-07T02:52:28.6139196Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6139420Z    |         ^^^^^^^
2020-04-07T02:52:28.6139563Z 
2020-04-07T02:52:28.6140042Z error: constants of type `fn() -> main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6141001Z    |
2020-04-07T02:52:28.6141001Z    |
2020-04-07T02:52:28.6141201Z LL |         Wrap(CFN3) => count += 1,
2020-04-07T02:52:28.6141594Z 
2020-04-07T02:52:28.6141771Z warning: unreachable pattern
2020-04-07T02:52:28.6142325Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:60:9
2020-04-07T02:52:28.6142638Z    |
2020-04-07T02:52:28.6142638Z    |
2020-04-07T02:52:28.6142819Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6143034Z    |         ^^^^^^^
2020-04-07T02:52:28.6143243Z 
2020-04-07T02:52:28.6143528Z error: constants of type `fn(main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6144771Z    |
2020-04-07T02:52:28.6144771Z    |
2020-04-07T02:52:28.6144988Z LL |         Wrap(CFN4) => count += 1,
2020-04-07T02:52:28.6145368Z 
2020-04-07T02:52:28.6145563Z warning: unreachable pattern
2020-04-07T02:52:28.6146118Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:68:9
2020-04-07T02:52:28.6146413Z    |
2020-04-07T02:52:28.6146413Z    |
2020-04-07T02:52:28.6146593Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6146817Z    |         ^^^^^^^
2020-04-07T02:52:28.6146960Z 
2020-04-07T02:52:28.6147447Z error: constants of type `fn() -> main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6148403Z    |
2020-04-07T02:52:28.6148403Z    |
2020-04-07T02:52:28.6148677Z LL |         Wrap(CFN5) => count += 1,
2020-04-07T02:52:28.6149079Z 
2020-04-07T02:52:28.6149257Z warning: unreachable pattern
2020-04-07T02:52:28.6149818Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:76:9
2020-04-07T02:52:28.6150125Z    |
2020-04-07T02:52:28.6150125Z    |
2020-04-07T02:52:28.6150305Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6150511Z    |         ^^^^^^^
2020-04-07T02:52:28.6150653Z 
2020-04-07T02:52:28.6151172Z error: constants of type `for<'r> fn(&'r main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6152151Z    |
2020-04-07T02:52:28.6152151Z    |
2020-04-07T02:52:28.6152351Z LL |         Wrap(CFN6) => count += 1,
2020-04-07T02:52:28.6152730Z 
2020-04-07T02:52:28.6152925Z warning: unreachable pattern
2020-04-07T02:52:28.6153669Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:84:9
2020-04-07T02:52:28.6153971Z    |
2020-04-07T02:52:28.6153971Z    |
2020-04-07T02:52:28.6154162Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6154371Z    |         ^^^^^^^
2020-04-07T02:52:28.6154516Z 
2020-04-07T02:52:28.6155069Z error: constants of type `for<'r> fn(&'r ()) -> &'r main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6156069Z    |
2020-04-07T02:52:28.6156069Z    |
2020-04-07T02:52:28.6156269Z LL |         Wrap(CFN7) => count += 1,
2020-04-07T02:52:28.6156663Z 
2020-04-07T02:52:28.6156840Z warning: unreachable pattern
2020-04-07T02:52:28.6157404Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:92:9
2020-04-07T02:52:28.6157692Z    |
2020-04-07T02:52:28.6157692Z    |
2020-04-07T02:52:28.6157872Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6158077Z    |         ^^^^^^^
2020-04-07T02:52:28.6158236Z 
2020-04-07T02:52:28.6158755Z error: constants of type `for<'r> fn(&'r main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6159745Z    |
2020-04-07T02:52:28.6159745Z    |
2020-04-07T02:52:28.6159945Z LL |         Wrap(CFN8) => count += 1,
2020-04-07T02:52:28.6160321Z 
2020-04-07T02:52:28.6160515Z warning: unreachable pattern
2020-04-07T02:52:28.6161071Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:100:9
2020-04-07T02:52:28.6161365Z    |
2020-04-07T02:52:28.6161365Z    |
2020-04-07T02:52:28.6161562Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6161770Z    |         ^^^^^^^
2020-04-07T02:52:28.6161910Z 
2020-04-07T02:52:28.6162458Z error: constants of type `for<'r> fn(&'r ()) -> &'r main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6163466Z    |
2020-04-07T02:52:28.6163466Z    |
2020-04-07T02:52:28.6163668Z LL |         Wrap(CFN9) => count += 1,
2020-04-07T02:52:28.6164132Z 
2020-04-07T02:52:28.6164311Z warning: unreachable pattern
2020-04-07T02:52:28.6164890Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:108:9
2020-04-07T02:52:28.6165181Z    |
2020-04-07T02:52:28.6165181Z    |
2020-04-07T02:52:28.6165360Z LL |         Wrap(_) => {}
2020-04-07T02:52:28.6165582Z    |         ^^^^^^^
2020-04-07T02:52:28.6165722Z 
2020-04-07T02:52:28.6165988Z error: constants of type `fn(main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6166941Z    |
2020-04-07T02:52:28.6166941Z    |
2020-04-07T02:52:28.6167136Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6167488Z 
2020-04-07T02:52:28.6167488Z 
2020-04-07T02:52:28.6167988Z error: constants of type `fn() -> main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6168935Z    |
2020-04-07T02:52:28.6168935Z    |
2020-04-07T02:52:28.6169211Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6169572Z 
2020-04-07T02:52:28.6169572Z 
2020-04-07T02:52:28.6169848Z error: constants of type `fn(main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6170790Z    |
2020-04-07T02:52:28.6170790Z    |
2020-04-07T02:52:28.6170999Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6171350Z 
2020-04-07T02:52:28.6171350Z 
2020-04-07T02:52:28.6172023Z error: constants of type `fn() -> main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6172997Z    |
2020-04-07T02:52:28.6172997Z    |
2020-04-07T02:52:28.6173189Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6173558Z 
2020-04-07T02:52:28.6173735Z warning: unreachable pattern
2020-04-07T02:52:28.6174285Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:130:9
2020-04-07T02:52:28.6174602Z    |
2020-04-07T02:52:28.6174602Z    |
2020-04-07T02:52:28.6174786Z LL |         Foo { .. } => {}
2020-04-07T02:52:28.6175006Z    |         ^^^^^^^^^^
2020-04-07T02:52:28.6175170Z 
2020-04-07T02:52:28.6175403Z error: constants of type `fn()` cannot be used in a pattern
2020-04-07T02:52:28.6176016Z   --> /checkout/src/test/ui/rfc1445/fn-ptr-is-structurally-matchable.rs:43:14
2020-04-07T02:52:28.6176324Z    |
2020-04-07T02:52:28.6176528Z LL |         Wrap(CFN1) => count += 1,
2020-04-07T02:52:28.6176908Z 
2020-04-07T02:52:28.6176908Z 
2020-04-07T02:52:28.6177187Z error: constants of type `fn(main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6178118Z    |
2020-04-07T02:52:28.6178118Z    |
2020-04-07T02:52:28.6178335Z LL |         Wrap(CFN2) => count += 1,
2020-04-07T02:52:28.6178713Z 
2020-04-07T02:52:28.6178713Z 
2020-04-07T02:52:28.6179193Z error: constants of type `fn() -> main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6180155Z    |
2020-04-07T02:52:28.6180155Z    |
2020-04-07T02:52:28.6180356Z LL |         Wrap(CFN3) => count += 1,
2020-04-07T02:52:28.6180752Z 
2020-04-07T02:52:28.6180752Z 
2020-04-07T02:52:28.6181019Z error: constants of type `fn(main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6181963Z    |
2020-04-07T02:52:28.6181963Z    |
2020-04-07T02:52:28.6182164Z LL |         Wrap(CFN4) => count += 1,
2020-04-07T02:52:28.6182559Z 
2020-04-07T02:52:28.6182559Z 
2020-04-07T02:52:28.6183042Z error: constants of type `fn() -> main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6183997Z    |
2020-04-07T02:52:28.6183997Z    |
2020-04-07T02:52:28.6184198Z LL |         Wrap(CFN5) => count += 1,
2020-04-07T02:52:28.6184650Z 
2020-04-07T02:52:28.6184650Z 
2020-04-07T02:52:28.6185181Z error: constants of type `for<'r> fn(&'r main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6186134Z    |
2020-04-07T02:52:28.6186134Z    |
2020-04-07T02:52:28.6186352Z LL |         Wrap(CFN6) => count += 1,
2020-04-07T02:52:28.6186993Z 
2020-04-07T02:52:28.6186993Z 
2020-04-07T02:52:28.6187599Z error: constants of type `for<'r> fn(&'r ()) -> &'r main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6188581Z    |
2020-04-07T02:52:28.6188581Z    |
2020-04-07T02:52:28.6188798Z LL |         Wrap(CFN7) => count += 1,
2020-04-07T02:52:28.6189173Z 
2020-04-07T02:52:28.6189173Z 
2020-04-07T02:52:28.6189684Z error: constants of type `for<'r> fn(&'r main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6190742Z    |
2020-04-07T02:52:28.6190742Z    |
2020-04-07T02:52:28.6190943Z LL |         Wrap(CFN8) => count += 1,
2020-04-07T02:52:28.6191339Z 
2020-04-07T02:52:28.6191339Z 
2020-04-07T02:52:28.6191894Z error: constants of type `for<'r> fn(&'r ()) -> &'r main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6192901Z    |
2020-04-07T02:52:28.6192901Z    |
2020-04-07T02:52:28.6193101Z LL |         Wrap(CFN9) => count += 1,
2020-04-07T02:52:28.6193494Z 
2020-04-07T02:52:28.6193494Z 
2020-04-07T02:52:28.6193763Z error: constants of type `fn(main::NotSM)` cannot be used in a pattern
2020-04-07T02:52:28.6194711Z    |
2020-04-07T02:52:28.6194711Z    |
2020-04-07T02:52:28.6194903Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6195263Z 
2020-04-07T02:52:28.6195263Z 
2020-04-07T02:52:28.6195766Z error: constants of type `fn() -> main::NotSM` cannot be used in a pattern
2020-04-07T02:52:28.6196709Z    |
2020-04-07T02:52:28.6196709Z    |
2020-04-07T02:52:28.6196916Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6197268Z 
2020-04-07T02:52:28.6197268Z 
2020-04-07T02:52:28.6197546Z error: constants of type `fn(main::SM)` cannot be used in a pattern
2020-04-07T02:52:28.6198474Z    |
2020-04-07T02:52:28.6198474Z    |
2020-04-07T02:52:28.6198686Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6199034Z 
2020-04-07T02:52:28.6199034Z 
2020-04-07T02:52:28.6199510Z error: constants of type `fn() -> main::SM` cannot be used in a pattern
2020-04-07T02:52:28.6200476Z    |
2020-04-07T02:52:28.6200476Z    |
2020-04-07T02:52:28.6200667Z LL |         CFOO => count += 1,
2020-04-07T02:52:28.6201032Z 
2020-04-07T02:52:28.6201231Z error: aborting due to 26 previous errors
2020-04-07T02:52:28.6201413Z 
2020-04-07T02:52:28.6201516Z 
2020-04-07T02:52:28.6201516Z 
2020-04-07T02:52:28.6201910Z ------------------------------------------
2020-04-07T02:52:28.6202093Z 
2020-04-07T02:52:28.6202196Z 
2020-04-07T02:52:28.6202684Z ---- [ui] ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs stdout ----
2020-04-07T02:52:28.6202956Z 
2020-04-07T02:52:28.6203368Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6203641Z status: exit code: 1
2020-04-07T02:52:28.6206405Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq/auxiliary"
2020-04-07T02:52:28.6208318Z ------------------------------------------
2020-04-07T02:52:28.6208504Z 
2020-04-07T02:52:28.6208884Z ------------------------------------------
2020-04-07T02:52:28.6209117Z stderr:
2020-04-07T02:52:28.6209117Z stderr:
2020-04-07T02:52:28.6209505Z ------------------------------------------
2020-04-07T02:52:28.6209813Z error: constants of type `B` cannot be used in a pattern
2020-04-07T02:52:28.6210487Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:31:9
2020-04-07T02:52:28.6210817Z    |
2020-04-07T02:52:28.6211191Z LL |         RR_B1 => { println!("CLAIM RR0: {:?} matches {:?}", RR_B1, RR_B0); }
2020-04-07T02:52:28.6211843Z 
2020-04-07T02:52:28.6212028Z warning: unreachable pattern
2020-04-07T02:52:28.6212978Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:34:9
2020-04-07T02:52:28.6213306Z    |
---
2020-04-07T02:52:28.6214294Z 
2020-04-07T02:52:28.6214540Z error: constants of type `B` cannot be used in a pattern
2020-04-07T02:52:28.6215197Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:38:9
2020-04-07T02:52:28.6215522Z    |
2020-04-07T02:52:28.6216102Z LL |         RR_B1 => { println!("CLAIM RR1: {:?} matches {:?}", RR_B1, RR_B1); }
2020-04-07T02:52:28.6216585Z 
2020-04-07T02:52:28.6216769Z warning: unreachable pattern
2020-04-07T02:52:28.6217411Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:41:9
2020-04-07T02:52:28.6217735Z    |
2020-04-07T02:52:28.6217735Z    |
2020-04-07T02:52:28.6217905Z LL |         _ => { }
2020-04-07T02:52:28.6218110Z    |         ^
2020-04-07T02:52:28.6218240Z 
2020-04-07T02:52:28.6218468Z error: constants of type `B` cannot be used in a pattern
2020-04-07T02:52:28.6219787Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:31:9
2020-04-07T02:52:28.6220116Z    |
2020-04-07T02:52:28.6220431Z LL |         RR_B1 => { println!("CLAIM RR0: {:?} matches {:?}", RR_B1, RR_B0); }
2020-04-07T02:52:28.6220922Z 
2020-04-07T02:52:28.6221152Z error: constants of type `B` cannot be used in a pattern
2020-04-07T02:52:28.6221815Z   --> /checkout/src/test/ui/rfc1445/issue-62307-match-ref-ref-forbidden-without-eq.rs:38:9
2020-04-07T02:52:28.6222159Z    |
2020-04-07T02:52:28.6222159Z    |
2020-04-07T02:52:28.6222479Z LL |         RR_B1 => { println!("CLAIM RR1: {:?} matches {:?}", RR_B1, RR_B1); }
2020-04-07T02:52:28.6222973Z 
2020-04-07T02:52:28.6223173Z error: aborting due to 4 previous errors
2020-04-07T02:52:28.6223356Z 
2020-04-07T02:52:28.6223460Z 
2020-04-07T02:52:28.6223460Z 
2020-04-07T02:52:28.6223855Z ------------------------------------------
2020-04-07T02:52:28.6224039Z 
2020-04-07T02:52:28.6224143Z 
2020-04-07T02:52:28.6224573Z ---- [ui] ui/rfc1445/issue-63479-match-fnptr.rs stdout ----
2020-04-07T02:52:28.6224789Z 
2020-04-07T02:52:28.6225218Z error: test compilation failed although it shouldn't!
2020-04-07T02:52:28.6225493Z status: exit code: 1
2020-04-07T02:52:28.6227482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/issue-63479-match-fnptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-63479-match-fnptr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/issue-63479-match-fnptr/auxiliary"
2020-04-07T02:52:28.6229232Z ------------------------------------------
2020-04-07T02:52:28.6229431Z 
2020-04-07T02:52:28.6229808Z ------------------------------------------
2020-04-07T02:52:28.6230022Z stderr:
2020-04-07T02:52:28.6230022Z stderr:
2020-04-07T02:52:28.6230425Z ------------------------------------------
2020-04-07T02:52:28.6230970Z error: constants of type `for<'r> fn(&'r [A])` cannot be used in a pattern
2020-04-07T02:52:28.6231876Z    |
2020-04-07T02:52:28.6231876Z    |
2020-04-07T02:52:28.6232084Z LL |     B(TEST) => println!("matched"),
2020-04-07T02:52:28.6232462Z 
2020-04-07T02:52:28.6232645Z warning: unreachable pattern
2020-04-07T02:52:28.6233245Z   --> /checkout/src/test/ui/rfc1445/issue-63479-match-fnptr.rs:34:5
2020-04-07T02:52:28.6233522Z    |
2020-04-07T02:52:28.6233522Z    |
2020-04-07T02:52:28.6233923Z LL |     _ => panic!("didn't match")
2020-04-07T02:52:28.6234137Z    |     ^
2020-04-07T02:52:28.6234286Z    |
2020-04-07T02:52:28.6234547Z    = note: `#[warn(unreachable_patterns)]` on by default
2020-04-07T02:52:28.6234758Z 
2020-04-07T02:52:28.6235224Z error: constants of type `for<'r> fn(&'r [A])` cannot be used in a pattern
2020-04-07T02:52:28.6236120Z    |
2020-04-07T02:52:28.6236120Z    |
2020-04-07T02:52:28.6236323Z LL |     B(TEST) => println!("matched"),
2020-04-07T02:52:28.6236698Z 
2020-04-07T02:52:28.6236896Z error: aborting due to 2 previous errors
2020-04-07T02:52:28.6237078Z 
2020-04-07T02:52:28.6237184Z 
---
2020-04-07T02:52:28.6250076Z test result: FAILED. 9797 passed; 22 failed; 60 ignored; 0 measured; 0 filtered out
2020-04-07T02:52:28.6250418Z 
2020-04-07T02:52:28.6250538Z 
2020-04-07T02:52:28.6250643Z 
2020-04-07T02:52:28.6254954Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T02:52:28.6257803Z 
2020-04-07T02:52:28.6257910Z 
2020-04-07T02:52:28.6258457Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-07T02:52:28.6259161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T02:52:28.6259161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T02:52:28.6286366Z Build completed unsuccessfully in 1:04:38
2020-04-07T02:52:28.6302175Z == clock drift check ==
2020-04-07T02:52:28.6303138Z   local time: Tue Apr  7 02:52:28 UTC 2020
2020-04-07T02:52:28.8874186Z   network time: Tue, 07 Apr 2020 02:52:28 GMT
2020-04-07T02:52:29.3317902Z 
2020-04-07T02:52:29.3317902Z 
2020-04-07T02:52:29.3397563Z ##[error]Bash exited with code '1'.
2020-04-07T02:52:29.3411554Z ##[section]Finishing: Run build
2020-04-07T02:52:29.3467196Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70872/merge to s
2020-04-07T02:52:29.3472416Z Task         : Get sources
2020-04-07T02:52:29.3472782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:52:29.3473134Z Version      : 1.0.0
2020-04-07T02:52:29.3473375Z Author       : Microsoft
2020-04-07T02:52:29.3473375Z Author       : Microsoft
2020-04-07T02:52:29.3473756Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T02:52:29.3474203Z ==============================================================================
2020-04-07T02:52:29.6882237Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T02:52:29.6928277Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70872/merge to s
2020-04-07T02:52:29.7015098Z Cleaning up task key
2020-04-07T02:52:29.7016431Z Start cleaning up orphan processes.
2020-04-07T02:52:29.7204543Z Terminate orphan process: pid (3339) (python)
2020-04-07T02:52:29.7375645Z ##[section]Finishing: Finalize Job
