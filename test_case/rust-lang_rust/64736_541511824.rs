plain
2019-10-14T05:49:13.3390770Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T05:49:13.3403461Z ##[command]git config gc.auto 0
2019-10-14T05:49:13.3406262Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T05:49:13.3408582Z ##[command]git config --get-all http.proxy
2019-10-14T05:49:13.3411552Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-14T05:55:05.9499204Z    Compiling serde_json v1.0.40
2019-10-14T05:55:07.8695591Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-14T05:55:19.8344238Z     Finished release [optimized] target(s) in 1m 33s
2019-10-14T05:55:19.8427095Z tidy check
2019-10-14T05:55:20.5913948Z tidy error: /checkout/src/librustc/mir/cache.rs:74: line longer than 100 chars
2019-10-14T05:55:20.5914160Z tidy error: /checkout/src/librustc/mir/cache.rs:97: line longer than 100 chars
2019-10-14T05:55:20.6781209Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:385: TODO is deprecated; use FIXME
2019-10-14T05:55:22.1528420Z some tidy checks failed
2019-10-14T05:55:22.1531304Z Found 482 error codes
2019-10-14T05:55:22.1531662Z Found 0 error codes with no tests
2019-10-14T05:55:22.1531957Z Done!
2019-10-14T05:55:22.1531957Z Done!
2019-10-14T05:55:22.1532212Z 
2019-10-14T05:55:22.1532834Z 
2019-10-14T05:55:22.1534162Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-14T05:55:22.1534844Z 
2019-10-14T05:55:22.1535052Z 
2019-10-14T05:55:22.1539862Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-14T05:55:22.1540144Z Build completed unsuccessfully in 0:01:37
2019-10-14T05:55:22.1540144Z Build completed unsuccessfully in 0:01:37
2019-10-14T05:55:22.1603026Z == clock drift check ==
2019-10-14T05:55:22.1622732Z   local time: Mon Oct 14 05:55:22 UTC 2019
2019-10-14T05:55:22.2995653Z   network time: Mon, 14 Oct 2019 05:55:22 GMT
2019-10-14T05:55:22.3000650Z == end clock drift check ==
2019-10-14T05:55:23.1190317Z ##[error]Bash exited with code '1'.
2019-10-14T05:55:23.1227938Z ##[section]Starting: Checkout
2019-10-14T05:55:23.1229970Z ==============================================================================
2019-10-14T05:55:23.1230033Z Task         : Get sources
2019-10-14T05:55:23.1230087Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
