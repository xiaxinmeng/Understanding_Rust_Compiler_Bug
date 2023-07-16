plain
2019-10-17T19:15:57.4477331Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-17T19:15:57.4690951Z ##[command]git config gc.auto 0
2019-10-17T19:15:57.4779873Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-17T19:15:57.4858480Z ##[command]git config --get-all http.proxy
2019-10-17T19:15:57.5007156Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-17T19:21:15.5315749Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-17T19:21:15.5342993Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-17T19:21:15.9410605Z    Compiling cc v1.0.35
2019-10-17T19:21:15.9419847Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-10-17T19:21:16.1573718Z error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `;`
2019-10-17T19:21:16.1574182Z    --> src/libcore/future/future.rs:110:44
2019-10-17T19:21:16.1574497Z     |
2019-10-17T19:21:16.1574954Z 110 |     type Future: Future<Output=Self::Output;
2019-10-17T19:21:16.1575460Z     |                                            ^ expected one of 7 possible tokens here
2019-10-17T19:21:16.1575502Z 
2019-10-17T19:21:18.5949610Z error[E0437]: type `Future` is not a member of trait `IntoFuture`
2019-10-17T19:21:18.5950023Z    --> src/libcore/future/future.rs:141:5
2019-10-17T19:21:18.5950549Z 141 |     type Future = F;
2019-10-17T19:21:18.5950549Z 141 |     type Future = F;
2019-10-17T19:21:18.5950845Z     |     ^^^^^^^^^^^^^^^^ not a member of trait `IntoFuture`
2019-10-17T19:21:18.5951210Z 
2019-10-17T19:21:20.2686195Z error[E0220]: associated type `Future` not found for `Self`
2019-10-17T19:21:20.2686545Z    --> src/libcore/future/future.rs:114:29
2019-10-17T19:21:20.2686778Z     |
2019-10-17T19:21:20.2687028Z 114 |     fn into_future(self) -> Self::Future;
2019-10-17T19:21:20.2687614Z     |                             ^^^^^^^^^^^^ associated type `Future` not found
2019-10-17T19:21:20.2687704Z 
2019-10-17T19:21:20.2701708Z error[E0220]: associated type `Future` not found for `Self`
2019-10-17T19:21:20.2701983Z    --> src/libcore/future/future.rs:143:29
2019-10-17T19:21:20.2702482Z 143 |     fn into_future(self) -> Self::Future {
2019-10-17T19:21:20.2702482Z 143 |     fn into_future(self) -> Self::Future {
2019-10-17T19:21:20.2702939Z     |                             ^^^^^^^^^^^^ associated type `Future` not found
2019-10-17T19:21:20.3095309Z error: aborting due to 4 previous errors
2019-10-17T19:21:20.3095436Z 
2019-10-17T19:21:20.3095763Z Some errors have detailed explanations: E0220, E0437.
2019-10-17T19:21:20.3096050Z For more information about an error, try `rustc --explain E0220`.
---
2019-10-17T19:21:22.6390401Z == clock drift check ==
2019-10-17T19:21:22.6410529Z   local time: Thu Oct 17 19:21:22 UTC 2019
2019-10-17T19:21:22.8055193Z   network time: Thu, 17 Oct 2019 19:21:22 GMT
2019-10-17T19:21:22.8067768Z == end clock drift check ==
2019-10-17T19:21:36.9830559Z ##[error]Bash exited with code '1'.
2019-10-17T19:21:36.9886401Z ##[section]Starting: Checkout
2019-10-17T19:21:36.9888565Z ==============================================================================
2019-10-17T19:21:36.9888616Z Task         : Get sources
2019-10-17T19:21:36.9888679Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
