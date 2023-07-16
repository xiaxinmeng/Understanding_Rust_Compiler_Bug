plain
2019-09-01T10:38:58.4577115Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T10:38:58.4777073Z ##[command]git config gc.auto 0
2019-09-01T10:38:58.4863700Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T10:38:58.4940699Z ##[command]git config --get-all http.proxy
2019-09-01T10:38:58.5070235Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64058/merge:refs/remotes/pull/64058/merge
---
2019-09-01T10:45:47.3218053Z    Compiling serde_json v1.0.40
2019-09-01T10:45:49.1574047Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-01T10:46:00.1333529Z     Finished release [optimized] target(s) in 1m 29s
2019-09-01T10:46:00.1409836Z tidy check
2019-09-01T10:46:00.4952235Z tidy error: /checkout/src/librustc_errors/emitter.rs:205: line longer than 100 chars
2019-09-01T10:46:02.0958885Z some tidy checks failed
2019-09-01T10:46:02.8902564Z 
2019-09-01T10:46:02.8902564Z 
2019-09-01T10:46:02.8903667Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-01T10:46:02.8905034Z 
2019-09-01T10:46:02.8905139Z 
2019-09-01T10:46:02.8905336Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-01T10:46:02.8905696Z Build completed unsuccessfully in 0:01:33
2019-09-01T10:46:02.8905696Z Build completed unsuccessfully in 0:01:33
2019-09-01T10:46:02.8905889Z == clock drift check ==
2019-09-01T10:46:02.8905966Z   local time: Sun Sep  1 10:46:02 UTC 2019
2019-09-01T10:46:02.8906042Z   network time: Sun, 01 Sep 2019 10:46:02 GMT
2019-09-01T10:46:02.8906089Z == end clock drift check ==
2019-09-01T10:46:03.6935872Z ##[error]Bash exited with code '1'.
2019-09-01T10:46:03.6967378Z ##[section]Starting: Checkout
2019-09-01T10:46:03.6968914Z ==============================================================================
2019-09-01T10:46:03.6968984Z Task         : Get sources
2019-09-01T10:46:03.6969031Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
