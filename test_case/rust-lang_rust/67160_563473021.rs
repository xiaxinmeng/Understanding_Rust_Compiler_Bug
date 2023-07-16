plain
2019-12-09T22:21:10.9289031Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T22:21:11.9336880Z ##[command]git config gc.auto 0
2019-12-09T22:21:11.9342357Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T22:21:11.9345290Z ##[command]git config --get-all http.proxy
2019-12-09T22:21:11.9348172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67160/merge:refs/remotes/pull/67160/merge
---
2019-12-09T22:28:35.7965884Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-09T22:28:38.4376878Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-09T22:28:40.3000256Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-09T22:29:20.6783715Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-09T22:29:23.5705092Z error[E0277]: `&[&rustc::ty::TyS<'_>; 1]` is not an iterator
2019-12-09T22:29:23.5705692Z    --> src/librustc_typeck/check/demand.rs:562:52
2019-12-09T22:29:23.5705920Z     |
2019-12-09T22:29:23.5706268Z 562 |                         substs: self.tcx.mk_substs(&[checked_ty]),
2019-12-09T22:29:23.5706715Z     |                                                    ^^^^^^^^^^^^^ `&[&rustc::ty::TyS<'_>; 1]` is not an iterator
2019-12-09T22:29:23.5706980Z     |
2019-12-09T22:29:23.5707308Z     = help: the trait `std::iter::Iterator` is not implemented for `&[&rustc::ty::TyS<'_>; 1]`
2019-12-09T22:29:23.5707929Z     = note: required because of the requirements on the impl of `rustc::ty::context::InternAs<[rustc::ty::subst::GenericArg<'_>], &rustc::ty::List<rustc::ty::subst::GenericArg<'_>>>` for `&[&rustc::ty::TyS<'_>; 1]`
2019-12-09T22:29:24.7264740Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-09T22:29:25.4191195Z error: aborting due to previous error
2019-12-09T22:29:25.4192354Z 
2019-12-09T22:29:25.4196451Z For more information about this error, try `rustc --explain E0277`.
---
2019-12-09T22:29:25.8764635Z   local time: Mon Dec  9 22:29:25 UTC 2019
2019-12-09T22:29:26.0356974Z   network time: Mon, 09 Dec 2019 22:29:26 GMT
2019-12-09T22:29:26.0361444Z == end clock drift check ==
2019-12-09T22:29:27.2248779Z 
2019-12-09T22:29:27.2354214Z ##[error]Bash exited with code '1'.
2019-12-09T22:29:27.2384712Z ##[section]Starting: Checkout
2019-12-09T22:29:27.2386803Z ==============================================================================
2019-12-09T22:29:27.2386876Z Task         : Get sources
2019-12-09T22:29:27.2386924Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
