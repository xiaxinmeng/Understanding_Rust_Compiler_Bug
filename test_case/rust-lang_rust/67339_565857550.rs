plain
2019-12-15T23:10:24.0591175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-15T23:10:25.0050080Z ##[command]git config gc.auto 0
2019-12-15T23:10:25.0056178Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-15T23:10:25.0062195Z ##[command]git config --get-all http.proxy
2019-12-15T23:10:25.0064902Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67339/merge:refs/remotes/pull/67339/merge
---
2019-12-15T23:15:47.5374405Z     Checking cfg-if v0.1.8
2019-12-15T23:15:47.5837834Z     Checking rustc-demangle v0.1.16
2019-12-15T23:15:47.9156291Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-12-15T23:15:48.0998866Z     Checking backtrace v0.3.40
2019-12-15T23:15:48.3505725Z error[E0609]: no field `value` on type `*const sync::ArcInner<T>`
2019-12-15T23:15:48.3506402Z     |
2019-12-15T23:15:48.3506402Z     |
2019-12-15T23:15:48.3506802Z 558 |             let offset = data_offset(&*ptr.value);
2019-12-15T23:15:48.3507586Z     |                                        |
2019-12-15T23:15:48.3507586Z     |                                        |
2019-12-15T23:15:48.3507992Z     |                                        help: `ptr` is a raw pointer; try dereferencing it: `(*ptr).value`
2019-12-15T23:15:48.3508057Z 
2019-12-15T23:15:49.1010552Z error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const T`
2019-12-15T23:15:49.1019141Z     |
2019-12-15T23:15:49.1019141Z     |
2019-12-15T23:15:49.1019768Z 559 |             (ptr as *mut u8).offset(offset) as *const u8 as *const T
2019-12-15T23:15:49.1020711Z 
2019-12-15T23:15:49.1020711Z 
2019-12-15T23:15:49.1021225Z error[E0609]: no field `value` on type `*const rc::RcBox<T>`
2019-12-15T23:15:49.1022037Z    --> src/liballoc/rc.rs:578:44
2019-12-15T23:15:49.1022548Z     |
2019-12-15T23:15:49.1023198Z 578 |             let offset = data_offset(&*ptr.value);
2019-12-15T23:15:49.1024541Z     |                                        |
2019-12-15T23:15:49.1024541Z     |                                        |
2019-12-15T23:15:49.1025120Z     |                                        help: `ptr` is a raw pointer; try dereferencing it: `(*ptr).value`
2019-12-15T23:15:49.1025680Z 
2019-12-15T23:15:49.1026205Z error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const T`
2019-12-15T23:15:49.1026686Z    --> src/liballoc/rc.rs:579:13
2019-12-15T23:15:49.1027143Z     |
2019-12-15T23:15:49.1027795Z 579 |             (ptr as *mut u8).offset(offset) as *const u8 as *const T
2019-12-15T23:15:49.1028637Z 
2019-12-15T23:15:49.1029121Z error: aborting due to 4 previous errors
2019-12-15T23:15:49.1029316Z 
2019-12-15T23:15:49.1030056Z Some errors have detailed explanations: E0607, E0609.
---
2019-12-15T23:15:49.1033783Z   local time: Sun Dec 15 23:15:48 UTC 2019
2019-12-15T23:15:49.1033938Z   network time: Sun, 15 Dec 2019 23:15:48 GMT
2019-12-15T23:15:49.1034121Z == end clock drift check ==
2019-12-15T23:16:02.5352957Z 
2019-12-15T23:16:02.5481683Z ##[error]Bash exited with code '1'.
2019-12-15T23:16:02.5512712Z ##[section]Starting: Checkout
2019-12-15T23:16:02.5514447Z ==============================================================================
2019-12-15T23:16:02.5514498Z Task         : Get sources
2019-12-15T23:16:02.5514560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
