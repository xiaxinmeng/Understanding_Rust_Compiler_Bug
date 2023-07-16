plain
2019-09-29T23:53:27.3593697Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T23:53:27.3794612Z ##[command]git config gc.auto 0
2019-09-29T23:53:27.3847847Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T23:53:27.3896756Z ##[command]git config --get-all http.proxy
2019-09-29T23:53:27.4034746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-30T00:00:55.9583843Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-09-30T00:00:56.2073081Z error[E0107]: wrong number of type arguments: expected 0, found 1
2019-09-30T00:00:56.2073430Z   --> src/librustc_data_structures/steal.rs:53:69
2019-09-30T00:00:56.2073598Z    |
2019-09-30T00:00:56.2073857Z 53 | ...                   hasher: &mut StableHasher<W>) {
2019-09-30T00:00:56.2074487Z    |                                                 ^ unexpected type argument
2019-09-30T00:00:56.2084737Z error: aborting due to previous error
2019-09-30T00:00:56.2092941Z 
2019-09-30T00:00:56.2116356Z For more information about this error, try `rustc --explain E0107`.
2019-09-30T00:00:56.2196086Z error: could not compile `rustc_data_structures`.
---
2019-09-30T00:01:00.8854064Z == clock drift check ==
2019-09-30T00:01:00.8870091Z   local time: Mon Sep 30 00:01:00 UTC 2019
2019-09-30T00:01:00.9708178Z   network time: Mon, 30 Sep 2019 00:01:00 GMT
2019-09-30T00:01:00.9712050Z == end clock drift check ==
2019-09-30T00:01:02.1255089Z ##[error]Bash exited with code '1'.
2019-09-30T00:01:02.1292235Z ##[section]Starting: Checkout
2019-09-30T00:01:02.1294180Z ==============================================================================
2019-09-30T00:01:02.1294415Z Task         : Get sources
2019-09-30T00:01:02.1294454Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
