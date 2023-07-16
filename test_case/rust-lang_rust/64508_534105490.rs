plain
2019-09-23T12:56:25.4047241Z 
2019-09-23T12:56:25.5598174Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:25.5599115Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:357:82
2019-09-23T12:56:25.5599487Z     |
2019-09-23T12:56:25.5599904Z 357 |                     println!("    if {}[{}].pats.len() == {};", arms_pat, i, arm.pats.len());
2019-09-23T12:56:25.5600451Z 
2019-09-23T12:56:25.5661503Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:25.5661945Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:358:41
2019-09-23T12:56:25.5662387Z     |
2019-09-23T12:56:25.5662387Z     |
2019-09-23T12:56:25.5662763Z 358 |                     for (j, pat) in arm.pats.iter().enumerate() {
2019-09-23T12:56:25.5663556Z 
2019-09-23T12:56:25.9278213Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:25.9279597Z    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:127:40
2019-09-23T12:56:25.9280002Z     |
---
2019-09-23T12:56:26.0542751Z [RUSTC-TIMING] cargo_metadata test:false 24.201
2019-09-23T12:56:26.5990084Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:26.5990636Z    --> src/tools/clippy/clippy_lints/src/cognitive_complexity.rs:115:61
2019-09-23T12:56:26.5990967Z     |
2019-09-23T12:56:26.5991375Z 115 |                 let arms_n: u64 = arms.iter().map(|arm| arm.pats.len() as u64).sum();
2019-09-23T12:56:26.5991928Z 
2019-09-23T12:56:26.6301265Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:26.6301710Z    --> src/tools/clippy/clippy_lints/src/copies.rs:196:54
2019-09-23T12:56:26.6302016Z     |
---
2019-09-23T12:56:26.6392590Z 
2019-09-23T12:56:26.6409775Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:26.6410176Z    --> src/tools/clippy/clippy_lints/src/copies.rs:217:49
2019-09-23T12:56:26.6410483Z     |
2019-09-23T12:56:26.6410868Z 217 |                         let lhs = snippet(cx, i.pats[0].span, "<pat1>");
2019-09-23T12:56:26.6411349Z 
2019-09-23T12:56:26.6429845Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:26.6430283Z    --> src/tools/clippy/clippy_lints/src/copies.rs:218:49
2019-09-23T12:56:26.6430571Z     |
2019-09-23T12:56:26.6430571Z     |
2019-09-23T12:56:26.6430990Z 218 |                         let rhs = snippet(cx, j.pats[0].span, "<pat2>");
2019-09-23T12:56:26.6431475Z 
2019-09-23T12:56:26.6449595Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:26.6450027Z    --> src/tools/clippy/clippy_lints/src/copies.rs:220:50
2019-09-23T12:56:26.6450335Z     |
---
2019-09-23T12:56:27.0756638Z 
2019-09-23T12:56:27.0783036Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.0783462Z   --> src/tools/clippy/clippy_lints/src/infallible_destructuring_match.rs:51:105
2019-09-23T12:56:27.0783798Z    |
2019-09-23T12:56:27.0784231Z 51 |             if let PatKind::TupleStruct(QPath::Resolved(None, ref variant_name), ref args, _) = arms[0].pats[0].node;
2019-09-23T12:56:27.0784896Z 
2019-09-23T12:56:27.3545921Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.3546567Z    --> src/tools/clippy/clippy_lints/src/loops.rs:520:44
2019-09-23T12:56:27.3546877Z     |
---
2019-09-23T12:56:27.3575045Z 
2019-09-23T12:56:27.3611497Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.3612335Z    --> src/tools/clippy/clippy_lints/src/loops.rs:544:80
2019-09-23T12:56:27.3612709Z     |
2019-09-23T12:56:27.3613116Z 544 | ...                   snippet_with_applicability(cx, arms[0].pats[0].span, "..", &mut applicability),
2019-09-23T12:56:27.3613714Z 
2019-09-23T12:56:27.3645864Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.3646327Z    --> src/tools/clippy/clippy_lints/src/loops.rs:557:32
2019-09-23T12:56:27.3646583Z     |
---
2019-09-23T12:56:27.6540303Z 
2019-09-23T12:56:27.6566193Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.6566563Z    --> src/tools/clippy/clippy_lints/src/matches.rs:370:77
2019-09-23T12:56:27.6566848Z     |
2019-09-23T12:56:27.6567241Z 370 |                     let exprs = if let PatKind::Lit(ref arm_bool) = arms[0].pats[0].node {
2019-09-23T12:56:27.6567884Z 
2019-09-23T12:56:27.6719476Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:27.6719998Z    --> src/tools/clippy/clippy_lints/src/matches.rs:449:71
2019-09-23T12:56:27.6720312Z     |
---
2019-09-23T12:56:27.6884411Z 
2019-09-23T12:56:27.7031651Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:27.7032086Z    --> src/tools/clippy/clippy_lints/src/matches.rs:591:50
2019-09-23T12:56:27.7032534Z     |
2019-09-23T12:56:27.7032912Z 591 |         suggs.extend(arms.iter().flat_map(|a| &a.pats).filter_map(|p| {
2019-09-23T12:56:27.7033364Z 
2019-09-23T12:56:27.7092447Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:27.7092854Z    --> src/tools/clippy/clippy_lints/src/matches.rs:609:20
2019-09-23T12:56:27.7093114Z     |
---
2019-09-23T12:56:27.7123287Z 
2019-09-23T12:56:27.7198997Z error[E0026]: struct `rustc::hir::Arm` does not have a field named `pats`
2019-09-23T12:56:27.7199449Z    --> src/tools/clippy/clippy_lints/src/matches.rs:669:21
2019-09-23T12:56:27.7199733Z     |
2019-09-23T12:56:27.7200091Z 669 |                 ref pats, guard: None, ..
2019-09-23T12:56:27.7200780Z     |                     |
2019-09-23T12:56:27.7201191Z     |                     struct `rustc::hir::Arm` does not have this field
2019-09-23T12:56:27.7201860Z     |                     help: a field with a similar name exists: `pat`
2019-09-23T12:56:27.7201967Z 
---
2019-09-23T12:56:27.7434854Z 
2019-09-23T12:56:28.4409517Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:28.4411641Z   --> src/tools/clippy/clippy_lints/src/ok_if_let.rs:45:89
2019-09-23T12:56:28.4411994Z    |
2019-09-23T12:56:28.4412487Z 45 |             if let PatKind::TupleStruct(QPath::Resolved(_, ref x), ref y, _)  = body[0].pats[0].node; //get operation
2019-09-23T12:56:28.4413293Z 
2019-09-23T12:56:28.6246549Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:28.6247101Z   --> src/tools/clippy/clippy_lints/src/redundant_pattern_matching.rs:60:16
2019-09-23T12:56:28.6247411Z    |
---
2019-09-23T12:56:30.9150478Z 
2019-09-23T12:56:31.0624160Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:31.0624651Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:357:82
2019-09-23T12:56:31.0624921Z     |
2019-09-23T12:56:31.0625346Z 357 |                     println!("    if {}[{}].pats.len() == {};", arms_pat, i, arm.pats.len());
2019-09-23T12:56:31.0626020Z 
2019-09-23T12:56:31.0649278Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:31.0649697Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:358:41
2019-09-23T12:56:31.0649997Z     |
2019-09-23T12:56:31.0649997Z     |
2019-09-23T12:56:31.0650753Z 358 |                     for (j, pat) in arm.pats.iter().enumerate() {
2019-09-23T12:56:31.0651223Z 
2019-09-23T12:56:31.4403986Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:31.4405685Z    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:127:40
2019-09-23T12:56:31.4406349Z     |
---
2019-09-23T12:56:31.5431638Z 
2019-09-23T12:56:32.0925446Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:32.0926022Z    --> src/tools/clippy/clippy_lints/src/cognitive_complexity.rs:115:61
2019-09-23T12:56:32.0926680Z     |
2019-09-23T12:56:32.0927065Z 115 |                 let arms_n: u64 = arms.iter().map(|arm| arm.pats.len() as u64).sum();
2019-09-23T12:56:32.0927568Z 
2019-09-23T12:56:32.1228271Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:32.1228796Z    --> src/tools/clippy/clippy_lints/src/copies.rs:196:54
2019-09-23T12:56:32.1229135Z     |
---
2019-09-23T12:56:32.1314911Z 
2019-09-23T12:56:32.1342514Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:32.1342991Z    --> src/tools/clippy/clippy_lints/src/copies.rs:217:49
2019-09-23T12:56:32.1343290Z     |
2019-09-23T12:56:32.1343679Z 217 |                         let lhs = snippet(cx, i.pats[0].span, "<pat1>");
2019-09-23T12:56:32.1344187Z 
2019-09-23T12:56:32.1363536Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:32.1364258Z    --> src/tools/clippy/clippy_lints/src/copies.rs:218:49
2019-09-23T12:56:32.1364552Z     |
2019-09-23T12:56:32.1364552Z     |
2019-09-23T12:56:32.1364927Z 218 |                         let rhs = snippet(cx, j.pats[0].span, "<pat2>");
2019-09-23T12:56:32.1365388Z 
2019-09-23T12:56:32.1385467Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:32.1385883Z    --> src/tools/clippy/clippy_lints/src/copies.rs:220:50
2019-09-23T12:56:32.1386157Z     |
---
2019-09-23T12:56:32.5922688Z 
2019-09-23T12:56:32.5939851Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:32.5940300Z   --> src/tools/clippy/clippy_lints/src/infallible_destructuring_match.rs:51:105
2019-09-23T12:56:32.5940654Z    |
2019-09-23T12:56:32.5941120Z 51 |             if let PatKind::TupleStruct(QPath::Resolved(None, ref variant_name), ref args, _) = arms[0].pats[0].node;
2019-09-23T12:56:32.5941844Z 
2019-09-23T12:56:32.8686480Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:32.8687027Z    --> src/tools/clippy/clippy_lints/src/loops.rs:520:44
2019-09-23T12:56:32.8687337Z     |
---
2019-09-23T12:56:32.8709042Z 
2019-09-23T12:56:32.8742583Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:32.8743071Z    --> src/tools/clippy/clippy_lints/src/loops.rs:544:80
2019-09-23T12:56:32.8743407Z     |
2019-09-23T12:56:32.8743871Z 544 | ...                   snippet_with_applicability(cx, arms[0].pats[0].span, "..", &mut applicability),
2019-09-23T12:56:32.8744549Z 
2019-09-23T12:56:32.8773741Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:32.8774362Z    --> src/tools/clippy/clippy_lints/src/loops.rs:557:32
2019-09-23T12:56:32.8774672Z     |
---
2019-09-23T12:56:33.1669332Z 
2019-09-23T12:56:33.1686181Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:33.1686617Z    --> src/tools/clippy/clippy_lints/src/matches.rs:370:77
2019-09-23T12:56:33.1686921Z     |
2019-09-23T12:56:33.1687329Z 370 |                     let exprs = if let PatKind::Lit(ref arm_bool) = arms[0].pats[0].node {
2019-09-23T12:56:33.1692841Z 
2019-09-23T12:56:33.1822869Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:33.1823295Z    --> src/tools/clippy/clippy_lints/src/matches.rs:449:71
2019-09-23T12:56:33.1823572Z     |
---
2019-09-23T12:56:33.1952806Z 
2019-09-23T12:56:33.2096425Z error[E0609]: no field `pats` on type `&rustc::hir::Arm`
2019-09-23T12:56:33.2096827Z    --> src/tools/clippy/clippy_lints/src/matches.rs:591:50
2019-09-23T12:56:33.2097139Z     |
2019-09-23T12:56:33.2097541Z 591 |         suggs.extend(arms.iter().flat_map(|a| &a.pats).filter_map(|p| {
2019-09-23T12:56:33.2098281Z 
2019-09-23T12:56:33.2152199Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:33.2152640Z    --> src/tools/clippy/clippy_lints/src/matches.rs:609:20
2019-09-23T12:56:33.2152943Z     |
---
2019-09-23T12:56:33.2179208Z 
2019-09-23T12:56:33.2241610Z error[E0026]: struct `rustc::hir::Arm` does not have a field named `pats`
2019-09-23T12:56:33.2242044Z    --> src/tools/clippy/clippy_lints/src/matches.rs:669:21
2019-09-23T12:56:33.2242513Z     |
2019-09-23T12:56:33.2243114Z 669 |                 ref pats, guard: None, ..
2019-09-23T12:56:33.2243800Z     |                     |
2019-09-23T12:56:33.2244467Z     |                     struct `rustc::hir::Arm` does not have this field
2019-09-23T12:56:33.2244990Z     |                     help: a field with a similar name exists: `pat`
2019-09-23T12:56:33.2250516Z 
---
2019-09-23T12:56:33.2489580Z 
2019-09-23T12:56:33.9479739Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:33.9480222Z   --> src/tools/clippy/clippy_lints/src/ok_if_let.rs:45:89
2019-09-23T12:56:33.9480559Z    |
2019-09-23T12:56:33.9481417Z 45 |             if let PatKind::TupleStruct(QPath::Resolved(_, ref x), ref y, _)  = body[0].pats[0].node; //get operation
2019-09-23T12:56:33.9482444Z 
2019-09-23T12:56:34.1242305Z error[E0609]: no field `pats` on type `rustc::hir::Arm`
2019-09-23T12:56:34.1243077Z   --> src/tools/clippy/clippy_lints/src/redundant_pattern_matching.rs:60:16
2019-09-23T12:56:34.1243465Z    |
---
2019-09-23T13:43:03.2530367Z == clock drift check ==
2019-09-23T13:43:03.2543833Z   local time: Mon Sep 23 13:43:03 UTC 2019
2019-09-23T13:43:03.3254202Z   network time: Mon, 23 Sep 2019 13:43:03 GMT
2019-09-23T13:43:03.3260122Z == end clock drift check ==
2019-09-23T13:43:04.0187603Z ##[error]Bash exited with code '1'.
2019-09-23T13:43:04.0226579Z ##[section]Starting: Upload CPU usage statistics
2019-09-23T13:43:04.0233304Z ==============================================================================
2019-09-23T13:43:04.0233428Z Task         : Bash
2019-09-23T13:43:04.0233523Z Description  : Run a Bash script on macOS, Linux, or Windows
