plain
2019-10-01T12:08:47.6930626Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T12:08:47.7128904Z ##[command]git config gc.auto 0
2019-10-01T12:08:47.7186061Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T12:08:47.7246075Z ##[command]git config --get-all http.proxy
2019-10-01T12:08:47.7391974Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64954/merge:refs/remotes/pull/64954/merge
---
2019-10-01T12:15:31.1351214Z    Compiling serde_json v1.0.40
2019-10-01T12:15:32.9717610Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-01T12:15:43.8017695Z     Finished release [optimized] target(s) in 1m 28s
2019-10-01T12:15:43.8091338Z tidy check
2019-10-01T12:15:44.4095115Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3016) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-01T12:15:45.6450266Z some tidy checks failed
2019-10-01T12:15:45.6459302Z 
2019-10-01T12:15:45.6459302Z 
2019-10-01T12:15:45.6460431Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-01T12:15:45.6500770Z 
2019-10-01T12:15:45.6501003Z 
2019-10-01T12:15:45.6501296Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-01T12:15:45.6501508Z Build completed unsuccessfully in 0:01:32
2019-10-01T12:15:45.6501508Z Build completed unsuccessfully in 0:01:32
2019-10-01T12:15:45.6522739Z == clock drift check ==
2019-10-01T12:15:45.6543851Z   local time: Tue Oct  1 12:15:45 UTC 2019
2019-10-01T12:15:45.7509504Z   network time: Tue, 01 Oct 2019 12:15:45 GMT
2019-10-01T12:15:45.7513992Z == end clock drift check ==
2019-10-01T12:15:47.0044832Z ##[error]Bash exited with code '1'.
2019-10-01T12:15:47.0082370Z ##[section]Starting: Checkout
2019-10-01T12:15:47.0084662Z ==============================================================================
2019-10-01T12:15:47.0084719Z Task         : Get sources
2019-10-01T12:15:47.0084785Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
