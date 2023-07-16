plain
2019-09-01T14:02:46.5694179Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T14:02:46.5890952Z ##[command]git config gc.auto 0
2019-09-01T14:02:46.5971216Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T14:02:46.6030302Z ##[command]git config --get-all http.proxy
2019-09-01T14:02:46.6189314Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64065/merge:refs/remotes/pull/64065/merge
---
2019-09-01T14:10:48.7322052Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-01T14:10:50.1474439Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-01T14:10:51.4117900Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-01T14:11:04.5473529Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-01T14:11:06.2706463Z error: variant is never constructed: `LtOp`
2019-09-01T14:11:06.2706949Z   --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:60:5
2019-09-01T14:11:06.2708453Z 60 |     LtOp,
2019-09-01T14:11:06.2708776Z    |     ^^^^
2019-09-01T14:11:06.2709025Z    |
2019-09-01T14:11:06.2709330Z    = note: `-D dead-code` implied by `-D warnings`
2019-09-01T14:11:06.2709330Z    = note: `-D dead-code` implied by `-D warnings`
2019-09-01T14:11:06.2709374Z 
2019-09-01T14:11:06.2739030Z error: variant is never constructed: `LeOp`
2019-09-01T14:11:06.2739411Z   --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:61:5
2019-09-01T14:11:06.2739985Z 61 |     LeOp,
2019-09-01T14:11:06.2740281Z    |     ^^^^
2019-09-01T14:11:06.2740323Z 
2019-09-01T14:11:06.2740323Z 
2019-09-01T14:11:06.2766543Z error: variant is never constructed: `GtOp`
2019-09-01T14:11:06.2767204Z   --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:62:5
2019-09-01T14:11:06.2767769Z 62 |     GtOp,
2019-09-01T14:11:06.2768051Z    |     ^^^^
2019-09-01T14:11:06.2768093Z 
2019-09-01T14:11:06.2768093Z 
2019-09-01T14:11:06.2803371Z error: variant is never constructed: `GeOp`
2019-09-01T14:11:06.2804207Z   --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:63:5
2019-09-01T14:11:06.2804787Z 63 |     GeOp,
2019-09-01T14:11:06.2805086Z    |     ^^^^
2019-09-01T14:11:06.2805130Z 
2019-09-01T14:11:06.2805130Z 
2019-09-01T14:11:06.2897866Z error: function is never used: `cs_op`
2019-09-01T14:11:06.2898226Z    --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:154:1
2019-09-01T14:11:06.2898501Z     |
2019-09-01T14:11:06.2898837Z 154 | / fn cs_op(less: bool,
2019-09-01T14:11:06.2899174Z 155 | |          inclusive: bool,
2019-09-01T14:11:06.2899540Z 156 | |          cx: &mut ExtCtxt<'_>,
2019-09-01T14:11:06.2899882Z 157 | |          span: Span,
2019-09-01T14:11:06.2900779Z 269 | |     }
2019-09-01T14:11:06.2901104Z 270 | | }
2019-09-01T14:11:06.2901374Z     | |_^
2019-09-01T14:11:06.2901435Z 
---
2019-09-01T14:11:55.5453531Z == clock drift check ==
2019-09-01T14:11:55.5466908Z   local time: Sun Sep  1 14:11:55 UTC 2019
2019-09-01T14:11:55.6992329Z   network time: Sun, 01 Sep 2019 14:11:55 GMT
2019-09-01T14:11:55.6992924Z == end clock drift check ==
2019-09-01T14:11:56.6279582Z ##[error]Bash exited with code '1'.
2019-09-01T14:11:56.6321583Z ##[section]Starting: Checkout
2019-09-01T14:11:56.6323495Z ==============================================================================
2019-09-01T14:11:56.6323558Z Task         : Get sources
2019-09-01T14:11:56.6323611Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
