plain
2019-08-21T18:12:35.2521077Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T18:12:35.2678050Z ##[command]git config gc.auto 0
2019-08-21T18:12:35.2755292Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T18:12:35.2808708Z ##[command]git config --get-all http.proxy
2019-08-21T18:12:35.2960933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-08-21T18:13:01.3480780Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T18:13:01.3480817Z 
2019-08-21T18:13:01.3481081Z   git checkout -b <new-branch-name>
2019-08-21T18:13:01.3481115Z 
2019-08-21T18:13:01.3481263Z HEAD is now at 2e9e70a7e Merge 41472c614bc0fea6d9c8d50185e537f1b5ef2844 into 7b0085a613e69cb69fc9e4eb5d422fa4a39d5de1
2019-08-21T18:13:01.3654160Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T18:13:01.3657574Z ==============================================================================
2019-08-21T18:13:01.3657799Z Task         : Bash
2019-08-21T18:13:01.3657850Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T18:48:57.7466323Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-21T18:49:15.7951596Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.7953003Z    --> src/librustc/hir/mod.rs:347:7
2019-08-21T18:49:15.7953631Z     |
2019-08-21T18:49:15.7954262Z 347 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.7958399Z 
2019-08-21T18:49:15.7980778Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.7981534Z    --> src/librustc/hir/mod.rs:858:7
2019-08-21T18:49:15.7982122Z     |
2019-08-21T18:49:15.7982122Z     |
2019-08-21T18:49:15.7982761Z 858 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.7986974Z 
2019-08-21T18:49:15.8009196Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8009913Z    --> src/librustc/hir/mod.rs:871:7
2019-08-21T18:49:15.8010502Z     |
2019-08-21T18:49:15.8010502Z     |
2019-08-21T18:49:15.8011409Z 871 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8015050Z 
2019-08-21T18:49:15.8037273Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8038149Z    --> src/librustc/hir/mod.rs:935:7
2019-08-21T18:49:15.8038768Z     |
2019-08-21T18:49:15.8038768Z     |
2019-08-21T18:49:15.8039410Z 935 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8043064Z 
2019-08-21T18:49:15.8064887Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8065962Z    --> src/librustc/hir/mod.rs:938:7
2019-08-21T18:49:15.8066632Z     |
2019-08-21T18:49:15.8066632Z     |
2019-08-21T18:49:15.8067263Z 938 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8073541Z 
2019-08-21T18:49:15.8096816Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8099016Z     --> src/librustc/hir/mod.rs:1266:7
2019-08-21T18:49:15.8099701Z      |
2019-08-21T18:49:15.8099701Z      |
2019-08-21T18:49:15.8100248Z 1266 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8104783Z 
2019-08-21T18:49:15.8127527Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8128175Z     --> src/librustc/hir/mod.rs:1285:7
2019-08-21T18:49:15.8128632Z      |
2019-08-21T18:49:15.8128632Z      |
2019-08-21T18:49:15.8129136Z 1285 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8132632Z 
2019-08-21T18:49:15.8155784Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8156679Z     --> src/librustc/hir/mod.rs:1884:7
2019-08-21T18:49:15.8157162Z      |
2019-08-21T18:49:15.8157162Z      |
2019-08-21T18:49:15.8157684Z 1884 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8161263Z 
2019-08-21T18:49:15.8184042Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8184639Z     --> src/librustc/hir/mod.rs:2216:7
2019-08-21T18:49:15.8185117Z      |
2019-08-21T18:49:15.8185117Z      |
2019-08-21T18:49:15.8185983Z 2216 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8189622Z 
2019-08-21T18:49:15.8212735Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8213375Z     --> src/librustc/hir/mod.rs:2256:7
2019-08-21T18:49:15.8213866Z      |
2019-08-21T18:49:15.8213866Z      |
2019-08-21T18:49:15.8214359Z 2256 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8218425Z 
2019-08-21T18:49:15.8241237Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8241836Z     --> src/librustc/hir/mod.rs:2325:7
2019-08-21T18:49:15.8242416Z      |
2019-08-21T18:49:15.8242416Z      |
2019-08-21T18:49:15.8242957Z 2325 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8247839Z 
2019-08-21T18:49:15.8272791Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8273441Z     --> src/librustc/hir/mod.rs:2520:7
2019-08-21T18:49:15.8273964Z      |
2019-08-21T18:49:15.8273964Z      |
2019-08-21T18:49:15.8274487Z 2520 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8281458Z 
2019-08-21T18:49:15.8309903Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8310246Z     --> src/librustc/hir/mod.rs:2536:7
2019-08-21T18:49:15.8310537Z      |
2019-08-21T18:49:15.8310537Z      |
2019-08-21T18:49:15.8311848Z 2536 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8317695Z 
2019-08-21T18:49:15.8347753Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8348154Z     --> src/librustc/hir/mod.rs:2554:7
2019-08-21T18:49:15.8348451Z      |
2019-08-21T18:49:15.8348451Z      |
2019-08-21T18:49:15.8348762Z 2554 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8353348Z 
2019-08-21T18:49:15.8374771Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8376927Z   --> src/librustc/ty/trait_def.rs:19:7
2019-08-21T18:49:15.8390369Z    |
2019-08-21T18:49:15.8390369Z    |
2019-08-21T18:49:15.8390763Z 19 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8391143Z 
2019-08-21T18:49:15.8419944Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8421143Z    --> src/librustc/ty/mod.rs:172:7
2019-08-21T18:49:15.8421823Z     |
2019-08-21T18:49:15.8421823Z     |
2019-08-21T18:49:15.8422181Z 172 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8422550Z 
2019-08-21T18:49:15.8426057Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8426504Z    --> src/librustc/ty/mod.rs:901:7
2019-08-21T18:49:15.8432377Z     |
2019-08-21T18:49:15.8432377Z     |
2019-08-21T18:49:15.8434415Z 901 |     #[stable_hasher(ignore)]
2019-08-21T18:49:15.8438094Z 
2019-08-21T18:49:15.8448642Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-21T18:49:15.8449360Z     --> src/librustc/ty/mod.rs:1934:7
2019-08-21T18:49:15.8450040Z      |
2019-08-21T18:49:15.8450040Z      |
2019-08-21T18:49:15.8450435Z 1934 |     #[stable_hasher(project(name))]
2019-08-21T18:49:15.8455699Z 
2019-08-21T18:49:15.8455699Z 
2019-08-21T18:49:19.7145097Z error: internal compiler error: src/librustc/hir/map/mod.rs:244: local_def_id: no entry for `HirId { owner: DefIndex(22797), local_id: 15 }`, which has a map of `Some(Entry { parent: HirId { owner: DefIndex(22797), local_id: 0 }, dep_node: 4294967040, node: Field(StructField { span: src/librustc/hir/mod.rs:859:5: 859:22, ident: hir_id#0, vis: Spanned { node: Public, span: src/librustc/hir/mod.rs:859:5: 859:8 }, hir_id: HirId { owner: DefIndex(22797), local_id: 15 }, ty: type(HirId), attrs: [Attribute { id: AttrId(1407), style: Outer, path: path(stable_hasher), tokens: TokenStream(Some([(Delimited(DelimSpan { open: src/librustc/hir/mod.rs:858:20: 858:21, close: src/librustc/hir/mod.rs:858:27: 858:28 }, Paren, TokenStream(Some([(Token(Token { kind: Ident(ignore, false), span: src/librustc/hir/mod.rs:858:21: 858:27 }), NonJoint)]))), NonJoint)])), is_sugared_doc: false, span: src/librustc/hir/mod.rs:858:5: 858:29 }] }) })`
2019-08-21T18:49:19.7180870Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-21T18:49:19.7187122Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-21T18:49:19.9183401Z error: aborting due to 19 previous errors
2019-08-21T18:49:19.9187230Z 
2019-08-21T18:49:19.9187230Z 
2019-08-21T18:49:19.9325953Z 
2019-08-21T18:49:19.9332426Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-21T18:49:19.9335785Z 
2019-08-21T18:49:19.9341942Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-21T18:49:19.9350933Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-21T18:49:19.9354556Z 
2019-08-21T18:49:19.9354556Z 
2019-08-21T18:49:19.9360390Z note: compiler flags: -Z binary-dep-depinfo -Z unstable-options -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-21T18:49:19.9363755Z 
2019-08-21T18:49:19.9395588Z note: some of the compiler flags provided by cargo are hidden
2019-08-21T18:49:19.9514064Z error: Could not compile `rustc`.
2019-08-21T18:49:19.9531390Z warning: build failed, waiting for other jobs to finish...
2019-08-21T18:50:34.0019276Z error: build failed
2019-08-21T18:50:34.0038325Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-21T18:50:34.0038325Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-21T18:50:34.0038709Z expected success, got: exit code: 101
2019-08-21T18:50:34.0046246Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-21T18:50:34.0046496Z Build completed unsuccessfully in 0:30:36
2019-08-21T18:50:34.0100541Z == clock drift check ==
2019-08-21T18:50:34.0113365Z   local time: Wed Aug 21 18:50:34 UTC 2019
2019-08-21T18:50:34.1626183Z   network time: Wed, 21 Aug 2019 18:50:34 GMT
2019-08-21T18:50:34.1628982Z == end clock drift check ==
2019-08-21T18:50:35.1967936Z ##[error]Bash exited with code '1'.
2019-08-21T18:50:35.2009847Z ##[section]Starting: Checkout
2019-08-21T18:50:35.2011579Z ==============================================================================
2019-08-21T18:50:35.2011633Z Task         : Get sources
2019-08-21T18:50:35.2011680Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
