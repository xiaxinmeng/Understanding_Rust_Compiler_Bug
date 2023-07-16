plain
2019-10-10T23:24:36.0570421Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T23:24:36.0646226Z ##[command]git config gc.auto 0
2019-10-10T23:24:36.0720616Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T23:24:36.0783111Z ##[command]git config --get-all http.proxy
2019-10-10T23:24:36.0909889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65288/merge:refs/remotes/pull/65288/merge
---
2019-10-10T23:31:02.3275216Z    Compiling serde_json v1.0.40
2019-10-10T23:31:04.1398343Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-10T23:31:16.2844646Z     Finished release [optimized] target(s) in 1m 31s
2019-10-10T23:31:16.2974812Z tidy check
2019-10-10T23:31:17.0656798Z tidy error: /checkout/src/librustc/ty/wf.rs:191: trailing whitespace
2019-10-10T23:31:18.4978217Z Found 482 error codes
2019-10-10T23:31:18.4979193Z Found 0 error codes with no tests
2019-10-10T23:31:18.4979367Z Done!
2019-10-10T23:31:18.4979501Z some tidy checks failed
2019-10-10T23:31:18.4979501Z some tidy checks failed
2019-10-10T23:31:18.4987273Z 
2019-10-10T23:31:18.4987540Z 
2019-10-10T23:31:18.4988510Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-10T23:31:18.4988910Z 
2019-10-10T23:31:18.4989034Z 
2019-10-10T23:31:18.4990409Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-10T23:31:18.4990597Z Build completed unsuccessfully in 0:01:34
2019-10-10T23:31:18.4990597Z Build completed unsuccessfully in 0:01:34
2019-10-10T23:31:18.5037578Z == clock drift check ==
2019-10-10T23:31:18.5053966Z   local time: Thu Oct 10 23:31:18 UTC 2019
2019-10-10T23:31:18.5889103Z   network time: Thu, 10 Oct 2019 23:31:18 GMT
2019-10-10T23:31:18.5893741Z == end clock drift check ==
2019-10-10T23:31:19.3889765Z ##[error]Bash exited with code '1'.
2019-10-10T23:31:19.3947776Z ##[section]Starting: Checkout
2019-10-10T23:31:19.3949176Z ==============================================================================
2019-10-10T23:31:19.3949366Z Task         : Get sources
2019-10-10T23:31:19.3949402Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
