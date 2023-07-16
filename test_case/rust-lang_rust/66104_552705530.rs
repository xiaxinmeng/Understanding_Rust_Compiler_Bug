plain
2019-11-12T02:16:46.3004959Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T02:16:46.3210629Z ##[command]git config gc.auto 0
2019-11-12T02:16:46.3293207Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T02:16:46.3352264Z ##[command]git config --get-all http.proxy
2019-11-12T02:16:47.2218375Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66104/merge:refs/remotes/pull/66104/merge
---
2019-11-12T02:27:23.0096163Z      |
2019-11-12T02:27:23.0096664Z 1183 | ...                   let node_id = self.sess.next_node_id();
2019-11-12T02:27:23.0097506Z      |                                               ^^^^^^^^^^^^ method not found in `&'a session::Session`
2019-11-12T02:27:23.0097977Z      |
2019-11-12T02:27:23.0098464Z      = help: items from traits can only be used if the trait is implemented and in scope
2019-11-12T02:27:23.0098939Z      = note: the following trait defines an item `next_node_id`, perhaps you need to implement it:
2019-11-12T02:27:23.0099367Z              candidate #1: `hir::lowering::Resolver`
2019-11-12T02:27:35.7738498Z error: aborting due to previous error
2019-11-12T02:27:35.7740732Z 
2019-11-12T02:27:35.7741560Z For more information about this error, try `rustc --explain E0599`.
2019-11-12T02:27:35.9378778Z error: could not compile `rustc`.
---
2019-11-12T02:27:41.6686661Z   local time: Tue Nov 12 02:27:41 UTC 2019
2019-11-12T02:27:41.6972523Z   network time: Tue, 12 Nov 2019 02:27:41 GMT
2019-11-12T02:27:41.6978563Z == end clock drift check ==
2019-11-12T02:27:42.6574530Z 
2019-11-12T02:27:42.6713638Z ##[error]Bash exited with code '1'.
2019-11-12T02:27:42.6738566Z ##[section]Starting: Checkout
2019-11-12T02:27:42.6740515Z ==============================================================================
2019-11-12T02:27:42.6740571Z Task         : Get sources
2019-11-12T02:27:42.6740638Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
