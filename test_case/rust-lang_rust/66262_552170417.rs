plain
2019-11-10T07:09:53.7614615Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T07:09:53.7816378Z ##[command]git config gc.auto 0
2019-11-10T07:09:53.7911866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T07:09:53.8015671Z ##[command]git config --get-all http.proxy
2019-11-10T07:09:53.8251306Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66262/merge:refs/remotes/pull/66262/merge
---
2019-11-10T07:17:38.1824273Z Done!
2019-11-10T07:17:38.1829801Z some tidy checks failed
2019-11-10T07:17:38.1832452Z 
2019-11-10T07:17:38.1842521Z 
2019-11-10T07:17:38.1843860Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-10T07:17:38.1844809Z 
2019-11-10T07:17:38.1845309Z 
2019-11-10T07:17:38.1845825Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-10T07:17:38.1846055Z Build completed unsuccessfully in 0:01:49
2019-11-10T07:17:38.1846055Z Build completed unsuccessfully in 0:01:49
2019-11-10T07:17:38.1896047Z == clock drift check ==
2019-11-10T07:17:38.1911908Z   local time: Sun Nov 10 07:17:38 UTC 2019
2019-11-10T07:17:38.3399634Z   network time: Sun, 10 Nov 2019 07:17:38 GMT
2019-11-10T07:17:38.3403477Z == end clock drift check ==
2019-11-10T07:17:39.6237504Z 
2019-11-10T07:17:39.6371921Z ##[error]Bash exited with code '1'.
2019-11-10T07:17:39.6440437Z ##[section]Starting: Checkout
2019-11-10T07:17:39.6442075Z ==============================================================================
2019-11-10T07:17:39.6442144Z Task         : Get sources
2019-11-10T07:17:39.6442191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
