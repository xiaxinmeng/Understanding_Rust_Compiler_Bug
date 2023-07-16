plain
2019-09-13T12:41:50.8874091Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T12:41:50.9059524Z ##[command]git config gc.auto 0
2019-09-13T12:41:50.9139509Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T12:41:50.9211798Z ##[command]git config --get-all http.proxy
2019-09-13T12:41:50.9360162Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64419/merge:refs/remotes/pull/64419/merge
---
2019-09-13T12:48:50.8881516Z    Compiling serde_json v1.0.40
2019-09-13T12:48:52.7322708Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-13T12:49:03.9094817Z     Finished release [optimized] target(s) in 1m 32s
2019-09-13T12:49:03.9210971Z tidy check
2019-09-13T12:49:04.5941854Z tidy error: /checkout/src/test/ui/consts/const-prop-read-static-in-const.rs: too many trailing newlines (2)
2019-09-13T12:49:05.9005670Z some tidy checks failed
2019-09-13T12:49:05.9005820Z 
2019-09-13T12:49:05.9005820Z 
2019-09-13T12:49:05.9006790Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-13T12:49:05.9011879Z 
2019-09-13T12:49:05.9014678Z 
2019-09-13T12:49:05.9021809Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-13T12:49:05.9021956Z Build completed unsuccessfully in 0:01:36
2019-09-13T12:49:05.9021956Z Build completed unsuccessfully in 0:01:36
2019-09-13T12:49:05.9071899Z == clock drift check ==
2019-09-13T12:49:05.9088439Z   local time: Fri Sep 13 12:49:05 UTC 2019
2019-09-13T12:49:06.1854476Z   network time: Fri, 13 Sep 2019 12:49:06 GMT
2019-09-13T12:49:06.1855417Z == end clock drift check ==
2019-09-13T12:49:07.6788373Z ##[error]Bash exited with code '1'.
2019-09-13T12:49:07.6820473Z ##[section]Starting: Checkout
2019-09-13T12:49:07.6822249Z ==============================================================================
2019-09-13T12:49:07.6822323Z Task         : Get sources
2019-09-13T12:49:07.6822372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
