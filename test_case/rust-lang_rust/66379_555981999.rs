plain
2019-11-20T12:08:08.1608369Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T12:08:08.1783487Z ##[command]git config gc.auto 0
2019-11-20T12:08:08.1844342Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T12:08:08.1895263Z ##[command]git config --get-all http.proxy
2019-11-20T12:08:08.2019859Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-20T12:13:25.8380907Z    Compiling serde_json v1.0.40
2019-11-20T12:13:27.2975860Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-20T12:13:37.0068794Z     Finished release [optimized] target(s) in 1m 18s
2019-11-20T12:13:37.0160919Z tidy check
2019-11-20T12:13:37.7907906Z tidy error: /checkout/src/libcore/ptr/mod.rs:1077: trailing whitespace
2019-11-20T12:13:37.7908037Z tidy error: /checkout/src/libcore/ptr/mod.rs:1080: trailing whitespace
2019-11-20T12:13:37.7908081Z tidy error: /checkout/src/libcore/ptr/mod.rs:1088: trailing whitespace
2019-11-20T12:13:37.7908132Z tidy error: /checkout/src/libcore/ptr/mod.rs:1089: trailing whitespace
2019-11-20T12:13:37.7917252Z tidy error: /checkout/src/libcore/ptr/mod.rs:1938: trailing whitespace
2019-11-20T12:13:37.7917325Z tidy error: /checkout/src/libcore/ptr/mod.rs:1946: trailing whitespace
2019-11-20T12:13:37.7917367Z tidy error: /checkout/src/libcore/ptr/mod.rs:1947: trailing whitespace
2019-11-20T12:13:37.7917427Z tidy error: /checkout/src/libcore/ptr/mod.rs:1948: line longer than 100 chars
2019-11-20T12:13:39.2674347Z some tidy checks failed
2019-11-20T12:13:39.2675146Z Found 441 error codes
2019-11-20T12:13:39.2675402Z Found 0 error codes with no tests
2019-11-20T12:13:39.2675679Z Done!
2019-11-20T12:13:39.2675679Z Done!
2019-11-20T12:13:39.2675889Z 
2019-11-20T12:13:39.2676081Z 
2019-11-20T12:13:39.2677167Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-20T12:13:39.2677860Z 
2019-11-20T12:13:39.2678001Z 
2019-11-20T12:13:39.2681029Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-20T12:13:39.2681276Z Build completed unsuccessfully in 0:01:21
2019-11-20T12:13:39.2681276Z Build completed unsuccessfully in 0:01:21
2019-11-20T12:13:39.2728008Z == clock drift check ==
2019-11-20T12:13:39.2737488Z   local time: Wed Nov 20 12:13:39 UTC 2019
2019-11-20T12:13:40.1915907Z   network time: Wed, 20 Nov 2019 12:13:39 GMT
2019-11-20T12:13:40.1917000Z == end clock drift check ==
2019-11-20T12:13:40.8688833Z 
2019-11-20T12:13:40.8749241Z ##[error]Bash exited with code '1'.
2019-11-20T12:13:40.8774442Z ##[section]Starting: Checkout
2019-11-20T12:13:40.8776138Z ==============================================================================
2019-11-20T12:13:40.8776195Z Task         : Get sources
2019-11-20T12:13:40.8776430Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
