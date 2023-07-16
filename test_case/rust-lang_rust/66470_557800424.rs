plain
2019-11-23T13:55:54.5147857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T13:55:54.5158368Z ##[command]git config gc.auto 0
2019-11-23T13:55:54.5161393Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T13:55:54.5163638Z ##[command]git config --get-all http.proxy
2019-11-23T13:55:54.5166371Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66470/merge:refs/remotes/pull/66470/merge
---
2019-11-23T14:01:03.5662326Z Found 0 error codes with no tests
2019-11-23T14:01:03.5662833Z Done!
2019-11-23T14:01:03.5663043Z 
2019-11-23T14:01:03.5663393Z 
2019-11-23T14:01:03.5664521Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-23T14:01:03.5665207Z 
2019-11-23T14:01:03.5665390Z 
2019-11-23T14:01:03.5665789Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-23T14:01:03.5666008Z Build completed unsuccessfully in 0:01:09
2019-11-23T14:01:03.5666008Z Build completed unsuccessfully in 0:01:09
2019-11-23T14:01:03.5707641Z == clock drift check ==
2019-11-23T14:01:03.5715743Z   local time: Sat Nov 23 14:01:03 UTC 2019
2019-11-23T14:01:04.1031824Z   network time: Sat, 23 Nov 2019 14:01:04 GMT
2019-11-23T14:01:04.1035107Z == end clock drift check ==
2019-11-23T14:01:05.4423815Z 
2019-11-23T14:01:05.4511568Z ##[error]Bash exited with code '1'.
2019-11-23T14:01:05.4535541Z ##[section]Starting: Checkout
2019-11-23T14:01:05.4536976Z ==============================================================================
2019-11-23T14:01:05.4537038Z Task         : Get sources
2019-11-23T14:01:05.4537074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
