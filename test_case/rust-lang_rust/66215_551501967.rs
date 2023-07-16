plain
2019-11-08T09:58:19.1118197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T09:58:19.1319127Z ##[command]git config gc.auto 0
2019-11-08T09:58:19.1409028Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T09:58:19.1458387Z ##[command]git config --get-all http.proxy
2019-11-08T09:58:19.1614398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-08T10:04:54.3243165Z Found 0 error codes with no tests
2019-11-08T10:04:54.3243424Z Done!
2019-11-08T10:04:54.3243618Z 
2019-11-08T10:04:54.3243793Z 
2019-11-08T10:04:54.3250513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-08T10:04:54.3252881Z 
2019-11-08T10:04:54.3253042Z 
2019-11-08T10:04:54.3253473Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-08T10:04:54.3253686Z Build completed unsuccessfully in 0:01:36
2019-11-08T10:04:54.3253686Z Build completed unsuccessfully in 0:01:36
2019-11-08T10:04:54.3297647Z == clock drift check ==
2019-11-08T10:04:54.3307422Z   local time: Fri Nov  8 10:04:54 UTC 2019
2019-11-08T10:04:54.8627627Z   network time: Fri, 08 Nov 2019 10:04:54 GMT
2019-11-08T10:04:54.8629826Z == end clock drift check ==
2019-11-08T10:04:56.1419578Z 
2019-11-08T10:04:56.1532210Z ##[error]Bash exited with code '1'.
2019-11-08T10:04:56.1561155Z ##[section]Starting: Checkout
2019-11-08T10:04:56.1563018Z ==============================================================================
2019-11-08T10:04:56.1563099Z Task         : Get sources
2019-11-08T10:04:56.1563152Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
