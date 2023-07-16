plain
2019-10-08T16:14:17.8059985Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T16:14:17.8254716Z ##[command]git config gc.auto 0
2019-10-08T16:14:17.8338645Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T16:14:17.8411264Z ##[command]git config --get-all http.proxy
2019-10-08T16:14:17.8559127Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65214/merge:refs/remotes/pull/65214/merge
---
2019-10-08T16:21:11.9140582Z    Compiling serde_json v1.0.40
2019-10-08T16:21:13.5519239Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-08T16:21:23.9120554Z     Finished release [optimized] target(s) in 1m 22s
2019-10-08T16:21:23.9206981Z tidy check
2019-10-08T16:21:24.5873263Z tidy error: /checkout/src/libcore/sync/atomic.rs:116: line longer than 100 chars
2019-10-08T16:21:24.5873900Z tidy error: /checkout/src/libcore/sync/atomic.rs:117: line longer than 100 chars
2019-10-08T16:21:25.7358083Z some tidy checks failed
2019-10-08T16:21:25.7358305Z 
2019-10-08T16:21:25.7358305Z 
2019-10-08T16:21:25.7359141Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-08T16:21:25.7359251Z 
2019-10-08T16:21:25.7359271Z 
2019-10-08T16:21:25.7414277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-08T16:21:25.7414367Z Build completed unsuccessfully in 0:01:25
2019-10-08T16:21:25.7414367Z Build completed unsuccessfully in 0:01:25
2019-10-08T16:21:25.7418806Z == clock drift check ==
2019-10-08T16:21:25.7437917Z   local time: Tue Oct  8 16:21:25 UTC 2019
2019-10-08T16:21:25.8399874Z   network time: Tue, 08 Oct 2019 16:21:25 GMT
2019-10-08T16:21:25.8407225Z == end clock drift check ==
2019-10-08T16:21:27.2358213Z ##[error]Bash exited with code '1'.
2019-10-08T16:21:27.2397885Z ##[section]Starting: Checkout
2019-10-08T16:21:27.2399370Z ==============================================================================
2019-10-08T16:21:27.2399427Z Task         : Get sources
2019-10-08T16:21:27.2399462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
