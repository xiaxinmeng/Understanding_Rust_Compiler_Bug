plain
2019-10-05T19:01:51.9385543Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T19:01:51.9635981Z ##[command]git config gc.auto 0
2019-10-05T19:01:51.9678974Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T19:01:51.9738747Z ##[command]git config --get-all http.proxy
2019-10-05T19:01:51.9913306Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64588/merge:refs/remotes/pull/64588/merge
---
2019-10-05T19:08:54.2166929Z    Compiling serde_json v1.0.40
2019-10-05T19:08:56.0124839Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-05T19:09:07.2658582Z     Finished release [optimized] target(s) in 1m 30s
2019-10-05T19:09:07.2762701Z tidy check
2019-10-05T19:09:07.7153479Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:802: line longer than 100 chars
2019-10-05T19:09:09.7923547Z some tidy checks failed
2019-10-05T19:09:09.7925231Z 
2019-10-05T19:09:09.7925231Z 
2019-10-05T19:09:09.7926988Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-05T19:09:09.7927682Z 
2019-10-05T19:09:09.7927902Z 
2019-10-05T19:09:09.7928076Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-05T19:09:09.7928230Z Build completed unsuccessfully in 0:01:33
2019-10-05T19:09:09.7928230Z Build completed unsuccessfully in 0:01:33
2019-10-05T19:09:09.7928402Z == clock drift check ==
2019-10-05T19:09:09.7928555Z   local time: Sat Oct  5 19:09:09 UTC 2019
2019-10-05T19:09:09.7928705Z   network time: Sat, 05 Oct 2019 19:09:09 GMT
2019-10-05T19:09:09.7928874Z == end clock drift check ==
2019-10-05T19:09:10.7997550Z ##[error]Bash exited with code '1'.
2019-10-05T19:09:10.8089090Z ##[section]Starting: Checkout
2019-10-05T19:09:10.8091037Z ==============================================================================
2019-10-05T19:09:10.8091099Z Task         : Get sources
2019-10-05T19:09:10.8091148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
