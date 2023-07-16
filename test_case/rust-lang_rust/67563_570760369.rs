plain
2020-01-04T05:51:21.0056570Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-04T05:51:21.0260941Z ##[command]git config gc.auto 0
2020-01-04T05:51:21.0359078Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-04T05:51:21.0405900Z ##[command]git config --get-all http.proxy
2020-01-04T05:51:21.0552903Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67563/merge:refs/remotes/pull/67563/merge
---
2020-01-04T05:57:36.9104904Z    Compiling serde_json v1.0.40
2020-01-04T05:57:38.5776655Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-04T05:57:48.2512362Z     Finished release [optimized] target(s) in 1m 21s
2020-01-04T05:57:48.2614231Z tidy check
2020-01-04T05:57:49.3433339Z tidy error: /checkout/src/librustdoc/passes/check_code_block_syntax.rs:103: TODO is deprecated; use FIXME
2020-01-04T05:57:51.0887327Z some tidy checks failed
2020-01-04T05:57:51.0888294Z Found 486 error codes
2020-01-04T05:57:51.0888747Z Found 0 error codes with no tests
2020-01-04T05:57:51.0889040Z Done!
2020-01-04T05:57:51.0889040Z Done!
2020-01-04T05:57:51.0899771Z 
2020-01-04T05:57:51.0900165Z 
2020-01-04T05:57:51.0901472Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-04T05:57:51.0902048Z 
2020-01-04T05:57:51.0902266Z 
2020-01-04T05:57:51.0943007Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-04T05:57:51.0943568Z Build completed unsuccessfully in 0:01:32
2020-01-04T05:57:51.0943568Z Build completed unsuccessfully in 0:01:32
2020-01-04T05:57:51.0977916Z == clock drift check ==
2020-01-04T05:57:51.0985976Z   local time: Sat Jan  4 05:57:51 UTC 2020
2020-01-04T05:57:51.2500141Z   network time: Sat, 04 Jan 2020 05:57:51 GMT
2020-01-04T05:57:51.2504714Z == end clock drift check ==
2020-01-04T05:57:52.6459741Z 
2020-01-04T05:57:52.6581066Z ##[error]Bash exited with code '1'.
2020-01-04T05:57:52.6611593Z ##[section]Starting: Checkout
2020-01-04T05:57:52.6613351Z ==============================================================================
2020-01-04T05:57:52.6613413Z Task         : Get sources
2020-01-04T05:57:52.6613482Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
