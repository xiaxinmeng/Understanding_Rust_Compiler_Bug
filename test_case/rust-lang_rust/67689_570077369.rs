plain
2020-01-01T17:06:50.6935669Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T17:06:50.7210758Z ##[command]git config gc.auto 0
2020-01-01T17:06:50.7287575Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T17:06:50.7350271Z ##[command]git config --get-all http.proxy
2020-01-01T17:06:50.7489530Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67689/merge:refs/remotes/pull/67689/merge
---
2020-01-01T18:43:00.6522220Z    Compiling mdbook v0.3.5
2020-01-01T18:43:25.8213795Z    Compiling mdbook-linkcheck v0.5.0
2020-01-01T18:43:47.1576520Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2020-01-01T18:43:51.5363762Z     Finished release [optimized] target(s) in 10m 22s
2020-01-01T18:45:39.0377527Z error: Unable to retrieve "https://perf.wiki.kernel.org/index.php/Main_Page": https://perf.wiki.kernel.org/index.php/Main_Page: timed out
2020-01-01T18:45:39.0378259Z 
2020-01-01T18:45:39.0379208Z    ┌── profiling/with_perf.md:3:47 ───
2020-01-01T18:45:39.0379665Z    │
2020-01-01T18:45:39.0380170Z  3 │ This is a guide for how to profile rustc with [perf](https://perf.wiki.kernel.org/index.php/Main_Page).
2020-01-01T18:45:39.0380851Z    │                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ https://perf.wiki.kernel.org/index.php/Main_Page: timed out
2020-01-01T18:45:39.0381370Z 
2020-01-01T18:45:39.0383772Z Error: One or more incorrect links
2020-01-01T18:45:39.0391105Z 
2020-01-01T18:45:39.0391346Z 
---
2020-01-01T19:28:52.1661503Z 
2020-01-01T19:28:52.1661763Z If you do intend to update 'rustc-guide', please check the error messages above and
2020-01-01T19:28:52.1661810Z commit another update.
2020-01-01T19:28:52.1661834Z 
2020-01-01T19:28:52.1662091Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2020-01-01T19:28:52.1662333Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2020-01-01T19:28:52.1662377Z proper steps.
2020-01-01T19:28:52.1662799Z Build completed unsuccessfully in 0:00:01
2020-01-01T19:28:52.1708584Z == clock drift check ==
2020-01-01T19:28:52.1730389Z   local time: Wed Jan  1 19:28:52 UTC 2020
2020-01-01T19:28:52.7205802Z   network time: Wed, 01 Jan 2020 19:28:52 GMT
2020-01-01T19:28:52.7205802Z   network time: Wed, 01 Jan 2020 19:28:52 GMT
2020-01-01T19:28:52.7206584Z == end clock drift check ==
2020-01-01T19:28:53.2365517Z 
2020-01-01T19:28:53.2460715Z ##[error]Bash exited with code '1'.
2020-01-01T19:28:53.2499971Z ##[section]Starting: Checkout
2020-01-01T19:28:53.2501471Z ==============================================================================
2020-01-01T19:28:53.2501515Z Task         : Get sources
2020-01-01T19:28:53.2501553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
