plain
2019-07-30T06:57:26.3732429Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T06:57:26.3953300Z ##[command]git config gc.auto 0
2019-07-30T06:57:26.4017584Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T06:57:26.4080295Z ##[command]git config --get-all http.proxy
2019-07-30T06:57:26.4221016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-30T06:58:02.2299652Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T06:58:02.2300945Z 
2019-07-30T06:58:02.2302653Z   git checkout -b <new-branch-name>
2019-07-30T06:58:02.2303934Z 
2019-07-30T06:58:02.2304917Z HEAD is now at e55e4c793 Merge 021dd91eb555c0a76aa04ba0b6215e60bb1ef089 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-30T06:58:02.2454338Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T06:58:02.2456664Z ==============================================================================
2019-07-30T06:58:02.2456711Z Task         : Bash
2019-07-30T06:58:02.2456767Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T07:04:32.8220944Z    Compiling serde_json v1.0.40
2019-07-30T07:04:37.4736272Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-30T07:04:46.6460225Z     Finished release [optimized] target(s) in 1m 37s
2019-07-30T07:04:46.6531541Z tidy check
2019-07-30T07:04:46.8160441Z tidy error: /checkout/src/libsyntax/mut_visit.rs:561: line longer than 100 chars
2019-07-30T07:04:48.5531265Z some tidy checks failed
2019-07-30T07:04:48.5531481Z 
2019-07-30T07:04:48.5531481Z 
2019-07-30T07:04:48.5532365Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-30T07:04:48.5532512Z 
2019-07-30T07:04:48.5532539Z 
2019-07-30T07:04:48.5543859Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-30T07:04:48.5543952Z Build completed unsuccessfully in 0:01:40
2019-07-30T07:04:48.5543952Z Build completed unsuccessfully in 0:01:40
2019-07-30T07:04:49.8956785Z ##[error]Bash exited with code '1'.
2019-07-30T07:04:49.9015654Z ##[section]Starting: Checkout
2019-07-30T07:04:49.9017223Z ==============================================================================
2019-07-30T07:04:49.9017275Z Task         : Get sources
2019-07-30T07:04:49.9017340Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
