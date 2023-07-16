plain
2019-10-01T17:26:56.5714304Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T17:26:56.5912777Z ##[command]git config gc.auto 0
2019-10-01T17:26:56.6001729Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T17:26:56.6055279Z ##[command]git config --get-all http.proxy
2019-10-01T17:26:56.6210138Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64906/merge:refs/remotes/pull/64906/merge
---
2019-10-01T17:34:41.6370491Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-01T17:34:43.3813655Z error: unexpected token: `.`
2019-10-01T17:34:43.3814556Z     --> src/libsyntax/parse/parser/item.rs:1347:21
2019-10-01T17:34:43.3814922Z      |
2019-10-01T17:34:43.3815186Z 1347 |                     .struct_span_err(self.token.span, "extern items cannot be `const`");
2019-10-01T17:34:43.3815475Z 
2019-10-01T17:34:43.3921616Z error: aborting due to previous error
2019-10-01T17:34:43.3922270Z 
2019-10-01T17:34:43.4022654Z error: could not compile `syntax`.
---
2019-10-01T17:34:43.4117505Z == clock drift check ==
2019-10-01T17:34:43.4133956Z   local time: Tue Oct  1 17:34:43 UTC 2019
2019-10-01T17:34:43.6918782Z   network time: Tue, 01 Oct 2019 17:34:43 GMT
2019-10-01T17:34:43.6920065Z == end clock drift check ==
2019-10-01T17:34:44.8227757Z ##[error]Bash exited with code '1'.
2019-10-01T17:34:44.8304485Z ##[section]Starting: Checkout
2019-10-01T17:34:44.8306059Z ==============================================================================
2019-10-01T17:34:44.8306125Z Task         : Get sources
2019-10-01T17:34:44.8306162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
