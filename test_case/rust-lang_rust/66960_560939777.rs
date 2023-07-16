plain
2019-12-02T23:56:49.3410637Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T23:56:49.3603957Z ##[command]git config gc.auto 0
2019-12-02T23:56:49.3695024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T23:56:49.3739473Z ##[command]git config --get-all http.proxy
2019-12-02T23:56:50.1537611Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66960/merge:refs/remotes/pull/66960/merge
---
2019-12-03T00:14:01.7985493Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-03T00:14:10.5887142Z error[E0282]: type annotations needed
2019-12-03T00:14:10.5888140Z     --> src/librustc_mir/interpret/place.rs:1068:13
2019-12-03T00:14:10.5889180Z      |
2019-12-03T00:14:10.5890045Z 1068 |         let variant_scalar = Scalar::from_u32(variant_index.as_u32()).into();
2019-12-03T00:14:10.5890776Z      |             ^^^^^^^^^^^^^^ consider giving `variant_scalar` a type
2019-12-03T00:14:12.7653976Z error: aborting due to previous error
2019-12-03T00:14:12.7659003Z 
2019-12-03T00:14:12.7665836Z For more information about this error, try `rustc --explain E0282`.
2019-12-03T00:14:12.8465273Z error: could not compile `rustc_mir`.
---
2019-12-03T00:16:13.1053440Z   local time: Tue Dec  3 00:16:13 UTC 2019
2019-12-03T00:16:13.3876274Z   network time: Tue, 03 Dec 2019 00:16:13 GMT
2019-12-03T00:16:13.3876400Z == end clock drift check ==
2019-12-03T00:16:16.1459937Z 
2019-12-03T00:16:16.1584685Z ##[error]Bash exited with code '1'.
2019-12-03T00:16:16.1624346Z ##[section]Starting: Checkout
2019-12-03T00:16:16.1626089Z ==============================================================================
2019-12-03T00:16:16.1626280Z Task         : Get sources
2019-12-03T00:16:16.1626344Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
