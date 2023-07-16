plain
2019-09-30T00:25:35.0524085Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T00:25:35.0713551Z ##[command]git config gc.auto 0
2019-09-30T00:25:35.0793616Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T00:25:35.0866078Z ##[command]git config --get-all http.proxy
2019-09-30T00:25:35.1023658Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-30T00:33:40.4132698Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-09-30T00:33:40.6471689Z error[E0412]: cannot find type `StableHashingContext` in this scope
2019-09-30T00:33:40.6472080Z   --> src/librustc_data_structures/steal.rs:50:24
2019-09-30T00:33:40.6472367Z    |
2019-09-30T00:33:40.6472699Z 50 | impl<'a, T> HashStable<StableHashingContext<'a>> for Steal<T>
2019-09-30T00:33:40.6477545Z 
2019-09-30T00:33:40.6515044Z error[E0412]: cannot find type `StableHashingContext` in this scope
2019-09-30T00:33:40.6515347Z   --> src/librustc_data_structures/steal.rs:52:19
2019-09-30T00:33:40.6515584Z    |
2019-09-30T00:33:40.6515584Z    |
2019-09-30T00:33:40.6515855Z 52 |     T: HashStable<StableHashingContext<'a>>,
2019-09-30T00:33:40.6520809Z 
2019-09-30T00:33:40.6560707Z error[E0412]: cannot find type `StableHashingContext` in this scope
2019-09-30T00:33:40.6561048Z   --> src/librustc_data_structures/steal.rs:54:37
2019-09-30T00:33:40.6561330Z    |
2019-09-30T00:33:40.6561330Z    |
2019-09-30T00:33:40.6561910Z 54 |     fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
2019-09-30T00:33:40.6603242Z 
2019-09-30T00:33:40.6941789Z error: unused import: `StableHasherResult`
2019-09-30T00:33:40.6942171Z  --> src/librustc_data_structures/steal.rs:1:42
2019-09-30T00:33:40.6942471Z   |
2019-09-30T00:33:40.6942471Z   |
2019-09-30T00:33:40.6942811Z 1 | use crate::stable_hasher::{StableHasher, StableHasherResult, HashStable};
2019-09-30T00:33:40.6943575Z   |
2019-09-30T00:33:40.6943840Z   = note: `-D unused-imports` implied by `-D warnings`
2019-09-30T00:33:40.6948513Z 
2019-09-30T00:33:41.7023240Z error: aborting due to 4 previous errors
---
2019-09-30T00:33:46.5292478Z == clock drift check ==
2019-09-30T00:33:46.5312604Z   local time: Mon Sep 30 00:33:46 UTC 2019
2019-09-30T00:33:46.6285534Z   network time: Mon, 30 Sep 2019 00:33:46 GMT
2019-09-30T00:33:46.6295641Z == end clock drift check ==
2019-09-30T00:33:47.5276181Z ##[error]Bash exited with code '1'.
2019-09-30T00:33:47.5313933Z ##[section]Starting: Checkout
2019-09-30T00:33:47.5315769Z ==============================================================================
2019-09-30T00:33:47.5315843Z Task         : Get sources
2019-09-30T00:33:47.5315891Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
