plain
2019-08-20T19:20:07.0651330Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T19:20:07.0864563Z ##[command]git config gc.auto 0
2019-08-20T19:20:07.1299588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T19:20:07.1362515Z ##[command]git config --get-all http.proxy
2019-08-20T19:20:07.1516728Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63756/merge:refs/remotes/pull/63756/merge
---
2019-08-20T19:20:41.2725184Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T19:20:41.2725230Z 
2019-08-20T19:20:41.2725436Z   git checkout -b <new-branch-name>
2019-08-20T19:20:41.2725475Z 
2019-08-20T19:20:41.2725522Z HEAD is now at 5ca57c7a4 Merge 80d345747ebb3d890aa08021b969cd24fd7cc0fa into 5a56e05abd34e1936df74625c1f40cb6fee0cd4a
2019-08-20T19:20:41.2892821Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T19:20:41.2895528Z ==============================================================================
2019-08-20T19:20:41.2895586Z Task         : Bash
2019-08-20T19:20:41.2895632Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T19:30:46.9097528Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-08-20T19:30:50.0986540Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-20T19:30:57.1455584Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-20T19:31:12.5147916Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-20T19:31:35.6448390Z error[E0599]: no method named `borrow` found for type `dep_graph::graph::CurrentDepGraph` in the current scope
2019-08-20T19:31:35.6448944Z    --> src/librustc/dep_graph/graph.rs:612:41
2019-08-20T19:31:35.6449175Z     |
2019-08-20T19:31:35.6449528Z 612 |             debug_assert!(!data.current.borrow().node_to_node_index.contains_key(dep_node));
2019-08-20T19:31:35.6450023Z ...
2019-08-20T19:31:35.6450023Z ...
2019-08-20T19:31:35.6450324Z 942 | pub(super) struct CurrentDepGraph {
2019-08-20T19:31:35.6450655Z     | --------------------------------- method `borrow` not found for this
2019-08-20T19:31:35.6461201Z     |
2019-08-20T19:31:35.6461539Z     = help: items from traits can only be used if the trait is in scope
2019-08-20T19:31:35.6461860Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-08-20T19:31:35.6462102Z             `use std::borrow::Borrow;`
2019-08-20T19:31:50.7942952Z error: aborting due to previous error
2019-08-20T19:31:50.7946791Z 
2019-08-20T19:31:50.7956209Z For more information about this error, try `rustc --explain E0599`.
2019-08-20T19:31:50.9721881Z error: Could not compile `rustc`.
---
2019-08-20T19:32:39.4985343Z == clock drift check ==
2019-08-20T19:32:39.5007344Z   local time: Tue Aug 20 19:32:39 UTC 2019
2019-08-20T19:32:39.6574654Z   network time: Tue, 20 Aug 2019 19:32:39 GMT
2019-08-20T19:32:39.6575475Z == end clock drift check ==
2019-08-20T19:32:40.7149248Z ##[error]Bash exited with code '1'.
2019-08-20T19:32:40.7189082Z ##[section]Starting: Checkout
2019-08-20T19:32:40.7191497Z ==============================================================================
2019-08-20T19:32:40.7191574Z Task         : Get sources
2019-08-20T19:32:40.7191645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
