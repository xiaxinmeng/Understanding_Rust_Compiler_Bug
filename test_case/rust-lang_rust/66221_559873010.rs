plain
2019-11-29T19:41:59.7065952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T19:41:59.7256592Z ##[command]git config gc.auto 0
2019-11-29T19:41:59.7339210Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T19:41:59.7395229Z ##[command]git config --get-all http.proxy
2019-11-29T19:41:59.7540947Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66221/merge:refs/remotes/pull/66221/merge
---
2019-11-29T19:48:07.6455943Z Found 0 error codes with no tests
2019-11-29T19:48:07.6456141Z Done!
2019-11-29T19:48:07.6456692Z 
2019-11-29T19:48:07.6456853Z 
2019-11-29T19:48:07.6460663Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-29T19:48:07.6460863Z 
2019-11-29T19:48:07.6460915Z 
2019-11-29T19:48:07.6460965Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-29T19:48:07.6461033Z Build completed unsuccessfully in 0:01:30
2019-11-29T19:48:07.6461033Z Build completed unsuccessfully in 0:01:30
2019-11-29T19:48:07.6522098Z == clock drift check ==
2019-11-29T19:48:07.6523517Z   local time: Fri Nov 29 19:48:07 UTC 2019
2019-11-29T19:48:07.6881495Z   network time: Fri, 29 Nov 2019 19:48:07 GMT
2019-11-29T19:48:07.6886511Z == end clock drift check ==
2019-11-29T19:48:09.0671336Z 
2019-11-29T19:48:09.0794770Z ##[error]Bash exited with code '1'.
2019-11-29T19:48:09.0823002Z ##[section]Starting: Checkout
2019-11-29T19:48:09.0825176Z ==============================================================================
2019-11-29T19:48:09.0825236Z Task         : Get sources
2019-11-29T19:48:09.0825271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
