plain
2019-12-12T03:37:05.6832011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T03:37:05.6853936Z ##[command]git config gc.auto 0
2019-12-12T03:37:05.6862465Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T03:37:05.6867364Z ##[command]git config --get-all http.proxy
2019-12-12T03:37:05.6872999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-12T03:43:02.0959951Z    Compiling serde_json v1.0.40
2019-12-12T03:43:03.8463074Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-12T03:43:14.8934959Z     Finished release [optimized] target(s) in 1m 27s
2019-12-12T03:43:14.9048803Z tidy check
2019-12-12T03:43:15.7932224Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1566: TODO is deprecated; use FIXME
2019-12-12T03:43:15.8098230Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:485: TODO is deprecated; use FIXME
2019-12-12T03:43:17.5182110Z some tidy checks failed
2019-12-12T03:43:17.5182198Z Found 485 error codes
2019-12-12T03:43:17.5182289Z Found 0 error codes with no tests
2019-12-12T03:43:17.5182330Z Done!
2019-12-12T03:43:17.5182330Z Done!
2019-12-12T03:43:17.5185701Z 
2019-12-12T03:43:17.5186002Z 
2019-12-12T03:43:17.5186940Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-12T03:43:17.5187304Z 
2019-12-12T03:43:17.5187533Z 
2019-12-12T03:43:17.5192434Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-12T03:43:17.5192524Z Build completed unsuccessfully in 0:01:30
2019-12-12T03:43:17.5192524Z Build completed unsuccessfully in 0:01:30
2019-12-12T03:43:17.5245761Z == clock drift check ==
2019-12-12T03:43:17.5255116Z   local time: Thu Dec 12 03:43:17 UTC 2019
2019-12-12T03:43:17.8046591Z   network time: Thu, 12 Dec 2019 03:43:17 GMT
2019-12-12T03:43:17.8050230Z == end clock drift check ==
2019-12-12T03:43:19.7919913Z 
2019-12-12T03:43:19.8005357Z ##[error]Bash exited with code '1'.
2019-12-12T03:43:19.8038079Z ##[section]Starting: Checkout
2019-12-12T03:43:19.8040130Z ==============================================================================
2019-12-12T03:43:19.8040187Z Task         : Get sources
2019-12-12T03:43:19.8040249Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
