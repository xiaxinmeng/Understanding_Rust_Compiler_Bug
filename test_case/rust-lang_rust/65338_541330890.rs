plain
2019-10-12T14:22:08.9784877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T14:22:09.0190060Z ##[command]git config gc.auto 0
2019-10-12T14:22:09.0286008Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T14:22:09.0352815Z ##[command]git config --get-all http.proxy
2019-10-12T14:22:09.0503491Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65338/merge:refs/remotes/pull/65338/merge
---
2019-10-12T14:28:52.7363120Z    Compiling serde_json v1.0.40
2019-10-12T14:28:54.5328277Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-12T14:29:05.9093288Z     Finished release [optimized] target(s) in 1m 28s
2019-10-12T14:29:05.9172467Z tidy check
2019-10-12T14:29:06.0458722Z tidy error: /checkout/src/libsyntax_ext/format.rs:452: line longer than 100 chars
2019-10-12T14:29:08.0318558Z some tidy checks failed
2019-10-12T14:29:08.0322588Z Found 482 error codes
2019-10-12T14:29:08.0323201Z Found 0 error codes with no tests
2019-10-12T14:29:08.0323564Z Done!
2019-10-12T14:29:08.0323564Z Done!
2019-10-12T14:29:08.0323776Z 
2019-10-12T14:29:08.0323968Z 
2019-10-12T14:29:08.0325418Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-12T14:29:08.0326134Z 
2019-10-12T14:29:08.0326312Z 
2019-10-12T14:29:08.0329777Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-12T14:29:08.0330049Z Build completed unsuccessfully in 0:01:31
2019-10-12T14:29:08.0330049Z Build completed unsuccessfully in 0:01:31
2019-10-12T14:29:08.0379116Z == clock drift check ==
2019-10-12T14:29:08.0397784Z   local time: Sat Oct 12 14:29:08 UTC 2019
2019-10-12T14:29:08.2010849Z   network time: Sat, 12 Oct 2019 14:29:08 GMT
2019-10-12T14:29:08.2014682Z == end clock drift check ==
2019-10-12T14:29:09.0684137Z ##[error]Bash exited with code '1'.
2019-10-12T14:29:09.0713567Z ##[section]Starting: Checkout
2019-10-12T14:29:09.0715810Z ==============================================================================
2019-10-12T14:29:09.0715877Z Task         : Get sources
2019-10-12T14:29:09.0715918Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
