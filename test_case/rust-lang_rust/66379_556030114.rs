plain
2019-11-20T14:19:31.2068214Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T14:19:31.2272213Z ##[command]git config gc.auto 0
2019-11-20T14:19:31.2351900Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T14:19:31.2413505Z ##[command]git config --get-all http.proxy
2019-11-20T14:19:31.2588324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-20T14:25:17.1187220Z    Compiling serde_json v1.0.40
2019-11-20T14:25:18.8867314Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-20T14:25:29.8287806Z     Finished release [optimized] target(s) in 1m 28s
2019-11-20T14:25:29.8426777Z tidy check
2019-11-20T14:25:30.7836759Z tidy error: /checkout/src/libcore/ptr/mod.rs:1948: line longer than 100 chars
2019-11-20T14:25:32.5739249Z Found 441 error codes
2019-11-20T14:25:32.5740443Z Found 0 error codes with no tests
2019-11-20T14:25:32.5740858Z Done!
2019-11-20T14:25:32.5740949Z some tidy checks failed
2019-11-20T14:25:32.5740949Z some tidy checks failed
2019-11-20T14:25:32.5748159Z 
2019-11-20T14:25:32.5748288Z 
2019-11-20T14:25:32.5749179Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-20T14:25:32.5749320Z 
2019-11-20T14:25:32.5749347Z 
2019-11-20T14:25:32.5761159Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-20T14:25:32.5761243Z Build completed unsuccessfully in 0:01:31
2019-11-20T14:25:32.5761243Z Build completed unsuccessfully in 0:01:31
2019-11-20T14:25:32.5815795Z == clock drift check ==
2019-11-20T14:25:32.5840307Z   local time: Wed Nov 20 14:25:32 UTC 2019
2019-11-20T14:25:32.8595891Z   network time: Wed, 20 Nov 2019 14:25:32 GMT
2019-11-20T14:25:32.8596684Z == end clock drift check ==
2019-11-20T14:25:34.0996380Z 
2019-11-20T14:25:34.1105013Z ##[error]Bash exited with code '1'.
2019-11-20T14:25:34.1134471Z ##[section]Starting: Checkout
2019-11-20T14:25:34.1136315Z ==============================================================================
2019-11-20T14:25:34.1136367Z Task         : Get sources
2019-11-20T14:25:34.1136434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
