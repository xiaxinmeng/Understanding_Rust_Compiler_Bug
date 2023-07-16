plain
2019-08-29T17:23:10.7046963Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T17:23:10.7238443Z ##[command]git config gc.auto 0
2019-08-29T17:23:10.7329349Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T17:23:10.7404626Z ##[command]git config --get-all http.proxy
2019-08-29T17:23:10.7569842Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-29T17:30:13.8049461Z * wasi 
2019-08-29T17:30:13.8523541Z some tidy checks failed
2019-08-29T17:30:13.8532656Z 
2019-08-29T17:30:13.8532742Z 
2019-08-29T17:30:13.8533727Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-29T17:30:13.8533849Z 
2019-08-29T17:30:13.8533878Z 
2019-08-29T17:30:13.8533953Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-29T17:30:13.8534044Z Build completed unsuccessfully in 0:01:36
2019-08-29T17:30:13.8534044Z Build completed unsuccessfully in 0:01:36
2019-08-29T17:30:13.8581651Z == clock drift check ==
2019-08-29T17:30:13.8609212Z   local time: Thu Aug 29 17:30:13 UTC 2019
2019-08-29T17:30:13.9492201Z   network time: Thu, 29 Aug 2019 17:30:13 GMT
2019-08-29T17:30:13.9495405Z == end clock drift check ==
2019-08-29T17:30:15.2504326Z ##[error]Bash exited with code '1'.
2019-08-29T17:30:15.2536444Z ##[section]Starting: Checkout
2019-08-29T17:30:15.2538141Z ==============================================================================
2019-08-29T17:30:15.2538198Z Task         : Get sources
2019-08-29T17:30:15.2538244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
