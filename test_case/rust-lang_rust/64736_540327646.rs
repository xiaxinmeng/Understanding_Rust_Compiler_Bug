plain
2019-10-10T03:24:06.3688977Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T03:24:06.3926195Z ##[command]git config gc.auto 0
2019-10-10T03:24:06.3986226Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T03:24:06.4042986Z ##[command]git config --get-all http.proxy
2019-10-10T03:24:06.4188955Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-10T03:31:32.7149449Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-10T03:31:34.1253651Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-10T03:31:35.4074641Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-10T03:31:47.7265787Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-10T03:32:36.2895206Z error[E0599]: no method named `ensure_predecessors` found for type `rustc::mir::Body<'_>` in the current scope
2019-10-10T03:32:36.2896853Z    --> src/librustc_metadata/decoder.rs:928:14
2019-10-10T03:32:36.2898050Z     |
2019-10-10T03:32:36.2898892Z 928 |         body.ensure_predecessors();
2019-10-10T03:32:36.2899703Z     |              ^^^^^^^^^^^^^^^^^^^ method not found in `rustc::mir::Body<'_>`
2019-10-10T03:32:36.4436489Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-10T03:32:36.9685220Z error: aborting due to previous error
2019-10-10T03:32:36.9690598Z 
2019-10-10T03:32:36.9698002Z For more information about this error, try `rustc --explain E0599`.
---
2019-10-10T03:32:44.0181484Z == clock drift check ==
2019-10-10T03:32:44.0196771Z   local time: Thu Oct 10 03:32:44 UTC 2019
2019-10-10T03:32:44.1172488Z   network time: Thu, 10 Oct 2019 03:32:44 GMT
2019-10-10T03:32:44.1177180Z == end clock drift check ==
2019-10-10T03:32:44.8695312Z ##[error]Bash exited with code '1'.
2019-10-10T03:32:44.8859376Z ##[section]Starting: Checkout
2019-10-10T03:32:44.8861329Z ==============================================================================
2019-10-10T03:32:44.8861387Z Task         : Get sources
2019-10-10T03:32:44.8861434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
