plain
2019-11-14T02:41:16.7352983Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-11-14T02:41:16.9128061Z error[E0425]: cannot find value `E0744` in this scope
2019-11-14T02:41:16.9128462Z    --> src/librustc_passes/check_const.rs:101:36
2019-11-14T02:41:16.9128875Z     |
2019-11-14T02:41:16.9129431Z 101 |         span_err!(self.sess, span, E0744, "`{}` is not allowed in a `{}`", bad_op, const_kind);
2019-11-14T02:41:16.9130200Z     |
2019-11-14T02:41:16.9130675Z help: possible candidate is found in another module, you can import it into scope
2019-11-14T02:41:16.9131094Z     |
2019-11-14T02:41:16.9131416Z 10  | use rustc_error_codes::E0744;
---
2019-11-14T02:41:26.8800065Z   local time: Thu Nov 14 02:41:26 UTC 2019
2019-11-14T02:41:27.0308184Z   network time: Thu, 14 Nov 2019 02:41:27 GMT
2019-11-14T02:41:27.0309538Z == end clock drift check ==
2019-11-14T02:41:28.3441733Z 
2019-11-14T02:41:28.3531751Z ##[error]Bash exited with code '1'.
2019-11-14T02:41:28.3567388Z ##[section]Starting: Checkout
2019-11-14T02:41:28.3569482Z ==============================================================================
2019-11-14T02:41:28.3569798Z Task         : Get sources
2019-11-14T02:41:28.3569892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
