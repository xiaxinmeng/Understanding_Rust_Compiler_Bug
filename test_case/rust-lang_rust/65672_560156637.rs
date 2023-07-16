plain
2019-12-01T20:33:15.3358547Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T20:33:15.3547362Z ##[command]git config gc.auto 0
2019-12-01T20:33:16.3159980Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T20:33:16.3161821Z ##[command]git config --get-all http.proxy
2019-12-01T20:33:16.3163953Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65672/merge:refs/remotes/pull/65672/merge
---
2019-12-01T20:41:15.6117281Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-01T20:41:16.9836583Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2019-12-01T20:41:18.3510535Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-12-01T20:41:35.2673804Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-12-01T20:41:58.0327690Z error[E0063]: missing fields `generator_kind`, `var_debug_info` in initializer of `mir::Body<'_>`
2019-12-01T20:41:58.0329067Z     |
2019-12-01T20:41:58.0329684Z 213 |         Body {
2019-12-01T20:41:58.0329684Z 213 |         Body {
2019-12-01T20:41:58.0330195Z     |         ^^^^ missing `generator_kind`, `var_debug_info`
2019-12-01T20:42:05.5345893Z error: aborting due to previous error
2019-12-01T20:42:05.5348796Z 
2019-12-01T20:42:05.5356348Z For more information about this error, try `rustc --explain E0063`.
2019-12-01T20:42:05.6793672Z error: could not compile `rustc`.
---
2019-12-01T20:42:07.2644274Z   local time: Sun Dec  1 20:42:07 UTC 2019
2019-12-01T20:42:07.5400495Z   network time: Sun, 01 Dec 2019 20:42:07 GMT
2019-12-01T20:42:07.5403717Z == end clock drift check ==
2019-12-01T20:42:08.5438702Z 
2019-12-01T20:42:08.5504676Z ##[error]Bash exited with code '1'.
2019-12-01T20:42:08.5527912Z ##[section]Starting: Checkout
2019-12-01T20:42:08.5529291Z ==============================================================================
2019-12-01T20:42:08.5529346Z Task         : Get sources
2019-12-01T20:42:08.5529381Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
