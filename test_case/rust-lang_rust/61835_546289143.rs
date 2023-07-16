plain
2019-10-25T09:36:27.0778933Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T09:36:27.1015597Z ##[command]git config gc.auto 0
2019-10-25T09:36:27.1112453Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T09:36:27.1182280Z ##[command]git config --get-all http.proxy
2019-10-25T09:36:27.1328760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-10-25T09:48:28.9335116Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-25T09:51:32.4844373Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-25T09:55:21.5044122Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-10-25T09:56:01.3233847Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-25T09:56:08.5288916Z error[E0609]: no field `const_transmute` on type `&syntax::feature_gate::Features`
2019-10-25T09:56:08.5292790Z    --> src/librustc_mir/transform/check_consts/ops.rs:308:29
2019-10-25T09:56:08.5293176Z     |
2019-10-25T09:56:08.5293501Z 308 |         Some(tcx.features().const_transmute)
2019-10-25T09:56:08.5293841Z 
2019-10-25T09:56:12.5482232Z error: aborting due to previous error
2019-10-25T09:56:12.5482507Z 
2019-10-25T09:56:12.5490445Z For more information about this error, try `rustc --explain E0609`.
---
2019-10-25T09:57:58.5610387Z   local time: Fri Oct 25 09:57:58 UTC 2019
2019-10-25T09:57:58.6628235Z   network time: Fri, 25 Oct 2019 09:57:58 GMT
2019-10-25T09:57:58.6634820Z == end clock drift check ==
2019-10-25T09:58:01.6606768Z 
2019-10-25T09:58:01.6723110Z ##[error]Bash exited with code '1'.
2019-10-25T09:58:01.6763668Z ##[section]Starting: Checkout
2019-10-25T09:58:01.6765838Z ==============================================================================
2019-10-25T09:58:01.6765922Z Task         : Get sources
2019-10-25T09:58:01.6765976Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
