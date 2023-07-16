plain
2019-12-18T18:07:46.4039270Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T18:07:46.4239849Z ##[command]git config gc.auto 0
2019-12-18T18:07:46.4291467Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T18:07:46.4339580Z ##[command]git config --get-all http.proxy
2019-12-18T18:07:46.4475871Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-18T18:13:24.3530337Z    Compiling serde_json v1.0.40
2019-12-18T18:13:25.8476398Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-18T18:13:35.3532412Z     Finished release [optimized] target(s) in 1m 15s
2019-12-18T18:13:35.3626048Z tidy check
2019-12-18T18:13:36.1896627Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1570: TODO is deprecated; use FIXME
2019-12-18T18:13:36.1897439Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1572: line longer than 100 chars
2019-12-18T18:13:37.7559087Z Found 485 error codes
2019-12-18T18:13:37.7560923Z Found 0 error codes with no tests
2019-12-18T18:13:37.7561185Z Done!
2019-12-18T18:13:37.7561391Z some tidy checks failed
2019-12-18T18:13:37.7561391Z some tidy checks failed
2019-12-18T18:13:37.7561557Z 
2019-12-18T18:13:37.7561717Z 
2019-12-18T18:13:37.7562896Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-18T18:13:37.7563524Z 
2019-12-18T18:13:37.7563644Z 
2019-12-18T18:13:37.7571732Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-18T18:13:37.7571995Z Build completed unsuccessfully in 0:01:19
2019-12-18T18:13:37.7571995Z Build completed unsuccessfully in 0:01:19
2019-12-18T18:13:37.7625057Z == clock drift check ==
2019-12-18T18:13:37.7635201Z   local time: Wed Dec 18 18:13:37 UTC 2019
2019-12-18T18:13:38.0427914Z   network time: Wed, 18 Dec 2019 18:13:38 GMT
2019-12-18T18:13:38.0431958Z == end clock drift check ==
2019-12-18T18:13:39.5388701Z 
2019-12-18T18:13:39.5513860Z ##[error]Bash exited with code '1'.
2019-12-18T18:13:39.5541277Z ##[section]Starting: Checkout
2019-12-18T18:13:39.5543017Z ==============================================================================
2019-12-18T18:13:39.5543097Z Task         : Get sources
2019-12-18T18:13:39.5543146Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
