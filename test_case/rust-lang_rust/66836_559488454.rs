plain
2019-11-28T12:53:55.1387378Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T12:53:55.1405176Z ##[command]git config gc.auto 0
2019-11-28T12:53:55.1409655Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T12:53:55.1413060Z ##[command]git config --get-all http.proxy
2019-11-28T12:53:56.3058364Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66836/merge:refs/remotes/pull/66836/merge
---
2019-11-28T12:59:54.5258837Z Done!
2019-11-28T12:59:54.5259027Z some tidy checks failed
2019-11-28T12:59:54.5263100Z 
2019-11-28T12:59:54.5263456Z 
2019-11-28T12:59:54.5268590Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-28T12:59:54.5271061Z 
2019-11-28T12:59:54.5271468Z 
2019-11-28T12:59:54.5271922Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-28T12:59:54.5272585Z Build completed unsuccessfully in 0:01:28
2019-11-28T12:59:54.5272585Z Build completed unsuccessfully in 0:01:28
2019-11-28T12:59:54.5323181Z == clock drift check ==
2019-11-28T12:59:54.5342399Z   local time: Thu Nov 28 12:59:54 UTC 2019
2019-11-28T12:59:54.8095764Z   network time: Thu, 28 Nov 2019 12:59:54 GMT
2019-11-28T12:59:54.8099886Z == end clock drift check ==
2019-11-28T12:59:56.1919243Z 
2019-11-28T12:59:56.2029559Z ##[error]Bash exited with code '1'.
2019-11-28T12:59:56.2057681Z ##[section]Starting: Checkout
2019-11-28T12:59:56.2059229Z ==============================================================================
2019-11-28T12:59:56.2059278Z Task         : Get sources
2019-11-28T12:59:56.2059338Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
