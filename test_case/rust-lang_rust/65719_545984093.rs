plain
2019-10-24T15:47:08.2751687Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:47:08.9890615Z ##[command]git config gc.auto 0
2019-10-24T15:47:08.9896207Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:47:08.9898624Z ##[command]git config --get-all http.proxy
2019-10-24T15:47:08.9903284Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65719/merge:refs/remotes/pull/65719/merge
---
2019-10-24T15:54:29.9707467Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-24T15:54:30.1286097Z    Compiling backtrace v0.3.37
2019-10-24T15:54:30.6239718Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-10-24T15:54:30.7424519Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-24T15:54:36.1413315Z error[E0594]: cannot assign to `node.next`, as `node` is not declared as mutable
2019-10-24T15:54:36.1415242Z     |
2019-10-24T15:54:36.1415242Z     |
2019-10-24T15:54:36.1415800Z 438 |     let node = Waiter {
2019-10-24T15:54:36.1416772Z     |         ---- help: consider changing this to be mutable: `mut node`
2019-10-24T15:54:36.1417306Z ...
2019-10-24T15:54:36.1417932Z 456 |         node.next = (old_head_and_status & !STATE_MASK) as *const Waiter;
2019-10-24T15:54:36.1418865Z 
2019-10-24T15:54:36.7169759Z error: aborting due to previous error
2019-10-24T15:54:36.7169852Z 
2019-10-24T15:54:36.7690610Z error: could not compile `std`.
---
2019-10-24T15:54:36.7810513Z   local time: Thu Oct 24 15:54:36 UTC 2019
2019-10-24T15:54:36.9341968Z   network time: Thu, 24 Oct 2019 15:54:36 GMT
2019-10-24T15:54:36.9346727Z == end clock drift check ==
2019-10-24T15:54:39.8247361Z 
2019-10-24T15:54:39.8367738Z ##[error]Bash exited with code '1'.
2019-10-24T15:54:39.8403925Z ##[section]Starting: Checkout
2019-10-24T15:54:39.8406148Z ==============================================================================
2019-10-24T15:54:39.8406268Z Task         : Get sources
2019-10-24T15:54:39.8406319Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
