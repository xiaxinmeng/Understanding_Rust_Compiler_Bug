plain
2019-12-21T06:23:23.1513587Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-21T06:23:23.1530688Z ##[command]git config gc.auto 0
2019-12-21T06:23:23.1536605Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-21T06:23:23.1540298Z ##[command]git config --get-all http.proxy
2019-12-21T06:23:23.1544922Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-21T06:29:44.9742341Z    Compiling serde_json v1.0.40
2019-12-21T06:29:46.8303004Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-21T06:29:57.9341604Z     Finished release [optimized] target(s) in 1m 30s
2019-12-21T06:29:57.9450107Z tidy check
2019-12-21T06:29:58.9305457Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1595: TODO is deprecated; use FIXME
2019-12-21T06:29:58.9394990Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:178: TODO is deprecated; use FIXME
2019-12-21T06:29:58.9430374Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:753: TODO is deprecated; use FIXME
2019-12-21T06:30:00.7725483Z some tidy checks failed
2019-12-21T06:30:00.7725597Z Found 485 error codes
2019-12-21T06:30:00.7725652Z Found 0 error codes with no tests
2019-12-21T06:30:00.7725747Z Done!
2019-12-21T06:30:00.7725747Z Done!
2019-12-21T06:30:00.7733164Z 
2019-12-21T06:30:00.7737650Z 
2019-12-21T06:30:00.7740998Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-21T06:30:00.7744925Z 
2019-12-21T06:30:00.7744957Z 
2019-12-21T06:30:00.7751445Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-21T06:30:00.7751514Z Build completed unsuccessfully in 0:01:34
2019-12-21T06:30:00.7751514Z Build completed unsuccessfully in 0:01:34
2019-12-21T06:30:00.7798635Z == clock drift check ==
2019-12-21T06:30:00.7837714Z   local time: Sat Dec 21 06:30:00 UTC 2019
2019-12-21T06:30:01.0749886Z   network time: Sat, 21 Dec 2019 06:30:01 GMT
2019-12-21T06:30:01.0750018Z == end clock drift check ==
2019-12-21T06:30:02.5243032Z 
2019-12-21T06:30:02.5373863Z ##[error]Bash exited with code '1'.
2019-12-21T06:30:02.5413252Z ##[section]Starting: Checkout
2019-12-21T06:30:02.5415361Z ==============================================================================
2019-12-21T06:30:02.5415423Z Task         : Get sources
2019-12-21T06:30:02.5415477Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
