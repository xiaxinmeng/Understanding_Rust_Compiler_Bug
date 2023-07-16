plain
2019-12-16T00:49:16.5245490Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T00:49:16.5469186Z ##[command]git config gc.auto 0
2019-12-16T00:49:16.5551772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T00:49:16.5607624Z ##[command]git config --get-all http.proxy
2019-12-16T00:49:17.2500103Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67339/merge:refs/remotes/pull/67339/merge
---
2019-12-16T00:54:29.0348386Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2019-12-16T00:54:30.3173351Z error[E0609]: no field `value` on type `sync::ArcInner<T>`
2019-12-16T00:54:30.3174675Z    --> src/liballoc/sync.rs:559:46
2019-12-16T00:54:30.3175400Z     |
2019-12-16T00:54:30.3176594Z 559 |             let offset = data_offset(&(*ptr).value);
2019-12-16T00:54:30.3177882Z     |
2019-12-16T00:54:30.3177882Z     |
2019-12-16T00:54:30.3178614Z     = note: available fields are: `strong`, `weak`, `data`
2019-12-16T00:54:30.5128469Z     Checking cfg-if v0.1.8
2019-12-16T00:54:30.5529913Z     Checking rustc-demangle v0.1.16
2019-12-16T00:54:30.6515329Z error: aborting due to previous error
2019-12-16T00:54:30.6515415Z 
---
2019-12-16T00:54:30.9121872Z   local time: Mon Dec 16 00:54:30 UTC 2019
2019-12-16T00:54:30.9942019Z   network time: Mon, 16 Dec 2019 00:54:30 GMT
2019-12-16T00:54:30.9947538Z == end clock drift check ==
2019-12-16T00:54:44.5499498Z 
2019-12-16T00:54:44.5601405Z ##[error]Bash exited with code '1'.
2019-12-16T00:54:44.5631431Z ##[section]Starting: Checkout
2019-12-16T00:54:44.5633177Z ==============================================================================
2019-12-16T00:54:44.5633234Z Task         : Get sources
2019-12-16T00:54:44.5633283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
