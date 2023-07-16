plain
2019-10-07T15:16:42.8248394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T15:16:42.8492386Z ##[command]git config gc.auto 0
2019-10-07T15:16:42.8557408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T15:16:42.8615101Z ##[command]git config --get-all http.proxy
2019-10-07T15:16:42.8754965Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65182/merge:refs/remotes/pull/65182/merge
---
2019-10-07T15:24:57.9944889Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-07T15:24:59.5047100Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-07T15:25:00.9008657Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-07T15:25:14.5751907Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-07T15:25:18.2847042Z error[E0425]: cannot find value `is_reify_shim` in this scope
2019-10-07T15:25:18.2847421Z    --> src/librustc/ty/instance.rs:311:12
2019-10-07T15:25:18.2848288Z 311 |         if is_reify_shim {
2019-10-07T15:25:18.2848605Z     |            ^^^^^^^^^^^^^ not found in this scope
2019-10-07T15:25:18.2848654Z 
2019-10-07T15:25:41.2371026Z error: aborting due to previous error
---
2019-10-07T15:25:41.4249437Z == clock drift check ==
2019-10-07T15:25:41.4281914Z   local time: Mon Oct  7 15:25:41 UTC 2019
2019-10-07T15:25:41.5166941Z   network time: Mon, 07 Oct 2019 15:25:41 GMT
2019-10-07T15:25:41.5169272Z == end clock drift check ==
2019-10-07T15:25:42.2454013Z ##[error]Bash exited with code '1'.
2019-10-07T15:25:42.2499493Z ##[section]Starting: Checkout
2019-10-07T15:25:42.2501229Z ==============================================================================
2019-10-07T15:25:42.2501300Z Task         : Get sources
2019-10-07T15:25:42.2501365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
