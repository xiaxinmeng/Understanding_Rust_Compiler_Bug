plain
2019-12-23T23:36:10.3952570Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T23:36:10.4252289Z ##[command]git config gc.auto 0
2019-12-23T23:36:10.4339186Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T23:36:10.4341729Z ##[command]git config --get-all http.proxy
2019-12-23T23:36:11.3672750Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67572/merge:refs/remotes/pull/67572/merge
---
2019-12-23T23:42:52.5888572Z    Compiling serde_json v1.0.40
2019-12-23T23:42:54.4206177Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-23T23:43:06.1320811Z     Finished release [optimized] target(s) in 1m 32s
2019-12-23T23:43:06.1417949Z tidy check
2019-12-23T23:43:06.9319905Z tidy error: /checkout/src/ci/scripts/install-msys2.sh:17: line longer than 100 chars
2019-12-23T23:43:08.8893338Z Found 485 error codes
2019-12-23T23:43:08.8893514Z Found 0 error codes with no tests
2019-12-23T23:43:08.8893559Z Done!
2019-12-23T23:43:08.8893600Z some tidy checks failed
2019-12-23T23:43:08.8893600Z some tidy checks failed
2019-12-23T23:43:08.8898765Z 
2019-12-23T23:43:08.8899024Z 
2019-12-23T23:43:08.8900526Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-23T23:43:08.8901141Z 
2019-12-23T23:43:08.8901281Z 
2019-12-23T23:43:08.8908783Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-23T23:43:08.8909132Z Build completed unsuccessfully in 0:01:44
2019-12-23T23:43:08.8909132Z Build completed unsuccessfully in 0:01:44
2019-12-23T23:43:08.8952572Z == clock drift check ==
2019-12-23T23:43:08.8974513Z   local time: Mon Dec 23 23:43:08 UTC 2019
2019-12-23T23:43:09.1777364Z   network time: Mon, 23 Dec 2019 23:43:09 GMT
2019-12-23T23:43:09.1786953Z == end clock drift check ==
2019-12-23T23:43:10.5745032Z 
2019-12-23T23:43:10.5848730Z ##[error]Bash exited with code '1'.
2019-12-23T23:43:10.5878539Z ##[section]Starting: Checkout
2019-12-23T23:43:10.5880397Z ==============================================================================
2019-12-23T23:43:10.5880451Z Task         : Get sources
2019-12-23T23:43:10.5880517Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
