plain
2019-10-27T05:07:20.7638680Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T05:07:20.7848131Z ##[command]git config gc.auto 0
2019-10-27T05:07:20.7928338Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T05:07:20.7994220Z ##[command]git config --get-all http.proxy
2019-10-27T05:07:20.8164098Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-27T05:15:03.1243038Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-27T05:15:14.4943101Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-27T05:15:16.3141064Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-27T05:15:18.6604171Z error[E0432]: unresolved import `syntax_pos::symbol::InternedString`
2019-10-27T05:15:18.6604612Z   --> src/librustc/ty/query/keys.rs:12:26
2019-10-27T05:15:18.6605238Z 12 | use syntax_pos::symbol::{InternedString, Symbol};
2019-10-27T05:15:18.6605656Z    |                          ^^^^^^^^^^^^^^ no `InternedString` in `symbol`
2019-10-27T05:15:18.6610743Z 
2019-10-27T05:15:40.4826832Z error: aborting due to previous error
---
2019-10-27T05:15:40.6713989Z   local time: Sun Oct 27 05:15:40 UTC 2019
2019-10-27T05:15:43.0276035Z   network time: Sun, 27 Oct 2019 05:15:43 GMT
2019-10-27T05:15:43.0277283Z == end clock drift check ==
2019-10-27T05:15:43.8159746Z 
2019-10-27T05:15:43.8273050Z ##[error]Bash exited with code '1'.
2019-10-27T05:15:43.8308518Z ##[section]Starting: Checkout
2019-10-27T05:15:43.8310258Z ==============================================================================
2019-10-27T05:15:43.8310338Z Task         : Get sources
2019-10-27T05:15:43.8310390Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
