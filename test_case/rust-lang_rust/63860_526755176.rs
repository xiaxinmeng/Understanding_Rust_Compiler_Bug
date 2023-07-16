plain
2019-08-30T21:15:44.5457797Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T21:15:44.5617229Z ##[command]git config gc.auto 0
2019-08-30T21:15:44.5684898Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T21:15:44.5739994Z ##[command]git config --get-all http.proxy
2019-08-30T21:15:44.5876740Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-30T21:22:29.3052211Z    Compiling serde_json v1.0.40
2019-08-30T21:22:31.1492563Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-30T21:22:41.8893652Z     Finished release [optimized] target(s) in 1m 28s
2019-08-30T21:22:41.8971965Z tidy check
2019-08-30T21:22:42.3856660Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:983: line longer than 100 chars
2019-08-30T21:22:42.3857318Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:1112: TODO is deprecated; use FIXME
2019-08-30T21:22:43.8450048Z some tidy checks failed
2019-08-30T21:22:43.8454365Z 
2019-08-30T21:22:43.8454365Z 
2019-08-30T21:22:43.8455811Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-30T21:22:43.8456235Z 
2019-08-30T21:22:43.8456362Z 
2019-08-30T21:22:43.8461866Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-30T21:22:43.8462164Z Build completed unsuccessfully in 0:01:31
2019-08-30T21:22:43.8462164Z Build completed unsuccessfully in 0:01:31
2019-08-30T21:22:43.8509847Z == clock drift check ==
2019-08-30T21:22:43.8528653Z   local time: Fri Aug 30 21:22:43 UTC 2019
2019-08-30T21:22:44.0013996Z   network time: Fri, 30 Aug 2019 21:22:43 GMT
2019-08-30T21:22:44.0016338Z == end clock drift check ==
2019-08-30T21:22:45.3210337Z ##[error]Bash exited with code '1'.
2019-08-30T21:22:45.3241535Z ##[section]Starting: Checkout
2019-08-30T21:22:45.3243439Z ==============================================================================
2019-08-30T21:22:45.3243509Z Task         : Get sources
2019-08-30T21:22:45.3243551Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
