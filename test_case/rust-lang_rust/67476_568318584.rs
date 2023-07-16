plain
2019-12-23T00:32:17.7098535Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T00:32:17.7115390Z ##[command]git config gc.auto 0
2019-12-23T00:32:17.7117866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T00:32:17.7119986Z ##[command]git config --get-all http.proxy
2019-12-23T00:32:17.7122580Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-23T00:37:48.6215144Z    Compiling serde_json v1.0.40
2019-12-23T00:37:50.1254132Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-23T00:38:00.2686871Z     Finished release [optimized] target(s) in 1m 20s
2019-12-23T00:38:00.2787687Z tidy check
2019-12-23T00:38:01.1196528Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1595: TODO is deprecated; use FIXME
2019-12-23T00:38:01.1270272Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:164: TODO is deprecated; use FIXME
2019-12-23T00:38:01.1294803Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:778: TODO is deprecated; use FIXME
2019-12-23T00:38:02.7195230Z some tidy checks failed
2019-12-23T00:38:02.7195379Z Found 485 error codes
2019-12-23T00:38:02.7195434Z Found 0 error codes with no tests
2019-12-23T00:38:02.7200776Z Done!
2019-12-23T00:38:02.7200776Z Done!
2019-12-23T00:38:02.7201083Z 
2019-12-23T00:38:02.7201246Z 
2019-12-23T00:38:02.7212162Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-23T00:38:02.7218007Z 
2019-12-23T00:38:02.7218304Z 
2019-12-23T00:38:02.7218613Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-23T00:38:02.7228551Z Build completed unsuccessfully in 0:01:23
2019-12-23T00:38:02.7228551Z Build completed unsuccessfully in 0:01:23
2019-12-23T00:38:02.7289833Z == clock drift check ==
2019-12-23T00:38:02.7294923Z   local time: Mon Dec 23 00:38:02 UTC 2019
2019-12-23T00:38:02.8698146Z   network time: Mon, 23 Dec 2019 00:38:02 GMT
2019-12-23T00:38:02.8700381Z == end clock drift check ==
2019-12-23T00:38:04.1434036Z 
2019-12-23T00:38:04.1542947Z ##[error]Bash exited with code '1'.
2019-12-23T00:38:04.1567042Z ##[section]Starting: Checkout
2019-12-23T00:38:04.1568576Z ==============================================================================
2019-12-23T00:38:04.1568644Z Task         : Get sources
2019-12-23T00:38:04.1568688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
