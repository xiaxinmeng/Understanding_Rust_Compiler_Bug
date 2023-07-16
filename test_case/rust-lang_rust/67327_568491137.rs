plain
2019-12-23T14:14:12.5158371Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T14:14:13.0470258Z ##[command]git config gc.auto 0
2019-12-23T14:14:13.0472662Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T14:14:13.0479642Z ##[command]git config --get-all http.proxy
2019-12-23T14:14:13.0484030Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67327/merge:refs/remotes/pull/67327/merge
---
2019-12-23T14:20:24.5092155Z    Compiling serde_json v1.0.40
2019-12-23T14:20:26.2404123Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-23T14:20:37.4323510Z     Finished release [optimized] target(s) in 1m 29s
2019-12-23T14:20:37.4435677Z tidy check
2019-12-23T14:20:38.4260296Z tidy error: /checkout/src/librustc_mir/const_eval/eval_queries.rs:304: line longer than 100 chars
2019-12-23T14:20:38.4261200Z tidy error: /checkout/src/librustc_mir/const_eval/eval_queries.rs:307: line longer than 100 chars
2019-12-23T14:20:38.4261552Z tidy error: /checkout/src/librustc_mir/const_eval/eval_queries.rs:318: line longer than 100 chars
2019-12-23T14:20:38.4261842Z tidy error: /checkout/src/librustc_mir/const_eval/eval_queries.rs:319: line longer than 100 chars
2019-12-23T14:20:38.4262109Z tidy error: /checkout/src/librustc_mir/const_eval/eval_queries.rs:336: line longer than 100 chars
2019-12-23T14:20:40.2660636Z some tidy checks failed
2019-12-23T14:20:40.2661529Z Found 485 error codes
2019-12-23T14:20:40.2662055Z Found 0 error codes with no tests
2019-12-23T14:20:40.2662297Z Done!
2019-12-23T14:20:40.2662297Z Done!
2019-12-23T14:20:40.2662498Z 
2019-12-23T14:20:40.2662697Z 
2019-12-23T14:20:40.2664063Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-23T14:20:40.2664654Z 
2019-12-23T14:20:40.2664851Z 
2019-12-23T14:20:40.2670218Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-23T14:20:40.2670589Z Build completed unsuccessfully in 0:01:40
2019-12-23T14:20:40.2670589Z Build completed unsuccessfully in 0:01:40
2019-12-23T14:20:40.2727693Z == clock drift check ==
2019-12-23T14:20:40.2736371Z   local time: Mon Dec 23 14:20:40 UTC 2019
2019-12-23T14:20:40.5587314Z   network time: Mon, 23 Dec 2019 14:20:40 GMT
2019-12-23T14:20:40.5587438Z == end clock drift check ==
2019-12-23T14:20:41.9447104Z 
2019-12-23T14:20:41.9548048Z ##[error]Bash exited with code '1'.
2019-12-23T14:20:41.9607682Z ##[section]Starting: Checkout
2019-12-23T14:20:41.9609448Z ==============================================================================
2019-12-23T14:20:41.9609504Z Task         : Get sources
2019-12-23T14:20:41.9609551Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
