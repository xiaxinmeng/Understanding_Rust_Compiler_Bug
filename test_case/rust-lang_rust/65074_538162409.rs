plain
2019-10-03T22:45:50.7129455Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T22:45:50.7361669Z ##[command]git config gc.auto 0
2019-10-03T22:45:50.7487592Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T22:45:50.7494507Z ##[command]git config --get-all http.proxy
2019-10-03T22:45:50.7619058Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65074/merge:refs/remotes/pull/65074/merge
---
2019-10-03T22:53:00.2779451Z    Compiling serde_json v1.0.40
2019-10-03T22:53:02.0718808Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-03T22:53:13.2074542Z     Finished release [optimized] target(s) in 1m 28s
2019-10-03T22:53:13.2150912Z tidy check
2019-10-03T22:53:14.1294428Z tidy error: /checkout/src/libsyntax/tests.rs: too many trailing newlines (2)
2019-10-03T22:53:15.2836883Z some tidy checks failed
2019-10-03T22:53:15.2843409Z 
2019-10-03T22:53:15.2843409Z 
2019-10-03T22:53:15.2845972Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T22:53:15.2846148Z 
2019-10-03T22:53:15.2846173Z 
2019-10-03T22:53:15.2854873Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T22:53:15.2854971Z Build completed unsuccessfully in 0:01:31
2019-10-03T22:53:15.2854971Z Build completed unsuccessfully in 0:01:31
2019-10-03T22:53:15.2900159Z == clock drift check ==
2019-10-03T22:53:15.2928628Z   local time: Thu Oct  3 22:53:15 UTC 2019
2019-10-03T22:53:15.4439225Z   network time: Thu, 03 Oct 2019 22:53:15 GMT
2019-10-03T22:53:15.4439311Z == end clock drift check ==
2019-10-03T22:53:16.8120859Z ##[error]Bash exited with code '1'.
2019-10-03T22:53:16.8151552Z ##[section]Starting: Checkout
2019-10-03T22:53:16.8154038Z ==============================================================================
2019-10-03T22:53:16.8154095Z Task         : Get sources
2019-10-03T22:53:16.8154160Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
