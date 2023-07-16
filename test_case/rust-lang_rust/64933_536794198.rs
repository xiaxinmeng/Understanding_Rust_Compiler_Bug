plain
2019-09-30T23:11:49.3295548Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T23:11:49.3468197Z ##[command]git config gc.auto 0
2019-09-30T23:11:49.3549657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T23:11:49.3605905Z ##[command]git config --get-all http.proxy
2019-09-30T23:11:49.3747692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64933/merge:refs/remotes/pull/64933/merge
---
2019-09-30T23:19:10.7286040Z    Compiling serde_json v1.0.40
2019-09-30T23:19:12.5286269Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-30T23:19:23.9643600Z     Finished release [optimized] target(s) in 1m 31s
2019-09-30T23:19:23.9724490Z tidy check
2019-09-30T23:19:24.8434594Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:4348: line longer than 100 chars
2019-09-30T23:19:24.8434792Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:4349: line longer than 100 chars
2019-09-30T23:19:26.0789513Z some tidy checks failed
2019-09-30T23:19:26.0790510Z 
2019-09-30T23:19:26.0790510Z 
2019-09-30T23:19:26.0791693Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-30T23:19:26.0797371Z 
2019-09-30T23:19:26.0797427Z 
2019-09-30T23:19:26.0808541Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-30T23:19:26.0808866Z Build completed unsuccessfully in 0:01:34
2019-09-30T23:19:26.0808866Z Build completed unsuccessfully in 0:01:34
2019-09-30T23:19:26.0865271Z == clock drift check ==
2019-09-30T23:19:26.0880568Z   local time: Mon Sep 30 23:19:26 UTC 2019
2019-09-30T23:19:26.1726195Z   network time: Mon, 30 Sep 2019 23:19:26 GMT
2019-09-30T23:19:26.1729297Z == end clock drift check ==
2019-09-30T23:19:28.0404474Z ##[error]Bash exited with code '1'.
2019-09-30T23:19:28.0440935Z ##[section]Starting: Checkout
2019-09-30T23:19:28.0443110Z ==============================================================================
2019-09-30T23:19:28.0443182Z Task         : Get sources
2019-09-30T23:19:28.0443382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
