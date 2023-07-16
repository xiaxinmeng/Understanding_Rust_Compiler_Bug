plain
2019-11-19T18:15:19.4692877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T18:15:19.4881260Z ##[command]git config gc.auto 0
2019-11-19T18:15:19.4972692Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T18:15:19.5039320Z ##[command]git config --get-all http.proxy
2019-11-19T18:15:19.5180861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66549/merge:refs/remotes/pull/66549/merge
---
2019-11-19T18:21:45.4864684Z Done!
2019-11-19T18:21:45.4880227Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T18:21:45.9312881Z    Compiling cc v1.0.47
2019-11-19T18:21:45.9313272Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-11-19T18:21:50.6761757Z error[E0119]: conflicting implementations of trait `cmp::PartialOrd<&mut _>` for type `&mut _`:
2019-11-19T18:21:50.6763026Z     --> src/libcore/cmp.rs:1229:5
2019-11-19T18:21:50.6763679Z      |
2019-11-19T18:21:50.6764551Z 1197 |     impl<A: ?Sized, B: ?Sized> PartialOrd<&mut B> for &mut A where A: PartialOrd<B> {
2019-11-19T18:21:50.6765581Z      |     ------------------------------------------------------------------------------- first implementation here
2019-11-19T18:21:50.6766089Z ...
2019-11-19T18:21:50.6766671Z 1229 |     impl<A: ?Sized, B: ?Sized> PartialOrd<&mut  B> for &mut A where A: PartialOrd<B> {
2019-11-19T18:21:50.6767354Z      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
2019-11-19T18:21:50.8027141Z error: aborting due to previous error
2019-11-19T18:21:50.8027238Z 
2019-11-19T18:21:50.8027736Z For more information about this error, try `rustc --explain E0119`.
2019-11-19T18:21:50.8900689Z error: could not compile `core`.
---
2019-11-19T18:21:52.2103065Z   local time: Tue Nov 19 18:21:52 UTC 2019
2019-11-19T18:21:52.4936870Z   network time: Tue, 19 Nov 2019 18:21:52 GMT
2019-11-19T18:21:52.4943512Z == end clock drift check ==
2019-11-19T18:21:53.7313201Z 
2019-11-19T18:21:53.7413855Z ##[error]Bash exited with code '1'.
2019-11-19T18:21:53.7442353Z ##[section]Starting: Checkout
2019-11-19T18:21:53.7456836Z ==============================================================================
2019-11-19T18:21:53.7456901Z Task         : Get sources
2019-11-19T18:21:53.7456969Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
