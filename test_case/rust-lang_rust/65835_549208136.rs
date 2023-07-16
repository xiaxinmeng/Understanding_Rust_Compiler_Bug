plain
2019-11-04T02:29:54.6616073Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-11-04T02:29:55.9373034Z error[E0599]: no method named `buffer_lint` found for type `&'b rustc::session::Session` in the current scope
2019-11-04T02:29:55.9376629Z    --> src/librustc_resolve/resolve_imports.rs:982:36
2019-11-04T02:29:55.9378510Z     |
2019-11-04T02:29:55.9379015Z 982 |                     self.r.session.buffer_lint(UNUSED_IMPORTS, directive.id, directive.span, msg);
2019-11-04T02:29:55.9379651Z     |                                    ^^^^^^^^^^^ method not found in `&'b rustc::session::Session`
2019-11-04T02:29:56.0025618Z error: aborting due to previous error
2019-11-04T02:29:56.0026642Z 
2019-11-04T02:29:56.0035113Z For more information about this error, try `rustc --explain E0599`.
2019-11-04T02:29:56.0214828Z error: could not compile `rustc_resolve`.
---
2019-11-04T02:30:07.1722273Z   local time: Mon Nov  4 02:30:07 UTC 2019
2019-11-04T02:30:07.3091555Z   network time: Mon, 04 Nov 2019 02:30:07 GMT
2019-11-04T02:30:07.3094644Z == end clock drift check ==
2019-11-04T02:30:08.5654743Z 
2019-11-04T02:30:08.5751144Z ##[error]Bash exited with code '1'.
2019-11-04T02:30:08.5781169Z ##[section]Starting: Checkout
2019-11-04T02:30:08.5783032Z ==============================================================================
2019-11-04T02:30:08.5783131Z Task         : Get sources
2019-11-04T02:30:08.5783219Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
