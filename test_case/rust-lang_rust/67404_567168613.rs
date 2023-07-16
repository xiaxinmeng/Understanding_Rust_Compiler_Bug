plain
2019-12-18T18:49:34.0647450Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T18:49:34.0847815Z ##[command]git config gc.auto 0
2019-12-18T18:49:34.0920917Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T18:49:34.0976819Z ##[command]git config --get-all http.proxy
2019-12-18T18:49:35.0243719Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67404/merge:refs/remotes/pull/67404/merge
---
2019-12-18T18:55:06.1277805Z    Compiling serde_json v1.0.40
2019-12-18T18:55:06.7813251Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-18T18:55:16.0068410Z     Finished release [optimized] target(s) in 1m 13s
2019-12-18T18:55:16.0155703Z tidy check
2019-12-18T18:55:16.8357954Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:1596: TODO is deprecated; use FIXME
2019-12-18T18:55:16.8358123Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:1597: TODO is deprecated; use FIXME
2019-12-18T18:55:16.8358410Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:1639: TODO is deprecated; use FIXME
2019-12-18T18:55:18.3299595Z Found 485 error codes
2019-12-18T18:55:18.3299895Z Found 0 error codes with no tests
2019-12-18T18:55:18.3299969Z Done!
2019-12-18T18:55:18.3300010Z some tidy checks failed
2019-12-18T18:55:18.3300010Z some tidy checks failed
2019-12-18T18:55:18.3300038Z 
2019-12-18T18:55:18.3300063Z 
2019-12-18T18:55:18.3300874Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-18T18:55:18.3300969Z 
2019-12-18T18:55:18.3300993Z 
2019-12-18T18:55:18.3306749Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-18T18:55:18.3306814Z Build completed unsuccessfully in 0:01:17
2019-12-18T18:55:18.3306814Z Build completed unsuccessfully in 0:01:17
2019-12-18T18:55:18.3354475Z == clock drift check ==
2019-12-18T18:55:18.3382896Z   local time: Wed Dec 18 18:55:18 UTC 2019
2019-12-18T18:55:18.6017409Z   network time: Wed, 18 Dec 2019 18:55:18 GMT
2019-12-18T18:55:18.6023144Z == end clock drift check ==
2019-12-18T18:55:19.9734533Z 
2019-12-18T18:55:19.9831044Z ##[error]Bash exited with code '1'.
2019-12-18T18:55:19.9862250Z ##[section]Starting: Checkout
2019-12-18T18:55:19.9864077Z ==============================================================================
2019-12-18T18:55:19.9864141Z Task         : Get sources
2019-12-18T18:55:19.9864182Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
