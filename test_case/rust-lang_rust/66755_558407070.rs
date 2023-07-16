plain
2019-11-26T00:15:43.1337860Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T00:15:43.9915876Z ##[command]git config gc.auto 0
2019-11-26T00:15:43.9922014Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T00:15:43.9925200Z ##[command]git config --get-all http.proxy
2019-11-26T00:15:43.9930226Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66755/merge:refs/remotes/pull/66755/merge
---
2019-11-26T00:43:57.3116729Z    Compiling rustc-demangle v0.1.16
2019-11-26T00:43:58.4026097Z error[E0599]: no associated item named `MAX` found for type `usize` in the current scope
2019-11-26T00:43:58.4027191Z   --> src/liballoc/raw_vec.rs:57:52
2019-11-26T00:43:58.4027493Z    |
2019-11-26T00:43:58.4027872Z 57 |             { if mem::size_of::<T>() == 0 { usize::MAX } else { 0 } }
2019-11-26T00:43:58.4028620Z    |                                                    ^^^ associated item not found in `usize`
2019-11-26T00:43:58.6799215Z error: aborting due to previous error
2019-11-26T00:43:58.6799422Z 
2019-11-26T00:43:58.6804073Z For more information about this error, try `rustc --explain E0599`.
2019-11-26T00:43:58.7008780Z error: could not compile `alloc`.
---
2019-11-26T00:43:58.9141867Z   local time: Tue Nov 26 00:43:58 UTC 2019
2019-11-26T00:43:59.1931606Z   network time: Tue, 26 Nov 2019 00:43:59 GMT
2019-11-26T00:43:59.1931719Z == end clock drift check ==
2019-11-26T00:44:01.9441504Z 
2019-11-26T00:44:01.9559051Z ##[error]Bash exited with code '1'.
2019-11-26T00:44:01.9635292Z ##[section]Starting: Checkout
2019-11-26T00:44:01.9638827Z ==============================================================================
2019-11-26T00:44:01.9638890Z Task         : Get sources
2019-11-26T00:44:01.9639274Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
