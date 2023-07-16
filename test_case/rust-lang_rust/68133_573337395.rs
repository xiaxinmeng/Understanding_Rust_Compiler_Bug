plain
2020-01-11T16:24:22.1235636Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T16:24:22.1329670Z ##[command]git config gc.auto 0
2020-01-11T16:24:22.1416416Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T16:24:22.1490503Z ##[command]git config --get-all http.proxy
2020-01-11T16:24:22.1692895Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68133/merge:refs/remotes/pull/68133/merge
---
2020-01-11T16:30:57.8122757Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-11T16:31:00.4839895Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-01-11T16:31:01.3224674Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-01-11T16:31:03.0890637Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-11T16:31:04.1556091Z error[E0425]: cannot find value `GLOBALS` in crate `syntax`
2020-01-11T16:31:04.1556481Z    --> src/librustc/ty/query/job.rs:438:34
2020-01-11T16:31:04.1556768Z     |
2020-01-11T16:31:04.1557118Z 438 |     let syntax_globals = syntax::GLOBALS.with(|syntax_globals| syntax_globals as *const _);
2020-01-11T16:31:04.1557766Z     |
2020-01-11T16:31:04.1558064Z help: possible candidates are found in other modules, you can import them into scope
2020-01-11T16:31:04.1558306Z     |
2020-01-11T16:31:04.1558582Z 1   | use rustc_attr::GLOBALS;
---
2020-01-11T16:31:04.1559831Z     |
2020-01-11T16:31:04.1560501Z 1   | use syntax::attr::GLOBALS;
2020-01-11T16:31:04.1561036Z     |
2020-01-11T16:31:04.1566875Z 
2020-01-11T16:31:04.1852293Z error[E0425]: cannot find value `GLOBALS` in crate `syntax`
2020-01-11T16:31:04.1903737Z    --> src/librustc/ty/query/job.rs:442:21
2020-01-11T16:31:04.1904093Z     |
2020-01-11T16:31:04.1904670Z 442 |             syntax::GLOBALS.set(syntax_globals, || {
2020-01-11T16:31:04.1905343Z     |
2020-01-11T16:31:04.1905638Z help: possible candidates are found in other modules, you can import them into scope
2020-01-11T16:31:04.1905858Z     |
2020-01-11T16:31:04.1906167Z 1   | use rustc_attr::GLOBALS;
---
2020-01-11T16:31:25.2962328Z For more information about this error, try `rustc --explain E0425`.
2020-01-11T16:31:25.3305888Z error: could not compile `rustc`.
2020-01-11T16:31:25.3306120Z 
2020-01-11T16:31:25.3306417Z To learn more, run the command again with --verbose.
2020-01-11T16:31:25.3333867Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-11T16:31:25.3340334Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-11T16:31:25.3340405Z Build completed unsuccessfully in 0:04:41
2020-01-11T16:31:25.3393640Z == clock drift check ==
2020-01-11T16:31:25.3407055Z   local time: Sat Jan 11 16:31:25 UTC 2020
2020-01-11T16:31:25.3407055Z   local time: Sat Jan 11 16:31:25 UTC 2020
2020-01-11T16:31:25.5038142Z   network time: Sat, 11 Jan 2020 16:31:25 GMT
2020-01-11T16:31:25.5044573Z == end clock drift check ==
2020-01-11T16:31:25.8999632Z 
2020-01-11T16:31:25.9115133Z ##[error]Bash exited with code '1'.
2020-01-11T16:31:25.9145095Z ##[section]Starting: Checkout
2020-01-11T16:31:25.9146898Z ==============================================================================
2020-01-11T16:31:25.9146978Z Task         : Get sources
2020-01-11T16:31:25.9147034Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
