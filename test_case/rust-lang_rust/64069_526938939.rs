plain
2019-09-01T17:30:24.9862540Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T17:30:25.6788628Z ##[command]git config gc.auto 0
2019-09-01T17:30:25.6791906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T17:30:25.6795190Z ##[command]git config --get-all http.proxy
2019-09-01T17:30:25.6799980Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64069/merge:refs/remotes/pull/64069/merge
---
2019-09-01T17:36:38.0473105Z     Checking backtrace v0.3.35
2019-09-01T17:36:43.2265818Z error: impl has missing stability attribute
2019-09-01T17:36:43.2267576Z    --> src/libstd/ffi/c_str.rs:722:1
2019-09-01T17:36:43.2268155Z     |
2019-09-01T17:36:43.2268657Z 722 | / impl From<Vec<NonZeroU8>> for CString {
2019-09-01T17:36:43.2269186Z 723 | |     /// Converts a [`Vec`]`<`[`NonZeroU8`]`>` into a [`CString`] without
2019-09-01T17:36:43.2269717Z 724 | |     /// copying nor checking for inner null bytes.
2019-09-01T17:36:43.2270910Z ...   |
2019-09-01T17:36:43.2271338Z 793 | |     }
2019-09-01T17:36:43.2271800Z 794 | | }
2019-09-01T17:36:43.2272242Z     | |_^
---
2019-09-01T17:36:43.4492110Z == clock drift check ==
2019-09-01T17:36:43.4512422Z   local time: Sun Sep  1 17:36:43 UTC 2019
2019-09-01T17:36:43.6012744Z   network time: Sun, 01 Sep 2019 17:36:43 GMT
2019-09-01T17:36:43.6017002Z == end clock drift check ==
2019-09-01T17:36:50.5865241Z ##[error]Bash exited with code '1'.
2019-09-01T17:36:50.5899521Z ##[section]Starting: Checkout
2019-09-01T17:36:50.5902055Z ==============================================================================
2019-09-01T17:36:50.5902124Z Task         : Get sources
2019-09-01T17:36:50.5902167Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
