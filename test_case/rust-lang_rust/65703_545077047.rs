plain
2019-10-22T16:21:31.7803739Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T16:21:31.8065067Z ##[command]git config gc.auto 0
2019-10-22T16:21:31.8168389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T16:21:31.8240218Z ##[command]git config --get-all http.proxy
2019-10-22T16:21:31.8380156Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65703/merge:refs/remotes/pull/65703/merge
---
2019-10-22T16:28:15.4882918Z Done!
2019-10-22T16:28:15.4882968Z some tidy checks failed
2019-10-22T16:28:15.4883003Z 
2019-10-22T16:28:15.4883033Z 
2019-10-22T16:28:15.4887129Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-22T16:28:15.4887585Z 
2019-10-22T16:28:15.4887649Z 
2019-10-22T16:28:15.4887817Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-22T16:28:15.4887880Z Build completed unsuccessfully in 0:01:34
2019-10-22T16:28:15.4887880Z Build completed unsuccessfully in 0:01:34
2019-10-22T16:28:15.4938015Z == clock drift check ==
2019-10-22T16:28:15.4947787Z   local time: Tue Oct 22 16:28:15 UTC 2019
2019-10-22T16:28:15.5908417Z   network time: Tue, 22 Oct 2019 16:28:15 GMT
2019-10-22T16:28:15.5908854Z == end clock drift check ==
2019-10-22T16:28:16.9418429Z 
2019-10-22T16:28:16.9533196Z ##[error]Bash exited with code '1'.
2019-10-22T16:28:16.9576630Z ##[section]Starting: Checkout
2019-10-22T16:28:16.9579185Z ==============================================================================
2019-10-22T16:28:16.9579277Z Task         : Get sources
2019-10-22T16:28:16.9579326Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
