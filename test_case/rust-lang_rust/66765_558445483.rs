plain
2019-11-26T03:32:55.0021645Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T03:32:55.8124903Z ##[command]git config gc.auto 0
2019-11-26T03:32:55.8129667Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T03:32:55.8131541Z ##[command]git config --get-all http.proxy
2019-11-26T03:32:55.8136608Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66765/merge:refs/remotes/pull/66765/merge
---
2019-11-26T03:39:53.0631306Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-11-26T03:39:53.5710871Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-11-26T03:39:59.0457683Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-11-26T03:40:01.4171318Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2019-11-26T03:40:01.4978546Z error: expected one of `->`, `where`, or `{`, found `-`
2019-11-26T03:40:01.4978920Z    --> src/liballoc/borrow.rs:427:22
2019-11-26T03:40:01.4979145Z     |
2019-11-26T03:40:01.4979421Z 427 |     fn as_ref(&self) -? &OsStr {
2019-11-26T03:40:01.4979849Z     |                      ^ expected one of `->`, `where`, or `{` here
2019-11-26T03:40:01.5033640Z error: aborting due to previous error
2019-11-26T03:40:01.5038027Z 
2019-11-26T03:40:01.5102876Z error: could not compile `alloc`.
2019-11-26T03:40:01.5103205Z warning: build failed, waiting for other jobs to finish...
---
2019-11-26T03:40:01.7704105Z   local time: Tue Nov 26 03:40:01 UTC 2019
2019-11-26T03:40:01.8559422Z   network time: Tue, 26 Nov 2019 03:40:01 GMT
2019-11-26T03:40:01.8559521Z == end clock drift check ==
2019-11-26T03:40:04.5093339Z 
2019-11-26T03:40:04.5207853Z ##[error]Bash exited with code '1'.
2019-11-26T03:40:04.5235662Z ##[section]Starting: Checkout
2019-11-26T03:40:04.5237515Z ==============================================================================
2019-11-26T03:40:04.5237569Z Task         : Get sources
2019-11-26T03:40:04.5237636Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
