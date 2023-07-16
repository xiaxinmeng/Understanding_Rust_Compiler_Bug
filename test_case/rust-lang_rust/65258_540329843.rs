plain
2019-10-10T03:19:04.4734466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T03:19:04.4778537Z ##[command]git config gc.auto 0
2019-10-10T03:19:04.4862679Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T03:19:04.4912689Z ##[command]git config --get-all http.proxy
2019-10-10T03:19:04.5052598Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65258/merge:refs/remotes/pull/65258/merge
---
2019-10-10T03:25:14.0966195Z    Compiling serde_json v1.0.40
2019-10-10T03:25:15.9617580Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-10T03:25:27.8474790Z     Finished release [optimized] target(s) in 1m 32s
2019-10-10T03:25:27.8552224Z tidy check
2019-10-10T03:25:28.1710078Z tidy error: /checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs: missing trailing newline
2019-10-10T03:25:28.3893742Z tidy error: /checkout/src/librustc_mir/shim.rs:48: TODO is deprecated; use FIXME
2019-10-10T03:25:30.0743002Z some tidy checks failed
2019-10-10T03:25:30.0743230Z Found 482 error codes
2019-10-10T03:25:30.0743285Z Found 0 error codes with no tests
2019-10-10T03:25:30.0743334Z Done!
2019-10-10T03:25:30.0743334Z Done!
2019-10-10T03:25:30.0743368Z 
2019-10-10T03:25:30.0743399Z 
2019-10-10T03:25:30.0747525Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-10T03:25:30.0747678Z 
2019-10-10T03:25:30.0747750Z 
2019-10-10T03:25:30.0748806Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-10T03:25:30.0748874Z Build completed unsuccessfully in 0:01:36
2019-10-10T03:25:30.0748874Z Build completed unsuccessfully in 0:01:36
2019-10-10T03:25:30.0806069Z == clock drift check ==
2019-10-10T03:25:30.0834454Z   local time: Thu Oct 10 03:25:30 UTC 2019
2019-10-10T03:25:30.2447378Z   network time: Thu, 10 Oct 2019 03:25:30 GMT
2019-10-10T03:25:30.2449800Z == end clock drift check ==
2019-10-10T03:25:31.0355117Z ##[error]Bash exited with code '1'.
2019-10-10T03:25:31.0414843Z ##[section]Starting: Checkout
2019-10-10T03:25:31.0417086Z ==============================================================================
2019-10-10T03:25:31.0417139Z Task         : Get sources
2019-10-10T03:25:31.0417201Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
