plain
2019-12-06T12:51:00.3621213Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-06T12:51:01.0994672Z ##[command]git config gc.auto 0
2019-12-06T12:51:01.1000074Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-06T12:51:01.1004616Z ##[command]git config --get-all http.proxy
2019-12-06T12:51:01.1009553Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67086/merge:refs/remotes/pull/67086/merge
---
2019-12-06T12:58:28.8784743Z 
2019-12-06T12:58:28.8859613Z error[E0432]: unresolved import `errors`
2019-12-06T12:58:28.8860397Z   --> src/librustc_parse/parser/ty.rs:17:5
2019-12-06T12:58:28.8861038Z    |
2019-12-06T12:58:28.8861770Z 17 | use errors::{PResult, Applicability, pluralize};
2019-12-06T12:58:28.8862514Z    |     ^^^^^^ help: a similar path exists: `syntax::errors`
2019-12-06T12:58:30.1894849Z error: aborting due to 4 previous errors
2019-12-06T12:58:30.1895968Z 
2019-12-06T12:58:30.1903804Z For more information about this error, try `rustc --explain E0432`.
2019-12-06T12:58:30.2103051Z error: could not compile `rustc_parse`.
---
2019-12-06T12:59:12.4196356Z   local time: Fri Dec  6 12:59:12 UTC 2019
2019-12-06T12:59:12.5756832Z   network time: Fri, 06 Dec 2019 12:59:12 GMT
2019-12-06T12:59:12.5758443Z == end clock drift check ==
2019-12-06T12:59:13.6163453Z 
2019-12-06T12:59:13.6271267Z ##[error]Bash exited with code '1'.
2019-12-06T12:59:13.6301293Z ##[section]Starting: Checkout
2019-12-06T12:59:13.6302968Z ==============================================================================
2019-12-06T12:59:13.6303039Z Task         : Get sources
2019-12-06T12:59:13.6303086Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
