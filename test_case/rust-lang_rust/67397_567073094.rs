plain
2019-12-18T15:04:35.7942702Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T15:04:35.7958526Z ##[command]git config gc.auto 0
2019-12-18T15:04:35.7963093Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T15:04:35.7968615Z ##[command]git config --get-all http.proxy
2019-12-18T15:04:35.7973609Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67397/merge:refs/remotes/pull/67397/merge
---
2019-12-18T15:11:17.5725067Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-18T15:11:20.0922788Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-18T15:11:21.8454192Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-18T15:11:34.7292865Z error[E0061]: this function takes 0 parameters but 1 parameter was supplied
2019-12-18T15:11:34.7294033Z    --> src/librustc/ty/query/plumbing.rs:128:69
2019-12-18T15:11:34.7294549Z     |
2019-12-18T15:11:34.7295124Z 128 | ...                   query_blocked_prof_timer = tcx.prof.query_blocked(Q::NAME);
2019-12-18T15:11:34.7296228Z 
2019-12-18T15:11:38.5716814Z error: aborting due to previous error
2019-12-18T15:11:38.5717728Z 
2019-12-18T15:11:38.5718531Z For more information about this error, try `rustc --explain E0061`.
2019-12-18T15:11:38.5718531Z For more information about this error, try `rustc --explain E0061`.
2019-12-18T15:11:38.7386699Z error: could not compile `rustc`.
2019-12-18T15:11:38.7387593Z 
2019-12-18T15:11:38.7388639Z To learn more, run the command again with --verbose.
2019-12-18T15:11:38.7412810Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-18T15:11:38.7424227Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-18T15:11:38.7424661Z Build completed unsuccessfully in 0:03:59
2019-12-18T15:11:38.7475808Z == clock drift check ==
2019-12-18T15:11:38.7493801Z   local time: Wed Dec 18 15:11:38 UTC 2019
2019-12-18T15:11:38.7493801Z   local time: Wed Dec 18 15:11:38 UTC 2019
2019-12-18T15:11:38.8269221Z   network time: Wed, 18 Dec 2019 15:11:38 GMT
2019-12-18T15:11:38.8288706Z == end clock drift check ==
2019-12-18T15:11:39.7626124Z 
2019-12-18T15:11:39.7739693Z ##[error]Bash exited with code '1'.
2019-12-18T15:11:39.7769237Z ##[section]Starting: Checkout
2019-12-18T15:11:39.7770885Z ==============================================================================
2019-12-18T15:11:39.7770940Z Task         : Get sources
2019-12-18T15:11:39.7770988Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
