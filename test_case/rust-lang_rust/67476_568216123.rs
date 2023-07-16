plain
2019-12-21T22:06:51.8243112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-21T22:06:51.8475870Z ##[command]git config gc.auto 0
2019-12-21T22:06:51.8556135Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-21T22:06:51.8609587Z ##[command]git config --get-all http.proxy
2019-12-21T22:06:51.8779702Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-21T22:12:42.3602521Z    Compiling serde_json v1.0.40
2019-12-21T22:12:44.0514068Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-21T22:12:55.3490920Z     Finished release [optimized] target(s) in 1m 29s
2019-12-21T22:12:55.3600689Z tidy check
2019-12-21T22:12:56.2763869Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1595: TODO is deprecated; use FIXME
2019-12-21T22:12:56.2850115Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:178: TODO is deprecated; use FIXME
2019-12-21T22:12:56.2877761Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:758: TODO is deprecated; use FIXME
2019-12-21T22:12:58.0726423Z some tidy checks failed
2019-12-21T22:12:58.0727603Z Found 485 error codes
2019-12-21T22:12:58.0727931Z Found 0 error codes with no tests
2019-12-21T22:12:58.0728178Z Done!
2019-12-21T22:12:58.0728178Z Done!
2019-12-21T22:12:58.0734523Z 
2019-12-21T22:12:58.0734877Z 
2019-12-21T22:12:58.0736210Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-21T22:12:58.0736457Z 
2019-12-21T22:12:58.0741585Z 
2019-12-21T22:12:58.0742138Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-21T22:12:58.0742517Z Build completed unsuccessfully in 0:01:33
2019-12-21T22:12:58.0742517Z Build completed unsuccessfully in 0:01:33
2019-12-21T22:12:58.0804941Z == clock drift check ==
2019-12-21T22:12:58.0812282Z   local time: Sat Dec 21 22:12:58 UTC 2019
2019-12-21T22:12:58.2347134Z   network time: Sat, 21 Dec 2019 22:12:58 GMT
2019-12-21T22:12:58.2350390Z == end clock drift check ==
2019-12-21T22:12:59.9709110Z 
2019-12-21T22:12:59.9819200Z ##[error]Bash exited with code '1'.
2019-12-21T22:12:59.9847041Z ##[section]Starting: Checkout
2019-12-21T22:12:59.9848656Z ==============================================================================
2019-12-21T22:12:59.9848705Z Task         : Get sources
2019-12-21T22:12:59.9848745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
