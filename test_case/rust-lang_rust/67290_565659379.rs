plain
2019-12-14T00:10:43.7112891Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T00:10:43.7310291Z ##[command]git config gc.auto 0
2019-12-14T00:10:43.7381639Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T00:10:43.7426752Z ##[command]git config --get-all http.proxy
2019-12-14T00:10:43.7559980Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67290/merge:refs/remotes/pull/67290/merge
---
2019-12-14T00:16:24.7916637Z    Compiling serde_json v1.0.40
2019-12-14T00:16:27.0868042Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-14T00:16:36.3711405Z     Finished release [optimized] target(s) in 1m 17s
2019-12-14T00:16:36.3804205Z tidy check
2019-12-14T00:16:36.7004675Z tidy error: /checkout/src/liballoc/collections/vec_deque.rs: too many lines (3018) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-12-14T00:16:38.8452169Z some tidy checks failed
2019-12-14T00:16:38.8452989Z Found 485 error codes
2019-12-14T00:16:38.8453294Z Found 0 error codes with no tests
2019-12-14T00:16:38.8453496Z Done!
2019-12-14T00:16:38.8453496Z Done!
2019-12-14T00:16:38.8453779Z 
2019-12-14T00:16:38.8453979Z 
2019-12-14T00:16:38.8455240Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-14T00:16:38.8455846Z 
2019-12-14T00:16:38.8456238Z 
2019-12-14T00:16:38.8459035Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-14T00:16:38.8459547Z Build completed unsuccessfully in 0:01:21
2019-12-14T00:16:38.8459547Z Build completed unsuccessfully in 0:01:21
2019-12-14T00:16:38.8509429Z == clock drift check ==
2019-12-14T00:16:38.8520316Z   local time: Sat Dec 14 00:16:38 UTC 2019
2019-12-14T00:16:38.8948448Z   network time: Sat, 14 Dec 2019 00:16:38 GMT
2019-12-14T00:16:38.8953945Z == end clock drift check ==
2019-12-14T00:16:40.2591988Z 
2019-12-14T00:16:40.2686065Z ##[error]Bash exited with code '1'.
2019-12-14T00:16:40.2713703Z ##[section]Starting: Checkout
2019-12-14T00:16:40.2715169Z ==============================================================================
2019-12-14T00:16:40.2715340Z Task         : Get sources
2019-12-14T00:16:40.2715383Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
