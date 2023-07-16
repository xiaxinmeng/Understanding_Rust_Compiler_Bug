plain
2019-10-12T19:43:15.3873268Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T19:43:15.3980735Z ##[command]git config gc.auto 0
2019-10-12T19:43:15.4046268Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T19:43:15.4101245Z ##[command]git config --get-all http.proxy
2019-10-12T19:43:15.4238147Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65331/merge:refs/remotes/pull/65331/merge
---
2019-10-12T19:49:02.8075491Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-12T19:49:05.2374083Z error[E0599]: no method named `cloned` found for type `T` in the current scope
2019-10-12T19:49:05.2375854Z     --> src/libstd/collections/hash/map.rs:2435:31
2019-10-12T19:49:05.2376158Z      |
2019-10-12T19:49:05.2376771Z 2435 |         self.base.extend(iter.cloned())
2019-10-12T19:49:05.2377065Z      |                               ^^^^^^ method not found in `T`
2019-10-12T19:49:05.2377551Z      = note: the method `cloned` exists but the following trait bounds were not satisfied:
2019-10-12T19:49:05.2377551Z      = note: the method `cloned` exists but the following trait bounds were not satisfied:
2019-10-12T19:49:05.2378032Z              `&mut T : core::iter::Iterator`
2019-10-12T19:49:05.2378398Z      = help: items from traits can only be used if the type parameter is bounded by the trait
2019-10-12T19:49:05.2378676Z help: the following trait defines an item `cloned`, perhaps you need to restrict type parameter `T` with it:
2019-10-12T19:49:05.2379024Z      |
2019-10-12T19:49:05.2381662Z 2434 |     fn extend<T: core::iter::Iterator + IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
2019-10-12T19:49:05.2382515Z 
2019-10-12T19:49:06.5642083Z error: aborting due to previous error
2019-10-12T19:49:06.5642896Z 
2019-10-12T19:49:06.5643562Z For more information about this error, try `rustc --explain E0599`.
---
2019-10-12T19:49:06.6089719Z == clock drift check ==
2019-10-12T19:49:06.6107562Z   local time: Sat Oct 12 19:49:06 UTC 2019
2019-10-12T19:49:06.7610721Z   network time: Sat, 12 Oct 2019 19:49:06 GMT
2019-10-12T19:49:06.7617364Z == end clock drift check ==
2019-10-12T19:49:07.9916693Z ##[error]Bash exited with code '1'.
2019-10-12T19:49:07.9959564Z ##[section]Starting: Checkout
2019-10-12T19:49:07.9960993Z ==============================================================================
2019-10-12T19:49:07.9961036Z Task         : Get sources
2019-10-12T19:49:07.9961077Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
