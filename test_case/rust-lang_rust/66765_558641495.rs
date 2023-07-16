plain
2019-11-26T13:50:17.0052978Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T13:50:17.0232897Z ##[command]git config gc.auto 0
2019-11-26T13:50:17.0302855Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T13:50:17.0349253Z ##[command]git config --get-all http.proxy
2019-11-26T13:50:17.0488010Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66765/merge:refs/remotes/pull/66765/merge
---
2019-11-26T13:59:11.4402287Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-26T13:59:21.3725041Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-26T13:59:22.9832496Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-26T13:59:42.1660899Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-26T14:00:11.8676348Z error[E0283]: type annotations needed: cannot resolve `std::borrow::Cow<'_, str>: std::convert::AsRef<_>`
2019-11-26T14:00:11.8677213Z   --> src/librustc/session/filesearch.rs:97:42
2019-11-26T14:00:11.8677481Z    |
2019-11-26T14:00:11.8678081Z 97 |         p.push(find_libdir(self.sysroot).as_ref());
2019-11-26T14:00:11.8696249Z 
2019-11-26T14:00:11.8696249Z 
2019-11-26T14:00:11.8729086Z error[E0283]: type annotations needed: cannot resolve `std::borrow::Cow<'_, str>: std::convert::AsRef<_>`
2019-11-26T14:00:11.8729355Z    --> src/librustc/session/filesearch.rs:106:52
2019-11-26T14:00:11.8729577Z     |
2019-11-26T14:00:11.8729822Z 106 |     let mut p = PathBuf::from(find_libdir(sysroot).as_ref());
2019-11-26T14:00:11.8735233Z 
2019-11-26T14:00:18.2515958Z error: aborting due to 2 previous errors
2019-11-26T14:00:18.2521529Z 
2019-11-26T14:00:18.2531998Z For more information about this error, try `rustc --explain E0283`.
---
2019-11-26T14:00:20.3381272Z   local time: Tue Nov 26 14:00:20 UTC 2019
2019-11-26T14:00:20.6170468Z   network time: Tue, 26 Nov 2019 14:00:20 GMT
2019-11-26T14:00:20.6170575Z == end clock drift check ==
2019-11-26T14:00:21.6331739Z 
2019-11-26T14:00:21.6428387Z ##[error]Bash exited with code '1'.
2019-11-26T14:00:21.6456148Z ##[section]Starting: Checkout
2019-11-26T14:00:21.6457915Z ==============================================================================
2019-11-26T14:00:21.6457958Z Task         : Get sources
2019-11-26T14:00:21.6457996Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
