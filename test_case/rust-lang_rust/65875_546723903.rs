plain
2019-10-27T18:50:06.1542157Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T18:50:06.1758349Z ##[command]git config gc.auto 0
2019-10-27T18:50:06.1840653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T18:50:06.1908059Z ##[command]git config --get-all http.proxy
2019-10-27T18:50:06.2067084Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65875/merge:refs/remotes/pull/65875/merge
---
2019-10-27T18:56:28.3592314Z    Compiling serde_json v1.0.40
2019-10-27T18:56:30.1127307Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-27T18:56:41.6571392Z     Finished release [optimized] target(s) in 1m 29s
2019-10-27T18:56:41.6650497Z tidy check
2019-10-27T18:56:42.4610661Z tidy error: /checkout/src/libstd/tests/block_on_future.rs: missing trailing newline
2019-10-27T18:56:42.4833005Z tidy error: /checkout/src/libstd/thread/block_on_future.rs: missing trailing newline
2019-10-27T18:56:42.4845324Z tidy error: /checkout/src/libstd/thread/mod.rs: missing trailing newline
2019-10-27T18:56:44.1190002Z some tidy checks failed
2019-10-27T18:56:44.1190872Z Found 484 error codes
2019-10-27T18:56:44.1191292Z Found 0 error codes with no tests
2019-10-27T18:56:44.1191383Z Done!
2019-10-27T18:56:44.1191383Z Done!
2019-10-27T18:56:44.1194976Z 
2019-10-27T18:56:44.1195076Z 
2019-10-27T18:56:44.1196640Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-27T18:56:44.1197278Z 
2019-10-27T18:56:44.1197367Z 
2019-10-27T18:56:44.1207448Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-27T18:56:44.1207561Z Build completed unsuccessfully in 0:01:33
2019-10-27T18:56:44.1207561Z Build completed unsuccessfully in 0:01:33
2019-10-27T18:56:44.1265119Z == clock drift check ==
2019-10-27T18:56:44.1277148Z   local time: Sun Oct 27 18:56:44 UTC 2019
2019-10-27T18:56:44.2757054Z   network time: Sun, 27 Oct 2019 18:56:44 GMT
2019-10-27T18:56:44.2759500Z == end clock drift check ==
2019-10-27T18:56:45.6228345Z 
2019-10-27T18:56:45.6352677Z ##[error]Bash exited with code '1'.
2019-10-27T18:56:45.6394795Z ##[section]Starting: Checkout
2019-10-27T18:56:45.6397118Z ==============================================================================
2019-10-27T18:56:45.6397184Z Task         : Get sources
2019-10-27T18:56:45.6397422Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
