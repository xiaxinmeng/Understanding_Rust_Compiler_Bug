plain
2019-12-13T05:16:11.3928506Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-13T05:16:11.4154377Z ##[command]git config gc.auto 0
2019-12-13T05:16:11.4240956Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-13T05:16:11.4279734Z ##[command]git config --get-all http.proxy
2019-12-13T05:16:11.4538070Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67268/merge:refs/remotes/pull/67268/merge
---
2019-12-13T05:22:25.2692594Z    Compiling serde_json v1.0.40
2019-12-13T05:22:27.1627362Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-13T05:22:39.6109563Z     Finished release [optimized] target(s) in 1m 37s
2019-12-13T05:22:39.6223915Z tidy check
2019-12-13T05:22:39.9411750Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0222.md:45: line longer than 80 chars
2019-12-13T05:22:42.7330970Z Found 486 error codes
2019-12-13T05:22:42.7334155Z Found 0 error codes with no tests
2019-12-13T05:22:42.7334278Z Done!
2019-12-13T05:22:42.7338981Z some tidy checks failed
2019-12-13T05:22:42.7338981Z some tidy checks failed
2019-12-13T05:22:42.7352845Z 
2019-12-13T05:22:42.7352958Z 
2019-12-13T05:22:42.7354359Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-13T05:22:42.7355026Z 
2019-12-13T05:22:42.7355078Z 
2019-12-13T05:22:42.7372370Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-13T05:22:42.7372445Z Build completed unsuccessfully in 0:01:41
2019-12-13T05:22:42.7372445Z Build completed unsuccessfully in 0:01:41
2019-12-13T05:22:42.7461428Z == clock drift check ==
2019-12-13T05:22:42.7506767Z   local time: Fri Dec 13 05:22:42 UTC 2019
2019-12-13T05:22:43.0299867Z   network time: Fri, 13 Dec 2019 05:22:43 GMT
2019-12-13T05:22:43.0302748Z == end clock drift check ==
2019-12-13T05:22:44.3836961Z 
2019-12-13T05:22:44.3918244Z ##[error]Bash exited with code '1'.
2019-12-13T05:22:44.3955469Z ##[section]Starting: Checkout
2019-12-13T05:22:44.3957446Z ==============================================================================
2019-12-13T05:22:44.3957499Z Task         : Get sources
2019-12-13T05:22:44.3957541Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
