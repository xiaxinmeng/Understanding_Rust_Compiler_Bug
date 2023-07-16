plain
2019-09-30T09:35:54.4767292Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T09:35:54.4783140Z ##[command]git config gc.auto 0
2019-09-30T09:35:54.4784829Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T09:35:54.4788585Z ##[command]git config --get-all http.proxy
2019-09-30T09:35:54.4793140Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64656/merge:refs/remotes/pull/64656/merge
---
2019-09-30T09:42:02.2843202Z     Checking backtrace v0.3.37
2019-09-30T09:42:04.0872648Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-30T09:42:04.1645854Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-30T09:42:04.1986093Z     Checking hashbrown v0.5.0
2019-09-30T09:42:06.4237412Z error[E0599]: no method named `insert_entry` found for type `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>` in the current scope
2019-09-30T09:42:06.4238821Z     --> src/libstd/collections/hash/map.rs:2398:30
2019-09-30T09:42:06.4239396Z      |
2019-09-30T09:42:06.4240180Z 2398 |         let base = self.base.insert_entry(value);
2019-09-30T09:42:06.4240607Z      |                              ^^^^^^^^^^^^ method not found in `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>`
2019-09-30T09:42:07.5811473Z error: aborting due to previous error
2019-09-30T09:42:07.5811571Z 
2019-09-30T09:42:07.5811835Z For more information about this error, try `rustc --explain E0599`.
2019-09-30T09:42:07.6147053Z error: could not compile `std`.
---
2019-09-30T09:42:07.6224768Z == clock drift check ==
2019-09-30T09:42:07.6242888Z   local time: Mon Sep 30 09:42:07 UTC 2019
2019-09-30T09:42:07.7752571Z   network time: Mon, 30 Sep 2019 09:42:07 GMT
2019-09-30T09:42:07.7757194Z == end clock drift check ==
2019-09-30T09:42:09.6879512Z ##[error]Bash exited with code '1'.
2019-09-30T09:42:09.6914124Z ##[section]Starting: Checkout
2019-09-30T09:42:09.6915554Z ==============================================================================
2019-09-30T09:42:09.6915599Z Task         : Get sources
2019-09-30T09:42:09.6915635Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
