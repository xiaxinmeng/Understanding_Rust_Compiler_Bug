plain
2019-12-18T08:59:14.4253070Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T08:59:14.4451551Z ##[command]git config gc.auto 0
2019-12-18T08:59:14.4515810Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T08:59:14.4561891Z ##[command]git config --get-all http.proxy
2019-12-18T08:59:14.4705627Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67315/merge:refs/remotes/pull/67315/merge
---
2019-12-18T09:04:25.5671258Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-12-18T09:04:25.7394499Z     Checking backtrace v0.3.40
2019-12-18T09:04:27.2508864Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-12-18T09:04:27.2868166Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-18T09:04:29.1767733Z error[E0541]: unknown meta item 'issue'
2019-12-18T09:04:29.1768418Z     |
2019-12-18T09:04:29.1768418Z     |
2019-12-18T09:04:29.1768693Z 128 |     #[rustc_const_unstable(feature = "const_socket_new", issue = "67390")]
2019-12-18T09:04:29.1769043Z     |                                                          ^^^^^^^^^^^^^^^ expected one of `feature`
2019-12-18T09:04:29.1769087Z 
2019-12-18T09:04:29.1769318Z error[E0541]: unknown meta item 'issue'
2019-12-18T09:04:29.1769720Z     |
2019-12-18T09:04:29.1769720Z     |
2019-12-18T09:04:29.1770355Z 276 |     #[rustc_const_unstable(feature = "const_socket_new", issue = "67390")]
2019-12-18T09:04:29.1770819Z     |                                                          ^^^^^^^^^^^^^^^ expected one of `feature`
2019-12-18T09:04:29.1770920Z 
2019-12-18T09:04:29.1776870Z error[E0541]: unknown meta item 'issue'
2019-12-18T09:04:29.1777657Z     |
2019-12-18T09:04:29.1777657Z     |
2019-12-18T09:04:29.1778065Z 369 |     #[rustc_const_unstable(feature = "const_socket_new", issue = "67390")]
2019-12-18T09:04:29.1778526Z     |                                                          ^^^^^^^^^^^^^^^ expected one of `feature`
2019-12-18T09:04:30.8522552Z error: aborting due to 3 previous errors
2019-12-18T09:04:30.8523081Z 
2019-12-18T09:04:30.8523569Z For more information about this error, try `rustc --explain E0541`.
2019-12-18T09:04:30.8883841Z error: could not compile `std`.
---
2019-12-18T09:04:30.8982242Z   local time: Wed Dec 18 09:04:30 UTC 2019
2019-12-18T09:04:31.1753465Z   network time: Wed, 18 Dec 2019 09:04:31 GMT
2019-12-18T09:04:31.1754149Z == end clock drift check ==
2019-12-18T09:04:38.9761385Z 
2019-12-18T09:04:38.9855744Z ##[error]Bash exited with code '1'.
2019-12-18T09:04:38.9879861Z ##[section]Starting: Checkout
2019-12-18T09:04:38.9881653Z ==============================================================================
2019-12-18T09:04:38.9881697Z Task         : Get sources
2019-12-18T09:04:38.9881733Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
