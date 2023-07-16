plain
2019-10-24T15:14:54.4188000Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:14:54.4388392Z ##[command]git config gc.auto 0
2019-10-24T15:14:54.4463758Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:14:54.4525218Z ##[command]git config --get-all http.proxy
2019-10-24T15:14:54.4667285Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-24T15:20:49.7197907Z    Compiling serde_json v1.0.40
2019-10-24T15:20:51.5317491Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-24T15:21:03.0982246Z     Finished release [optimized] target(s) in 1m 28s
2019-10-24T15:21:03.1065424Z tidy check
2019-10-24T15:21:03.5830460Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics/caller_location.rs:23: TODO is deprecated; use FIXME
2019-10-24T15:21:03.5830610Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics/caller_location.rs:24: TODO is deprecated; use FIXME
2019-10-24T15:21:03.5830716Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics/caller_location.rs:25: TODO is deprecated; use FIXME
2019-10-24T15:21:03.5831041Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics/caller_location.rs:26: TODO is deprecated; use FIXME
2019-10-24T15:21:05.5044570Z some tidy checks failed
2019-10-24T15:21:05.5044725Z Found 483 error codes
2019-10-24T15:21:05.5044771Z Found 0 error codes with no tests
2019-10-24T15:21:05.5044813Z Done!
2019-10-24T15:21:05.5044813Z Done!
2019-10-24T15:21:05.5044841Z 
2019-10-24T15:21:05.5044883Z 
2019-10-24T15:21:05.5045663Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-24T15:21:05.5045785Z 
2019-10-24T15:21:05.5045811Z 
2019-10-24T15:21:05.5045859Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-24T15:21:05.5045936Z Build completed unsuccessfully in 0:01:32
2019-10-24T15:21:05.5045936Z Build completed unsuccessfully in 0:01:32
2019-10-24T15:21:05.5093518Z == clock drift check ==
2019-10-24T15:21:05.5106987Z   local time: Thu Oct 24 15:21:05 UTC 2019
2019-10-24T15:21:05.6743030Z   network time: Thu, 24 Oct 2019 15:21:05 GMT
2019-10-24T15:21:05.6746172Z == end clock drift check ==
2019-10-24T15:21:07.0829432Z 
2019-10-24T15:21:07.0960668Z ##[error]Bash exited with code '1'.
2019-10-24T15:21:07.1002026Z ##[section]Starting: Checkout
2019-10-24T15:21:07.1004209Z ==============================================================================
2019-10-24T15:21:07.1004280Z Task         : Get sources
2019-10-24T15:21:07.1004336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
