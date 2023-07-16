plain
2019-11-13T23:27:12.4647621Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T23:27:12.4826360Z ##[command]git config gc.auto 0
2019-11-13T23:27:12.4900357Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T23:27:12.4957603Z ##[command]git config --get-all http.proxy
2019-11-13T23:27:12.5104210Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-13T23:48:32.1119191Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-11-13T23:48:32.2897622Z error[E0425]: cannot find value `E0744` in this scope
2019-11-13T23:48:32.2898697Z    --> src/librustc_passes/check_const.rs:101:36
2019-11-13T23:48:32.2899271Z     |
2019-11-13T23:48:32.2899817Z 101 |         span_err!(self.sess, span, E0744, "`{}` is not allowed in a `{}`", bad_op, const_kind);
2019-11-13T23:48:32.2900827Z     |
2019-11-13T23:48:32.2901566Z help: possible candidate is found in another module, you can import it into scope
2019-11-13T23:48:32.2902308Z     |
2019-11-13T23:48:32.2902689Z 10  | use rustc_error_codes::E0744;
---
2019-11-13T23:50:16.7973514Z   local time: Wed Nov 13 23:50:16 UTC 2019
2019-11-13T23:50:17.0782061Z   network time: Wed, 13 Nov 2019 23:50:17 GMT
2019-11-13T23:50:17.0786020Z == end clock drift check ==
2019-11-13T23:50:18.7460117Z 
2019-11-13T23:50:18.7581935Z ##[error]Bash exited with code '1'.
2019-11-13T23:50:18.7629891Z ##[section]Starting: Checkout
2019-11-13T23:50:18.7631774Z ==============================================================================
2019-11-13T23:50:18.7631846Z Task         : Get sources
2019-11-13T23:50:18.7631889Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
