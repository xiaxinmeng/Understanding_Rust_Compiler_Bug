plain
2019-12-30T12:27:03.9850601Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T12:27:03.9866578Z ##[command]git config gc.auto 0
2019-12-30T12:27:03.9871612Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T12:27:03.9876519Z ##[command]git config --get-all http.proxy
2019-12-30T12:27:03.9882510Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67686/merge:refs/remotes/pull/67686/merge
---
2019-12-30T12:32:31.6718325Z     Checking backtrace v0.3.40
2019-12-30T12:32:31.7593053Z error[E0107]: wrong number of type arguments: expected 2, found 3
2019-12-30T12:32:31.7593498Z    --> src/liballoc/collections/btree/node.rs:535:46
2019-12-30T12:32:31.7593745Z     |
2019-12-30T12:32:31.7594093Z 535 |         if (mem::align_of::<NodeHeader<K, V, K>>() > mem::align_of::<NodeHeader<K, V>>()
2019-12-30T12:32:31.7599073Z 
2019-12-30T12:32:31.7619798Z error[E0107]: wrong number of type arguments: expected 2, found 3
2019-12-30T12:32:31.7620170Z    --> src/liballoc/collections/btree/node.rs:536:48
2019-12-30T12:32:31.7620409Z     |
2019-12-30T12:32:31.7620409Z     |
2019-12-30T12:32:31.7620776Z 536 |             || mem::size_of::<NodeHeader<K, V, K>>() != mem::size_of::<NodeHeader<K, V>>())
2019-12-30T12:32:31.7621182Z 
2019-12-30T12:32:32.3157566Z error: aborting due to 2 previous errors
2019-12-30T12:32:32.3157966Z 
2019-12-30T12:32:32.3159124Z For more information about this error, try `rustc --explain E0107`.
---
2019-12-30T12:32:32.3365773Z   local time: Mon Dec 30 12:32:32 UTC 2019
2019-12-30T12:32:32.5990703Z   network time: Mon, 30 Dec 2019 12:32:32 GMT
2019-12-30T12:32:32.5998374Z == end clock drift check ==
2019-12-30T12:32:39.4825986Z 
2019-12-30T12:32:39.4953460Z ##[error]Bash exited with code '1'.
2019-12-30T12:32:39.4992546Z ##[section]Starting: Checkout
2019-12-30T12:32:39.4994350Z ==============================================================================
2019-12-30T12:32:39.4994410Z Task         : Get sources
2019-12-30T12:32:39.4994459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
