plain
2019-12-16T00:33:41.8567945Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T00:33:41.8777282Z ##[command]git config gc.auto 0
2019-12-16T00:33:41.8837186Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T00:33:41.8888840Z ##[command]git config --get-all http.proxy
2019-12-16T00:33:41.9056035Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67339/merge:refs/remotes/pull/67339/merge
---
2019-12-16T00:38:42.6562916Z     Checking backtrace v0.3.40
2019-12-16T00:38:42.9475891Z error[E0609]: no field `value` on type `sync::ArcInner<T>`
2019-12-16T00:38:42.9476304Z    --> src/liballoc/sync.rs:559:46
2019-12-16T00:38:42.9476685Z     |
2019-12-16T00:38:42.9476958Z 559 |             let offset = data_offset(&(*ptr).value);
2019-12-16T00:38:42.9478064Z     |
2019-12-16T00:38:42.9478064Z     |
2019-12-16T00:38:42.9478408Z     = note: available fields are: `strong`, `weak`, `data`
2019-12-16T00:38:42.9614643Z error[E0308]: mismatched types
2019-12-16T00:38:42.9615105Z    --> src/liballoc/sync.rs:560:26
2019-12-16T00:38:42.9615338Z     |
2019-12-16T00:38:42.9615338Z     |
2019-12-16T00:38:42.9615623Z 560 |             set_data_ptr(fake_ptr, (ptr as *mut u8).offset(offset))
2019-12-16T00:38:42.9616082Z     |                          ^^^^^^^^ types differ in mutability
2019-12-16T00:38:42.9616704Z     = note: expected type `*mut _`
2019-12-16T00:38:42.9616974Z                found type `*const T`
2019-12-16T00:38:42.9617120Z 
2019-12-16T00:38:43.9716596Z error[E0308]: mismatched types
2019-12-16T00:38:43.9716596Z error[E0308]: mismatched types
2019-12-16T00:38:43.9719205Z    --> src/liballoc/rc.rs:580:26
2019-12-16T00:38:43.9719794Z     |
2019-12-16T00:38:43.9720449Z 580 |             set_data_ptr(fake_ptr, (ptr as *mut u8).offset(offset))
2019-12-16T00:38:43.9721330Z     |                          ^^^^^^^^ types differ in mutability
2019-12-16T00:38:43.9722793Z     = note: expected type `*mut _`
2019-12-16T00:38:43.9723226Z                found type `*const T`
2019-12-16T00:38:43.9723482Z 
2019-12-16T00:38:43.9723865Z error: aborting due to 3 previous errors
---
2019-12-16T00:38:43.9727868Z   local time: Mon Dec 16 00:38:43 UTC 2019
2019-12-16T00:38:43.9728085Z   network time: Mon, 16 Dec 2019 00:38:43 GMT
2019-12-16T00:38:43.9728262Z == end clock drift check ==
2019-12-16T00:38:53.2224156Z 
2019-12-16T00:38:53.2326154Z ##[error]Bash exited with code '1'.
2019-12-16T00:38:53.2356258Z ##[section]Starting: Checkout
2019-12-16T00:38:53.2358483Z ==============================================================================
2019-12-16T00:38:53.2358593Z Task         : Get sources
2019-12-16T00:38:53.2358641Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
