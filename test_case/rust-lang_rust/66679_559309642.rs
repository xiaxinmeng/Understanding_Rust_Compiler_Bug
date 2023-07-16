plain
2019-11-28T01:33:03.3582393Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T01:33:03.3766918Z ##[command]git config gc.auto 0
2019-11-28T01:33:03.3852344Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T01:33:03.3906386Z ##[command]git config --get-all http.proxy
2019-11-28T01:33:03.4053497Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66679/merge:refs/remotes/pull/66679/merge
---
2019-11-28T01:46:46.2614698Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-11-28T01:47:50.1012421Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-28T01:51:25.1719098Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-11-28T01:52:02.3444682Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-28T01:52:06.5009920Z error[E0599]: no method named `synthesize_region_name` found for type `&borrow_check::nll::region_infer::RegionInferenceContext<'tcx>` in the current scope
2019-11-28T01:52:06.5013859Z     |
2019-11-28T01:52:06.5013859Z     |
2019-11-28T01:52:06.5014213Z 312 |                         let name = self.synthesize_region_name(renctx);
2019-11-28T01:52:06.5014779Z     |                                         ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&borrow_check::nll::region_infer::RegionInferenceContext<'tcx>`
2019-11-28T01:52:12.4188321Z error: aborting due to previous error
2019-11-28T01:52:12.4188422Z 
2019-11-28T01:52:12.4189015Z For more information about this error, try `rustc --explain E0599`.
2019-11-28T01:52:12.5010823Z error: could not compile `rustc_mir`.
---
2019-11-28T01:54:06.5159482Z   local time: Thu Nov 28 01:54:06 UTC 2019
2019-11-28T01:54:06.8158913Z   network time: Thu, 28 Nov 2019 01:54:06 GMT
2019-11-28T01:54:06.8160071Z == end clock drift check ==
2019-11-28T01:54:09.6815800Z 
2019-11-28T01:54:09.6914662Z ##[error]Bash exited with code '1'.
2019-11-28T01:54:09.6945626Z ##[section]Starting: Checkout
2019-11-28T01:54:09.6947350Z ==============================================================================
2019-11-28T01:54:09.6947421Z Task         : Get sources
2019-11-28T01:54:09.6947467Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
