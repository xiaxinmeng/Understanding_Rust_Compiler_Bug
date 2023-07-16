plain
2019-12-16T00:14:58.0757464Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T00:14:58.0987066Z ##[command]git config gc.auto 0
2019-12-16T00:14:58.1068142Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T00:14:58.1118041Z ##[command]git config --get-all http.proxy
2019-12-16T00:14:58.1274726Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67339/merge:refs/remotes/pull/67339/merge
---
2019-12-16T00:20:05.6399267Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2019-12-16T00:20:05.8778309Z error[E0412]: cannot find type `ArcInner` in this scope
2019-12-16T00:20:05.8778951Z    --> src/liballoc/rc.rs:574:25
2019-12-16T00:20:05.8779195Z     |
2019-12-16T00:20:05.8779507Z 574 |         let ptr: *const ArcInner<T> = NonNull::as_ptr(this.ptr);
2019-12-16T00:20:05.8780087Z     |
2019-12-16T00:20:05.8780355Z help: possible candidate is found in another module, you can import it into scope
2019-12-16T00:20:05.8780591Z     |
2019-12-16T00:20:05.8780847Z 233 | use crate::sync::ArcInner;
2019-12-16T00:20:05.8780847Z 233 | use crate::sync::ArcInner;
2019-12-16T00:20:05.8781056Z     |
2019-12-16T00:20:05.8781106Z 
2019-12-16T00:20:06.7801847Z error[E0609]: no field `value` on type `sync::ArcInner<T>`
2019-12-16T00:20:06.7802991Z    --> src/liballoc/sync.rs:559:46
2019-12-16T00:20:06.7803633Z     |
2019-12-16T00:20:06.7804290Z 559 |             let offset = data_offset(&(*ptr).value);
2019-12-16T00:20:06.7806014Z     |
2019-12-16T00:20:06.7806014Z     |
2019-12-16T00:20:06.7806573Z     = note: available fields are: `strong`, `weak`, `data`
2019-12-16T00:20:06.7960020Z error[E0308]: mismatched types
2019-12-16T00:20:06.7960978Z    --> src/liballoc/sync.rs:560:26
2019-12-16T00:20:06.7961396Z     |
2019-12-16T00:20:06.7961396Z     |
2019-12-16T00:20:06.7962040Z 560 |             set_data_ptr(fake_ptr, (ptr as *mut u8).offset(offset))
2019-12-16T00:20:06.7962816Z     |                          ^^^^^^^^ types differ in mutability
2019-12-16T00:20:06.7963666Z     = note: expected type `*mut _`
2019-12-16T00:20:06.7964291Z                found type `*const T`
2019-12-16T00:20:06.7965047Z 
2019-12-16T00:20:07.0941629Z error: aborting due to 3 previous errors
---
2019-12-16T00:20:07.4172035Z   local time: Mon Dec 16 00:20:07 UTC 2019
2019-12-16T00:20:07.5649451Z   network time: Mon, 16 Dec 2019 00:20:07 GMT
2019-12-16T00:20:07.5654909Z == end clock drift check ==
2019-12-16T00:20:16.8858529Z 
2019-12-16T00:20:16.8957202Z ##[error]Bash exited with code '1'.
2019-12-16T00:20:16.8990929Z ##[section]Starting: Checkout
2019-12-16T00:20:16.8992435Z ==============================================================================
2019-12-16T00:20:16.8992497Z Task         : Get sources
2019-12-16T00:20:16.8992535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
