plain
2019-10-05T18:38:09.8361100Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T18:38:09.8585169Z ##[command]git config gc.auto 0
2019-10-05T18:38:09.8642141Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T18:38:10.8151783Z ##[command]git config --get-all http.proxy
2019-10-05T18:38:10.8158672Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65077/merge:refs/remotes/pull/65077/merge
---
2019-10-05T18:47:54.1634193Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-05T18:47:55.2955341Z error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2019-10-05T18:47:55.2955779Z    --> src/librustc_typeck/check/method/mod.rs:218:39
2019-10-05T18:47:55.2956090Z     |
2019-10-05T18:47:55.2956425Z 218 |             let trait_type = self.tcx.mk_ref(region, t_type, mutability.not());
2019-10-05T18:47:55.2956843Z 
2019-10-05T18:47:55.6525990Z error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2019-10-05T18:47:55.6527078Z    --> src/librustc_typeck/check/method/suggest.rs:576:51
2019-10-05T18:47:55.6527608Z     |
2019-10-05T18:47:55.6527608Z     |
2019-10-05T18:47:55.6528149Z 576 |                         let trait_type = self.tcx.mk_ref(region, t_type, mutability.not());
2019-10-05T18:47:55.6528886Z 
2019-10-05T18:47:55.8171391Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-05T18:47:56.8839801Z error: aborting due to 2 previous errors
2019-10-05T18:47:56.8841591Z 
---
2019-10-05T18:48:16.9928169Z == clock drift check ==
2019-10-05T18:48:16.9928236Z   local time: Sat Oct  5 18:48:15 UTC 2019
2019-10-05T18:48:16.9928289Z   network time: Sat, 05 Oct 2019 18:48:15 GMT
2019-10-05T18:48:16.9928340Z == end clock drift check ==
2019-10-05T18:48:16.9997818Z ##[error]Bash exited with code '1'.
2019-10-05T18:48:17.0034156Z ##[section]Starting: Checkout
2019-10-05T18:48:17.0036584Z ==============================================================================
2019-10-05T18:48:17.0036650Z Task         : Get sources
2019-10-05T18:48:17.0036723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
