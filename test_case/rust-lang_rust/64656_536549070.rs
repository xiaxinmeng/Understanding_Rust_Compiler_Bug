plain
2019-09-30T12:51:10.7561090Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T12:51:10.7742367Z ##[command]git config gc.auto 0
2019-09-30T12:51:10.7809025Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T12:51:10.7864617Z ##[command]git config --get-all http.proxy
2019-09-30T12:51:10.8006210Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64656/merge:refs/remotes/pull/64656/merge
---
2019-09-30T12:57:32.2260874Z     Checking backtrace v0.3.37
2019-09-30T12:57:33.8398790Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-30T12:57:33.8801601Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-30T12:57:34.0141297Z     Checking hashbrown v0.5.0
2019-09-30T12:57:36.4783350Z error[E0599]: no method named `insert_entry` found for type `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>` in the current scope
2019-09-30T12:57:36.4785008Z     --> src/libstd/collections/hash/map.rs:2399:30
2019-09-30T12:57:36.4785562Z      |
2019-09-30T12:57:36.4786072Z 2399 |         let base = self.base.insert_entry(value);
2019-09-30T12:57:36.4786645Z      |                              ^^^^^^^^^^^^ method not found in `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>`
2019-09-30T12:57:37.8707566Z error: aborting due to previous error
2019-09-30T12:57:37.8708183Z 
2019-09-30T12:57:37.8708871Z For more information about this error, try `rustc --explain E0599`.
2019-09-30T12:57:37.9078062Z error: could not compile `std`.
---
2019-09-30T12:57:37.9223137Z == clock drift check ==
2019-09-30T12:57:37.9235497Z   local time: Mon Sep 30 12:57:37 UTC 2019
2019-09-30T12:57:38.2604990Z   network time: Mon, 30 Sep 2019 12:57:38 GMT
2019-09-30T12:57:38.2605889Z == end clock drift check ==
2019-09-30T12:57:41.7513219Z ##[error]Bash exited with code '1'.
2019-09-30T12:57:41.7553456Z ##[section]Starting: Checkout
2019-09-30T12:57:41.7556090Z ==============================================================================
2019-09-30T12:57:41.7556152Z Task         : Get sources
2019-09-30T12:57:41.7556220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
