plain
2019-10-03T23:10:54.7568197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T23:10:54.7775060Z ##[command]git config gc.auto 0
2019-10-03T23:10:54.7858069Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T23:10:54.7924593Z ##[command]git config --get-all http.proxy
2019-10-03T23:10:54.8085253Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65075/merge:refs/remotes/pull/65075/merge
---
2019-10-03T23:18:09.8425559Z    Compiling serde_json v1.0.40
2019-10-03T23:18:11.7351466Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-03T23:18:23.2740593Z     Finished release [optimized] target(s) in 1m 31s
2019-10-03T23:18:23.2828991Z tidy check
2019-10-03T23:18:24.2127357Z tidy error: duplicate error code: 735
2019-10-03T23:18:24.2128383Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:1709: E0735: r##"
2019-10-03T23:18:24.2128744Z tidy error: /checkout/src/librustc/error_codes.rs:2308:     E0735, // invalid track_caller application/syntax
2019-10-03T23:18:25.8339569Z some tidy checks failed
2019-10-03T23:18:25.8341003Z 
2019-10-03T23:18:25.8341003Z 
2019-10-03T23:18:25.8342070Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T23:18:25.8342497Z 
2019-10-03T23:18:25.8342626Z 
2019-10-03T23:18:25.8343483Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T23:18:25.8343969Z Build completed unsuccessfully in 0:01:34
2019-10-03T23:18:25.8343969Z Build completed unsuccessfully in 0:01:34
2019-10-03T23:18:25.8344101Z == clock drift check ==
2019-10-03T23:18:25.8344408Z   local time: Thu Oct  3 23:18:25 UTC 2019
2019-10-03T23:18:27.7654267Z   network time: Thu, 03 Oct 2019 23:18:27 GMT
2019-10-03T23:18:27.7655977Z == end clock drift check ==
2019-10-03T23:18:29.4061823Z ##[error]Bash exited with code '1'.
2019-10-03T23:18:29.4096315Z ##[section]Starting: Checkout
2019-10-03T23:18:29.4098590Z ==============================================================================
2019-10-03T23:18:29.4098651Z Task         : Get sources
2019-10-03T23:18:29.4098720Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
