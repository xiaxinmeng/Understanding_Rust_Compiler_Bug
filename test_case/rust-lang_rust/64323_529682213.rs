plain
2019-09-09T21:41:55.0792495Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T21:41:55.0980180Z ##[command]git config gc.auto 0
2019-09-09T21:41:55.1055700Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T21:41:55.1105145Z ##[command]git config --get-all http.proxy
2019-09-09T21:41:55.1244244Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64323/merge:refs/remotes/pull/64323/merge
---
2019-09-09T21:48:05.8255969Z     Checking backtrace v0.3.37
2019-09-09T21:48:07.7744965Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-09T21:48:07.8117840Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-09T21:48:07.9382237Z     Checking hashbrown v0.5.0
2019-09-09T21:48:08.8037356Z error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
2019-09-09T21:48:08.8037944Z    --> src/libstd/sys_common/backtrace.rs:123:36
2019-09-09T21:48:08.8038183Z     |
2019-09-09T21:48:08.8038398Z 123 |         return Some(PrintFmt::Full),
2019-09-09T21:48:08.8038716Z     |                                    ^ expected one of `.`, `;`, `?`, `}`, or an operator here
2019-09-09T21:48:13.0519255Z error: aborting due to previous error
2019-09-09T21:48:13.0520949Z 
2019-09-09T21:48:13.0883881Z error: Could not compile `std`.
2019-09-09T21:48:13.0884208Z 
---
2019-09-09T21:48:13.0959787Z == clock drift check ==
2019-09-09T21:48:13.0974200Z   local time: Mon Sep  9 21:48:13 UTC 2019
2019-09-09T21:48:13.2481474Z   network time: Mon, 09 Sep 2019 21:48:13 GMT
2019-09-09T21:48:13.2486098Z == end clock drift check ==
2019-09-09T21:48:20.6516609Z ##[error]Bash exited with code '1'.
2019-09-09T21:48:20.6546559Z ##[section]Starting: Checkout
2019-09-09T21:48:20.6548071Z ==============================================================================
2019-09-09T21:48:20.6548113Z Task         : Get sources
2019-09-09T21:48:20.6548148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
