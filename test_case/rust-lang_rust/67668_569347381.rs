plain
2019-12-27T20:58:06.1816763Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T20:58:06.7518889Z ##[command]git config gc.auto 0
2019-12-27T20:58:06.7522305Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T20:58:06.7525960Z ##[command]git config --get-all http.proxy
2019-12-27T20:58:06.7528983Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67668/merge:refs/remotes/pull/67668/merge
---
2019-12-27T21:04:12.5744776Z    Compiling serde_json v1.0.40
2019-12-27T21:04:14.1466881Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-27T21:04:24.7285968Z     Finished release [optimized] target(s) in 1m 21s
2019-12-27T21:04:24.7411710Z tidy check
2019-12-27T21:04:25.1067900Z tidy error: /checkout/src/test/mir-opt/exponential-or.rs:34: line longer than 100 chars
2019-12-27T21:04:27.4140627Z Found 486 error codes
2019-12-27T21:04:27.4141320Z Found 0 error codes with no tests
2019-12-27T21:04:27.4141784Z Done!
2019-12-27T21:04:27.4142248Z some tidy checks failed
2019-12-27T21:04:27.4142248Z some tidy checks failed
2019-12-27T21:04:27.4142427Z 
2019-12-27T21:04:27.4142597Z 
2019-12-27T21:04:27.4143767Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-27T21:04:27.4144334Z 
2019-12-27T21:04:27.4144501Z 
2019-12-27T21:04:27.4151125Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-27T21:04:27.4151684Z Build completed unsuccessfully in 0:01:31
2019-12-27T21:04:27.4151684Z Build completed unsuccessfully in 0:01:31
2019-12-27T21:04:27.4204466Z == clock drift check ==
2019-12-27T21:04:27.4214758Z   local time: Fri Dec 27 21:04:27 UTC 2019
2019-12-27T21:04:27.7139308Z   network time: Fri, 27 Dec 2019 21:04:27 GMT
2019-12-27T21:04:27.7143310Z == end clock drift check ==
2019-12-27T21:04:29.0841058Z 
2019-12-27T21:04:29.0948786Z ##[error]Bash exited with code '1'.
2019-12-27T21:04:29.0978656Z ##[section]Starting: Checkout
2019-12-27T21:04:29.0981259Z ==============================================================================
2019-12-27T21:04:29.0981305Z Task         : Get sources
2019-12-27T21:04:29.0981360Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
