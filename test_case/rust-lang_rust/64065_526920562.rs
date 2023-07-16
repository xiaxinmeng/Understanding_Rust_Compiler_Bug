plain
2019-09-01T13:28:52.5147112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T13:28:52.5326902Z ##[command]git config gc.auto 0
2019-09-01T13:28:53.1298171Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T13:28:53.1305468Z ##[command]git config --get-all http.proxy
2019-09-01T13:28:53.1309426Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64065/merge:refs/remotes/pull/64065/merge
---
2019-09-01T13:37:22.7703688Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-01T13:37:23.0216388Z error: unused import: `path_local`
2019-09-01T13:37:23.0239480Z  --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:3:23
2019-09-01T13:37:23.0240258Z   |
2019-09-01T13:37:23.0241360Z 3 | use crate::deriving::{path_local, pathvec_std, path_std};
2019-09-01T13:37:23.0242030Z   |
2019-09-01T13:37:23.0242364Z   = note: `-D unused-imports` implied by `-D warnings`
2019-09-01T13:37:23.0242548Z 
2019-09-01T13:37:23.7561441Z error: aborting due to previous error
---
2019-09-01T13:38:12.6936507Z == clock drift check ==
2019-09-01T13:38:12.6951207Z   local time: Sun Sep  1 13:38:12 UTC 2019
2019-09-01T13:38:12.8458745Z   network time: Sun, 01 Sep 2019 13:38:12 GMT
2019-09-01T13:38:12.8459170Z == end clock drift check ==
2019-09-01T13:38:13.8130219Z ##[error]Bash exited with code '1'.
2019-09-01T13:38:13.8167220Z ##[section]Starting: Checkout
2019-09-01T13:38:13.8168828Z ==============================================================================
2019-09-01T13:38:13.8169003Z Task         : Get sources
2019-09-01T13:38:13.8169069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
