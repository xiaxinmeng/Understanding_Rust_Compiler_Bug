plain
2019-10-24T22:36:37.4326335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T22:36:37.4570952Z ##[command]git config gc.auto 0
2019-10-24T22:36:37.4662095Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T22:36:37.4723585Z ##[command]git config --get-all http.proxy
2019-10-24T22:36:37.4871520Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65752/merge:refs/remotes/pull/65752/merge
---
2019-10-24T22:52:38.8315653Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-24T22:52:40.6735454Z error[E0599]: no method named `as_interned_str` found for type `syntax::symbol::Symbol` in the current scope
2019-10-24T22:52:40.6737155Z     --> src/librustc_typeck/check/mod.rs:1996:69
2019-10-24T22:52:40.6738002Z      |
2019-10-24T22:52:40.6738637Z 1996 |                     ty::Param(param) if param.name == kw::SelfUpper.as_interned_str() => {
2019-10-24T22:52:40.6739472Z      |                                                                     ^^^^^^^^^^^^^^^ method not found in `syntax::symbol::Symbol`
2019-10-24T22:52:40.6852138Z error[E0599]: no method named `as_interned_str` found for type `syntax::symbol::Symbol` in the current scope
2019-10-24T22:52:40.6853443Z     --> src/librustc_typeck/check/mod.rs:2013:60
2019-10-24T22:52:40.6855061Z      |
2019-10-24T22:52:40.6855061Z      |
2019-10-24T22:52:40.6855828Z 2013 | ...                   if param.name == kw::SelfUpper.as_interned_str() => {
2019-10-24T22:52:40.6856501Z      |                                                      ^^^^^^^^^^^^^^^ method not found in `syntax::symbol::Symbol`
2019-10-24T22:52:43.5521906Z error: aborting due to 2 previous errors
2019-10-24T22:52:43.5523177Z 
2019-10-24T22:52:43.5532168Z For more information about this error, try `rustc --explain E0599`.
2019-10-24T22:52:43.5952371Z error: could not compile `rustc_typeck`.
---
2019-10-24T22:56:46.2233983Z   local time: Thu Oct 24 22:56:46 UTC 2019
2019-10-24T22:56:46.3760391Z   network time: Thu, 24 Oct 2019 22:56:46 GMT
2019-10-24T22:56:46.3761292Z == end clock drift check ==
2019-10-24T22:56:49.2017425Z 
2019-10-24T22:56:49.2137484Z ##[error]Bash exited with code '1'.
2019-10-24T22:56:49.2180396Z ##[section]Starting: Checkout
2019-10-24T22:56:49.2183124Z ==============================================================================
2019-10-24T22:56:49.2183191Z Task         : Get sources
2019-10-24T22:56:49.2183230Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
