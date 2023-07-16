plain
2019-09-29T16:16:03.0765144Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T16:16:03.9308289Z ##[command]git config gc.auto 0
2019-09-29T16:16:03.9315403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T16:16:03.9318231Z ##[command]git config --get-all http.proxy
2019-09-29T16:16:03.9325183Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-29T16:25:56.4042948Z 
2019-09-29T16:25:57.5749384Z error[E0599]: no function or associated item named `new` found for type `rustc::mir::Local` in the current scope
2019-09-29T16:25:57.5750635Z    --> src/librustc_codegen_ssa/mir/block.rs:232:43
2019-09-29T16:25:57.5750916Z     |
2019-09-29T16:25:57.5751237Z 232 |             match self.locals[mir::Local::new(1 + va_list_arg_idx)] {
2019-09-29T16:25:57.5751674Z     |                                           ^^^ function or associated item not found in `rustc::mir::Local`
2019-09-29T16:25:57.5752214Z     = help: items from traits can only be used if the trait is in scope
2019-09-29T16:25:57.5752214Z     = help: items from traits can only be used if the trait is in scope
2019-09-29T16:25:57.5752869Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-09-29T16:25:57.5753114Z             `use rustc_index::vec::Idx;`
2019-09-29T16:25:58.9907442Z error: aborting due to 2 previous errors
2019-09-29T16:25:58.9910973Z 
2019-09-29T16:25:58.9922871Z Some errors have detailed explanations: E0432, E0599.
2019-09-29T16:25:58.9932416Z For more information about an error, try `rustc --explain E0432`.
---
2019-09-29T16:26:05.5367857Z == clock drift check ==
2019-09-29T16:26:05.5388208Z   local time: Sun Sep 29 16:26:05 UTC 2019
2019-09-29T16:26:05.7027017Z   network time: Sun, 29 Sep 2019 16:26:05 GMT
2019-09-29T16:26:05.7030562Z == end clock drift check ==
2019-09-29T16:26:07.0126898Z ##[error]Bash exited with code '1'.
2019-09-29T16:26:07.0164760Z ##[section]Starting: Checkout
2019-09-29T16:26:07.0166707Z ==============================================================================
2019-09-29T16:26:07.0166769Z Task         : Get sources
2019-09-29T16:26:07.0166831Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
