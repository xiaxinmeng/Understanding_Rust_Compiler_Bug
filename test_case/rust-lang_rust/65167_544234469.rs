plain
2019-10-20T08:49:37.4591515Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T08:49:37.4868050Z ##[command]git config gc.auto 0
2019-10-20T08:49:37.4946551Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T08:49:37.5013996Z ##[command]git config --get-all http.proxy
2019-10-20T08:49:37.5170256Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65167/merge:refs/remotes/pull/65167/merge
---
2019-10-20T08:56:18.2109570Z Done!
2019-10-20T08:56:18.2109707Z some tidy checks failed
2019-10-20T08:56:18.2109744Z 
2019-10-20T08:56:18.2109812Z 
2019-10-20T08:56:18.2112398Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-20T08:56:18.2113250Z 
2019-10-20T08:56:18.2113303Z 
2019-10-20T08:56:18.2119358Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-20T08:56:18.2119553Z Build completed unsuccessfully in 0:01:34
2019-10-20T08:56:18.2119553Z Build completed unsuccessfully in 0:01:34
2019-10-20T08:56:18.2172158Z == clock drift check ==
2019-10-20T08:56:18.2202837Z   local time: Sun Oct 20 08:56:18 UTC 2019
2019-10-20T08:56:18.3682553Z   network time: Sun, 20 Oct 2019 08:56:18 GMT
2019-10-20T08:56:18.3683413Z == end clock drift check ==
2019-10-20T08:56:20.4939706Z 
2019-10-20T08:56:20.5084194Z ##[error]Bash exited with code '1'.
2019-10-20T08:56:20.5124806Z ##[section]Starting: Checkout
2019-10-20T08:56:20.5126991Z ==============================================================================
2019-10-20T08:56:20.5127056Z Task         : Get sources
2019-10-20T08:56:20.5127131Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
