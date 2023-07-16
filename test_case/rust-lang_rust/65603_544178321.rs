plain
2019-10-19T17:28:19.4295857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T17:28:19.4482915Z ##[command]git config gc.auto 0
2019-10-19T17:28:19.4558003Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T17:28:19.4637829Z ##[command]git config --get-all http.proxy
2019-10-19T17:28:19.4779538Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65603/merge:refs/remotes/pull/65603/merge
---
2019-10-19T17:36:01.5471795Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-19T17:36:01.7972598Z error[E0425]: cannot find value `cx` in this scope
2019-10-19T17:36:01.7973921Z     --> src/libsyntax_expand/expand.rs:1421:42
2019-10-19T17:36:01.7974316Z      |
2019-10-19T17:36:01.7975061Z 1421 |                     let filename = match cx.resolve_path(file, sp) {
2019-10-19T17:36:01.7975670Z      |                                          ^^ help: you might have meant to use the available field: `self.cx`
2019-10-19T17:36:01.7984107Z error[E0425]: cannot find value `sp` in this scope
2019-10-19T17:36:01.7985247Z     --> src/libsyntax_expand/expand.rs:1421:64
2019-10-19T17:36:01.7985867Z      |
2019-10-19T17:36:01.7985867Z      |
2019-10-19T17:36:01.7986480Z 1421 |                     let filename = match cx.resolve_path(file, sp) {
2019-10-19T17:36:01.7990566Z 
2019-10-19T17:36:02.6762754Z error: aborting due to 2 previous errors
2019-10-19T17:36:02.6762859Z 
2019-10-19T17:36:02.6763200Z For more information about this error, try `rustc --explain E0425`.
---
2019-10-19T17:36:02.7110771Z   local time: Sat Oct 19 17:36:02 UTC 2019
2019-10-19T17:36:02.8608549Z   network time: Sat, 19 Oct 2019 17:36:02 GMT
2019-10-19T17:36:02.8608890Z == end clock drift check ==
2019-10-19T17:36:04.1394511Z 
2019-10-19T17:36:04.1499185Z ##[error]Bash exited with code '1'.
2019-10-19T17:36:04.1531087Z ##[section]Starting: Checkout
2019-10-19T17:36:04.1532702Z ==============================================================================
2019-10-19T17:36:04.1532750Z Task         : Get sources
2019-10-19T17:36:04.1532810Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
