plain
2019-11-07T04:41:49.3434349Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-07T04:41:49.3618220Z ##[command]git config gc.auto 0
2019-11-07T04:41:49.3718078Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-07T04:41:49.3769717Z ##[command]git config --get-all http.proxy
2019-11-07T04:41:49.3947900Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66178/merge:refs/remotes/pull/66178/merge
---
2019-11-07T04:47:55.1645879Z    Compiling serde_json v1.0.40
2019-11-07T04:47:56.9925172Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-07T04:48:08.5280568Z     Finished release [optimized] target(s) in 1m 28s
2019-11-07T04:48:08.5360347Z tidy check
2019-11-07T04:48:08.7378949Z tidy error: /checkout/src/test/ui/impl-trait/type-alias-generic-param.rs: too many trailing newlines (2)
2019-11-07T04:48:10.9980448Z Found 485 error codes
2019-11-07T04:48:10.9980550Z Found 0 error codes with no tests
2019-11-07T04:48:10.9980653Z Done!
2019-11-07T04:48:10.9980687Z some tidy checks failed
2019-11-07T04:48:10.9980687Z some tidy checks failed
2019-11-07T04:48:10.9980709Z 
2019-11-07T04:48:10.9980743Z 
2019-11-07T04:48:10.9981526Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-07T04:48:10.9981644Z 
2019-11-07T04:48:10.9981863Z 
2019-11-07T04:48:10.9988414Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-07T04:48:10.9988486Z Build completed unsuccessfully in 0:01:31
2019-11-07T04:48:10.9988486Z Build completed unsuccessfully in 0:01:31
2019-11-07T04:48:11.0044292Z == clock drift check ==
2019-11-07T04:48:11.0060534Z   local time: Thu Nov  7 04:48:11 UTC 2019
2019-11-07T04:48:11.1553233Z   network time: Thu, 07 Nov 2019 04:48:11 GMT
2019-11-07T04:48:11.1553339Z == end clock drift check ==
2019-11-07T04:48:12.4517293Z 
2019-11-07T04:48:12.4594158Z ##[error]Bash exited with code '1'.
2019-11-07T04:48:12.4620764Z ##[section]Starting: Checkout
2019-11-07T04:48:12.4622275Z ==============================================================================
2019-11-07T04:48:12.4622317Z Task         : Get sources
2019-11-07T04:48:12.4622353Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
