plain
2019-12-10T20:32:37.0351781Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T20:32:37.0546542Z ##[command]git config gc.auto 0
2019-12-10T20:32:37.7335452Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T20:32:37.7344626Z ##[command]git config --get-all http.proxy
2019-12-10T20:32:37.7350091Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66721/merge:refs/remotes/pull/66721/merge
---
2019-12-10T20:38:52.8853900Z Found 0 error codes with no tests
2019-12-10T20:38:52.8854353Z Done!
2019-12-10T20:38:52.8854500Z 
2019-12-10T20:38:52.8854827Z 
2019-12-10T20:38:52.8856405Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-10T20:38:52.8856841Z 
2019-12-10T20:38:52.8856964Z 
2019-12-10T20:38:52.8857116Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-10T20:38:52.8857292Z Build completed unsuccessfully in 0:01:32
2019-12-10T20:38:52.8857292Z Build completed unsuccessfully in 0:01:32
2019-12-10T20:38:52.8857436Z some tidy checks failed
2019-12-10T20:38:52.8890142Z == clock drift check ==
2019-12-10T20:38:52.8912970Z   local time: Tue Dec 10 20:38:52 UTC 2019
2019-12-10T20:38:53.1860830Z   network time: Tue, 10 Dec 2019 20:38:53 GMT
2019-12-10T20:38:53.1866832Z == end clock drift check ==
2019-12-10T20:38:54.5206970Z 
2019-12-10T20:38:54.5314071Z ##[error]Bash exited with code '1'.
2019-12-10T20:38:54.5342081Z ##[section]Starting: Checkout
2019-12-10T20:38:54.5343602Z ==============================================================================
2019-12-10T20:38:54.5343650Z Task         : Get sources
2019-12-10T20:38:54.5343693Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
