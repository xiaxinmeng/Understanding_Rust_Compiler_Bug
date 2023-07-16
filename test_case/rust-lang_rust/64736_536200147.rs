plain
2019-09-28T15:19:52.3296849Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T15:19:52.3504968Z ##[command]git config gc.auto 0
2019-09-28T15:19:52.3588205Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T15:19:52.3651080Z ##[command]git config --get-all http.proxy
2019-09-28T15:19:52.3792332Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-28T15:26:58.1602292Z    Compiling serde_json v1.0.40
2019-09-28T15:27:00.0563192Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-28T15:27:11.3834856Z     Finished release [optimized] target(s) in 1m 30s
2019-09-28T15:27:11.3919434Z tidy check
2019-09-28T15:27:11.8427510Z tidy error: /checkout/src/librustc_mir/transform/simplify.rs:280: line longer than 100 chars
2019-09-28T15:27:12.0614326Z tidy error: /checkout/src/librustc/mir/visit.rs:850: TODO is deprecated; use FIXME
2019-09-28T15:27:12.0646391Z tidy error: /checkout/src/librustc/mir/mod.rs:216: line longer than 100 chars
2019-09-28T15:27:12.0646990Z tidy error: /checkout/src/librustc/mir/mod.rs:266: TODO is deprecated; use FIXME
2019-09-28T15:27:13.3602466Z some tidy checks failed
2019-09-28T15:27:13.3612593Z 
2019-09-28T15:27:13.3612593Z 
2019-09-28T15:27:13.3614305Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-28T15:27:13.3614712Z 
2019-09-28T15:27:13.3614739Z 
2019-09-28T15:27:13.3614816Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-28T15:27:13.3614907Z Build completed unsuccessfully in 0:01:33
2019-09-28T15:27:13.3614907Z Build completed unsuccessfully in 0:01:33
2019-09-28T15:27:13.3659794Z == clock drift check ==
2019-09-28T15:27:13.3678089Z   local time: Sat Sep 28 15:27:13 UTC 2019
2019-09-28T15:27:13.4871413Z   network time: Sat, 28 Sep 2019 15:27:13 GMT
2019-09-28T15:27:13.4875180Z == end clock drift check ==
2019-09-28T15:27:14.8721107Z ##[error]Bash exited with code '1'.
2019-09-28T15:27:14.8759701Z ##[section]Starting: Checkout
2019-09-28T15:27:14.8761652Z ==============================================================================
2019-09-28T15:27:14.8761727Z Task         : Get sources
2019-09-28T15:27:14.8761776Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
