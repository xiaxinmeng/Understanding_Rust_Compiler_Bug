plain
2019-09-19T04:49:06.6097939Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T04:49:06.6323710Z ##[command]git config gc.auto 0
2019-09-19T04:49:06.6416443Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T04:49:06.6490425Z ##[command]git config --get-all http.proxy
2019-09-19T04:49:06.6635076Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64342/merge:refs/remotes/pull/64342/merge
---
2019-09-19T04:55:49.2124589Z    Compiling serde_json v1.0.40
2019-09-19T04:55:50.9044647Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-19T04:56:01.3973613Z     Finished release [optimized] target(s) in 1m 27s
2019-09-19T04:56:01.4052005Z tidy check
2019-09-19T04:56:01.5346802Z tidy error: /checkout/src/librustc_lint/unused.rs:205: line longer than 100 chars
2019-09-19T04:56:03.3185279Z some tidy checks failed
2019-09-19T04:56:03.3187882Z 
2019-09-19T04:56:03.3187882Z 
2019-09-19T04:56:03.3188940Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-19T04:56:03.3189123Z 
2019-09-19T04:56:03.3189150Z 
2019-09-19T04:56:03.3200031Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-19T04:56:03.3200339Z Build completed unsuccessfully in 0:01:30
2019-09-19T04:56:03.3200339Z Build completed unsuccessfully in 0:01:30
2019-09-19T04:56:03.3248038Z == clock drift check ==
2019-09-19T04:56:03.3263756Z   local time: Thu Sep 19 04:56:03 UTC 2019
2019-09-19T04:56:03.4103936Z   network time: Thu, 19 Sep 2019 04:56:03 GMT
2019-09-19T04:56:03.4108431Z == end clock drift check ==
2019-09-19T04:56:04.7393262Z ##[error]Bash exited with code '1'.
2019-09-19T04:56:04.7425163Z ##[section]Starting: Checkout
2019-09-19T04:56:04.7426662Z ==============================================================================
2019-09-19T04:56:04.7426881Z Task         : Get sources
2019-09-19T04:56:04.7426937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
