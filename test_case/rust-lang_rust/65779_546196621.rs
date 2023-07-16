plain
2019-10-25T04:12:52.0720376Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T04:12:52.0969585Z ##[command]git config gc.auto 0
2019-10-25T04:12:52.1028540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T04:12:52.1093215Z ##[command]git config --get-all http.proxy
2019-10-25T04:12:52.1238746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65779/merge:refs/remotes/pull/65779/merge
---
2019-10-25T04:19:19.9705752Z    Compiling serde_json v1.0.40
2019-10-25T04:19:21.9218351Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-25T04:19:34.2667458Z     Finished release [optimized] target(s) in 1m 35s
2019-10-25T04:19:34.2750378Z tidy check
2019-10-25T04:19:35.0204366Z tidy error: /checkout/src/librustc/infer/error_reporting/mod.rs:1078: trailing whitespace
2019-10-25T04:19:36.7381460Z Found 483 error codes
2019-10-25T04:19:36.7384281Z Found 0 error codes with no tests
2019-10-25T04:19:36.7384342Z Done!
2019-10-25T04:19:36.7384409Z some tidy checks failed
2019-10-25T04:19:36.7384409Z some tidy checks failed
2019-10-25T04:19:36.7389978Z 
2019-10-25T04:19:36.7390306Z 
2019-10-25T04:19:36.7391372Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-25T04:19:36.7391804Z 
2019-10-25T04:19:36.7392089Z 
2019-10-25T04:19:36.7392249Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-25T04:19:36.7392389Z Build completed unsuccessfully in 0:01:38
2019-10-25T04:19:36.7392389Z Build completed unsuccessfully in 0:01:38
2019-10-25T04:19:36.7433010Z == clock drift check ==
2019-10-25T04:19:36.7445631Z   local time: Fri Oct 25 04:19:36 UTC 2019
2019-10-25T04:19:36.8322323Z   network time: Fri, 25 Oct 2019 04:19:36 GMT
2019-10-25T04:19:36.8332388Z == end clock drift check ==
2019-10-25T04:19:38.1834387Z 
2019-10-25T04:19:38.1945197Z ##[error]Bash exited with code '1'.
2019-10-25T04:19:38.1979662Z ##[section]Starting: Checkout
2019-10-25T04:19:38.1981736Z ==============================================================================
2019-10-25T04:19:38.1981789Z Task         : Get sources
2019-10-25T04:19:38.1981833Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
