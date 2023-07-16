plain
2019-12-18T15:41:16.5246366Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T15:41:16.5260934Z ##[command]git config gc.auto 0
2019-12-18T15:41:16.5264221Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T15:41:16.5266420Z ##[command]git config --get-all http.proxy
2019-12-18T15:41:16.5272627Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67397/merge:refs/remotes/pull/67397/merge
---
2019-12-18T15:48:32.1067128Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-18T15:48:34.6107903Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-18T15:48:36.3202523Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-18T15:48:50.2864069Z error[E0282]: type annotations needed for `std::option::Option<T>`
2019-12-18T15:48:50.2865430Z    --> src/librustc/ty/query/plumbing.rs:126:29
2019-12-18T15:48:50.2866151Z     |
2019-12-18T15:48:50.2866957Z 105 |         let mut query_blocked_prof_timer = None;
2019-12-18T15:48:50.2868046Z     |             ---------------------------- consider giving `query_blocked_prof_timer` the explicit type `std::option::Option<T>`, with the type parameters specified
2019-12-18T15:48:50.2869406Z ...
2019-12-18T15:48:50.2869982Z 126 |                             prof_timer.finish_with_query_invocation_id(value.index.into());
2019-12-18T15:48:50.2871000Z     |
2019-12-18T15:48:50.2871687Z     = note: type must be known at this point
2019-12-18T15:48:50.2871937Z 
2019-12-18T15:48:54.4198148Z error: aborting due to previous error
2019-12-18T15:48:54.4198148Z error: aborting due to previous error
2019-12-18T15:48:54.4198465Z 
2019-12-18T15:48:54.4198725Z For more information about this error, try `rustc --explain E0282`.
2019-12-18T15:48:54.5821862Z error: could not compile `rustc`.
2019-12-18T15:48:54.5821935Z 
2019-12-18T15:48:54.5822339Z To learn more, run the command again with --verbose.
2019-12-18T15:48:54.5841834Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-18T15:48:54.5853869Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-18T15:48:54.5853943Z Build completed unsuccessfully in 0:04:16
2019-12-18T15:48:54.5906265Z == clock drift check ==
2019-12-18T15:48:54.5921844Z   local time: Wed Dec 18 15:48:54 UTC 2019
2019-12-18T15:48:54.5921844Z   local time: Wed Dec 18 15:48:54 UTC 2019
2019-12-18T15:48:54.6707792Z   network time: Wed, 18 Dec 2019 15:48:54 GMT
2019-12-18T15:48:54.6710989Z == end clock drift check ==
2019-12-18T15:48:55.5346125Z 
2019-12-18T15:48:55.5407652Z ##[error]Bash exited with code '1'.
2019-12-18T15:48:55.5431718Z ##[section]Starting: Checkout
2019-12-18T15:48:55.5433238Z ==============================================================================
2019-12-18T15:48:55.5433286Z Task         : Get sources
2019-12-18T15:48:55.5433328Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
