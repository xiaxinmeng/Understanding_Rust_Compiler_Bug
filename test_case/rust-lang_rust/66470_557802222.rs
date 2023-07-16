plain
2019-11-23T14:16:31.3608577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T14:16:31.3624384Z ##[command]git config gc.auto 0
2019-11-23T14:16:31.3627151Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T14:16:31.3632337Z ##[command]git config --get-all http.proxy
2019-11-23T14:16:31.3638507Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66470/merge:refs/remotes/pull/66470/merge
---
2019-11-23T14:26:30.5478314Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-23T14:27:02.8712977Z error[E0308]: mismatched types
2019-11-23T14:27:02.8713428Z    --> src/librustc/mir/interpret/allocation.rs:340:23
2019-11-23T14:27:02.8713785Z     |
2019-11-23T14:27:02.8714148Z 340 |            .position(|&(l,r)| l == 0 && r == 0) {
2019-11-23T14:27:02.8714839Z     |
2019-11-23T14:27:02.8714839Z     |
2019-11-23T14:27:02.8715168Z     = note: expected type `(&u8, &u8)`
2019-11-23T14:27:02.8715564Z 
2019-11-23T14:27:10.8181221Z error: aborting due to previous error
2019-11-23T14:27:10.8186352Z 
2019-11-23T14:27:10.8191628Z For more information about this error, try `rustc --explain E0308`.
---
2019-11-23T14:27:12.8457549Z   local time: Sat Nov 23 14:27:12 UTC 2019
2019-11-23T14:27:14.0587234Z   network time: Sat, 23 Nov 2019 14:27:13 GMT
2019-11-23T14:27:14.0587917Z == end clock drift check ==
2019-11-23T14:27:14.2361295Z 
2019-11-23T14:27:14.2468024Z ##[error]Bash exited with code '1'.
2019-11-23T14:27:14.2496844Z ##[section]Starting: Checkout
2019-11-23T14:27:14.2498537Z ==============================================================================
2019-11-23T14:27:14.2498609Z Task         : Get sources
2019-11-23T14:27:14.2498656Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
