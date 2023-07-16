plain
2019-11-20T22:25:46.8668907Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T22:25:46.8865123Z ##[command]git config gc.auto 0
2019-11-20T22:25:46.8973537Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T22:25:46.9067982Z ##[command]git config --get-all http.proxy
2019-11-20T22:25:46.9277939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66590/merge:refs/remotes/pull/66590/merge
---
2019-11-20T22:37:03.8313587Z     |
2019-11-20T22:37:03.8314621Z 532 |             false
2019-11-20T22:37:03.8315094Z     |             ^^^^^ expected enum `std::option::Option`, found bool
2019-11-20T22:37:03.8315330Z     |
2019-11-20T22:37:03.8315653Z     = note: expected type `std::option::Option<&'tcx ty::TyS<'tcx>>`
2019-11-20T22:37:03.8315963Z 
2019-11-20T22:37:10.6203681Z error: aborting due to previous error
2019-11-20T22:37:10.6203784Z 
2019-11-20T22:37:10.6204088Z For more information about this error, try `rustc --explain E0308`.
---
2019-11-20T22:37:18.3653101Z   local time: Wed Nov 20 22:37:18 UTC 2019
2019-11-20T22:37:18.4176308Z   network time: Wed, 20 Nov 2019 22:37:18 GMT
2019-11-20T22:37:18.4176389Z == end clock drift check ==
2019-11-20T22:37:19.3606268Z 
2019-11-20T22:37:19.3746150Z ##[error]Bash exited with code '1'.
2019-11-20T22:37:19.3780073Z ##[section]Starting: Checkout
2019-11-20T22:37:19.3783501Z ==============================================================================
2019-11-20T22:37:19.3783561Z Task         : Get sources
2019-11-20T22:37:19.3783608Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
