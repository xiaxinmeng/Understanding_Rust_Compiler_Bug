plain
2019-11-29T17:14:39.2266114Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T17:14:39.2436067Z ##[command]git config gc.auto 0
2019-11-29T17:14:39.2513431Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T17:14:39.2570664Z ##[command]git config --get-all http.proxy
2019-11-29T17:14:39.2726656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66815/merge:refs/remotes/pull/66815/merge
---
2019-11-29T17:25:22.7964484Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-11-29T17:26:14.0298288Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-11-29T17:27:24.5926545Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-29T17:31:20.1358147Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-29T17:31:39.2546688Z error[E0451]: field `local_names` of struct `borrow_check::diagnostics::region_errors::ErrorReportingCtx` is private
2019-11-29T17:31:39.2552068Z    --> src/librustc_mir/borrow_check/diagnostics/region_name.rs:217:38
2019-11-29T17:31:39.2578345Z     |
2019-11-29T17:31:39.2579177Z 217 |             infcx, body, mir_def_id, local_names, upvars, ..
2019-11-29T17:31:39.2579891Z     |                                      ^^^^^^^^^^^ field `local_names` is private
2019-11-29T17:31:39.2580255Z 
2019-11-29T17:31:39.2612055Z error[E0451]: field `local_names` of struct `borrow_check::diagnostics::region_errors::ErrorReportingCtx` is private
2019-11-29T17:31:39.2612769Z    --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:129:13
2019-11-29T17:31:39.2614551Z 129 |             local_names: self.local_names,
2019-11-29T17:31:39.2614551Z 129 |             local_names: self.local_names,
2019-11-29T17:31:39.2618182Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `local_names` is private
2019-11-29T17:31:39.4364634Z error: aborting due to 2 previous errors
2019-11-29T17:31:39.4368300Z 
2019-11-29T17:31:39.4378982Z For more information about this error, try `rustc --explain E0451`.
2019-11-29T17:31:39.5785312Z error: could not compile `rustc_mir`.
---
2019-11-29T17:34:00.3215546Z   local time: Fri Nov 29 17:34:00 UTC 2019
2019-11-29T17:34:00.4750238Z   network time: Fri, 29 Nov 2019 17:34:00 GMT
2019-11-29T17:34:00.4750986Z == end clock drift check ==
2019-11-29T17:34:03.3148608Z 
2019-11-29T17:34:03.3270181Z ##[error]Bash exited with code '1'.
2019-11-29T17:34:03.3302413Z ##[section]Starting: Checkout
2019-11-29T17:34:03.3304756Z ==============================================================================
2019-11-29T17:34:03.3304814Z Task         : Get sources
2019-11-29T17:34:03.3304895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
