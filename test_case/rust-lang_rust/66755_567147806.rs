plain
2019-12-18T18:05:42.3486264Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T18:05:43.0102717Z ##[command]git config gc.auto 0
2019-12-18T18:05:43.0108182Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T18:05:43.0113801Z ##[command]git config --get-all http.proxy
2019-12-18T18:05:43.0118983Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66755/merge:refs/remotes/pull/66755/merge
---
2019-12-18T18:10:59.4597524Z     Checking backtrace v0.3.40
2019-12-18T18:11:00.9063860Z error[E0019]: constant function contains unimplemented expression type
2019-12-18T18:11:00.9065020Z   --> src/liballoc/raw_vec.rs:55:22
2019-12-18T18:11:00.9065565Z    |
2019-12-18T18:11:00.9066087Z 55 |         let cap = if mem::size_of::<T>() == 0 { core::usize::MAX } else { 0 };
2019-12-18T18:11:00.9067439Z 
2019-12-18T18:11:00.9067958Z error[E0019]: constant function contains unimplemented expression type
2019-12-18T18:11:00.9068453Z   --> src/liballoc/raw_vec.rs:55:19
2019-12-18T18:11:00.9068856Z    |
2019-12-18T18:11:00.9068856Z    |
2019-12-18T18:11:00.9069359Z 55 |         let cap = if mem::size_of::<T>() == 0 { core::usize::MAX } else { 0 };
2019-12-18T18:11:00.9070632Z 
2019-12-18T18:11:01.1512245Z error: aborting due to 2 previous errors
2019-12-18T18:11:01.1512950Z 
2019-12-18T18:11:01.1513467Z For more information about this error, try `rustc --explain E0019`.
---
2019-12-18T18:11:01.1893303Z   local time: Wed Dec 18 18:11:01 UTC 2019
2019-12-18T18:11:01.4795283Z   network time: Wed, 18 Dec 2019 18:11:01 GMT
2019-12-18T18:11:01.4802555Z == end clock drift check ==
2019-12-18T18:11:09.2134208Z 
2019-12-18T18:11:09.2244246Z ##[error]Bash exited with code '1'.
2019-12-18T18:11:09.2286036Z ##[section]Starting: Checkout
2019-12-18T18:11:09.2288136Z ==============================================================================
2019-12-18T18:11:09.2288190Z Task         : Get sources
2019-12-18T18:11:09.2288238Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
