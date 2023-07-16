plain
2019-09-15T20:54:13.7676224Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T20:54:13.7961912Z ##[command]git config gc.auto 0
2019-09-15T20:54:13.8079351Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T20:54:13.8140019Z ##[command]git config --get-all http.proxy
2019-09-15T20:54:13.8287254Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64429/merge:refs/remotes/pull/64429/merge
---
2019-09-15T21:02:26.8744965Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-15T21:02:27.4454367Z error[E0308]: mismatched types
2019-09-15T21:02:27.4454688Z     --> src/librustc_errors/emitter.rs:1150:44
2019-09-15T21:02:27.4454861Z      |
2019-09-15T21:02:27.4455085Z 1150 |             if Some(Level::FailureNote) != level {
2019-09-15T21:02:27.4455439Z      |                                            ^^^^^ expected enum `std::option::Option`, found &Level
2019-09-15T21:02:27.4455881Z      = note: expected type `std::option::Option<Level>`
2019-09-15T21:02:27.4455881Z      = note: expected type `std::option::Option<Level>`
2019-09-15T21:02:27.4456064Z                 found type `&Level`
2019-09-15T21:02:27.4918871Z error[E0308]: mismatched types
2019-09-15T21:02:27.4919302Z     --> src/librustc_errors/emitter.rs:1161:44
2019-09-15T21:02:27.4920137Z      |
2019-09-15T21:02:27.4920137Z      |
2019-09-15T21:02:27.4920466Z 1161 |             if Some(Level::FailureNote) != level {
2019-09-15T21:02:27.4921146Z      |                                            ^^^^^ expected enum `std::option::Option`, found &Level
2019-09-15T21:02:27.4921702Z      = note: expected type `std::option::Option<Level>`
2019-09-15T21:02:27.4921702Z      = note: expected type `std::option::Option<Level>`
2019-09-15T21:02:27.4921941Z                 found type `&Level`
2019-09-15T21:02:27.6423574Z error: aborting due to 2 previous errors
2019-09-15T21:02:27.6423650Z 
2019-09-15T21:02:27.6434799Z For more information about this error, try `rustc --explain E0308`.
2019-09-15T21:02:27.6566643Z error: Could not compile `rustc_errors`.
---
2019-09-15T21:02:29.7768442Z == clock drift check ==
2019-09-15T21:02:29.7787781Z   local time: Sun Sep 15 21:02:29 UTC 2019
2019-09-15T21:02:29.9281708Z   network time: Sun, 15 Sep 2019 21:02:29 GMT
2019-09-15T21:02:29.9286836Z == end clock drift check ==
2019-09-15T21:02:31.2105322Z ##[error]Bash exited with code '1'.
2019-09-15T21:02:31.2136730Z ##[section]Starting: Checkout
2019-09-15T21:02:31.2138309Z ==============================================================================
2019-09-15T21:02:31.2138353Z Task         : Get sources
2019-09-15T21:02:31.2138392Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
