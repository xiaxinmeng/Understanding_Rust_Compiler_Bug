plain
2019-12-02T17:34:02.9316322Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T17:34:03.6542947Z ##[command]git config gc.auto 0
2019-12-02T17:34:03.6545992Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T17:34:03.6548373Z ##[command]git config --get-all http.proxy
2019-12-02T17:34:03.6550868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66835/merge:refs/remotes/pull/66835/merge
---
2019-12-02T17:40:23.6220552Z Done!
2019-12-02T17:40:23.6224226Z some tidy checks failed
2019-12-02T17:40:23.6224657Z 
2019-12-02T17:40:23.6224944Z 
2019-12-02T17:40:23.6226161Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-02T17:40:23.6226720Z 
2019-12-02T17:40:23.6226912Z 
2019-12-02T17:40:23.6227159Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-02T17:40:23.6227427Z Build completed unsuccessfully in 0:01:31
2019-12-02T17:40:23.6227427Z Build completed unsuccessfully in 0:01:31
2019-12-02T17:40:23.6277165Z == clock drift check ==
2019-12-02T17:40:23.6283829Z   local time: Mon Dec  2 17:40:23 UTC 2019
2019-12-02T17:40:23.9053461Z   network time: Mon, 02 Dec 2019 17:40:23 GMT
2019-12-02T17:40:23.9053656Z == end clock drift check ==
2019-12-02T17:40:25.1809888Z 
2019-12-02T17:40:25.1908318Z ##[error]Bash exited with code '1'.
2019-12-02T17:40:25.1936553Z ##[section]Starting: Checkout
2019-12-02T17:40:25.1938034Z ==============================================================================
2019-12-02T17:40:25.1938099Z Task         : Get sources
2019-12-02T17:40:25.1938140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
