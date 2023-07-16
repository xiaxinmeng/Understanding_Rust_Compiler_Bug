plain
2019-11-16T00:19:43.6866540Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T00:19:43.7046411Z ##[command]git config gc.auto 0
2019-11-16T00:19:43.7130307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T00:19:44.2690503Z ##[command]git config --get-all http.proxy
2019-11-16T00:19:44.2693616Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66461/merge:refs/remotes/pull/66461/merge
---
2019-11-16T00:32:45.6421018Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-16T00:32:46.2825771Z error[E0425]: cannot find value `E0641` in this scope
2019-11-16T00:32:46.2827017Z    --> src/librustc_typeck/check/cast.rs:316:73
2019-11-16T00:32:46.2827644Z     |
2019-11-16T00:32:46.2828294Z 316 |                 let mut err = struct_span_err!(fcx.tcx.sess, self.span, E0641,
2019-11-16T00:32:46.2829040Z     |                                                                         ^^^^^ help: a constant with a similar name exists: `E0541`
2019-11-16T00:32:49.9098824Z error: aborting due to previous error
2019-11-16T00:32:49.9103277Z 
2019-11-16T00:32:49.9112835Z For more information about this error, try `rustc --explain E0425`.
2019-11-16T00:32:49.9568717Z error: could not compile `rustc_typeck`.
---
2019-11-16T00:36:28.8942862Z   local time: Sat Nov 16 00:36:28 UTC 2019
2019-11-16T00:36:29.1738211Z   network time: Sat, 16 Nov 2019 00:36:29 GMT
2019-11-16T00:36:29.1747619Z == end clock drift check ==
2019-11-16T00:36:31.9676234Z 
2019-11-16T00:36:31.9774881Z ##[error]Bash exited with code '1'.
2019-11-16T00:36:31.9805135Z ##[section]Starting: Checkout
2019-11-16T00:36:31.9807003Z ==============================================================================
2019-11-16T00:36:31.9807482Z Task         : Get sources
2019-11-16T00:36:31.9807555Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
