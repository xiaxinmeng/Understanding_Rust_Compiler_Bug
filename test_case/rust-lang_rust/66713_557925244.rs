plain
2019-11-24T20:15:54.8987372Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T20:15:54.9159600Z ##[command]git config gc.auto 0
2019-11-24T20:15:54.9226721Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T20:15:54.9276474Z ##[command]git config --get-all http.proxy
2019-11-24T20:15:54.9397131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66713/merge:refs/remotes/pull/66713/merge
---
2019-11-24T20:21:36.0731654Z   = note: `#[warn(unused_imports)]` on by default
2019-11-24T20:21:36.0736532Z 
2019-11-24T20:21:45.9640375Z     Finished release [optimized] target(s) in 1m 19s
2019-11-24T20:21:45.9739195Z tidy check
2019-11-24T20:21:46.4733843Z tidy error: /checkout/src/librustc_target/spec/x86_64_unknown_hermit_kernel.rs:7: line longer than 100 chars
2019-11-24T20:21:48.3814502Z some tidy checks failed
2019-11-24T20:21:48.3814594Z Found 485 error codes
2019-11-24T20:21:48.3814669Z Found 0 error codes with no tests
2019-11-24T20:21:48.3814714Z Done!
2019-11-24T20:21:48.3814714Z Done!
2019-11-24T20:21:48.3814744Z 
2019-11-24T20:21:48.3814772Z 
2019-11-24T20:21:48.3815914Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T20:21:48.3816023Z 
2019-11-24T20:21:48.3816043Z 
2019-11-24T20:21:48.3819005Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T20:21:48.3819059Z Build completed unsuccessfully in 0:01:23
2019-11-24T20:21:48.3819059Z Build completed unsuccessfully in 0:01:23
2019-11-24T20:21:48.3869568Z == clock drift check ==
2019-11-24T20:21:48.3897552Z   local time: Sun Nov 24 20:21:48 UTC 2019
2019-11-24T20:21:48.4153390Z   network time: Sun, 24 Nov 2019 20:21:48 GMT
2019-11-24T20:21:48.4153511Z == end clock drift check ==
2019-11-24T20:21:49.7382959Z 
2019-11-24T20:21:49.7471907Z ##[error]Bash exited with code '1'.
2019-11-24T20:21:49.7498217Z ##[section]Starting: Checkout
2019-11-24T20:21:49.7499878Z ==============================================================================
2019-11-24T20:21:49.7499954Z Task         : Get sources
2019-11-24T20:21:49.7499992Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
