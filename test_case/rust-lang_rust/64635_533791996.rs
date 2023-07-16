plain
2019-09-21T11:31:52.6200433Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T11:31:52.6392441Z ##[command]git config gc.auto 0
2019-09-21T11:31:53.1606103Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T11:31:53.1612588Z ##[command]git config --get-all http.proxy
2019-09-21T11:31:53.1618779Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64635/merge:refs/remotes/pull/64635/merge
---
2019-09-21T11:38:59.5847844Z     Finished release [optimized] target(s) in 1m 23s
2019-09-21T11:38:59.5935408Z tidy check
2019-09-21T11:39:00.4488759Z * 578 error codes
2019-09-21T11:39:00.4488879Z * highest error code: E0733
2019-09-21T11:39:00.4821032Z tidy error: /checkout/src/libsyntax/feature_gate/active.rs:412: feature const_fn_union is not sorted by since
2019-09-21T11:39:01.4035625Z some tidy checks failed
2019-09-21T11:39:01.4041598Z 
2019-09-21T11:39:01.4041598Z 
2019-09-21T11:39:01.4042468Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-21T11:39:01.4043148Z 
2019-09-21T11:39:01.4043174Z 
2019-09-21T11:39:01.4049308Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-21T11:39:01.4049600Z Build completed unsuccessfully in 0:01:26
2019-09-21T11:39:01.4049600Z Build completed unsuccessfully in 0:01:26
2019-09-21T11:39:01.4094993Z == clock drift check ==
2019-09-21T11:39:01.4111294Z   local time: Sat Sep 21 11:39:01 UTC 2019
2019-09-21T11:39:01.5603227Z   network time: Sat, 21 Sep 2019 11:39:01 GMT
2019-09-21T11:39:01.5607225Z == end clock drift check ==
2019-09-21T11:39:03.2497270Z ##[error]Bash exited with code '1'.
2019-09-21T11:39:03.2539979Z ##[section]Starting: Checkout
2019-09-21T11:39:03.2541459Z ==============================================================================
2019-09-21T11:39:03.2541504Z Task         : Get sources
2019-09-21T11:39:03.2541555Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
