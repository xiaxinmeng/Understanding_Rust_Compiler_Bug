plain
2019-10-24T22:58:38.8514240Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T22:58:39.6581827Z ##[command]git config gc.auto 0
2019-10-24T22:58:39.6584983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T22:58:39.6587156Z ##[command]git config --get-all http.proxy
2019-10-24T22:58:39.6591266Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65640/merge:refs/remotes/pull/65640/merge
---
2019-10-24T23:08:29.3684102Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-24T23:08:38.5815275Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-24T23:08:40.1039758Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-24T23:08:59.2144102Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-24T23:09:01.3294236Z error: expected `{`, found `=>`
2019-10-24T23:09:01.3302287Z     |
2019-10-24T23:09:01.3302287Z     |
2019-10-24T23:09:01.3303148Z 330 |         if self.prev_span == syntax_pos::DUMMY_SP => {
2019-10-24T23:09:01.3303957Z     |         --                                        ^^ expected `{`
2019-10-24T23:09:01.3305028Z     |         this `if` statement has a condition, but no block
2019-10-24T23:09:01.3305297Z 
2019-10-24T23:09:07.7064250Z error: aborting due to previous error
2019-10-24T23:09:07.7069522Z 
---
2019-10-24T23:09:11.4356399Z   local time: Thu Oct 24 23:09:11 UTC 2019
2019-10-24T23:09:11.5863308Z   network time: Thu, 24 Oct 2019 23:09:11 GMT
2019-10-24T23:09:11.5864839Z == end clock drift check ==
2019-10-24T23:09:12.6453898Z 
2019-10-24T23:09:12.6573142Z ##[error]Bash exited with code '1'.
2019-10-24T23:09:12.6625688Z ##[section]Starting: Checkout
2019-10-24T23:09:12.6627837Z ==============================================================================
2019-10-24T23:09:12.6627913Z Task         : Get sources
2019-10-24T23:09:12.6627961Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
