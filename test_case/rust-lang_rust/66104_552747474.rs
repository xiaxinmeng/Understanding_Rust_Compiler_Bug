plain
2019-11-12T05:48:43.8941057Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T05:48:44.4392170Z ##[command]git config gc.auto 0
2019-11-12T05:48:44.4396747Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T05:48:44.4398974Z ##[command]git config --get-all http.proxy
2019-11-12T05:48:44.4404848Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66104/merge:refs/remotes/pull/66104/merge
---
2019-11-12T05:59:05.1218613Z      |
2019-11-12T05:59:05.1218947Z 1183 | ...                   let node_id = self.sess.next_node_id();
2019-11-12T05:59:05.1219376Z      |                                               ^^^^^^^^^^^^ method not found in `&'a session::Session`
2019-11-12T05:59:05.1219611Z      |
2019-11-12T05:59:05.1219975Z      = help: items from traits can only be used if the trait is implemented and in scope
2019-11-12T05:59:05.1220309Z      = note: the following trait defines an item `next_node_id`, perhaps you need to implement it:
2019-11-12T05:59:05.1220571Z              candidate #1: `hir::lowering::Resolver`
2019-11-12T05:59:17.6554572Z error: aborting due to previous error
2019-11-12T05:59:17.6559101Z 
2019-11-12T05:59:17.6565596Z For more information about this error, try `rustc --explain E0599`.
2019-11-12T05:59:17.8230361Z error: could not compile `rustc`.
---
2019-11-12T05:59:22.7322623Z   local time: Tue Nov 12 05:59:22 UTC 2019
2019-11-12T05:59:22.8053015Z   network time: Tue, 12 Nov 2019 05:59:22 GMT
2019-11-12T05:59:22.8053121Z == end clock drift check ==
2019-11-12T05:59:23.8633026Z 
2019-11-12T05:59:23.8738770Z ##[error]Bash exited with code '1'.
2019-11-12T05:59:23.8770389Z ##[section]Starting: Checkout
2019-11-12T05:59:23.8772611Z ==============================================================================
2019-11-12T05:59:23.8772695Z Task         : Get sources
2019-11-12T05:59:23.8772748Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
