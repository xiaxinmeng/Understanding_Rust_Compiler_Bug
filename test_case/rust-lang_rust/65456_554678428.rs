plain
2019-11-16T21:52:28.1781178Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T21:52:28.1964743Z ##[command]git config gc.auto 0
2019-11-16T21:52:28.2042841Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T21:52:28.2119558Z ##[command]git config --get-all http.proxy
2019-11-16T21:52:28.2254907Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65456/merge:refs/remotes/pull/65456/merge
---
2019-11-16T22:01:48.3787239Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-16T22:02:04.1867197Z error[E0432]: unresolved import `errors::Style`
2019-11-16T22:02:04.1867539Z   --> src/librustc/traits/error_reporting.rs:36:59
2019-11-16T22:02:04.1867747Z    |
2019-11-16T22:02:04.1868056Z 36 | use errors::{Applicability, DiagnosticBuilder, pluralize, Style};
2019-11-16T22:02:04.1868393Z    |                                                           ^^^^^ no `Style` in the root
2019-11-16T22:02:23.8014121Z error: aborting due to previous error
2019-11-16T22:02:23.8014502Z 
2019-11-16T22:02:23.8023888Z For more information about this error, try `rustc --explain E0432`.
2019-11-16T22:02:23.9620324Z error: could not compile `rustc`.
---
2019-11-16T22:02:25.8845991Z   local time: Sat Nov 16 22:02:25 UTC 2019
2019-11-16T22:02:26.4186421Z   network time: Sat, 16 Nov 2019 22:02:26 GMT
2019-11-16T22:02:26.4190120Z == end clock drift check ==
2019-11-16T22:02:27.2379217Z 
2019-11-16T22:02:27.2447293Z ##[error]Bash exited with code '1'.
2019-11-16T22:02:27.2468984Z ##[section]Starting: Checkout
2019-11-16T22:02:27.2470467Z ==============================================================================
2019-11-16T22:02:27.2470534Z Task         : Get sources
2019-11-16T22:02:27.2470573Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
