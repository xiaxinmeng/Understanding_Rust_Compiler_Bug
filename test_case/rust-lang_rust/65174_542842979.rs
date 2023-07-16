plain
2019-10-16T18:34:54.3912255Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T18:34:54.4098898Z ##[command]git config gc.auto 0
2019-10-16T18:34:54.4185459Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T18:34:54.4250759Z ##[command]git config --get-all http.proxy
2019-10-16T18:34:55.0155008Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65174/merge:refs/remotes/pull/65174/merge
---
2019-10-16T18:42:14.4937529Z    Compiling serde_json v1.0.40
2019-10-16T18:42:16.4223505Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-16T18:42:28.6972252Z     Finished release [optimized] target(s) in 1m 36s
2019-10-16T18:42:28.7052623Z tidy check
2019-10-16T18:42:28.8372721Z tidy error: /checkout/src/liballoc/tests/boxed.rs: too many trailing newlines (2)
2019-10-16T18:42:30.8079203Z some tidy checks failed
2019-10-16T18:42:30.8079301Z Found 482 error codes
2019-10-16T18:42:30.8079355Z Found 0 error codes with no tests
2019-10-16T18:42:30.8079607Z Done!
2019-10-16T18:42:30.8079607Z Done!
2019-10-16T18:42:30.8079638Z 
2019-10-16T18:42:30.8079667Z 
2019-10-16T18:42:30.8080535Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-16T18:42:30.8080673Z 
2019-10-16T18:42:30.8080700Z 
2019-10-16T18:42:30.8086476Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-16T18:42:30.8087326Z Build completed unsuccessfully in 0:01:39
2019-10-16T18:42:30.8087326Z Build completed unsuccessfully in 0:01:39
2019-10-16T18:42:30.8137988Z == clock drift check ==
2019-10-16T18:42:30.8160231Z   local time: Wed Oct 16 18:42:30 UTC 2019
2019-10-16T18:42:30.9049248Z   network time: Wed, 16 Oct 2019 18:42:30 GMT
2019-10-16T18:42:30.9052471Z == end clock drift check ==
2019-10-16T18:42:32.2598861Z ##[error]Bash exited with code '1'.
2019-10-16T18:42:32.2633494Z ##[section]Starting: Checkout
2019-10-16T18:42:32.2635221Z ==============================================================================
2019-10-16T18:42:32.2635296Z Task         : Get sources
2019-10-16T18:42:32.2635347Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
