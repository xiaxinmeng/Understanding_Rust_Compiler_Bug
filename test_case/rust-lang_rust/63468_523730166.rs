plain
2019-08-22T02:39:58.5391234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T02:39:58.5582742Z ##[command]git config gc.auto 0
2019-08-22T02:39:58.5650215Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T02:39:58.5706021Z ##[command]git config --get-all http.proxy
2019-08-22T02:39:58.5853400Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
2019-08-22T02:40:00.0024646Z remote:                                                                                         
---
2019-08-22T02:40:32.5212568Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T02:40:32.5213879Z 
2019-08-22T02:40:32.5215512Z   git checkout -b <new-branch-name>
2019-08-22T02:40:32.5216607Z 
2019-08-22T02:40:32.5217223Z HEAD is now at a7d605485 Merge e1c536835cedb9ce2a7ee61864d2e1fcd4fc0c0a into 42dcd4b7c5fb7b61bc2f4c0842f66e5ad40057e4
2019-08-22T02:40:32.5395124Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T02:40:32.5398464Z ==============================================================================
2019-08-22T02:40:32.5398522Z Task         : Bash
2019-08-22T02:40:32.5398731Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T03:16:26.6838499Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-22T03:16:45.6264118Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6265163Z    --> src/librustc/hir/mod.rs:347:7
2019-08-22T03:16:45.6265736Z     |
2019-08-22T03:16:45.6266568Z 347 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6267669Z 
2019-08-22T03:16:45.6280071Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6280653Z    --> src/librustc/hir/mod.rs:858:7
2019-08-22T03:16:45.6281047Z     |
2019-08-22T03:16:45.6281047Z     |
2019-08-22T03:16:45.6281447Z 858 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6282266Z 
2019-08-22T03:16:45.6297572Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6298206Z    --> src/librustc/hir/mod.rs:871:7
2019-08-22T03:16:45.6298681Z     |
2019-08-22T03:16:45.6298681Z     |
2019-08-22T03:16:45.6299172Z 871 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6299831Z 
2019-08-22T03:16:45.6314839Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6315376Z    --> src/librustc/hir/mod.rs:935:7
2019-08-22T03:16:45.6316008Z     |
2019-08-22T03:16:45.6316008Z     |
2019-08-22T03:16:45.6316978Z 935 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6317685Z 
2019-08-22T03:16:45.6333461Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6334343Z    --> src/librustc/hir/mod.rs:938:7
2019-08-22T03:16:45.6334934Z     |
2019-08-22T03:16:45.6334934Z     |
2019-08-22T03:16:45.6335509Z 938 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6336078Z 
2019-08-22T03:16:45.6355682Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6357490Z     --> src/librustc/hir/mod.rs:1266:7
2019-08-22T03:16:45.6358128Z      |
2019-08-22T03:16:45.6358128Z      |
2019-08-22T03:16:45.6358864Z 1266 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6359342Z 
2019-08-22T03:16:45.6379346Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6381308Z     --> src/librustc/hir/mod.rs:1285:7
2019-08-22T03:16:45.6381597Z      |
2019-08-22T03:16:45.6381597Z      |
2019-08-22T03:16:45.6382280Z 1285 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6382618Z 
2019-08-22T03:16:45.6401707Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6402251Z     --> src/librustc/hir/mod.rs:1884:7
2019-08-22T03:16:45.6402463Z      |
2019-08-22T03:16:45.6402463Z      |
2019-08-22T03:16:45.6402744Z 1884 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6403178Z 
2019-08-22T03:16:45.6423307Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6423631Z     --> src/librustc/hir/mod.rs:2216:7
2019-08-22T03:16:45.6423840Z      |
2019-08-22T03:16:45.6423840Z      |
2019-08-22T03:16:45.6424111Z 2216 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6424405Z 
2019-08-22T03:16:45.6443602Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6444097Z     --> src/librustc/hir/mod.rs:2256:7
2019-08-22T03:16:45.6444313Z      |
2019-08-22T03:16:45.6444313Z      |
2019-08-22T03:16:45.6444979Z 2256 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6445306Z 
2019-08-22T03:16:45.6465175Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6465556Z     --> src/librustc/hir/mod.rs:2325:7
2019-08-22T03:16:45.6465781Z      |
2019-08-22T03:16:45.6465781Z      |
2019-08-22T03:16:45.6466075Z 2325 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6467275Z 
2019-08-22T03:16:45.6486984Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6487435Z     --> src/librustc/hir/mod.rs:2520:7
2019-08-22T03:16:45.6487672Z      |
2019-08-22T03:16:45.6487672Z      |
2019-08-22T03:16:45.6488000Z 2520 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6488323Z 
2019-08-22T03:16:45.6508295Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6508716Z     --> src/librustc/hir/mod.rs:2536:7
2019-08-22T03:16:45.6508990Z      |
2019-08-22T03:16:45.6508990Z      |
2019-08-22T03:16:45.6509321Z 2536 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6509644Z 
2019-08-22T03:16:45.6532791Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6533096Z     --> src/librustc/hir/mod.rs:2554:7
2019-08-22T03:16:45.6533340Z      |
2019-08-22T03:16:45.6533340Z      |
2019-08-22T03:16:45.6533598Z 2554 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6533900Z 
2019-08-22T03:16:45.6553509Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6553796Z   --> src/librustc/ty/trait_def.rs:19:7
2019-08-22T03:16:45.6554022Z    |
2019-08-22T03:16:45.6554022Z    |
2019-08-22T03:16:45.6554263Z 19 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6554526Z 
2019-08-22T03:16:45.6575525Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6575840Z    --> src/librustc/ty/mod.rs:172:7
2019-08-22T03:16:45.6576041Z     |
2019-08-22T03:16:45.6576041Z     |
2019-08-22T03:16:45.6576317Z 172 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6577144Z 
2019-08-22T03:16:45.6600930Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6601200Z    --> src/librustc/ty/mod.rs:901:7
2019-08-22T03:16:45.6601398Z     |
2019-08-22T03:16:45.6601398Z     |
2019-08-22T03:16:45.6601667Z 901 |     #[stable_hasher(ignore)]
2019-08-22T03:16:45.6601946Z 
2019-08-22T03:16:45.6625780Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-22T03:16:45.6626080Z     --> src/librustc/ty/mod.rs:1934:7
2019-08-22T03:16:45.6626776Z      |
2019-08-22T03:16:45.6626776Z      |
2019-08-22T03:16:45.6627131Z 1934 |     #[stable_hasher(project(name))]
2019-08-22T03:16:45.6627689Z 
2019-08-22T03:16:45.6627689Z 
2019-08-22T03:16:49.5838356Z error: internal compiler error: src/librustc/hir/map/mod.rs:244: local_def_id: no entry for `HirId { owner: DefIndex(22797), local_id: 15 }`, which has a map of `Some(Entry { parent: HirId { owner: DefIndex(22797), local_id: 0 }, dep_node: 4294967040, node: Field(StructField { span: src/librustc/hir/mod.rs:859:5: 859:22, ident: hir_id#0, vis: Spanned { node: Public, span: src/librustc/hir/mod.rs:859:5: 859:8 }, hir_id: HirId { owner: DefIndex(22797), local_id: 15 }, ty: type(HirId), attrs: [Attribute { id: AttrId(1407), style: Outer, path: path(stable_hasher), tokens: TokenStream(Some([(Delimited(DelimSpan { open: src/librustc/hir/mod.rs:858:20: 858:21, close: src/librustc/hir/mod.rs:858:27: 858:28 }, Paren, TokenStream(Some([(Token(Token { kind: Ident(ignore, false), span: src/librustc/hir/mod.rs:858:21: 858:27 }), NonJoint)]))), NonJoint)])), is_sugared_doc: false, span: src/librustc/hir/mod.rs:858:5: 858:29 }] }) })`
2019-08-22T03:16:49.5838952Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-22T03:16:49.5839791Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-22T03:16:49.7708518Z error: aborting due to 19 previous errors
2019-08-22T03:16:49.7708652Z 
2019-08-22T03:16:49.7708652Z 
2019-08-22T03:16:49.7875033Z 
2019-08-22T03:16:49.7879497Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-22T03:16:49.7884920Z 
2019-08-22T03:16:49.7885736Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-22T03:16:49.7886044Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-22T03:16:49.7886076Z 
2019-08-22T03:16:49.7886076Z 
2019-08-22T03:16:49.7887108Z note: compiler flags: -Z binary-dep-depinfo -Z unstable-options -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-22T03:16:49.7887204Z 
2019-08-22T03:16:49.7887260Z note: some of the compiler flags provided by cargo are hidden
2019-08-22T03:16:49.7966943Z error: Could not compile `rustc`.
2019-08-22T03:16:49.7967260Z warning: build failed, waiting for other jobs to finish...
2019-08-22T03:18:04.5208073Z error: build failed
2019-08-22T03:18:04.5224492Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T03:18:04.5224492Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T03:18:04.5224584Z expected success, got: exit code: 101
2019-08-22T03:18:04.5236399Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-22T03:18:04.5237315Z Build completed unsuccessfully in 0:30:42
2019-08-22T03:18:04.5289298Z == clock drift check ==
2019-08-22T03:18:04.5307430Z   local time: Thu Aug 22 03:18:04 UTC 2019
2019-08-22T03:18:04.6195532Z   network time: Thu, 22 Aug 2019 03:18:04 GMT
2019-08-22T03:18:04.6195665Z == end clock drift check ==
2019-08-22T03:18:05.6666433Z ##[error]Bash exited with code '1'.
2019-08-22T03:18:05.6709210Z ##[section]Starting: Checkout
2019-08-22T03:18:05.6711364Z ==============================================================================
2019-08-22T03:18:05.6711420Z Task         : Get sources
2019-08-22T03:18:05.6711484Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
