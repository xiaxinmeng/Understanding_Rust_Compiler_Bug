plain
2019-11-24T13:42:51.0599714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T13:42:51.0793426Z ##[command]git config gc.auto 0
2019-11-24T13:42:51.0838981Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T13:42:51.0888856Z ##[command]git config --get-all http.proxy
2019-11-24T13:42:51.1037988Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66700/merge:refs/remotes/pull/66700/merge
---
2019-11-24T13:48:01.0915051Z   = note: `#[warn(unused_imports)]` on by default
2019-11-24T13:48:01.0915192Z 
2019-11-24T13:48:09.9446284Z     Finished release [optimized] target(s) in 1m 11s
2019-11-24T13:48:09.9529641Z tidy check
2019-11-24T13:48:10.1253835Z tidy error: /checkout/src/test/ui/unsized/unsized-fn-param.rs: too many trailing newlines (2)
2019-11-24T13:48:10.7141083Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:3760: line longer than 100 chars
2019-11-24T13:48:10.7141208Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:3792: line longer than 100 chars
2019-11-24T13:48:12.0061873Z some tidy checks failed
2019-11-24T13:48:12.0062587Z Found 485 error codes
2019-11-24T13:48:12.0062777Z Found 0 error codes with no tests
2019-11-24T13:48:12.0063082Z Done!
2019-11-24T13:48:12.0063082Z Done!
2019-11-24T13:48:12.0063211Z 
2019-11-24T13:48:12.0063313Z 
2019-11-24T13:48:12.0066097Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T13:48:12.0067008Z 
2019-11-24T13:48:12.0067133Z 
2019-11-24T13:48:12.0067330Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T13:48:12.0067604Z Build completed unsuccessfully in 0:01:14
2019-11-24T13:48:12.0067604Z Build completed unsuccessfully in 0:01:14
2019-11-24T13:48:12.0109775Z == clock drift check ==
2019-11-24T13:48:12.0123604Z   local time: Sun Nov 24 13:48:12 UTC 2019
2019-11-24T13:48:12.3026133Z   network time: Sun, 24 Nov 2019 13:48:12 GMT
2019-11-24T13:48:12.3037320Z == end clock drift check ==
2019-11-24T13:48:13.6371622Z 
2019-11-24T13:48:13.6450038Z ##[error]Bash exited with code '1'.
2019-11-24T13:48:13.6483597Z ##[section]Starting: Checkout
2019-11-24T13:48:13.6485397Z ==============================================================================
2019-11-24T13:48:13.6485442Z Task         : Get sources
2019-11-24T13:48:13.6485498Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
