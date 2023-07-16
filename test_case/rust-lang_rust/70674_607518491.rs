plain
2020-04-01T21:50:40.9342976Z ========================== Starting Command Output ===========================
2020-04-01T21:50:40.9346021Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/610e7f80-0c8f-43d7-a689-7d2067122c76.sh
2020-04-01T21:50:40.9346301Z 
2020-04-01T21:50:40.9351139Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T21:50:40.9370001Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-01T21:50:40.9373047Z Task         : Get sources
2020-04-01T21:50:40.9373516Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T21:50:40.9373801Z Version      : 1.0.0
2020-04-01T21:50:40.9373987Z Author       : Microsoft
---
2020-04-01T21:50:41.9511777Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T21:50:41.9518859Z ##[command]git config gc.auto 0
2020-04-01T21:50:41.9524822Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T21:50:41.9529402Z ##[command]git config --get-all http.proxy
2020-04-01T21:50:41.9539381Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70674/merge:refs/remotes/pull/70674/merge
---
2020-04-01T21:52:28.5013773Z Looks like docker image is the same as before, not uploading
2020-04-01T21:52:36.4641582Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T21:52:36.4884632Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T21:52:36.4912551Z == clock drift check ==
2020-04-01T21:52:36.4921348Z   local time: Wed Apr  1 21:52:36 UTC 2020
2020-04-01T21:52:36.7846205Z   network time: Wed, 01 Apr 2020 21:52:36 GMT
2020-04-01T21:52:36.7872619Z Starting sccache server...
2020-04-01T21:52:36.8705576Z configure: processing command line
2020-04-01T21:52:36.8709082Z configure: 
2020-04-01T21:52:36.8711492Z configure: rust.dist-src        := False
---
2020-04-01T21:57:22.0983695Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T21:57:23.4074737Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T21:57:24.7907011Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T21:57:26.1918765Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T21:57:33.8310096Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T21:57:37.0290317Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T21:57:41.2906819Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T21:57:45.2709651Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T21:57:52.2750116Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T22:18:32.1731424Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T22:18:33.8506005Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T22:18:35.7145892Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T22:18:38.6599804Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T22:18:47.4822860Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T22:18:51.9549604Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T22:18:57.2490144Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T22:19:02.3825823Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T22:19:11.3711082Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-01T22:19:11.3711082Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-01T22:19:46.0010961Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0011631Z     --> src/librustc_middle/query/mod.rs:106:20
2020-04-01T22:19:46.0012141Z      |
2020-04-01T22:19:46.0012735Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0013626Z 39   | |     Other {
2020-04-01T22:19:46.0014515Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0015437Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0016092Z ...    |
2020-04-01T22:19:46.0016788Z 106  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0018624Z ...    |
2020-04-01T22:19:46.0019245Z 1290 | |     }
2020-04-01T22:19:46.0019918Z 1291 | | }
2020-04-01T22:19:46.0019918Z 1291 | | }
2020-04-01T22:19:46.0020663Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0021240Z      | 
2020-04-01T22:19:46.0021784Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0022278Z      |
2020-04-01T22:19:46.0022904Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0024422Z      |
2020-04-01T22:19:46.0025008Z      = note: `-D unused-parens` implied by `-D warnings`
2020-04-01T22:19:46.0025311Z 
2020-04-01T22:19:46.0092398Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0092398Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0093085Z     --> src/librustc_middle/query/mod.rs:106:20
2020-04-01T22:19:46.0093734Z      |
2020-04-01T22:19:46.0094395Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0095163Z 39   | |     Other {
2020-04-01T22:19:46.0096121Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0097414Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0098119Z ...    |
2020-04-01T22:19:46.0098889Z 106  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0100870Z ...    |
2020-04-01T22:19:46.0101559Z 1290 | |     }
2020-04-01T22:19:46.0102370Z 1291 | | }
2020-04-01T22:19:46.0102370Z 1291 | | }
2020-04-01T22:19:46.0103117Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0103728Z      | 
2020-04-01T22:19:46.0104216Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0104645Z      |
2020-04-01T22:19:46.0105175Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0106292Z 
2020-04-01T22:19:46.0170992Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0171650Z     --> src/librustc_middle/query/mod.rs:139:20
2020-04-01T22:19:46.0172145Z      |
2020-04-01T22:19:46.0172145Z      |
2020-04-01T22:19:46.0172755Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0173564Z 39   | |     Other {
2020-04-01T22:19:46.0174458Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0175381Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0176017Z ...    |
2020-04-01T22:19:46.0176733Z 139  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0178584Z ...    |
2020-04-01T22:19:46.0179190Z 1290 | |     }
2020-04-01T22:19:46.0179856Z 1291 | | }
2020-04-01T22:19:46.0179856Z 1291 | | }
2020-04-01T22:19:46.0180619Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0181181Z      | 
2020-04-01T22:19:46.0181741Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0182376Z      |
2020-04-01T22:19:46.0183045Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0184344Z 
2020-04-01T22:19:46.0249264Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0249972Z     --> src/librustc_middle/query/mod.rs:162:20
2020-04-01T22:19:46.0250502Z      |
2020-04-01T22:19:46.0250502Z      |
2020-04-01T22:19:46.0251159Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0251929Z 39   | |     Other {
2020-04-01T22:19:46.0252899Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0254010Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0254698Z ...    |
2020-04-01T22:19:46.0255470Z 162  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0257445Z ...    |
2020-04-01T22:19:46.0258102Z 1290 | |     }
2020-04-01T22:19:46.0258829Z 1291 | | }
2020-04-01T22:19:46.0258829Z 1291 | | }
2020-04-01T22:19:46.0259661Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0260266Z      | 
2020-04-01T22:19:46.0260866Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0261399Z      |
2020-04-01T22:19:46.0262059Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0263435Z 
2020-04-01T22:19:46.0351305Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0352583Z     --> src/librustc_middle/query/mod.rs:177:20
2020-04-01T22:19:46.0353534Z      |
2020-04-01T22:19:46.0353534Z      |
2020-04-01T22:19:46.0354703Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0356080Z 39   | |     Other {
2020-04-01T22:19:46.0357764Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0359564Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0360795Z ...    |
2020-04-01T22:19:46.0362166Z 177  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0366533Z ...    |
2020-04-01T22:19:46.0367742Z 1290 | |     }
2020-04-01T22:19:46.0369034Z 1291 | | }
2020-04-01T22:19:46.0369034Z 1291 | | }
2020-04-01T22:19:46.0370495Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0371577Z      | 
2020-04-01T22:19:46.0372645Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0374280Z      |
2020-04-01T22:19:46.0375477Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0377941Z 
2020-04-01T22:19:46.0461194Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0461902Z     --> src/librustc_middle/query/mod.rs:186:20
2020-04-01T22:19:46.0462432Z      |
2020-04-01T22:19:46.0462432Z      |
2020-04-01T22:19:46.0463094Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0463866Z 39   | |     Other {
2020-04-01T22:19:46.0464915Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0465853Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0466489Z ...    |
2020-04-01T22:19:46.0467204Z 186  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0469037Z ...    |
2020-04-01T22:19:46.0469642Z 1290 | |     }
2020-04-01T22:19:46.0470317Z 1291 | | }
2020-04-01T22:19:46.0470317Z 1291 | | }
2020-04-01T22:19:46.0471076Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0471638Z      | 
2020-04-01T22:19:46.0472193Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0472691Z      |
2020-04-01T22:19:46.0473300Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0474581Z 
2020-04-01T22:19:46.0537381Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0538167Z     --> src/librustc_middle/query/mod.rs:195:20
2020-04-01T22:19:46.0538673Z      |
2020-04-01T22:19:46.0538673Z      |
2020-04-01T22:19:46.0539281Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0539995Z 39   | |     Other {
2020-04-01T22:19:46.0540864Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0541805Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0542441Z ...    |
2020-04-01T22:19:46.0543160Z 195  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0544996Z ...    |
2020-04-01T22:19:46.0545601Z 1290 | |     }
2020-04-01T22:19:46.0546265Z 1291 | | }
2020-04-01T22:19:46.0546265Z 1291 | | }
2020-04-01T22:19:46.0547028Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0547585Z      | 
2020-04-01T22:19:46.0548145Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0548644Z      |
2020-04-01T22:19:46.0549259Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0550537Z 
2020-04-01T22:19:46.0617312Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0618015Z     --> src/librustc_middle/query/mod.rs:202:20
2020-04-01T22:19:46.0618545Z      |
2020-04-01T22:19:46.0618545Z      |
2020-04-01T22:19:46.0619201Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0619974Z 39   | |     Other {
2020-04-01T22:19:46.0620934Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0621947Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0622636Z ...    |
2020-04-01T22:19:46.0623408Z 202  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0625390Z ...    |
2020-04-01T22:19:46.0626045Z 1290 | |     }
2020-04-01T22:19:46.0626980Z 1291 | | }
2020-04-01T22:19:46.0626980Z 1291 | | }
2020-04-01T22:19:46.0627819Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0628424Z      | 
2020-04-01T22:19:46.0629023Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0629558Z      |
2020-04-01T22:19:46.0630216Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0631600Z 
2020-04-01T22:19:46.0699881Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0700550Z     --> src/librustc_middle/query/mod.rs:215:20
2020-04-01T22:19:46.0701082Z      |
2020-04-01T22:19:46.0701082Z      |
2020-04-01T22:19:46.0701694Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0702626Z 39   | |     Other {
2020-04-01T22:19:46.0703568Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0704586Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0705275Z ...    |
2020-04-01T22:19:46.0706058Z 215  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0708038Z ...    |
2020-04-01T22:19:46.0708643Z 1290 | |     }
2020-04-01T22:19:46.0709312Z 1291 | | }
2020-04-01T22:19:46.0709312Z 1291 | | }
2020-04-01T22:19:46.0710073Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0710633Z      | 
2020-04-01T22:19:46.0711194Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0711690Z      |
2020-04-01T22:19:46.0712302Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0714050Z 
2020-04-01T22:19:46.0786749Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0787414Z     --> src/librustc_middle/query/mod.rs:264:20
2020-04-01T22:19:46.0787901Z      |
2020-04-01T22:19:46.0787901Z      |
2020-04-01T22:19:46.0788510Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0789221Z 39   | |     Other {
2020-04-01T22:19:46.0790288Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0792437Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0793084Z ...    |
2020-04-01T22:19:46.0793801Z 264  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0795815Z ...    |
2020-04-01T22:19:46.0796585Z 1290 | |     }
2020-04-01T22:19:46.0797263Z 1291 | | }
2020-04-01T22:19:46.0797263Z 1291 | | }
2020-04-01T22:19:46.0798248Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0798855Z      | 
2020-04-01T22:19:46.0799460Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0800038Z      |
2020-04-01T22:19:46.0800702Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0802086Z 
2020-04-01T22:19:46.0873985Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0874695Z     --> src/librustc_middle/query/mod.rs:303:20
2020-04-01T22:19:46.0875250Z      |
2020-04-01T22:19:46.0875250Z      |
2020-04-01T22:19:46.0875895Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0876665Z 39   | |     Other {
2020-04-01T22:19:46.0877626Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0878621Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0879328Z ...    |
2020-04-01T22:19:46.0880092Z 303  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0882074Z ...    |
2020-04-01T22:19:46.0882728Z 1290 | |     }
2020-04-01T22:19:46.0883569Z 1291 | | }
2020-04-01T22:19:46.0883569Z 1291 | | }
2020-04-01T22:19:46.0884313Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0884889Z      | 
2020-04-01T22:19:46.0885601Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0886156Z      |
2020-04-01T22:19:46.0886797Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0888069Z 
2020-04-01T22:19:46.0951567Z error: unnecessary parentheses around type
2020-04-01T22:19:46.0952380Z     --> src/librustc_middle/query/mod.rs:306:20
2020-04-01T22:19:46.0952930Z      |
2020-04-01T22:19:46.0952930Z      |
2020-04-01T22:19:46.0953574Z 38   | / rustc_queries! {
2020-04-01T22:19:46.0954356Z 39   | |     Other {
2020-04-01T22:19:46.0955319Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.0956319Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.0957022Z ...    |
2020-04-01T22:19:46.0957775Z 306  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.0959749Z ...    |
2020-04-01T22:19:46.0960405Z 1290 | |     }
2020-04-01T22:19:46.0961145Z 1291 | | }
2020-04-01T22:19:46.0961145Z 1291 | | }
2020-04-01T22:19:46.0961951Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.0962571Z      | 
2020-04-01T22:19:46.0963160Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.0963693Z      |
2020-04-01T22:19:46.0964463Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.0965883Z 
2020-04-01T22:19:46.1031787Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1058051Z     --> src/librustc_middle/query/mod.rs:369:20
2020-04-01T22:19:46.1058675Z      |
2020-04-01T22:19:46.1058675Z      |
2020-04-01T22:19:46.1059320Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1060091Z 39   | |     Other {
2020-04-01T22:19:46.1061057Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1062052Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1062924Z ...    |
2020-04-01T22:19:46.1063774Z 369  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1065794Z ...    |
2020-04-01T22:19:46.1066401Z 1290 | |     }
2020-04-01T22:19:46.1067085Z 1291 | | }
2020-04-01T22:19:46.1067085Z 1291 | | }
2020-04-01T22:19:46.1067832Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1068611Z      | 
2020-04-01T22:19:46.1069471Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1070013Z      |
2020-04-01T22:19:46.1070693Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1072059Z 
2020-04-01T22:19:46.1116102Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1116771Z     --> src/librustc_middle/query/mod.rs:381:20
2020-04-01T22:19:46.1117293Z      |
2020-04-01T22:19:46.1117293Z      |
2020-04-01T22:19:46.1117884Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1118627Z 39   | |     Other {
2020-04-01T22:19:46.1119505Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1120430Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1121089Z ...    |
2020-04-01T22:19:46.1121787Z 381  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1123624Z ...    |
2020-04-01T22:19:46.1124237Z 1290 | |     }
2020-04-01T22:19:46.1124926Z 1291 | | }
2020-04-01T22:19:46.1124926Z 1291 | | }
2020-04-01T22:19:46.1125673Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1126255Z      | 
2020-04-01T22:19:46.1126799Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1127297Z      |
2020-04-01T22:19:46.1127929Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1129350Z 
2020-04-01T22:19:46.1197278Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1198032Z     --> src/librustc_middle/query/mod.rs:395:20
2020-04-01T22:19:46.1198585Z      |
2020-04-01T22:19:46.1198585Z      |
2020-04-01T22:19:46.1199228Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1200013Z 39   | |     Other {
2020-04-01T22:19:46.1201005Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1201928Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1202593Z ...    |
2020-04-01T22:19:46.1203293Z 395  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1205128Z ...    |
2020-04-01T22:19:46.1205734Z 1290 | |     }
2020-04-01T22:19:46.1206618Z 1291 | | }
2020-04-01T22:19:46.1206618Z 1291 | | }
2020-04-01T22:19:46.1207423Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1208052Z      | 
2020-04-01T22:19:46.1208642Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1209178Z      |
2020-04-01T22:19:46.1209854Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1211232Z 
2020-04-01T22:19:46.1286112Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1286869Z     --> src/librustc_middle/query/mod.rs:520:20
2020-04-01T22:19:46.1287425Z      |
2020-04-01T22:19:46.1287425Z      |
2020-04-01T22:19:46.1288065Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1288874Z 39   | |     Other {
2020-04-01T22:19:46.1289820Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1290817Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1291527Z ...    |
2020-04-01T22:19:46.1292280Z 520  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1294560Z ...    |
2020-04-01T22:19:46.1295292Z 1290 | |     }
2020-04-01T22:19:46.1296180Z 1291 | | }
2020-04-01T22:19:46.1296180Z 1291 | | }
2020-04-01T22:19:46.1296930Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1297507Z      | 
2020-04-01T22:19:46.1298051Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1298545Z      |
2020-04-01T22:19:46.1299172Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1300461Z 
2020-04-01T22:19:46.1367552Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1368279Z     --> src/librustc_middle/query/mod.rs:536:20
2020-04-01T22:19:46.1368836Z      |
2020-04-01T22:19:46.1368836Z      |
2020-04-01T22:19:46.1369477Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1370268Z 39   | |     Other {
2020-04-01T22:19:46.1371211Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1372229Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1372925Z ...    |
2020-04-01T22:19:46.1373787Z 536  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1375765Z ...    |
2020-04-01T22:19:46.1376525Z 1290 | |     }
2020-04-01T22:19:46.1377194Z 1291 | | }
2020-04-01T22:19:46.1377194Z 1291 | | }
2020-04-01T22:19:46.1377938Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1378521Z      | 
2020-04-01T22:19:46.1379071Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1379584Z      |
2020-04-01T22:19:46.1380196Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1381473Z 
2020-04-01T22:19:46.1448134Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1448810Z     --> src/librustc_middle/query/mod.rs:638:20
2020-04-01T22:19:46.1449426Z      |
2020-04-01T22:19:46.1449426Z      |
2020-04-01T22:19:46.1449942Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1450786Z 39   | |     Other {
2020-04-01T22:19:46.1451561Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1452811Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1453698Z ...    |
2020-04-01T22:19:46.1454456Z 638  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1456442Z ...    |
2020-04-01T22:19:46.1457120Z 1290 | |     }
2020-04-01T22:19:46.1457847Z 1291 | | }
2020-04-01T22:19:46.1457847Z 1291 | | }
2020-04-01T22:19:46.1458656Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1459286Z      | 
2020-04-01T22:19:46.1459873Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1460427Z      |
2020-04-01T22:19:46.1461090Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1462491Z 
2020-04-01T22:19:46.1531326Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1533873Z     --> src/librustc_middle/query/mod.rs:713:20
2020-04-01T22:19:46.1534541Z      |
2020-04-01T22:19:46.1534541Z      |
2020-04-01T22:19:46.1535134Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1535864Z 39   | |     Other {
2020-04-01T22:19:46.1536736Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1537679Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1538334Z ...    |
2020-04-01T22:19:46.1539031Z 713  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1541085Z ...    |
2020-04-01T22:19:46.1541756Z 1290 | |     }
2020-04-01T22:19:46.1542477Z 1291 | | }
2020-04-01T22:19:46.1542477Z 1291 | | }
2020-04-01T22:19:46.1543281Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1544084Z      | 
2020-04-01T22:19:46.1544731Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1545298Z      |
2020-04-01T22:19:46.1545957Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1547336Z 
2020-04-01T22:19:46.1614821Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1615813Z     --> src/librustc_middle/query/mod.rs:717:20
2020-04-01T22:19:46.1616371Z      |
2020-04-01T22:19:46.1616371Z      |
2020-04-01T22:19:46.1617023Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1617813Z 39   | |     Other {
2020-04-01T22:19:46.1618756Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1619771Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1620537Z ...    |
2020-04-01T22:19:46.1621239Z 717  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1623321Z ...    |
2020-04-01T22:19:46.1623993Z 1290 | |     }
2020-04-01T22:19:46.1624712Z 1291 | | }
2020-04-01T22:19:46.1624712Z 1291 | | }
2020-04-01T22:19:46.1625520Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1626143Z      | 
2020-04-01T22:19:46.1626727Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1627277Z      |
2020-04-01T22:19:46.1627934Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1629326Z 
2020-04-01T22:19:46.1707475Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1708234Z     --> src/librustc_middle/query/mod.rs:854:20
2020-04-01T22:19:46.1708768Z      |
2020-04-01T22:19:46.1708768Z      |
2020-04-01T22:19:46.1709410Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1710200Z 39   | |     Other {
2020-04-01T22:19:46.1711141Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1712316Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1713068Z ...    |
2020-04-01T22:19:46.1713835Z 854  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1715811Z ...    |
2020-04-01T22:19:46.1716483Z 1290 | |     }
2020-04-01T22:19:46.1717204Z 1291 | | }
2020-04-01T22:19:46.1717204Z 1291 | | }
2020-04-01T22:19:46.1718009Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1718645Z      | 
2020-04-01T22:19:46.1719237Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1719788Z      |
2020-04-01T22:19:46.1720450Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1721836Z 
2020-04-01T22:19:46.1809316Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1816081Z     --> src/librustc_middle/query/mod.rs:871:20
2020-04-01T22:19:46.1816636Z      |
2020-04-01T22:19:46.1816636Z      |
2020-04-01T22:19:46.1817293Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1818085Z 39   | |     Other {
2020-04-01T22:19:46.1819030Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1820044Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1820842Z ...    |
2020-04-01T22:19:46.1821541Z 871  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1823380Z ...    |
2020-04-01T22:19:46.1824003Z 1290 | |     }
2020-04-01T22:19:46.1824672Z 1291 | | }
2020-04-01T22:19:46.1824672Z 1291 | | }
2020-04-01T22:19:46.1825433Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1825995Z      | 
2020-04-01T22:19:46.1826539Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1827050Z      |
2020-04-01T22:19:46.1827661Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1829153Z 
2020-04-01T22:19:46.1891083Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1891787Z     --> src/librustc_middle/query/mod.rs:957:20
2020-04-01T22:19:46.1892280Z      |
2020-04-01T22:19:46.1892280Z      |
2020-04-01T22:19:46.1892873Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1893795Z 39   | |     Other {
2020-04-01T22:19:46.1894676Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1895860Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1896554Z ...    |
2020-04-01T22:19:46.1897309Z 957  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1899291Z ...    |
2020-04-01T22:19:46.1899962Z 1290 | |     }
2020-04-01T22:19:46.1900683Z 1291 | | }
2020-04-01T22:19:46.1900683Z 1291 | | }
2020-04-01T22:19:46.1901509Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1902127Z      | 
2020-04-01T22:19:46.1902716Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1903272Z      |
2020-04-01T22:19:46.1903931Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1905318Z 
2020-04-01T22:19:46.1966802Z error: unnecessary parentheses around type
2020-04-01T22:19:46.1967507Z     --> src/librustc_middle/query/mod.rs:976:20
2020-04-01T22:19:46.1968084Z      |
2020-04-01T22:19:46.1968084Z      |
2020-04-01T22:19:46.1968733Z 38   | / rustc_queries! {
2020-04-01T22:19:46.1969503Z 39   | |     Other {
2020-04-01T22:19:46.1970703Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.1971847Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.1972692Z ...    |
2020-04-01T22:19:46.1973706Z 976  | |             storage(ArenaCacheSelector<'tcx>)
2020-04-01T22:19:46.1976182Z ...    |
2020-04-01T22:19:46.1976982Z 1290 | |     }
2020-04-01T22:19:46.1977838Z 1291 | | }
2020-04-01T22:19:46.1977838Z 1291 | | }
2020-04-01T22:19:46.1978787Z      | |_- in this expansion of `rustc_query_append!`
2020-04-01T22:19:46.1979518Z      | 
2020-04-01T22:19:46.1980253Z     ::: src/librustc_middle/ty/query/mod.rs:104:1
2020-04-01T22:19:46.1980911Z      |
2020-04-01T22:19:46.1981695Z 104  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-04-01T22:19:46.1983331Z 
2020-04-01T22:19:46.2053764Z error: unnecessary parentheses around type
2020-04-01T22:19:46.2054468Z     --> src/librustc_middle/query/mod.rs:1010:20
2020-04-01T22:19:46.2055000Z      |
2020-04-01T22:19:46.2055000Z      |
2020-04-01T22:19:46.2055657Z 38   | / rustc_queries! {
2020-04-01T22:19:46.2056423Z 39   | |     Other {
2020-04-01T22:19:46.2057382Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-04-01T22:19:46.2058398Z 41   | |             desc { "trigger a delay span bug" }
2020-04-01T22:19:46.2059356Z ...    |
---
2020-04-01T22:19:57.7549114Z expected success, got: exit code: 101
2020-04-01T22:19:57.7562506Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T22:19:57.7563128Z Build completed unsuccessfully in 0:25:49
2020-04-01T22:19:57.7621446Z == clock drift check ==
2020-04-01T22:19:57.7640225Z   local time: Wed Apr  1 22:19:57 UTC 2020
2020-04-01T22:19:58.0540172Z   network time: Wed, 01 Apr 2020 22:19:58 GMT
2020-04-01T22:19:58.6716475Z 
2020-04-01T22:19:58.6716475Z 
2020-04-01T22:19:58.6779971Z ##[error]Bash exited with code '1'.
2020-04-01T22:19:58.6798918Z ##[section]Finishing: Run build
2020-04-01T22:19:58.6851969Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-01T22:19:58.6857824Z Task         : Get sources
2020-04-01T22:19:58.6858204Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T22:19:58.6858548Z Version      : 1.0.0
2020-04-01T22:19:58.6858783Z Author       : Microsoft
2020-04-01T22:19:58.6858783Z Author       : Microsoft
2020-04-01T22:19:58.6859170Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T22:19:58.6859588Z ==============================================================================
2020-04-01T22:19:59.0010512Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T22:19:59.0063387Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-01T22:19:59.0147900Z Cleaning up task key
2020-04-01T22:19:59.0149149Z Start cleaning up orphan processes.
2020-04-01T22:19:59.0356514Z Terminate orphan process: pid (3493) (python)
2020-04-01T22:19:59.0523862Z ##[section]Finishing: Finalize Job
