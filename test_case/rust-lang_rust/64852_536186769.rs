plain
2019-09-28T12:27:35.5117070Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T12:27:35.5337762Z ##[command]git config gc.auto 0
2019-09-28T12:27:35.5401628Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T12:27:35.5454985Z ##[command]git config --get-all http.proxy
2019-09-28T12:27:35.5590869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64852/merge:refs/remotes/pull/64852/merge
---
2019-09-28T12:34:20.9057652Z    Compiling serde_json v1.0.40
2019-09-28T12:34:22.7945895Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-28T12:34:34.0335171Z     Finished release [optimized] target(s) in 1m 29s
2019-09-28T12:34:34.0427188Z tidy check
2019-09-28T12:34:34.8066741Z tidy error: /checkout/src/librustc_typeck/check/expr.rs:1417: trailing whitespace
2019-09-28T12:34:36.0770378Z some tidy checks failed
2019-09-28T12:34:36.0771404Z 
2019-09-28T12:34:36.0771404Z 
2019-09-28T12:34:36.0772347Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-28T12:34:36.0773071Z 
2019-09-28T12:34:36.0773143Z 
2019-09-28T12:34:36.0816221Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-28T12:34:36.0816304Z Build completed unsuccessfully in 0:01:33
2019-09-28T12:34:36.0816304Z Build completed unsuccessfully in 0:01:33
2019-09-28T12:34:36.0839954Z == clock drift check ==
2019-09-28T12:34:36.0857348Z   local time: Sat Sep 28 12:34:36 UTC 2019
2019-09-28T12:34:36.2491333Z   network time: Sat, 28 Sep 2019 12:34:36 GMT
2019-09-28T12:34:36.2491454Z == end clock drift check ==
2019-09-28T12:34:37.6665397Z ##[error]Bash exited with code '1'.
2019-09-28T12:34:37.6710299Z ##[section]Starting: Checkout
2019-09-28T12:34:37.6712201Z ==============================================================================
2019-09-28T12:34:37.6712258Z Task         : Get sources
2019-09-28T12:34:37.6712307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
