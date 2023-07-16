plain
2019-11-12T09:53:23.7103066Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T09:53:23.7257362Z ##[command]git config gc.auto 0
2019-11-12T09:53:23.7327659Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T09:53:23.7382053Z ##[command]git config --get-all http.proxy
2019-11-12T09:53:23.7525745Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66325/merge:refs/remotes/pull/66325/merge
---
2019-11-12T10:33:40.2849785Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-12T10:33:41.9704946Z error: unused label
2019-11-12T10:33:41.9706747Z    --> src/librustc_mir/borrow_check/nll/type_check/liveness/trace.rs:252:9
2019-11-12T10:33:41.9707308Z     |
2019-11-12T10:33:41.9708053Z 252 |         'next_block: while let Some(term_point) = self.stack.pop() {
2019-11-12T10:33:41.9709125Z     |
2019-11-12T10:33:41.9709788Z     = note: `-D unused-labels` implied by `-D warnings`
2019-11-12T10:33:41.9714059Z 
2019-11-12T10:33:51.0343975Z error: aborting due to previous error
---
2019-11-12T10:35:44.4902934Z   local time: Tue Nov 12 10:35:44 UTC 2019
2019-11-12T10:35:44.7691139Z   network time: Tue, 12 Nov 2019 10:35:44 GMT
2019-11-12T10:35:44.7691825Z == end clock drift check ==
2019-11-12T10:35:47.8469473Z 
2019-11-12T10:35:47.8568570Z ##[error]Bash exited with code '1'.
2019-11-12T10:35:47.8602006Z ##[section]Starting: Checkout
2019-11-12T10:35:47.8603597Z ==============================================================================
2019-11-12T10:35:47.8603773Z Task         : Get sources
2019-11-12T10:35:47.8603827Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
