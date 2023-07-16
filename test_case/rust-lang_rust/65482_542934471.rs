plain
2019-10-16T23:18:09.4264019Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T23:18:09.4438237Z ##[command]git config gc.auto 0
2019-10-16T23:18:09.4508929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T23:18:10.1692244Z ##[command]git config --get-all http.proxy
2019-10-16T23:18:10.1696604Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65482/merge:refs/remotes/pull/65482/merge
---
2019-10-16T23:25:11.0236940Z    Compiling serde_json v1.0.40
2019-10-16T23:25:12.9318082Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-16T23:25:25.0789755Z     Finished release [optimized] target(s) in 1m 34s
2019-10-16T23:25:25.0882079Z tidy check
2019-10-16T23:25:25.2212416Z tidy error: /checkout/src/liballoc/tests/boxed.rs: too many trailing newlines (2)
2019-10-16T23:25:27.1533214Z some tidy checks failed
2019-10-16T23:25:27.1534153Z Found 482 error codes
2019-10-16T23:25:27.1534400Z Found 0 error codes with no tests
2019-10-16T23:25:27.1534642Z Done!
2019-10-16T23:25:27.1534642Z Done!
2019-10-16T23:25:27.1538610Z 
2019-10-16T23:25:27.1538914Z 
2019-10-16T23:25:27.1540455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-16T23:25:27.1541454Z 
2019-10-16T23:25:27.1541661Z 
2019-10-16T23:25:27.1551314Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-16T23:25:27.1551744Z Build completed unsuccessfully in 0:01:38
2019-10-16T23:25:27.1551744Z Build completed unsuccessfully in 0:01:38
2019-10-16T23:25:27.1597808Z == clock drift check ==
2019-10-16T23:25:27.1629841Z   local time: Wed Oct 16 23:25:27 UTC 2019
2019-10-16T23:25:27.2643019Z   network time: Wed, 16 Oct 2019 23:25:27 GMT
2019-10-16T23:25:27.2644147Z == end clock drift check ==
2019-10-16T23:25:28.7305622Z ##[error]Bash exited with code '1'.
2019-10-16T23:25:28.7342043Z ##[section]Starting: Checkout
2019-10-16T23:25:28.7343695Z ==============================================================================
2019-10-16T23:25:28.7343769Z Task         : Get sources
2019-10-16T23:25:28.7343836Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
