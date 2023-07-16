plain
2019-08-22T01:39:20.9814011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T01:39:21.0095277Z ##[command]git config gc.auto 0
2019-08-22T01:39:21.0158590Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T01:39:21.0210999Z ##[command]git config --get-all http.proxy
2019-08-22T01:39:21.0345989Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63580/merge:refs/remotes/pull/63580/merge
2019-08-22T01:39:23.6454886Z remote:                                                                                         
---
2019-08-22T01:39:55.7925997Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T01:39:55.7926348Z 
2019-08-22T01:39:55.7926892Z   git checkout -b <new-branch-name>
2019-08-22T01:39:55.7927703Z 
2019-08-22T01:39:55.7927812Z HEAD is now at f0a715258 Merge a472fa6d78d3e4c55cb9e3f07704bb95e17000d0 into e44fdf97929d1315add3b76208adf99e8299252d
2019-08-22T01:39:55.8079453Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T01:39:55.8082013Z ==============================================================================
2019-08-22T01:39:55.8082063Z Task         : Bash
2019-08-22T01:39:55.8082122Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T01:47:22.2656669Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-22T01:47:23.4623275Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-22T01:47:35.9924442Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-08-22T01:48:25.5775771Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-08-22T01:48:26.3145112Z error[E0277]: the trait bound `&&rustc_data_structures::indexed_vec::IndexVec<rustc::mir::Promoted, rustc::mir::Body<'_>>: encoder::EncodeContentsForLazy<rustc_data_structures::indexed_vec::IndexVec<rustc::mir::Promoted, rustc::mir::Body<'_>>>` is not satisfied
2019-08-22T01:48:26.3145564Z     --> src/librustc_metadata/encoder.rs:1070:23
2019-08-22T01:48:26.3145793Z      |
2019-08-22T01:48:26.3146059Z 1070 |             Some(self.lazy(&promoted))
2019-08-22T01:48:26.3147140Z      |                       ^^^^ the trait `encoder::EncodeContentsForLazy<rustc_data_structures::indexed_vec::IndexVec<rustc::mir::Promoted, rustc::mir::Body<'_>>>` is not implemented for `&&rustc_data_structures::indexed_vec::IndexVec<rustc::mir::Promoted, rustc::mir::Body<'_>>`
2019-08-22T01:48:27.0898206Z error: aborting due to previous error
2019-08-22T01:48:27.0898310Z 
2019-08-22T01:48:27.0898657Z For more information about this error, try `rustc --explain E0277`.
2019-08-22T01:48:27.1111342Z error: Could not compile `rustc_metadata`.
---
2019-08-22T01:48:33.2828560Z == clock drift check ==
2019-08-22T01:48:33.2844788Z   local time: Thu Aug 22 01:48:33 UTC 2019
2019-08-22T01:48:33.3309952Z   network time: Thu, 22 Aug 2019 01:48:33 GMT
2019-08-22T01:48:33.3312114Z == end clock drift check ==
2019-08-22T01:48:34.4386807Z ##[error]Bash exited with code '1'.
2019-08-22T01:48:34.4440453Z ##[section]Starting: Checkout
2019-08-22T01:48:34.4442955Z ==============================================================================
2019-08-22T01:48:34.4443018Z Task         : Get sources
2019-08-22T01:48:34.4443073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
