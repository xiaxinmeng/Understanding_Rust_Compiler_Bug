plain
2020-01-07T15:54:02.4589818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T15:54:02.4675058Z ##[command]git config gc.auto 0
2020-01-07T15:54:02.4759986Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T15:54:02.4829368Z ##[command]git config --get-all http.proxy
2020-01-07T15:54:02.4970119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67502/merge:refs/remotes/pull/67502/merge
---
2020-01-07T15:58:40.4496895Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-01-07T15:58:40.7700380Z     Checking cfg-if v0.1.8
2020-01-07T15:58:40.8156581Z     Checking rustc-demangle v0.1.16
2020-01-07T15:58:41.2595695Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-07T15:58:41.2967617Z error: couldn't read src/libpanic_abort/../libpanic_unwind/payload.rs: No such file or directory (os error 2)
2020-01-07T15:58:41.2967972Z   --> src/libpanic_abort/lib.rs:24:1
2020-01-07T15:58:41.2968240Z    |
2020-01-07T15:58:41.2968501Z 24 | include!("../libpanic_unwind/payload.rs");
2020-01-07T15:58:41.2969002Z    | 
2020-01-07T15:58:41.2969233Z   ::: <::core::macros::builtin::include macros>:1:1
2020-01-07T15:58:41.2969418Z    |
2020-01-07T15:58:41.2969418Z    |
2020-01-07T15:58:41.2969716Z 1  | ($ file : expr) => { { } } ; ($ file : expr,) => { { } } ;
2020-01-07T15:58:41.2970035Z    | ---------------------------------------------------------- in this expansion of `include!`
2020-01-07T15:58:41.2990782Z error: aborting due to previous error
2020-01-07T15:58:41.2996142Z 
2020-01-07T15:58:41.3048740Z error: could not compile `panic_abort`.
2020-01-07T15:58:41.3049040Z warning: build failed, waiting for other jobs to finish...
---
2020-01-07T15:58:42.8482419Z   local time: Tue Jan  7 15:58:42 UTC 2020
2020-01-07T15:58:43.1308741Z   network time: Tue, 07 Jan 2020 15:58:43 GMT
2020-01-07T15:58:43.1314377Z == end clock drift check ==
2020-01-07T15:58:44.4231747Z 
2020-01-07T15:58:44.4340624Z ##[error]Bash exited with code '1'.
2020-01-07T15:58:44.4370787Z ##[section]Starting: Checkout
2020-01-07T15:58:44.4373469Z ==============================================================================
2020-01-07T15:58:44.4373544Z Task         : Get sources
2020-01-07T15:58:44.4373593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
