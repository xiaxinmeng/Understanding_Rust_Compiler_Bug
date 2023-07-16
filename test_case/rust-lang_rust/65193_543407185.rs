plain
2019-10-17T23:18:30.4790173Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-17T23:18:30.4961758Z ##[command]git config gc.auto 0
2019-10-17T23:18:30.5040044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-17T23:18:30.5100476Z ##[command]git config --get-all http.proxy
2019-10-17T23:18:30.5238372Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-17T23:26:31.4670258Z 26 |     report_in_external_macro: true
2019-10-17T23:26:31.4670641Z    |                               ^^^^ expected identifier, found keyword
2019-10-17T23:26:31.4671340Z help: you can escape reserved keywords to use them as identifiers
2019-10-17T23:26:31.4671573Z    |
2019-10-17T23:26:31.4671851Z 26 |     report_in_external_macro: r#true
2019-10-17T23:26:31.4672193Z 
2019-10-17T23:26:31.4672193Z 
2019-10-17T23:26:31.4705179Z error: expected type, found keyword `true`
2019-10-17T23:26:31.4705496Z   --> src/librustc/lint/builtin.rs:26:31
2019-10-17T23:26:31.4706053Z 26 |     report_in_external_macro: true
2019-10-17T23:26:31.4706417Z    |                             - ^^^^ expected type
2019-10-17T23:26:31.4706714Z    |                             |
2019-10-17T23:26:31.4707035Z    |                             tried to parse a type due to this type ascription
2019-10-17T23:26:31.4707035Z    |                             tried to parse a type due to this type ascription
2019-10-17T23:26:31.4707271Z    |
2019-10-17T23:26:31.4707601Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2019-10-17T23:26:31.4708034Z    = note: for more information, see ***/issues/23416
2019-10-17T23:26:31.5256500Z error: aborting due to 2 previous errors
2019-10-17T23:26:31.5256924Z 
2019-10-17T23:26:31.5316363Z error: could not compile `rustc`.
2019-10-17T23:26:31.5316729Z warning: build failed, waiting for other jobs to finish...
---
2019-10-17T23:26:32.2585105Z == clock drift check ==
2019-10-17T23:26:32.2605259Z   local time: Thu Oct 17 23:26:32 UTC 2019
2019-10-17T23:26:32.5523292Z   network time: Thu, 17 Oct 2019 23:26:32 GMT
2019-10-17T23:26:32.5523513Z == end clock drift check ==
2019-10-17T23:26:34.0235923Z ##[error]Bash exited with code '1'.
2019-10-17T23:26:34.0271536Z ##[section]Starting: Checkout
2019-10-17T23:26:34.0273766Z ==============================================================================
2019-10-17T23:26:34.0273838Z Task         : Get sources
2019-10-17T23:26:34.0273885Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
