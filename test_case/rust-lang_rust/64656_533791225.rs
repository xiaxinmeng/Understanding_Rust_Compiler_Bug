plain
2019-09-21T11:31:54.2466283Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T11:31:54.2664221Z ##[command]git config gc.auto 0
2019-09-21T11:31:54.2736007Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T11:31:54.2785343Z ##[command]git config --get-all http.proxy
2019-09-21T11:31:54.2925697Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64656/merge:refs/remotes/pull/64656/merge
---
2019-09-21T11:38:13.5746691Z     Checking backtrace v0.3.37
2019-09-21T11:38:15.3711429Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-21T11:38:15.3736375Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-21T11:38:15.5245754Z     Checking hashbrown v0.5.0
2019-09-21T11:38:17.6918371Z error[E0599]: no method named `insert_and_return` found for type `hashbrown::rustc_entry::RustcVacantEntry<'a, K, V>` in the current scope
2019-09-21T11:38:17.6919879Z     --> src/libstd/collections/hash/map.rs:2402:30
2019-09-21T11:38:17.6920557Z      |
2019-09-21T11:38:17.6921094Z 2402 |         let base = self.base.insert_and_return(value);
2019-09-21T11:38:17.6921878Z 
2019-09-21T11:38:18.9418845Z error: aborting due to previous error
2019-09-21T11:38:18.9420065Z 
2019-09-21T11:38:18.9420704Z For more information about this error, try `rustc --explain E0599`.
---
2019-09-21T11:38:18.9837256Z == clock drift check ==
2019-09-21T11:38:18.9854219Z   local time: Sat Sep 21 11:38:18 UTC 2019
2019-09-21T11:38:19.0710362Z   network time: Sat, 21 Sep 2019 11:38:19 GMT
2019-09-21T11:38:19.0713701Z == end clock drift check ==
2019-09-21T11:38:24.8051745Z ##[error]Bash exited with code '1'.
2019-09-21T11:38:24.8084124Z ##[section]Starting: Checkout
2019-09-21T11:38:24.8086451Z ==============================================================================
2019-09-21T11:38:24.8086507Z Task         : Get sources
2019-09-21T11:38:24.8086572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
