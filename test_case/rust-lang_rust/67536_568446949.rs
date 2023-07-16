plain
2019-12-23T11:05:48.5182574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T11:05:48.5387172Z ##[command]git config gc.auto 0
2019-12-23T11:05:48.5451351Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T11:05:48.5516007Z ##[command]git config --get-all http.proxy
2019-12-23T11:05:48.5666921Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67536/merge:refs/remotes/pull/67536/merge
---
2019-12-23T11:13:25.6483140Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-23T11:13:25.8172184Z error[E0433]: failed to resolve: use of undeclared type or module `sym`
2019-12-23T11:13:25.8172742Z     --> src/librustc/hir/mod.rs:1617:58
2019-12-23T11:13:25.8172998Z      |
2019-12-23T11:13:25.8173324Z 1617 |                     let new_call = segment.ident.name == sym::new;
2019-12-23T11:13:25.8173752Z      |                                                          ^^^ use of undeclared type or module `sym`
2019-12-23T11:13:26.6897706Z error[E0412]: cannot find type `SourceMap` in this scope
2019-12-23T11:13:26.6898179Z     --> src/librustc/hir/mod.rs:1572:30
2019-12-23T11:13:26.6898414Z      |
2019-12-23T11:13:26.6898414Z      |
2019-12-23T11:13:26.6898755Z 1572 | pub fn is_range_literal(sm: &SourceMap, expr: &Expr) -> bool {
2019-12-23T11:13:26.6899342Z      |
2019-12-23T11:13:26.6899662Z help: possible candidates are found in other modules, you can import them into scope
2019-12-23T11:13:26.6899884Z      |
2019-12-23T11:13:26.6900164Z 5    | use syntax::source_map::SourceMap;
2019-12-23T11:13:26.6900164Z 5    | use syntax::source_map::SourceMap;
2019-12-23T11:13:26.6900432Z      |
2019-12-23T11:13:26.6900719Z 5    | use syntax_pos::source_map::SourceMap;
2019-12-23T11:13:26.6900938Z      |
2019-12-23T11:13:26.6905618Z 
2019-12-23T11:13:26.7012730Z error[E0412]: cannot find type `SourceMap` in this scope
2019-12-23T11:13:26.7013137Z     --> src/librustc/hir/mod.rs:1590:20
2019-12-23T11:13:26.7013417Z      |
2019-12-23T11:13:26.7013720Z 1590 |     fn is_lit(sm: &SourceMap, span: &Span) -> bool {
2019-12-23T11:13:26.7014307Z      |
2019-12-23T11:13:26.7015028Z help: possible candidates are found in other modules, you can import them into scope
2019-12-23T11:13:26.7015541Z      |
2019-12-23T11:13:26.7015815Z 5    | use syntax::source_map::SourceMap;
---
2019-12-23T11:13:52.5683018Z For more information about an error, try `rustc --explain E0412`.
2019-12-23T11:13:52.6052955Z error: could not compile `rustc`.
2019-12-23T11:13:52.6053036Z 
2019-12-23T11:13:52.6053378Z To learn more, run the command again with --verbose.
2019-12-23T11:13:52.6094871Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T11:13:52.6095069Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T11:13:52.6095126Z Build completed unsuccessfully in 0:04:42
2019-12-23T11:13:52.6143615Z == clock drift check ==
2019-12-23T11:13:52.6168317Z   local time: Mon Dec 23 11:13:52 UTC 2019
2019-12-23T11:13:52.6168317Z   local time: Mon Dec 23 11:13:52 UTC 2019
2019-12-23T11:13:52.7674350Z   network time: Mon, 23 Dec 2019 11:13:52 GMT
2019-12-23T11:13:52.7676445Z == end clock drift check ==
2019-12-23T11:13:53.6045445Z 
2019-12-23T11:13:53.6139755Z ##[error]Bash exited with code '1'.
2019-12-23T11:13:53.6170702Z ##[section]Starting: Checkout
2019-12-23T11:13:53.6172853Z ==============================================================================
2019-12-23T11:13:53.6173009Z Task         : Get sources
2019-12-23T11:13:53.6173077Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
