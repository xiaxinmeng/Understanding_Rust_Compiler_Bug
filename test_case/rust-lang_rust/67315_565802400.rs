plain
2019-12-15T11:44:34.5115012Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-15T11:44:35.4655442Z ##[command]git config gc.auto 0
2019-12-15T11:44:35.4662368Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-15T11:44:35.4669456Z ##[command]git config --get-all http.proxy
2019-12-15T11:44:35.4677270Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67315/merge:refs/remotes/pull/67315/merge
---
2019-12-15T11:49:36.8972483Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-15T11:49:41.2543519Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-12-15T11:49:41.2544847Z    --> src/libstd/net/addr.rs:130:13
2019-12-15T11:49:41.2545322Z     |
2019-12-15T11:49:41.2545909Z 130 |             IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
2019-12-15T11:49:41.2546843Z     |
2019-12-15T11:49:41.2546843Z     |
2019-12-15T11:49:41.2547445Z     = note: for more information, see issue ***/issues/57563
2019-12-15T11:49:41.2548171Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-15T11:49:41.2548577Z 
2019-12-15T11:49:41.2563242Z error[E0723]: can only call other `const fn` within a `const fn`, but `const net::hton::<u16>` is not stable as `const fn`
2019-12-15T11:49:41.2564537Z     |
2019-12-15T11:49:41.2564537Z     |
2019-12-15T11:49:41.2565089Z 279 |                 sin_port: hton(port),
2019-12-15T11:49:41.2566107Z     |
2019-12-15T11:49:41.2566107Z     |
2019-12-15T11:49:41.2566885Z     = note: for more information, see issue ***/issues/57563
2019-12-15T11:49:41.2567434Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-15T11:49:41.2571144Z 
2019-12-15T11:49:41.2578426Z error[E0723]: can only call other `const fn` within a `const fn`, but `const net::hton::<u16>` is not stable as `const fn`
2019-12-15T11:49:41.2581376Z     |
2019-12-15T11:49:41.2581376Z     |
2019-12-15T11:49:41.2583718Z 371 |                 sin6_port: hton(port),
2019-12-15T11:49:41.2585997Z     |
2019-12-15T11:49:41.2585997Z     |
2019-12-15T11:49:41.2587354Z     = note: for more information, see issue ***/issues/57563
2019-12-15T11:49:41.2588686Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-15T11:49:42.0212674Z error: aborting due to 3 previous errors
2019-12-15T11:49:42.0212761Z 
2019-12-15T11:49:42.0213032Z For more information about this error, try `rustc --explain E0723`.
2019-12-15T11:49:42.0670601Z error: could not compile `std`.
---
2019-12-15T11:49:42.0796053Z   local time: Sun Dec 15 11:49:42 UTC 2019
2019-12-15T11:49:42.3561560Z   network time: Sun, 15 Dec 2019 11:49:42 GMT
2019-12-15T11:49:42.3561743Z == end clock drift check ==
2019-12-15T11:49:49.2429295Z 
2019-12-15T11:49:49.2536789Z ##[error]Bash exited with code '1'.
2019-12-15T11:49:49.2560521Z ##[section]Starting: Checkout
2019-12-15T11:49:49.2561989Z ==============================================================================
2019-12-15T11:49:49.2562194Z Task         : Get sources
2019-12-15T11:49:49.2562249Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
