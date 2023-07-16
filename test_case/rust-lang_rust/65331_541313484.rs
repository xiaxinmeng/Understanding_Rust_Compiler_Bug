plain
2019-10-12T10:46:32.4077856Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T10:46:32.4161885Z ##[command]git config gc.auto 0
2019-10-12T10:46:32.4244545Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T10:46:32.4328393Z ##[command]git config --get-all http.proxy
2019-10-12T10:46:32.4467313Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65331/merge:refs/remotes/pull/65331/merge
---
2019-10-12T10:51:50.2216512Z     Checking backtrace v0.3.37
2019-10-12T10:51:51.4588590Z error[E0507]: cannot move out of a shared reference
2019-10-12T10:51:51.4589089Z     --> src/liballoc/collections/btree/map.rs:1766:43
2019-10-12T10:51:51.4589334Z      |
2019-10-12T10:51:51.4589666Z 1766 |         self.extend(iter.into_iter().map(|(&key, &value)| (key.clone(), value.clone())));
2019-10-12T10:51:51.4590039Z      |                                           ^^---^^^^^^^^^
2019-10-12T10:51:51.4590642Z      |                                             data moved here
2019-10-12T10:51:51.4590642Z      |                                             data moved here
2019-10-12T10:51:51.4591084Z      |                                             move occurs because `key` has type `K`, which does not implement the `Copy` trait
2019-10-12T10:51:51.4591848Z error[E0507]: cannot move out of a shared reference
2019-10-12T10:51:51.4592217Z     --> src/liballoc/collections/btree/map.rs:1766:43
2019-10-12T10:51:51.4592440Z      |
2019-10-12T10:51:51.4592440Z      |
2019-10-12T10:51:51.4592771Z 1766 |         self.extend(iter.into_iter().map(|(&key, &value)| (key.clone(), value.clone())));
2019-10-12T10:51:51.4593460Z      |                                                   |
2019-10-12T10:51:51.4593798Z      |                                                   data moved here
2019-10-12T10:51:51.4593798Z      |                                                   data moved here
2019-10-12T10:51:51.4594362Z      |                                                   move occurs because `value` has type `V`, which does not implement the `Copy` trait
2019-10-12T10:51:52.0041920Z error: aborting due to 2 previous errors
2019-10-12T10:51:52.0042047Z 
2019-10-12T10:51:52.0042386Z For more information about this error, try `rustc --explain E0507`.
2019-10-12T10:51:52.0304569Z error: could not compile `alloc`.
---
2019-10-12T10:51:52.0388921Z == clock drift check ==
2019-10-12T10:51:52.0408478Z   local time: Sat Oct 12 10:51:52 UTC 2019
2019-10-12T10:51:52.1902685Z   network time: Sat, 12 Oct 2019 10:51:52 GMT
2019-10-12T10:51:52.1905801Z == end clock drift check ==
2019-10-12T10:51:53.1873022Z ##[error]Bash exited with code '1'.
2019-10-12T10:51:53.1915716Z ##[section]Starting: Checkout
2019-10-12T10:51:53.1917568Z ==============================================================================
2019-10-12T10:51:53.1917645Z Task         : Get sources
2019-10-12T10:51:53.1917694Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
