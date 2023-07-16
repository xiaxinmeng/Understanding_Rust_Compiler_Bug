plain
2020-01-14T21:26:01.8773583Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T21:26:01.8854327Z ##[command]git config gc.auto 0
2020-01-14T21:26:01.8938106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T21:26:01.8985603Z ##[command]git config --get-all http.proxy
2020-01-14T21:26:01.9129514Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68198/merge:refs/remotes/pull/68198/merge
---
2020-01-14T21:30:18.3981027Z     |
2020-01-14T21:30:18.3981308Z 460 |         ManuallyDrop::new(self);
2020-01-14T21:30:18.3981685Z     |         ^^^^^^^^^^^^ use of undeclared type or module `ManuallyDrop`
2020-01-14T21:30:18.3985641Z 
2020-01-14T21:30:20.0019123Z error[E0609]: no field `0` on type `&mut lazy::Once<T>`
2020-01-14T21:30:20.0019462Z    --> src/libstd/lazy.rs:474:17
2020-01-14T21:30:20.0019953Z 474 |         if self.0.is_initialized() {
2020-01-14T21:30:20.0020532Z     |                 ^ unknown field
2020-01-14T21:30:20.0020767Z     |
2020-01-14T21:30:20.0020767Z     |
2020-01-14T21:30:20.0021134Z     = note: available fields are: `state_and_queue`, `_marker`, `value`
2020-01-14T21:30:20.4741862Z error: aborting due to 2 previous errors
2020-01-14T21:30:20.4741975Z 
2020-01-14T21:30:20.4742251Z Some errors have detailed explanations: E0433, E0609.
2020-01-14T21:30:20.4742510Z For more information about an error, try `rustc --explain E0433`.
---
2020-01-14T21:30:20.4938859Z   local time: Tue Jan 14 21:30:20 UTC 2020
2020-01-14T21:30:20.5332662Z   network time: Tue, 14 Jan 2020 21:30:20 GMT
2020-01-14T21:30:20.5332785Z == end clock drift check ==
2020-01-14T21:30:21.4880527Z 
2020-01-14T21:30:21.4977401Z ##[error]Bash exited with code '1'.
2020-01-14T21:30:21.5007258Z ##[section]Starting: Checkout
2020-01-14T21:30:21.5008941Z ==============================================================================
2020-01-14T21:30:21.5009000Z Task         : Get sources
2020-01-14T21:30:21.5009072Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
