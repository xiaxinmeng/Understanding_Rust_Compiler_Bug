plain
2019-09-16T03:28:43.7486755Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T03:28:43.7676399Z ##[command]git config gc.auto 0
2019-09-16T03:28:43.7750480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T03:28:43.7815349Z ##[command]git config --get-all http.proxy
2019-09-16T03:28:43.7973399Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64429/merge:refs/remotes/pull/64429/merge
---
2019-09-16T03:37:06.7932201Z     Checking arena v0.0.0 (/checkout/src/libarena)
2019-09-16T03:37:10.6822574Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-09-16T03:37:19.7346159Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-16T03:37:21.2524896Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-16T03:37:21.4023451Z error[E0425]: cannot find value `level_str` in this scope
2019-09-16T03:37:21.4024291Z      |
2019-09-16T03:37:21.4024291Z      |
2019-09-16T03:37:21.4024670Z 1170 |                 if !level_str.is_empty() {
2019-09-16T03:37:21.4027121Z 
2019-09-16T03:37:21.9948941Z error: aborting due to previous error
2019-09-16T03:37:21.9949177Z 
2019-09-16T03:37:21.9958935Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-16T03:37:24.6079454Z == clock drift check ==
2019-09-16T03:37:24.6096974Z   local time: Mon Sep 16 03:37:24 UTC 2019
2019-09-16T03:37:25.0363880Z   network time: Mon, 16 Sep 2019 03:37:25 GMT
2019-09-16T03:37:25.0363966Z == end clock drift check ==
2019-09-16T03:37:26.4919263Z ##[error]Bash exited with code '1'.
2019-09-16T03:37:26.4956024Z ##[section]Starting: Checkout
2019-09-16T03:37:26.4958258Z ==============================================================================
2019-09-16T03:37:26.4958309Z Task         : Get sources
2019-09-16T03:37:26.4958352Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
