plain
2020-02-21T23:52:14.4292100Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-02-21T23:52:36.7424762Z error[E0433]: failed to resolve: use of undeclared type or module `rustc_span`
2020-02-21T23:52:36.7425628Z   --> src/librustc/hir/map/hir_id_validator.rs:24:29
2020-02-21T23:52:36.7426602Z    |
2020-02-21T23:52:36.7427340Z 24 |         sess.delay_span_bug(rustc_span::DUMMY_SP, &message);
2020-02-21T23:52:36.7434699Z 
2020-02-21T23:52:56.0679566Z error: aborting due to previous error
2020-02-21T23:52:56.0683091Z 
2020-02-21T23:52:56.0691551Z For more information about this error, try `rustc --explain E0433`.
---
2020-02-21T23:52:58.4699339Z   local time: Fri Feb 21 23:52:58 UTC 2020
2020-02-21T23:52:59.3101423Z   network time: Fri, 21 Feb 2020 23:52:59 GMT
2020-02-21T23:52:59.3101741Z == end clock drift check ==
2020-02-21T23:52:59.9118123Z 
2020-02-21T23:52:59.9190669Z ##[error]Bash exited with code '1'.
2020-02-21T23:52:59.9254137Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-21T23:52:59.9258595Z ==============================================================================
2020-02-21T23:52:59.9258894Z Task         : Get sources
2020-02-21T23:52:59.9259251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
