plain
2019-11-28T23:10:36.8905791Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T23:10:37.6676485Z ##[command]git config gc.auto 0
2019-11-28T23:10:37.6682748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T23:10:37.6687786Z ##[command]git config --get-all http.proxy
2019-11-28T23:10:37.6693394Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65672/merge:refs/remotes/pull/65672/merge
---
2019-11-28T23:19:55.6071588Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-28T23:20:06.6668633Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-28T23:20:08.3983350Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-28T23:20:28.9205156Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-28T23:20:58.8894123Z error[E0560]: struct `mir::Body<'_>` has no field named `__upvar_debuginfo_codegen_only_do_not_use`
2019-11-28T23:20:58.8895785Z     |
2019-11-28T23:20:58.8895785Z     |
2019-11-28T23:20:58.8896360Z 218 |             __upvar_debuginfo_codegen_only_do_not_use: Vec::new(),
2019-11-28T23:20:58.8896952Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `mir::Body<'_>` does not have this field
2019-11-28T23:20:58.8897409Z     |
2019-11-28T23:20:58.8897952Z     = note: available fields are: `basic_blocks`, `phase`, `source_scopes`, `source_scope_local_data`, `yield_ty` ... and 10 others
2019-11-28T23:21:08.1098416Z error: aborting due to previous error
2019-11-28T23:21:08.1099367Z 
2019-11-28T23:21:08.1104796Z For more information about this error, try `rustc --explain E0560`.
2019-11-28T23:21:08.2784586Z error: could not compile `rustc`.
---
2019-11-28T23:21:10.4011575Z   local time: Thu Nov 28 23:21:10 UTC 2019
2019-11-28T23:21:10.5491418Z   network time: Thu, 28 Nov 2019 23:21:10 GMT
2019-11-28T23:21:10.5495105Z == end clock drift check ==
2019-11-28T23:21:11.6216517Z 
2019-11-28T23:21:11.6329596Z ##[error]Bash exited with code '1'.
2019-11-28T23:21:11.6366115Z ##[section]Starting: Checkout
2019-11-28T23:21:11.6368656Z ==============================================================================
2019-11-28T23:21:11.6368725Z Task         : Get sources
2019-11-28T23:21:11.6368778Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
