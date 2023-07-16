plain
2019-09-05T15:19:47.8596323Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T15:19:47.8786993Z ##[command]git config gc.auto 0
2019-09-05T15:19:47.8853842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T15:19:47.8897383Z ##[command]git config --get-all http.proxy
2019-09-05T15:19:47.9028883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63919/merge:refs/remotes/pull/63919/merge
---
2019-09-05T15:27:58.0552737Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-05T15:27:59.4785312Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-05T15:28:00.7414844Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-05T15:28:13.7373279Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-05T15:28:15.4449508Z error[E0716]: temporary value dropped while borrowed
2019-09-05T15:28:15.4449991Z   --> src/libsyntax_ext/standard_library_imports.rs:65:9
2019-09-05T15:28:15.4450218Z    |
2019-09-05T15:28:15.4450451Z 64 |     let import_segements = if rust_2018 {
2019-09-05T15:28:15.4450806Z    |         ---------------- borrow later stored here
2019-09-05T15:28:15.4451049Z 65 |         [name, sym::prelude, sym::v1].iter()
2019-09-05T15:28:15.4452425Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
2019-09-05T15:28:15.4453085Z    |     - temporary value is freed at the end of this statement
2019-09-05T15:28:15.4453359Z    |
2019-09-05T15:28:15.4453359Z    |
2019-09-05T15:28:15.4453660Z    = note: consider using a `let` binding to create a longer lived value
2019-09-05T15:28:15.4459001Z 
2019-09-05T15:28:15.4502437Z error[E0716]: temporary value dropped while borrowed
2019-09-05T15:28:15.4502770Z   --> src/libsyntax_ext/standard_library_imports.rs:67:9
2019-09-05T15:28:15.4503028Z    |
2019-09-05T15:28:15.4503326Z 64 |     let import_segements = if rust_2018 {
2019-09-05T15:28:15.4503656Z    |         ---------------- borrow later stored here
2019-09-05T15:28:15.4503896Z ...
2019-09-05T15:28:15.4504200Z 67 |         [kw::PathRoot, name, sym::prelude, sym::v1].iter()
2019-09-05T15:28:15.4504596Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
2019-09-05T15:28:15.4506018Z    |     - temporary value is freed at the end of this statement
2019-09-05T15:28:15.4506265Z    |
2019-09-05T15:28:15.4506265Z    |
2019-09-05T15:28:15.4506723Z    = note: consider using a `let` binding to create a longer lived value
2019-09-05T15:28:15.4687605Z error: aborting due to 2 previous errors
2019-09-05T15:28:15.4692217Z 
2019-09-05T15:28:15.4699148Z For more information about this error, try `rustc --explain E0716`.
2019-09-05T15:28:15.4898741Z error: Could not compile `syntax_ext`.
---
2019-09-05T15:29:03.9076555Z == clock drift check ==
2019-09-05T15:29:03.9097956Z   local time: Thu Sep  5 15:29:03 UTC 2019
2019-09-05T15:29:04.0691149Z   network time: Thu, 05 Sep 2019 15:29:04 GMT
2019-09-05T15:29:04.0696261Z == end clock drift check ==
2019-09-05T15:29:05.1190126Z ##[error]Bash exited with code '1'.
2019-09-05T15:29:05.1232933Z ##[section]Starting: Checkout
2019-09-05T15:29:05.1234376Z ==============================================================================
2019-09-05T15:29:05.1234421Z Task         : Get sources
2019-09-05T15:29:05.1234474Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
