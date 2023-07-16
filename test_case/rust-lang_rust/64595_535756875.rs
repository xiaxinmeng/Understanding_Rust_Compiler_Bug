plain
2019-09-27T01:53:26.6490387Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T01:53:26.6676458Z ##[command]git config gc.auto 0
2019-09-27T01:53:26.6743765Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T01:53:26.6802161Z ##[command]git config --get-all http.proxy
2019-09-27T01:53:26.6951297Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64595/merge:refs/remotes/pull/64595/merge
---
2019-09-27T02:02:47.6411505Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-09-27T02:02:47.8141259Z error[E0412]: cannot find type `Kind` in module `ty::subst`
2019-09-27T02:02:47.8141897Z    --> src/librustc_traits/dropck_outlives.rs:234:40
2019-09-27T02:02:47.8142141Z     |
2019-09-27T02:02:47.8142452Z 234 |                 .map(|t| -> ty::subst::Kind<'tcx> { t.into() }));
2019-09-27T02:02:47.8147382Z 
2019-09-27T02:02:48.3441328Z error: aborting due to previous error
2019-09-27T02:02:48.3446105Z 
2019-09-27T02:02:48.3457816Z For more information about this error, try `rustc --explain E0412`.
---
2019-09-27T02:02:59.6882533Z == clock drift check ==
2019-09-27T02:02:59.6902885Z   local time: Fri Sep 27 02:02:59 UTC 2019
2019-09-27T02:02:59.8409457Z   network time: Fri, 27 Sep 2019 02:02:59 GMT
2019-09-27T02:02:59.8410205Z == end clock drift check ==
2019-09-27T02:03:01.0456996Z ##[error]Bash exited with code '1'.
2019-09-27T02:03:01.0513182Z ##[section]Starting: Checkout
2019-09-27T02:03:01.0515208Z ==============================================================================
2019-09-27T02:03:01.0515388Z Task         : Get sources
2019-09-27T02:03:01.0515450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
