plain
2019-09-30T08:49:34.7490971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T08:49:34.7727710Z ##[command]git config gc.auto 0
2019-09-30T08:49:34.7784813Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T08:49:34.7835297Z ##[command]git config --get-all http.proxy
2019-09-30T08:49:34.7975845Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64656/merge:refs/remotes/pull/64656/merge
---
2019-09-30T08:55:51.6580367Z     Checking backtrace v0.3.37
2019-09-30T08:55:53.4206638Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-30T08:55:53.4986989Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-30T08:55:53.5330549Z     Checking hashbrown v0.5.0
2019-09-30T08:55:56.8664986Z error[E0599]: no method named `insert_and_return` found for type `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>` in the current scope
2019-09-30T08:55:56.8666032Z     --> src/libstd/collections/hash/map.rs:2400:30
2019-09-30T08:55:56.8666406Z      |
2019-09-30T08:55:56.8666819Z 2400 |         let base = self.base.insert_and_return(value);
2019-09-30T08:55:56.8667445Z      |                              ^^^^^^^^^^^^^^^^^ method not found in `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>`
2019-09-30T08:55:58.1192889Z error: aborting due to previous error
2019-09-30T08:55:58.1193047Z 
2019-09-30T08:55:58.1193339Z For more information about this error, try `rustc --explain E0599`.
2019-09-30T08:55:58.1538410Z error: could not compile `std`.
---
2019-09-30T08:55:58.1620872Z == clock drift check ==
2019-09-30T08:55:58.9903892Z   local time: Mon Sep 30 08:55:58 UTC 2019
2019-09-30T08:55:59.1735870Z   network time: Mon, 30 Sep 2019 08:55:59 GMT
2019-09-30T08:55:59.1736605Z == end clock drift check ==
2019-09-30T08:56:06.0860845Z ##[error]Bash exited with code '1'.
2019-09-30T08:56:06.0897580Z ##[section]Starting: Checkout
2019-09-30T08:56:06.0899825Z ==============================================================================
2019-09-30T08:56:06.0899872Z Task         : Get sources
2019-09-30T08:56:06.0899911Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
