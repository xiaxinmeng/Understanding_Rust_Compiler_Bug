plain
2019-11-12T02:55:51.5195173Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T02:55:51.5403722Z ##[command]git config gc.auto 0
2019-11-12T02:55:51.5475762Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T02:55:51.5549834Z ##[command]git config --get-all http.proxy
2019-11-12T02:55:51.5683693Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66104/merge:refs/remotes/pull/66104/merge
---
2019-11-12T03:06:29.3048553Z      |
2019-11-12T03:06:29.3048918Z 1183 | ...                   let node_id = self.sess.next_node_id();
2019-11-12T03:06:29.3049333Z      |                                               ^^^^^^^^^^^^ method not found in `&'a session::Session`
2019-11-12T03:06:29.3049587Z      |
2019-11-12T03:06:29.3050073Z      = help: items from traits can only be used if the trait is implemented and in scope
2019-11-12T03:06:29.3050383Z      = note: the following trait defines an item `next_node_id`, perhaps you need to implement it:
2019-11-12T03:06:29.3050610Z              candidate #1: `hir::lowering::Resolver`
2019-11-12T03:06:41.7255823Z error: aborting due to previous error
2019-11-12T03:06:41.7257403Z 
2019-11-12T03:06:41.7258504Z For more information about this error, try `rustc --explain E0599`.
2019-11-12T03:06:41.8788653Z error: could not compile `rustc`.
---
2019-11-12T03:06:47.0321850Z   local time: Tue Nov 12 03:06:47 UTC 2019
2019-11-12T03:06:47.1185108Z   network time: Tue, 12 Nov 2019 03:06:47 GMT
2019-11-12T03:06:47.1185229Z == end clock drift check ==
2019-11-12T03:06:48.1766653Z 
2019-11-12T03:06:48.1880383Z ##[error]Bash exited with code '1'.
2019-11-12T03:06:48.1908605Z ##[section]Starting: Checkout
2019-11-12T03:06:48.1910277Z ==============================================================================
2019-11-12T03:06:48.1910322Z Task         : Get sources
2019-11-12T03:06:48.1910361Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
