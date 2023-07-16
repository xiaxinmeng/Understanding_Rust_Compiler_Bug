plain
2019-09-25T01:30:38.4764890Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T01:30:38.4946454Z ##[command]git config gc.auto 0
2019-09-25T01:30:39.0132450Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T01:30:39.0134883Z ##[command]git config --get-all http.proxy
2019-09-25T01:30:39.0137915Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-25T01:38:58.2729467Z    Compiling serde_json v1.0.40
2019-09-25T01:39:00.6653862Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-25T01:39:11.3646400Z     Finished release [optimized] target(s) in 1m 32s
2019-09-25T01:39:11.3737582Z tidy check
2019-09-25T01:39:11.7109249Z tidy error: /checkout/src/librustc/mir/mod.rs:257: TODO is deprecated; use FIXME
2019-09-25T01:39:13.4020179Z some tidy checks failed
2019-09-25T01:39:13.4022898Z 
2019-09-25T01:39:13.4022898Z 
2019-09-25T01:39:13.4024798Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-25T01:39:13.4025573Z 
2019-09-25T01:39:13.4025705Z 
2019-09-25T01:39:13.4037475Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-25T01:39:13.4037870Z Build completed unsuccessfully in 0:01:35
2019-09-25T01:39:13.4037870Z Build completed unsuccessfully in 0:01:35
2019-09-25T01:39:13.4094495Z == clock drift check ==
2019-09-25T01:39:13.4109191Z   local time: Wed Sep 25 01:39:13 UTC 2019
2019-09-25T01:39:13.6894193Z   network time: Wed, 25 Sep 2019 01:39:13 GMT
2019-09-25T01:39:13.6897026Z == end clock drift check ==
2019-09-25T01:39:15.2912747Z ##[error]Bash exited with code '1'.
2019-09-25T01:39:15.3068814Z ##[section]Starting: Checkout
2019-09-25T01:39:15.3070576Z ==============================================================================
2019-09-25T01:39:15.3070789Z Task         : Get sources
2019-09-25T01:39:15.3070851Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
