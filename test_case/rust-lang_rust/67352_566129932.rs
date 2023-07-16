plain
2019-12-16T15:45:28.9692890Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T15:45:28.9712364Z ##[command]git config gc.auto 0
2019-12-16T15:45:28.9721782Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T15:45:28.9733527Z ##[command]git config --get-all http.proxy
2019-12-16T15:45:28.9739416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67352/merge:refs/remotes/pull/67352/merge
---
2019-12-16T16:14:41.4184391Z    Compiling backtrace-sys v0.1.32
2019-12-16T16:14:41.5282737Z error: stable const functions must have either `rustc_const_stable` or `rustc_const_unstable` attribute
2019-12-16T16:14:41.5283245Z    --> src/libcore/alloc.rs:122:5
2019-12-16T16:14:41.5283508Z     |
2019-12-16T16:14:41.5283816Z 122 | /     pub const fn new<T>() -> Self {
2019-12-16T16:14:41.5284154Z 123 | |         let (size, align) = size_align::<T>();
2019-12-16T16:14:41.5284518Z 124 | |         // Note that the align is guaranteed by rustc to be a power of two and
2019-12-16T16:14:41.5284891Z 125 | |         // the size+align combo is guaranteed to fit in our address space. As a
2019-12-16T16:14:41.5285430Z 130 | |         }
2019-12-16T16:14:41.5285738Z 131 | |     }
2019-12-16T16:14:41.5285978Z     | |_____^
2019-12-16T16:14:41.5286010Z 
---
2019-12-16T16:14:43.5136752Z   local time: Mon Dec 16 16:14:43 UTC 2019
2019-12-16T16:14:43.8084033Z   network time: Mon, 16 Dec 2019 16:14:43 GMT
2019-12-16T16:14:43.8087878Z == end clock drift check ==
2019-12-16T16:14:46.6389293Z 
2019-12-16T16:14:46.6499357Z ##[error]Bash exited with code '1'.
2019-12-16T16:14:46.6540413Z ##[section]Starting: Checkout
2019-12-16T16:14:46.6564473Z ==============================================================================
2019-12-16T16:14:46.6564536Z Task         : Get sources
2019-12-16T16:14:46.6564592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
