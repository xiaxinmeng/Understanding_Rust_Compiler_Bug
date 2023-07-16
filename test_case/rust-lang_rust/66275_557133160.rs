plain
2019-11-21T14:54:46.1203103Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T14:54:46.1271599Z ##[command]git config gc.auto 0
2019-11-21T14:54:46.1277452Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T14:54:46.1282393Z ##[command]git config --get-all http.proxy
2019-11-21T14:54:46.1289765Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66275/merge:refs/remotes/pull/66275/merge
---
2019-11-21T15:22:50.5213527Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-11-21T15:22:50.5253061Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-21T15:22:50.9154190Z    Compiling cc v1.0.47
2019-11-21T15:22:50.9164636Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-11-21T15:22:55.8562821Z error: internal compiler error: src/librustc_typeck/collect.rs:1868: unexpected sort of node in fn_sig(): AnonConst(AnonConst { hir_id: HirId { owner: DefIndex(27348), local_id: 2 }, body: BodyId { hir_id: HirId { owner: DefIndex(27348), local_id: 4 } } })
2019-11-21T15:22:55.8572056Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
2019-11-21T15:22:55.8578024Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-21T15:22:55.8580927Z 
2019-11-21T15:22:55.8585690Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-21T15:22:55.8585690Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-21T15:22:55.8590832Z 
2019-11-21T15:22:55.8596269Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-21T15:22:55.8604956Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-11-21T15:22:55.8605309Z 
2019-11-21T15:22:55.8605309Z 
2019-11-21T15:22:55.8606138Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-11-21T15:22:55.8606699Z note: some of the compiler flags provided by cargo are hidden
2019-11-21T15:22:55.8606889Z 
2019-11-21T15:22:55.9671551Z error: aborting due to previous error
2019-11-21T15:22:55.9676096Z 
---
2019-11-21T15:22:56.9638478Z   local time: Thu Nov 21 15:22:56 UTC 2019
2019-11-21T15:22:57.1018038Z   network time: Thu, 21 Nov 2019 15:22:57 GMT
2019-11-21T15:22:57.1018158Z == end clock drift check ==
2019-11-21T15:22:57.9043916Z 
2019-11-21T15:22:57.9147505Z ##[error]Bash exited with code '1'.
2019-11-21T15:22:57.9177055Z ##[section]Starting: Checkout
2019-11-21T15:22:57.9178891Z ==============================================================================
2019-11-21T15:22:57.9178961Z Task         : Get sources
2019-11-21T15:22:57.9179005Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
