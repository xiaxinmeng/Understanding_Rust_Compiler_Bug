plain
2019-10-12T10:59:23.9720162Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T10:59:23.9737706Z ##[command]git config gc.auto 0
2019-10-12T10:59:23.9743024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T10:59:23.9746918Z ##[command]git config --get-all http.proxy
2019-10-12T10:59:23.9752807Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65331/merge:refs/remotes/pull/65331/merge
---
2019-10-12T11:05:00.5455230Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-12T11:05:03.4322003Z error[E0599]: no method named `cloned` found for type `T` in the current scope
2019-10-12T11:05:03.4324550Z     --> src/libstd/collections/hash/map.rs:2435:31
2019-10-12T11:05:03.4325512Z      |
2019-10-12T11:05:03.4326718Z 2435 |         self.base.extend(iter.cloned())
2019-10-12T11:05:03.4327280Z      |                               ^^^^^^ method not found in `T`
2019-10-12T11:05:03.4327896Z      = note: the method `cloned` exists but the following trait bounds were not satisfied:
2019-10-12T11:05:03.4327896Z      = note: the method `cloned` exists but the following trait bounds were not satisfied:
2019-10-12T11:05:03.4328133Z              `&mut T : core::iter::Iterator`
2019-10-12T11:05:03.4328524Z      = help: items from traits can only be used if the type parameter is bounded by the trait
2019-10-12T11:05:03.4328848Z help: the following trait defines an item `cloned`, perhaps you need to restrict type parameter `T` with it:
2019-10-12T11:05:03.4329270Z      |
2019-10-12T11:05:03.4329609Z 2434 |     fn extend<T: core::iter::Iterator + IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
2019-10-12T11:05:03.4329932Z 
2019-10-12T11:05:04.9136817Z error: aborting due to previous error
2019-10-12T11:05:04.9136938Z 
2019-10-12T11:05:04.9137292Z For more information about this error, try `rustc --explain E0599`.
---
2019-10-12T11:05:04.9651627Z == clock drift check ==
2019-10-12T11:05:04.9676728Z   local time: Sat Oct 12 11:05:04 UTC 2019
2019-10-12T11:05:05.0573289Z   network time: Sat, 12 Oct 2019 11:05:05 GMT
2019-10-12T11:05:05.0573483Z == end clock drift check ==
2019-10-12T11:05:05.9299864Z ##[error]Bash exited with code '1'.
2019-10-12T11:05:05.9344776Z ##[section]Starting: Checkout
2019-10-12T11:05:05.9346883Z ==============================================================================
2019-10-12T11:05:05.9346957Z Task         : Get sources
2019-10-12T11:05:05.9347004Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
