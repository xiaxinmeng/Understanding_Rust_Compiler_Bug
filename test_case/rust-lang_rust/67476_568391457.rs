plain
2019-12-23T07:30:25.6618274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T07:30:25.6839403Z ##[command]git config gc.auto 0
2019-12-23T07:30:25.6914504Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T07:30:25.6994659Z ##[command]git config --get-all http.proxy
2019-12-23T07:30:25.7170810Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-23T07:36:30.5496111Z    Compiling serde_json v1.0.40
2019-12-23T07:36:32.2463326Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-23T07:36:42.9173911Z     Finished release [optimized] target(s) in 1m 24s
2019-12-23T07:36:42.9263613Z tidy check
2019-12-23T07:36:43.7902125Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1483: TODO is deprecated; use FIXME
2019-12-23T07:36:43.7972426Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:164: TODO is deprecated; use FIXME
2019-12-23T07:36:43.8004555Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:778: TODO is deprecated; use FIXME
2019-12-23T07:36:45.5280882Z some tidy checks failed
2019-12-23T07:36:45.5283118Z Found 485 error codes
2019-12-23T07:36:45.5283354Z Found 0 error codes with no tests
2019-12-23T07:36:45.5283539Z Done!
2019-12-23T07:36:45.5283539Z Done!
2019-12-23T07:36:45.5283682Z 
2019-12-23T07:36:45.5283813Z 
2019-12-23T07:36:45.5284936Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-23T07:36:45.5285482Z 
2019-12-23T07:36:45.5285636Z 
2019-12-23T07:36:45.5290954Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-23T07:36:45.5291265Z Build completed unsuccessfully in 0:01:35
2019-12-23T07:36:45.5291265Z Build completed unsuccessfully in 0:01:35
2019-12-23T07:36:45.5346646Z == clock drift check ==
2019-12-23T07:36:45.5358298Z   local time: Mon Dec 23 07:36:45 UTC 2019
2019-12-23T07:36:45.8145405Z   network time: Mon, 23 Dec 2019 07:36:45 GMT
2019-12-23T07:36:45.8150675Z == end clock drift check ==
2019-12-23T07:36:47.3118751Z 
2019-12-23T07:36:47.3223555Z ##[error]Bash exited with code '1'.
2019-12-23T07:36:47.3251754Z ##[section]Starting: Checkout
2019-12-23T07:36:47.3253417Z ==============================================================================
2019-12-23T07:36:47.3253469Z Task         : Get sources
2019-12-23T07:36:47.3253529Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
