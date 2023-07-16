plain
2019-12-02T18:56:00.4739634Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T18:56:00.4754793Z ##[command]git config gc.auto 0
2019-12-02T18:56:00.4757183Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T18:56:00.4759924Z ##[command]git config --get-all http.proxy
2019-12-02T18:56:00.4763065Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66925/merge:refs/remotes/pull/66925/merge
---
2019-12-02T20:27:38.2707006Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/rustc-bug-fix-procedure.html"
2019-12-02T20:27:38.2707833Z 
2019-12-02T20:27:38.2708285Z     ┌── compiler-team.md:40:3 ───
2019-12-02T20:27:38.2708626Z     │
2019-12-02T20:27:38.2709001Z  40 │   [aim to give warnings first][procedure]).
2019-12-02T20:27:38.2709658Z     │
2019-12-02T20:27:38.2709875Z 
2019-12-02T20:27:38.2710257Z error: The server responded with 404 Not Found for "https://forge.rust-lang.org/platform-support.html"
2019-12-02T20:27:38.2710416Z 
2019-12-02T20:27:38.2710416Z 
2019-12-02T20:27:38.2710755Z      ┌── tests/intro.md:116:8 ───
2019-12-02T20:27:38.2711259Z      │
2019-12-02T20:27:38.2711858Z  116 │ Rust's [platform tiers]).
2019-12-02T20:27:38.2712567Z      │
2019-12-02T20:27:38.2712723Z 
2019-12-02T20:27:38.2713184Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/middle/expr_use_visitor/struct.ExprUseVisitor.html"
2019-12-02T20:27:38.2713376Z 
---
2019-12-02T20:57:21.4702865Z 
2019-12-02T20:57:21.4703093Z If you do intend to update 'miri', please check the error messages above and
2019-12-02T20:57:21.4703137Z commit another update.
2019-12-02T20:57:21.4703160Z 
2019-12-02T20:57:21.4704132Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-12-02T20:57:21.4708307Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-12-02T20:57:21.4708370Z proper steps.
2019-12-02T20:57:21.4722712Z   local time: Mon Dec  2 20:57:21 UTC 2019
2019-12-02T20:57:21.5225690Z   network time: Mon, 02 Dec 2019 20:57:21 GMT
2019-12-02T20:57:21.5231052Z == end clock drift check ==
2019-12-02T20:57:22.6936004Z 
2019-12-02T20:57:22.6936004Z 
2019-12-02T20:57:22.7042209Z ##[error]Bash exited with code '3'.
2019-12-02T20:57:22.7073099Z ##[section]Starting: Checkout
2019-12-02T20:57:22.7075303Z ==============================================================================
2019-12-02T20:57:22.7075354Z Task         : Get sources
2019-12-02T20:57:22.7075398Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
