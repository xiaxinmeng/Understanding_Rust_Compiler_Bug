plain
2019-12-08T18:23:08.2448797Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T18:23:08.2660726Z ##[command]git config gc.auto 0
2019-12-08T18:23:09.0829201Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T18:23:09.0839762Z ##[command]git config --get-all http.proxy
2019-12-08T18:23:09.0843255Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67160/merge:refs/remotes/pull/67160/merge
---
2019-12-08T18:32:43.3913261Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-12-08T18:32:47.2659652Z error: lifetime may not live long enough
2019-12-08T18:32:47.2660808Z     --> src/librustdoc/clean/mod.rs:1461:28
2019-12-08T18:32:47.2661686Z      |
2019-12-08T18:32:47.2662337Z 1459 | impl<'tcx> Clean<Type> for ty::ProjectionTy<'tcx> {
2019-12-08T18:32:47.2662951Z      |      ---- lifetime `'tcx` defined here
2019-12-08T18:32:47.2663563Z 1460 |     fn clean(&self, cx: &DocContext<'_>) -> Type {
2019-12-08T18:32:47.2664199Z      |                                     -- let's call this `'1`
2019-12-08T18:32:47.2664828Z 1461 |         let trait_ = match self.trait_ref(cx.tcx).clean(cx) {
2019-12-08T18:32:47.2665497Z      |                            ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'1`
2019-12-08T18:32:49.2772635Z error: aborting due to previous error
2019-12-08T18:32:49.2773133Z 
2019-12-08T18:32:49.3420791Z error: could not compile `rustdoc`.
2019-12-08T18:32:49.3421504Z 
---
2019-12-08T18:32:49.3576649Z   local time: Sun Dec  8 18:32:49 UTC 2019
2019-12-08T18:32:49.6396923Z   network time: Sun, 08 Dec 2019 18:32:49 GMT
2019-12-08T18:32:49.6401060Z == end clock drift check ==
2019-12-08T18:32:51.2401401Z 
2019-12-08T18:32:51.2504585Z ##[error]Bash exited with code '1'.
2019-12-08T18:32:51.2567142Z ##[section]Starting: Checkout
2019-12-08T18:32:51.2569546Z ==============================================================================
2019-12-08T18:32:51.2569605Z Task         : Get sources
2019-12-08T18:32:51.2569652Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
