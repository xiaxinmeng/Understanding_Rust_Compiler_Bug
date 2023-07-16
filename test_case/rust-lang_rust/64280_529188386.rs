plain
2019-09-08T09:46:30.8237702Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T09:46:30.8800402Z ##[command]git config gc.auto 0
2019-09-08T09:46:30.8880540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T09:46:30.8943818Z ##[command]git config --get-all http.proxy
2019-09-08T09:46:30.9079462Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64280/merge:refs/remotes/pull/64280/merge
---
2019-09-08T09:53:21.7750307Z    Compiling serde_json v1.0.40
2019-09-08T09:53:23.6180280Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-08T09:53:34.7037508Z     Finished release [optimized] target(s) in 1m 31s
2019-09-08T09:53:34.7129700Z tidy check
2019-09-08T09:53:35.0539837Z tidy error: /checkout/src/librustc_errors/lib.rs: too many trailing newlines (2)
2019-09-08T09:53:36.6891336Z some tidy checks failed
2019-09-08T09:53:36.6896554Z 
2019-09-08T09:53:36.6896554Z 
2019-09-08T09:53:36.6898143Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-08T09:53:36.6898791Z 
2019-09-08T09:53:36.6898834Z 
2019-09-08T09:53:36.6906693Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-08T09:53:36.6906764Z Build completed unsuccessfully in 0:01:35
2019-09-08T09:53:36.6906764Z Build completed unsuccessfully in 0:01:35
2019-09-08T09:53:36.6955145Z == clock drift check ==
2019-09-08T09:53:36.6980525Z   local time: Sun Sep  8 09:53:36 UTC 2019
2019-09-08T09:53:36.8466955Z   network time: Sun, 08 Sep 2019 09:53:36 GMT
2019-09-08T09:53:36.8469762Z == end clock drift check ==
2019-09-08T09:53:38.1680288Z ##[error]Bash exited with code '1'.
2019-09-08T09:53:38.1714530Z ##[section]Starting: Checkout
2019-09-08T09:53:38.1716270Z ==============================================================================
2019-09-08T09:53:38.1716502Z Task         : Get sources
2019-09-08T09:53:38.1716544Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
