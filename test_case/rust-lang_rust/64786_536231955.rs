plain
2019-09-28T22:45:59.8867675Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T22:45:59.9094442Z ##[command]git config gc.auto 0
2019-09-28T22:45:59.9156346Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T22:45:59.9209083Z ##[command]git config --get-all http.proxy
2019-09-28T22:45:59.9364952Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64786/merge:refs/remotes/pull/64786/merge
---
2019-09-28T22:52:49.7870070Z    Compiling serde_json v1.0.40
2019-09-28T22:52:51.6063587Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-28T22:53:02.9371440Z     Finished release [optimized] target(s) in 1m 31s
2019-09-28T22:53:02.9428584Z tidy check
2019-09-28T22:53:03.4172243Z tidy error: /checkout/src/ci/docker/dist-x86_64-linux/build-curl.sh:8: line longer than 100 chars
2019-09-28T22:53:05.0770161Z some tidy checks failed
2019-09-28T22:53:05.0772879Z 
2019-09-28T22:53:05.0772879Z 
2019-09-28T22:53:05.0774240Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-28T22:53:05.0774804Z 
2019-09-28T22:53:05.0774915Z 
2019-09-28T22:53:05.0786251Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-28T22:53:05.0786639Z Build completed unsuccessfully in 0:01:34
2019-09-28T22:53:05.0786639Z Build completed unsuccessfully in 0:01:34
2019-09-28T22:53:05.0842867Z == clock drift check ==
2019-09-28T22:53:05.0860593Z   local time: Sat Sep 28 22:53:05 UTC 2019
2019-09-28T22:53:05.3636003Z   network time: Sat, 28 Sep 2019 22:53:05 GMT
2019-09-28T22:53:05.3638438Z == end clock drift check ==
2019-09-28T22:53:06.6905914Z ##[error]Bash exited with code '1'.
2019-09-28T22:53:06.6964985Z ##[section]Starting: Checkout
2019-09-28T22:53:06.6967399Z ==============================================================================
2019-09-28T22:53:06.6967466Z Task         : Get sources
2019-09-28T22:53:06.6967758Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
