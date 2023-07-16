plain
2019-11-10T14:00:24.1126700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T14:00:24.1369736Z ##[command]git config gc.auto 0
2019-11-10T14:00:24.1460322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T14:00:24.1525609Z ##[command]git config --get-all http.proxy
2019-11-10T14:00:24.1674062Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66253/merge:refs/remotes/pull/66253/merge
---
2019-11-10T14:06:44.8867025Z    Compiling serde_json v1.0.40
2019-11-10T14:06:45.9749407Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-10T14:06:56.9164790Z     Finished release [optimized] target(s) in 1m 24s
2019-11-10T14:06:56.9201009Z tidy check
2019-11-10T14:06:58.1630535Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:2130: line longer than 80 chars
2019-11-10T14:06:59.7165803Z Found 485 error codes
2019-11-10T14:06:59.7165949Z Found 0 error codes with no tests
2019-11-10T14:06:59.7166031Z Done!
2019-11-10T14:06:59.7166080Z some tidy checks failed
2019-11-10T14:06:59.7166080Z some tidy checks failed
2019-11-10T14:06:59.7166149Z 
2019-11-10T14:06:59.7166179Z 
2019-11-10T14:06:59.7167117Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-10T14:06:59.7167255Z 
2019-11-10T14:06:59.7167284Z 
2019-11-10T14:06:59.7170021Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-10T14:06:59.7170403Z Build completed unsuccessfully in 0:01:28
2019-11-10T14:06:59.7170403Z Build completed unsuccessfully in 0:01:28
2019-11-10T14:06:59.7212346Z == clock drift check ==
2019-11-10T14:06:59.7232881Z   local time: Sun Nov 10 14:06:59 UTC 2019
2019-11-10T14:06:59.7938197Z   network time: Sun, 10 Nov 2019 14:06:59 GMT
2019-11-10T14:06:59.7938287Z == end clock drift check ==
2019-11-10T14:07:01.2616504Z 
2019-11-10T14:07:01.2733235Z ##[error]Bash exited with code '1'.
2019-11-10T14:07:01.2769031Z ##[section]Starting: Checkout
2019-11-10T14:07:01.2771159Z ==============================================================================
2019-11-10T14:07:01.2771246Z Task         : Get sources
2019-11-10T14:07:01.2771317Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
