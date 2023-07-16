plain
2019-11-08T23:22:22.7799585Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T23:22:22.7998520Z ##[command]git config gc.auto 0
2019-11-08T23:22:22.8080262Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T23:22:22.8155466Z ##[command]git config --get-all http.proxy
2019-11-08T23:22:22.8358653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66231/merge:refs/remotes/pull/66231/merge
---
2019-11-08T23:49:38.5212920Z    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin/deprecated)
2019-11-08T23:49:39.5840989Z error: lifetime may not live long enough
2019-11-08T23:49:39.5841421Z    --> src/librustc_interface/util.rs:791:63
2019-11-08T23:49:39.5841688Z     |
2019-11-08T23:49:39.5842046Z 724 | impl<'a, 'b> ReplaceBodyWithLoop<'a, 'b> {
2019-11-08T23:49:39.5842825Z     |      |
2019-11-08T23:49:39.5843182Z     |      lifetime `'a` defined here
2019-11-08T23:49:39.5843427Z ...
2019-11-08T23:49:39.5843427Z ...
2019-11-08T23:49:39.5843826Z 791 |         sig.header.constness.node == ast::Constness::Const || Self::should_ignore_fn(&sig.decl)
2019-11-08T23:49:39.5844649Z     |                                                               ^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
2019-11-08T23:49:39.6136299Z error: aborting due to previous error
2019-11-08T23:49:39.6142135Z 
2019-11-08T23:49:39.6371716Z error: could not compile `rustc_interface`.
2019-11-08T23:49:39.6387322Z warning: build failed, waiting for other jobs to finish...
---
2019-11-08T23:49:56.8487755Z   local time: Fri Nov  8 23:49:56 UTC 2019
2019-11-08T23:49:57.1277906Z   network time: Fri, 08 Nov 2019 23:49:57 GMT
2019-11-08T23:49:57.1280897Z == end clock drift check ==
2019-11-08T23:49:58.2073453Z 
2019-11-08T23:49:58.2197858Z ##[error]Bash exited with code '1'.
2019-11-08T23:49:58.2237651Z ##[section]Starting: Checkout
2019-11-08T23:49:58.2240035Z ==============================================================================
2019-11-08T23:49:58.2240101Z Task         : Get sources
2019-11-08T23:49:58.2240175Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
