plain
2019-11-08T16:12:34.1738940Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T16:12:34.1930451Z ##[command]git config gc.auto 0
2019-11-08T16:12:34.2364565Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T16:12:34.2422367Z ##[command]git config --get-all http.proxy
2019-11-08T16:12:34.2566970Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66204/merge:refs/remotes/pull/66204/merge
---
2019-11-08T16:18:43.5000117Z    Compiling serde_json v1.0.40
2019-11-08T16:18:45.3114973Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-08T16:18:57.0277802Z     Finished release [optimized] target(s) in 1m 30s
2019-11-08T16:18:57.0357660Z tidy check
2019-11-08T16:18:57.6019596Z tidy error: /checkout/src/librustc_target/spec/x86_64_apple_darwin.rs:9: line longer than 100 chars
2019-11-08T16:18:59.8162461Z Found 485 error codes
2019-11-08T16:18:59.8163299Z Found 0 error codes with no tests
2019-11-08T16:18:59.8163550Z Done!
2019-11-08T16:18:59.8163885Z some tidy checks failed
2019-11-08T16:18:59.8163885Z some tidy checks failed
2019-11-08T16:18:59.8170255Z 
2019-11-08T16:18:59.8170370Z 
2019-11-08T16:18:59.8172001Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-08T16:18:59.8172460Z 
2019-11-08T16:18:59.8172489Z 
2019-11-08T16:18:59.8178195Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-08T16:18:59.8179447Z Build completed unsuccessfully in 0:01:34
2019-11-08T16:18:59.8179447Z Build completed unsuccessfully in 0:01:34
2019-11-08T16:18:59.8229667Z == clock drift check ==
2019-11-08T16:18:59.8246106Z   local time: Fri Nov  8 16:18:59 UTC 2019
2019-11-08T16:18:59.9155856Z   network time: Fri, 08 Nov 2019 16:18:59 GMT
2019-11-08T16:18:59.9159091Z == end clock drift check ==
2019-11-08T16:19:01.1952144Z 
2019-11-08T16:19:01.2050815Z ##[error]Bash exited with code '1'.
2019-11-08T16:19:01.2079662Z ##[section]Starting: Checkout
2019-11-08T16:19:01.2081852Z ==============================================================================
2019-11-08T16:19:01.2081912Z Task         : Get sources
2019-11-08T16:19:01.2081962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
