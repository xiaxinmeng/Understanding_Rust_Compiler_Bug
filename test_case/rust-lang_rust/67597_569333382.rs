plain
2019-12-27T19:22:03.5030083Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T19:22:03.5251422Z ##[command]git config gc.auto 0
2019-12-27T19:22:03.5333303Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T19:22:03.5402714Z ##[command]git config --get-all http.proxy
2019-12-27T19:22:03.5583082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67597/merge:refs/remotes/pull/67597/merge
---
2019-12-27T19:28:48.4213519Z Found 0 error codes with no tests
2019-12-27T19:28:48.4213715Z Done!
2019-12-27T19:28:48.4217042Z 
2019-12-27T19:28:48.4217297Z 
2019-12-27T19:28:48.4218527Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-27T19:28:48.4219022Z 
2019-12-27T19:28:48.4219174Z 
2019-12-27T19:28:48.4231501Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-27T19:28:48.4231957Z Build completed unsuccessfully in 0:01:40
2019-12-27T19:28:48.4231957Z Build completed unsuccessfully in 0:01:40
2019-12-27T19:28:48.4289320Z == clock drift check ==
2019-12-27T19:28:48.4307202Z   local time: Fri Dec 27 19:28:48 UTC 2019
2019-12-27T19:28:48.7100727Z   network time: Fri, 27 Dec 2019 19:28:48 GMT
2019-12-27T19:28:48.7105378Z == end clock drift check ==
2019-12-27T19:28:49.9818895Z 
2019-12-27T19:28:49.9930822Z ##[error]Bash exited with code '1'.
2019-12-27T19:28:49.9964538Z ##[section]Starting: Checkout
2019-12-27T19:28:49.9966762Z ==============================================================================
2019-12-27T19:28:49.9966827Z Task         : Get sources
2019-12-27T19:28:49.9966897Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
