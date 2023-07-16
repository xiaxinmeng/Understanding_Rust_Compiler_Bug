plain
2019-09-15T21:57:07.4358346Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T21:57:07.4576298Z ##[command]git config gc.auto 0
2019-09-15T21:57:07.4652382Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T21:57:07.4721995Z ##[command]git config --get-all http.proxy
2019-09-15T21:57:08.0727824Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64136/merge:refs/remotes/pull/64136/merge
---
2019-09-15T22:05:25.5043785Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-09-15T22:05:34.5770600Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-15T22:05:36.0612747Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-15T22:05:38.2195280Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-15T22:05:39.3539413Z error: unknown start of token: \u{a0}
2019-09-15T22:05:39.3540220Z    |
2019-09-15T22:05:39.3540220Z    |
2019-09-15T22:05:39.3540561Z 70 |     /// and `None` into `LhsExpr::NotYetParsed`.
2019-09-15T22:05:39.3540848Z    | ^
2019-09-15T22:05:39.3541211Z help: Unicode character ' ' (No-Break Space) looks like ' ' (Space), but it is not
2019-09-15T22:05:39.3541462Z    |
2019-09-15T22:05:39.3541817Z 70 |     /// and `None` into `LhsExpr::NotYetParsed`.
2019-09-15T22:05:39.3542181Z 
2019-09-15T22:05:45.1635390Z error: aborting due to previous error
2019-09-15T22:05:45.1636573Z 
2019-09-15T22:05:45.2143906Z error: Could not compile `syntax`.
---
2019-09-15T22:05:45.2238304Z == clock drift check ==
2019-09-15T22:05:45.2273095Z   local time: Sun Sep 15 22:05:45 UTC 2019
2019-09-15T22:05:45.5108326Z   network time: Sun, 15 Sep 2019 22:05:45 GMT
2019-09-15T22:05:45.5108496Z == end clock drift check ==
2019-09-15T22:05:46.6310476Z ##[error]Bash exited with code '1'.
2019-09-15T22:05:46.6345044Z ##[section]Starting: Checkout
2019-09-15T22:05:46.6347295Z ==============================================================================
2019-09-15T22:05:46.6347360Z Task         : Get sources
2019-09-15T22:05:46.6347425Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
