plain
2019-09-17T00:14:17.3888468Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T00:14:17.4076267Z ##[command]git config gc.auto 0
2019-09-17T00:14:17.4164414Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T00:14:17.4234969Z ##[command]git config --get-all http.proxy
2019-09-17T00:14:17.4383276Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64532/merge:refs/remotes/pull/64532/merge
---
2019-09-17T00:24:21.1640857Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin/deprecated)
2019-09-17T00:24:21.2618948Z error: method is never used: `get`
2019-09-17T00:24:21.2619329Z    --> src/librustc_mir/dataflow/mod.rs:457:5
2019-09-17T00:24:21.2619721Z     |
2019-09-17T00:24:21.2620119Z 457 |     pub fn get(&self) -> &BitSet<BD::Idx> {
2019-09-17T00:24:21.2630640Z 
2019-09-17T00:24:21.6701530Z error: aborting due to previous error
2019-09-17T00:24:21.6702746Z 
2019-09-17T00:24:21.7960453Z error: Could not compile `rustc_mir`.
---
2019-09-17T00:24:21.8060651Z == clock drift check ==
2019-09-17T00:24:21.8079464Z   local time: Tue Sep 17 00:24:21 UTC 2019
2019-09-17T00:24:22.0946960Z   network time: Tue, 17 Sep 2019 00:24:22 GMT
2019-09-17T00:24:22.0947137Z == end clock drift check ==
2019-09-17T00:24:23.2605709Z ##[error]Bash exited with code '1'.
2019-09-17T00:24:23.2645365Z ##[section]Starting: Checkout
2019-09-17T00:24:23.2647834Z ==============================================================================
2019-09-17T00:24:23.2647921Z Task         : Get sources
2019-09-17T00:24:23.2647975Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
