plain
2019-10-10T00:46:04.8782629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T00:46:04.8795667Z ##[command]git config gc.auto 0
2019-10-10T00:46:04.8800391Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T00:46:04.8804520Z ##[command]git config --get-all http.proxy
2019-10-10T00:46:04.8807721Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64890/merge:refs/remotes/pull/64890/merge
---
2019-10-10T00:55:09.0464803Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-10-10T00:55:09.5972566Z error[E0624]: method `cast` is private
2019-10-10T00:55:09.5972953Z    --> src/librustc_mir/interpret/step.rs:270:22
2019-10-10T00:55:09.5973216Z     |
2019-10-10T00:55:09.5974315Z 270 |                 self.cast(src, kind, dest)?;
2019-10-10T00:55:09.5979582Z 
2019-10-10T00:55:09.6268351Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-10T00:55:10.9958860Z error: aborting due to previous error
2019-10-10T00:55:10.9963253Z 
---
2019-10-10T00:55:14.2182398Z == clock drift check ==
2019-10-10T00:55:14.2210029Z   local time: Thu Oct 10 00:55:14 UTC 2019
2019-10-10T00:55:14.3491496Z   network time: Thu, 10 Oct 2019 00:55:14 GMT
2019-10-10T00:55:14.3493237Z == end clock drift check ==
2019-10-10T00:55:15.2353064Z ##[error]Bash exited with code '1'.
2019-10-10T00:55:15.2438990Z ##[section]Starting: Checkout
2019-10-10T00:55:15.2440893Z ==============================================================================
2019-10-10T00:55:15.2441208Z Task         : Get sources
2019-10-10T00:55:15.2441329Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
